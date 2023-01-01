use std::iter::Peekable;

use crate::{syntactic_analysis::{ast_nodes::Identifier, ast::NodeParseError}, lexical_analysis::{Token, TokenType}};


/// Represents a list of identifier tokens, such as 'a, b, c'.
pub struct IdentifierList {
    pub identifiers: Vec<Identifier>,
}

impl IdentifierList {
    pub fn new<'a, T: Iterator<Item = Token>>(tokens: &'a mut Peekable<T>) -> Result<Box<Self>, NodeParseError> {
        let mut identifiers: Vec<Identifier> = Vec::new();
        loop {
            identifiers.push(*Identifier::new(tokens)?);
            match tokens.peek() {
                Some(&Token {type_: TokenType::Comma, ..}) => {
                    tokens.next();
                },
                _ => {
                    break;
                },
            };
        };

        Ok(
            Box::new(Self {
                identifiers,
            })
        )

    }
}