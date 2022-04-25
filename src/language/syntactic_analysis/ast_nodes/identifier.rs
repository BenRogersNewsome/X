use super::Node;
use crate::lexical_analysis::tokens::Token;
use crate::lexical_analysis::tokens::TokenType;

pub struct Identifier {
    lexeme: Vec<u8>,
}

impl Node for Identifier {
    fn to_str(&self) -> String {
        format!("IDENTIFIER: {}", str::from_utf8(&self.lexeme).unwrap())
    }

    fn new(tokens: &'a mut dyn crate::core::Stream<Token>) -> Result<Box<Self>> {
        match tokens.next() {
            Token {lexeme, token_type: TokenType::Identifier} => Ok(Identifier::new(lexeme)),
            _ => Err(),
        }
    }
}

