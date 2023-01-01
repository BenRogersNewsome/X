
mod assertion;
mod let_;
// mod create;
// mod definition;
// mod equation;
mod identifier;
mod math_expression;
mod structure_definition;
mod symbol;

pub mod common;

use std::iter::Peekable;

use crate::lexical_analysis::Token;

pub use assertion::Assertion;
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
    ($tokens:ident, $token:ident $(, $execute:expr)?) => {
        match $tokens.next() {
            Some(crate::lexical_analysis::Token {type_: crate::lexical_analysis::TokenType::$token, ..}) => {
                $($execute)?
            },
            Some(x) => return Err(NodeParseError::UnexpectedToken(x, vec![crate::lexical_analysis::TokenType::$token])),
            _ => return Err(NodeParseError::UnexpectedEndOfInput),
        }
    };
}

macro_rules! skip_whitespace {
    ($tokens:ident) => {
        while let Some(x) = $tokens.peek() {
            if x.type_ == crate::lexical_analysis::TokenType::Newline {
                $tokens.next();
            }else{
                break;
            };
        };
    };
}

macro_rules! optional_token {
    ($tokens:ident, $token:ident) => {
        match $tokens.peek() {
            Some(&crate::lexical_analysis::Token { type_: crate::lexical_analysis::TokenType::$token, .. }) => { $tokens.next(); },
            _ => {},
        };
    };
}

macro_rules! break_on_token {
    ($tokens:ident, $token_type:ident) => {
        match $tokens.peek() {
            Some(&crate::lexical_analysis::Token { type_: crate::lexical_analysis::TokenType::$token_type, .. }) => {
                $tokens.next();
                break;
            },
            _ => {},
        };
    };
}

macro_rules! do_while_token {
    ($tokens:ident, $token_type:ident, $execute:stmt ) => {
        match $tokens.peek() {
            Some(&crate::lexical_analysis::Token { type_: crate::lexical_analysis::TokenType::$token_type, .. }) => {
                $tokens.next();
                $execute
            },
            _ => {},
        };
    };
}

macro_rules! token_of_type {
    ($token:ident) => {
        Token {type_: TokenType::$token, ..}
    };
}

pub(super) use expect_token;
pub(super) use skip_whitespace;
pub(super) use optional_token;
pub(super) use break_on_token;
pub(super) use do_while_token;
pub(super) use token_of_type;

use crate::scope::Scope;

use super::ast::NodeParseError;