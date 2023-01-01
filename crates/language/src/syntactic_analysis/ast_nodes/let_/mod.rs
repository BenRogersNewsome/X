use std::iter::{Peekable, Iterator};

use crate::lexical_analysis::TokenType;
use crate::lexical_analysis::Token;
use crate::syntactic_analysis::ast::{NodeParseError, NodeVisitationError};
use crate::scope::Scope;

mod set_element_creation;
mod struct_creation;
mod expression_assignment;

use set_element_creation::ElementCreation;

use self::struct_creation::StructCreation;

use super::common::IdentifierList;
use super::common::StructSignature;

pub enum LetAssignment {
    Element(ElementCreation),
    Struct(StructCreation),
}

pub enum LetBinding {
    Identifiers(IdentifierList),
    StructSignature(StructSignature),
}

struct Let {
    binding: LetBinding,
    assignment: LetAssignment,
}

impl Let {
    pub fn new<'a, T: Iterator<Item = Token>>(tokens: &'a mut Peekable<T>) -> Result<Box<Self>, NodeParseError> {

        let binding: LetBinding = match tokens.peek() {
            Some(&Token {type_: TokenType::Identifier(_), ..}) => LetBinding::Identifiers(*IdentifierList::new(tokens)?),
            Some(&Token {type_: TokenType::LeftBrace, ..}) => LetBinding::StructSignature(StructSignature::new(tokens)?),
            Some(x) => return Err(NodeParseError::UnexpectedToken(x.to_owned(), vec![TokenType::Identifier(vec![]), TokenType::LeftBrace])),
            None => return Err(NodeParseError::UnexpectedEndOfInput),
        };

        let assignment: LetAssignment = match tokens.next() {
            Some(Token { type_: TokenType::Be, ..}) => LetAssignment::Struct(*StructCreation::new(tokens)?),
            Some(Token { type_: TokenType::In, ..}) => LetAssignment::Element(*ElementCreation::new(tokens)?),
            Some(x) => return Err(NodeParseError::UnexpectedToken(x, vec![TokenType::In, TokenType::Be])),
            None => return Err(NodeParseError::UnexpectedEndOfInput),
        };

        Ok(Box::new(Self {
            binding,
            assignment,
        }))
    }

    pub fn visit<'a, 'b: 'a>(self, scope: &'a mut Scope) -> Result<(), NodeVisitationError> {
        match (self.binding, self.assignment) {
            (LetBinding::Identifiers(binding), LetAssignment::Element(assignment)) => {
                assignment.visit(scope, binding)?;
            },
            (LetBinding::Identifiers(binding), LetAssignment::Struct(assignment)) => {
                assignment.visit_as_set(scope, binding)?;
            },
            (LetBinding::StructSignature(binding), LetAssignment::Struct(assignment)) => {
                assignment.visit(scope, binding)?;
            },
            (LetBinding::StructSignature(_), LetAssignment::Element(_)) => {
                return Err(NodeVisitationError::Custom("Cannot assign struct signature to element of set."))
            },
        };

        Ok(())
    }
}