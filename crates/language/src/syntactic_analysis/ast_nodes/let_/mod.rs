use std::iter::{Peekable, Iterator};
use std::rc::Rc;

use zsft::{SetElement, Set, SetDefinition};

use crate::ScopedItem;
use crate::lexical_analysis::TokenType;
use crate::lexical_analysis::Token;
use crate::syntactic_analysis::ast::{NodeParseError, NodeVisitationError};
use crate::scope::Scope;

use super::Identifier;
use super::MathExpression;
use super::common::{IdentifierList, SetLiteralElement};
use super::common::SetLiteral;
use super::common::StructSignature;


/// ## Possible Syntaxes
/// 
/// `let S be {...}`
/// `let a, b in S`
/// `let c = a + b`
/// `let (F; +, *, 0, 1) be Field {...}`
pub enum Let {
    ElementCreationInSet(IdentifierList, Identifier),
    ElementCreationAsExpression(IdentifierList, MathExpression),
    SetCreation(IdentifierList, SetLiteral),
    StructInstantiation(StructSignature, Identifier, SetLiteral),
}

impl Let {
    pub fn new<'a, T: Iterator<Item = Token>>(tokens: &'a mut Peekable<T>) -> Result<Box<Self>, NodeParseError> {

        match tokens.peek() {
            Some(&Token {type_: TokenType::Identifier(_), ..}) => {
                Self::_new_from_identifier_leading_token(tokens)
            },
            Some(&Token {type_: TokenType::LeftParen, ..}) => {
                todo!("Structs in progress");
                Self::_new_struct_instantiation(tokens)
            },
            Some(x) => return Err(NodeParseError::UnexpectedToken(x.to_owned(), vec![TokenType::Identifier(vec![]), TokenType::LeftParen])),
            None => return Err(NodeParseError::UnexpectedEndOfInput),
        }
    }

    pub fn visit<'a, 'b: 'a>(self, scope: &'a mut Scope) -> Result<(), NodeVisitationError> {
        match self {
            Self::ElementCreationInSet(element_identifiers, set_identifier) => {
                Self::_visit_element_creation_in_set(scope, element_identifiers, set_identifier)
            },
            Self::ElementCreationAsExpression(element_identifiers, expression) => {
                Self::_visit_element_creation_as_expression(scope, element_identifiers, expression)
            },
            Self::SetCreation(set_identifiers, set_literal) => {
                Self::_visit_set_creation(scope, set_identifiers, set_literal)
            },
            Self::StructInstantiation(struct_signature, struct_identifier, set_literal) => {
                Self::_visit_struct_instantiation(scope, struct_signature, struct_identifier, set_literal)
            },
        }
    }

    #[inline]
    fn _new_from_identifier_leading_token<'a, T: Iterator<Item = Token>>(tokens: &'a mut Peekable<T>) -> Result<Box<Self>, NodeParseError> {
        let identifiers = *IdentifierList::new(tokens)?;

        match tokens.next() {
            Some(Token { type_: TokenType::Be, ..}) => Ok(
                Box::new(
                    Self::SetCreation(identifiers, *SetLiteral::new(tokens)?)
                )
            ),
            Some(Token { type_: TokenType::In, ..}) => Ok(
                Box::new(
                    Self::ElementCreationInSet(identifiers, *Identifier::new(tokens)?)
                )
            ),
            Some(Token { type_: TokenType::Equality, ..}) => Ok(
                Box::new(
                    Self::ElementCreationAsExpression(identifiers, *MathExpression::new(tokens)?)
                )
            ),

            Some(x) => return Err(NodeParseError::UnexpectedToken(x.to_owned(), vec![TokenType::In, TokenType::Be])),
            None => return Err(NodeParseError::UnexpectedEndOfInput),
        }
    }

    #[inline]
    fn _new_struct_instantiation<'a, T: Iterator<Item = Token>>(tokens: &'a mut Peekable<T>) -> Result<Box<Self>, NodeParseError> {
        Ok(
            Box::new(
                Self::StructInstantiation(
                    StructSignature::new(tokens)?,
                    *Identifier::new(tokens)?,
                    *SetLiteral::new(tokens)?,
                )
            )
        )
    }

    #[inline]
    fn _visit_element_creation_in_set<'a>(scope: &'a mut Scope, element_identifiers: IdentifierList, set_identifier: Identifier) -> Result<(), NodeVisitationError> {

        let containing_set = match scope.get(&set_identifier.lexeme) {
            Some(x) => match &*x {
                ScopedItem::Set(set) => set.clone(),
                x => return Err(NodeVisitationError::UnexpectedRegisteredItem(x.to_owned())),
            },
            None => return Err(NodeVisitationError::CantResolveToken(String::from_utf8_lossy(&set_identifier.lexeme).to_string())),
        };


        for element in element_identifiers.identifiers {
            match scope.add(
                element.lexeme.to_vec(),
                ScopedItem::SetElement(SetElement::element_of(&containing_set))
            ) {
                Ok(()) => {},
                Err(x) => return Err(NodeVisitationError::ItemAlreadyExists(x.to_owned())),
            }
        };

        Ok(())
    }

    #[inline]
    fn _visit_element_creation_as_expression<'a>(scope: &'a mut Scope, element_identifiers: IdentifierList, expression: MathExpression) -> Result<(), NodeVisitationError> {
        for element in element_identifiers.identifiers {

            let expression: lazymath::core::Expression = expression.clone().into_expression(&scope)?;

            match scope.add(
                element.lexeme,
                ScopedItem::Expression(Rc::new(expression))
            ) {
                Ok(()) => {},
                Err(x) => return Err(NodeVisitationError::ItemAlreadyExists(x.to_owned())),
            }
        };

        Ok(())
    }

    #[inline]
    fn _visit_set_creation<'a>(scope: &'a mut Scope, set_identifiers: IdentifierList, set_literal: SetLiteral) -> Result<(), NodeVisitationError> {
        
        let mut is_set_anonymous = false;
        let mut elements_in_set: Vec<SetElement> = vec![];

        for set_literal_element in set_literal.elements {
            match set_literal_element {
                SetLiteralElement::Identifier(id) => {
                    match scope.get(&id.lexeme) {
                        Some(ScopedItem::SetElement(x)) => {elements_in_set.push(x.clone())},
                        Some(_) => return Err(NodeVisitationError::Custom("Expected set element")),
                        None => return Err(NodeVisitationError::CantResolveToken(String::from_utf8(id.lexeme).unwrap()))
                    };
                },
                SetLiteralElement::Spread => {
                    is_set_anonymous = true;
                },
                SetLiteralElement::Close => {
                    return Err(NodeVisitationError::Custom("Can't use close syntax in set definition"))
                },
            }
        };

        let set_definition = match (is_set_anonymous, elements_in_set.len()) {
            (true, 0) => SetDefinition::Anonymous,
            (true, _) => SetDefinition::AnonymousWithElement(elements_in_set),
            (false, 0) => SetDefinition::Empty,
            (false, _) => SetDefinition::FromElements(elements_in_set),
        };

        for identifier in set_identifiers.identifiers {
            match scope.add(identifier.lexeme, ScopedItem::Set(Set::new(set_definition.clone()))) {
                Err(x) => return Err(NodeVisitationError::ItemAlreadyExists(x.to_owned())),
                Ok(()) => {},
            }
        };

        Ok(())
    }

    #[inline]
    fn _visit_struct_instantiation<'a>(scope: &'a mut Scope, struct_signature: StructSignature, struct_identifier: Identifier, set_literal: SetLiteral) -> Result<(), NodeVisitationError> {

        todo!("Structs in progress");

        let structure = match scope.get(&struct_identifier.lexeme) {
            Some(ScopedItem::Structure(s)) => s.instantiate(vec![]).unwrap(),
            Some(x) => return Err(NodeVisitationError::TokenOfWrongType(struct_identifier.lexeme, x.to_owned())),
            None => return Err(NodeVisitationError::CantResolveToken(String::from_utf8_lossy(&struct_identifier.lexeme).to_string()))
        };

        struct_signature.bind_struct_to_scope(structure, scope)?;
        Ok(())
    }
}