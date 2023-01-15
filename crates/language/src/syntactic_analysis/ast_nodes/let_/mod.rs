use std::iter::{Peekable, Iterator};
use std::rc::Rc;

use lazymath::abstract_algebra::MathStructure;
use zsft::{SetElement, Set, Item, WithItems, HasSize};

use crate::ScopedItem;
use crate::lexical_analysis::TokenType;
use crate::lexical_analysis::Token;
use crate::syntactic_analysis::ast::{NodeParseError, NodeVisitationError};
use crate::scope::Scope;

use super::{Identifier, identifier, expect_token};
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
    ItemCreation(IdentifierList),
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
            Self::ItemCreation(item_identifiers) => {
                Self::_visit_item_creation(scope, item_identifiers)
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
            Some(Token { type_: TokenType::Newline, ..}) => Ok(
                Box::new(
                    Self::ItemCreation(identifiers)
                )
            ),
            Some(x) => return Err(NodeParseError::UnexpectedToken(x.to_owned(), vec![TokenType::In, TokenType::Be])),
            None => return Err(NodeParseError::UnexpectedEndOfInput),
        }
    }

    #[inline]
    fn _new_struct_instantiation<'a, T: Iterator<Item = Token>>(tokens: &'a mut Peekable<T>) -> Result<Box<Self>, NodeParseError> {
        let signature = StructSignature::new(tokens)?;

        expect_token!(tokens, Be);

        let struct_identifier = *Identifier::new(tokens)?;
        let set_literal = *SetLiteral::new(tokens)?;
        Ok(
            Box::new(
                Self::StructInstantiation(
                    signature,
                    struct_identifier,
                    set_literal,
                )
            )
        )
    }

    fn _visit_item_creation<'a>(scope: &'a mut Scope, item_identifiers: IdentifierList) -> Result<(), NodeVisitationError> {
        for item in item_identifiers.identifiers {
            match scope.add(
                item.lexeme.to_vec(),
                ScopedItem::Item(Item::new()),
            ) {
                Ok(()) => {},
                Err(x) => return Err(NodeVisitationError::ItemAlreadyExists(x.to_owned())),
            }
        };
        Ok(())
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
        let mut items_in_set: Vec<&Item> = vec![];

        for set_literal_element in set_literal.elements {
            match set_literal_element {
                SetLiteralElement::Identifier(id) => {
                    match scope.get(&id.lexeme) {
                        Some(ScopedItem::Item(x)) => {items_in_set.push(x)},
                        Some(_) => return Err(NodeVisitationError::Custom("Expected Item")),
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

        let new_set = Set::anonymous();
        

        match (is_set_anonymous, items_in_set.len()) {
            (true, 0) => {},
            (true, _) => WithItems::assert_on(&new_set, items_in_set).expect(),
            (false, 0) => HasSize::assert_on(zsft::NumBound::Eq(zsft::Number::Ordinal(0)), &new_set).expect(),
            (false, x) => {
                WithItems::assert_on(&new_set, items_in_set).expect();
                HasSize::assert_on(zsft::NumBound::Eq(zsft::Number::Ordinal(x)), &new_set).expect();
            },
        };

        for identifier in set_identifiers.identifiers {
            match scope.add(identifier.lexeme, ScopedItem::Set(new_set.clone())) {
                Err(x) => return Err(NodeVisitationError::ItemAlreadyExists(x.to_owned())),
                Ok(()) => {},
            }
        };

        Ok(())
    }

    #[inline]
    fn _visit_struct_instantiation<'a>(scope: &'a mut Scope, struct_signature: StructSignature, struct_identifier: Identifier, set_literal: SetLiteral) -> Result<(), NodeVisitationError> {
        let structure: &Rc<MathStructure> = match scope.get(&struct_identifier.lexeme) {
            Some(ScopedItem::Structure(s)) => s,
            Some(x) => return Err(NodeVisitationError::TokenOfWrongType(struct_identifier.lexeme, x.to_owned())),
            None => return Err(NodeVisitationError::CantResolveToken(String::from_utf8_lossy(&struct_identifier.lexeme).to_string()))
        };

        let structure_instance =
            structure.instantiate(Vec::new())
                .expect("Failed to instantiate");

        struct_signature.bind_struct_to_scope(structure_instance, scope)?;
        Ok(())
    }
}