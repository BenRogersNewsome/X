use std::iter::{Peekable, Iterator};

use crate::lexical_analysis::TokenType;
use crate::lexical_analysis::Token;
use crate::syntactic_analysis::ast::{NodeParseError, NodeVisitationError};
use crate::scope::Scope;

mod set_element_creation;
mod struct_creation;

use set_element_creation::ElementCreation;

use self::struct_creation::StructCreation;

pub enum Let {
    Element(ElementCreation),
    Struct(StructCreation),
}

impl Let {
    pub fn new<'a, T: Iterator<Item = Token>>(tokens: &'a mut Peekable<T>) -> Result<Box<Self>, NodeParseError> {
        Ok(Box::new(match tokens.peek() {
            Some(Token { type_: TokenType::LeftParen, ..}) => Self::Struct(*StructCreation::new(tokens)?),
            Some(_) => Self::Element(*ElementCreation::new(tokens)?),
            None => return Err(NodeParseError::UnexpectedEndOfInput),
        }))
    }

    pub fn visit<'a, 'b: 'a>(self, scope: &'a mut Scope) -> Result<(), NodeVisitationError> {
        match self {
            Self::Element(e) => e.visit(scope),
            Self::Struct(s) => s.visit(scope),
        }
    }
}