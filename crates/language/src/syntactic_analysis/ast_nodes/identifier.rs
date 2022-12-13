use std::iter::{Iterator, Peekable};
use crate::{lang::tokens::Token, syntactic_analysis::ast::NodeParseError};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Identifier {
    pub lexeme: Vec<u8>,
}

impl Identifier {
    pub fn new<'a, T: Iterator<Item=Token>>(tokens: &'a mut Peekable<T>) -> Result<Box<Self>, NodeParseError> {
        match tokens.next() {
            Some(Token::Identifier(lexeme)) => {
                Ok(Box::new(Self { lexeme }))
            },
            Some(x) => return Err(NodeParseError::UnexpectedToken(x, vec![Token::Identifier(b"".to_vec())])),
            None => return Err(NodeParseError::UnexpectedEndOfInput),
        }
    }

    pub fn from_lexeme(lexeme: Vec<u8>) -> Self {
        Self {
            lexeme,
        }
    }
}

