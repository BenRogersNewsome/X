use std::{iter::{Iterator, Peekable}};
use super::token::Token;

pub struct TokenStream {
    upstream_next: Box<dyn Fn() -> Option<Token>>,
}

impl Iterator for TokenStream {

    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        (self.upstream_next)()
    }

}

impl TokenStream {
    pub fn new(next: Box<dyn Fn() -> Option<Token>>) -> Peekable<Self> {
        let iter = Self {
            upstream_next: next,
        };

        iter.peekable()
    }
}