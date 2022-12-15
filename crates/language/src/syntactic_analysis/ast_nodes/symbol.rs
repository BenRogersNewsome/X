use std::iter::{Iterator, Peekable};

use crate::{lexical_analysis::{TokenType, MathOperatorSymbols}, syntactic_analysis::ast::NodeParseError, lexical_analysis::Token};

#[derive(Debug)]
pub struct Symbol {
    pub symbol: MathOperatorSymbols,
}

impl Symbol {
    pub fn new<'a, T: Iterator<Item = Token>>(tokens: &'a mut Peekable<T>) -> Result<Box<Self>, NodeParseError> {
        match tokens.next() {
            Some(Token { type_: TokenType::Symbol(symbol), ..}) => Ok(Box::new(Self { symbol })),
            Some(x) => return Err(NodeParseError::UnexpectedToken(x, vec![TokenType::Symbol(MathOperatorSymbols::Bang)])),
            None => return Err(NodeParseError::UnexpectedEndOfInput),
        }
    }
}