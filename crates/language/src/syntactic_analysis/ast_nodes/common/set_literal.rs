use std::iter::Peekable;

use crate::{syntactic_analysis::{ast_nodes::{Identifier, expect_token}, ast::NodeParseError}, lexical_analysis::{Token, TokenType}};


pub enum SetLiteralElement {
    Identifier(Identifier),
    Close,  // '..'
    Spread, // '...'
}

pub struct SetLiteral {
    pub elements: Vec<SetLiteralElement>,
}

impl SetLiteral {
    pub fn new<'a, T: Iterator<Item = Token>>(tokens: &'a mut Peekable<T>) -> Result<Box<Self>, NodeParseError> {

        expect_token!(tokens, LeftBrace);

        let mut elements: Vec<SetLiteralElement> = Vec::new();
        loop {
            elements.push(match tokens.next() {
                Some(Token {type_: TokenType::Identifier(id), ..}) => 
                    SetLiteralElement::Identifier(Identifier::from_lexeme(id)),
                Some(Token {type_: TokenType::Close, ..}) => SetLiteralElement::Close,
                Some(Token {type_: TokenType::Spread, ..}) => SetLiteralElement::Spread,
                Some(Token {type_: TokenType::RightBrace, ..}) => {break;}
                Some(x) => {
                    return Err(NodeParseError::UnexpectedToken(x, vec![TokenType::Identifier(vec![]), TokenType::Spread, TokenType::Close]));
                }
                None => return Err(NodeParseError::UnexpectedEndOfInput),
            });

            match tokens.next() {
                Some(Token {type_: TokenType::RightBrace, ..}) => {break;}
                Some(Token {type_: TokenType::Comma, ..}) => {continue;}
                Some(x) => {
                    return Err(NodeParseError::UnexpectedToken(x, vec![TokenType::RightBrace, TokenType::Comma]));
                },
                None => {
                    return Err(NodeParseError::UnexpectedEndOfInput);
                },
            };
        };
        
        Ok(Box::new(Self {
            elements,
        }))

    }
}