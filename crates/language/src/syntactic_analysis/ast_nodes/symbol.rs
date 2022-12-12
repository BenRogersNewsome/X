use std::iter::{Iterator, Peekable};

use crate::{lang::tokens::{Token, MathOperatorSymbols}, syntactic_analysis::ast::NodeParseError};

#[derive(Debug)]
pub struct Symbol {
    pub symbol: MathOperatorSymbols,
}

impl Symbol {
    pub fn new<'a, T: Iterator<Item = Token>>(tokens: &'a mut Peekable<T>) -> Result<Box<Self>, NodeParseError> {
        match tokens.next() {
            Some(Token::Symbol(symbol)) => Ok(Box::new(Self { symbol })),
            Some(x) => return Err(NodeParseError::UnexpectedToken(x, vec![Token::Symbol(MathOperatorSymbols::Bang)])),
            None => return Err(NodeParseError::UnexpectedEndOfInput),
        }
    }
}