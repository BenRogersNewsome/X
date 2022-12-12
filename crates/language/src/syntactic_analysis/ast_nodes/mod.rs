mod math_expression;

mod let_;
// mod create;
// mod definition;
// mod equation;
mod identifier;
mod structure_definition;
mod symbol;

pub mod common;

use std::iter::Peekable;

pub use let_::Let;
pub use identifier::Identifier;
pub use math_expression::MathExpression;
pub use structure_definition::StructDefinition;
pub use symbol::Symbol;

pub trait TopLevelNode {
    fn new<'a, T: Iterator<Item = Token>>(tokens: &'a mut Peekable<T>) -> Result<Box<Self>, NodeParseError>;

    fn visit<'a, 'b: 'a>(self, scope: &'a mut Scope) -> Result<(), ()>;
}

macro_rules! expect_token {
    ($tokens:ident, $token:ident) => {
        match $tokens.next() {
            Some(Token::$token) => {},
            Some(x) => return Err(NodeParseError::UnexpectedToken(x, vec![Token::$token])),
            _ => return Err(NodeParseError::UnexpectedEndOfInput),
        }; 
    };
}

macro_rules! skip_whitespace {
    ($tokens:ident) => {
        while $tokens.peek() == Some(&Token::Newline) {
            $tokens.next();
        };
    };
}

pub(super) use expect_token;
pub(super) use skip_whitespace;

use crate::{lang::tokens::Token, scope::Scope};

use super::ast::NodeParseError;