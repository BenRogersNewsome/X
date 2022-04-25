use super::tokens::Token;
use super::tokens::TokenType;
use crate::core::Stream;

pub struct TokenStream<'a> {
    tokens: &'a Vec<Token>,
    pos: usize,
}

impl Stream<Token> for TokenStream<'_> {

    fn next(&mut self) -> Token {
        let token = self.tokens.get(self.pos).unwrap().clone();
        self.pos += 1;
        
        return token;
    }

    fn peek<'a>(&self) -> Token {
        self.tokens.get(self.pos).unwrap().clone()
    }

    fn is_end(self) -> bool {
        matches!(self.tokens.get(self.pos).unwrap().token_type, TokenType::EOF)
    }
}

impl TokenStream<'_> {
    pub fn new(tokens: &Vec<Token>) -> TokenStream {
        TokenStream {
            tokens,
            pos: 0,
        }
    }
}