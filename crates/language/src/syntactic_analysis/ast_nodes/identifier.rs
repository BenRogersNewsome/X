use std::iter::{Iterator, Peekable};
use crate::{lexical_analysis::TokenType, syntactic_analysis::ast::NodeParseError, lexical_analysis::Token};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Identifier {
    pub lexeme: Vec<u8>,
}

impl Identifier {
    pub fn new<'a, T: Iterator<Item=Token>>(tokens: &'a mut Peekable<T>) -> Result<Box<Self>, NodeParseError> {
        match tokens.next() {
            Some(Token { type_: TokenType::Identifier(lexeme), .. }) => {
                Ok(Box::new(Self { lexeme }))
            },
            Some(x) => return Err(NodeParseError::UnexpectedToken(x, vec![TokenType::Identifier(b"".to_vec())])),
            None => return Err(NodeParseError::UnexpectedEndOfInput),
        }
    }

    pub fn from_lexeme(lexeme: Vec<u8>) -> Self {
        Self {
            lexeme,
        }
    }
}

impl Into<String> for Identifier {
    fn into(self) -> String {
        String::from_utf8(self.lexeme).unwrap()
    }
}