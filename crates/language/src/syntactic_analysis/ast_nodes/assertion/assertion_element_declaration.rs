use std::iter::Peekable;

use crate::{syntactic_analysis::{ast_nodes::{common::IdentifierList, Identifier, expect_token}, ast::{NodeParseError, NodeVisitationError}}, lexical_analysis::{Token, TokenType}, Scope};


pub enum AssertionDeclarationType {
    Forall,
    ThereExists,
}

pub struct AssertionElementDeclaration {
    assertion_type: AssertionDeclarationType,
    identifiers: IdentifierList,
    set_identifier: Identifier,
}

impl AssertionElementDeclaration {

    pub fn new<'a, T: Iterator<Item = Token>>(tokens: &'a mut Peekable<T>) -> Result<Box<Self>, NodeParseError> {
        
        let assertion_type: AssertionDeclarationType = match tokens.next() {
            Some(Token {type_: TokenType::ForAll, ..}) => AssertionDeclarationType::Forall,
            Some(Token {type_: TokenType::ThereEx, ..}) => AssertionDeclarationType::ThereExists,
            Some(x) => return Err(NodeParseError::UnexpectedToken(x, vec![
                TokenType::ForAll,
                TokenType::ThereEx,
            ])),
            None => return Err(NodeParseError::UnexpectedEndOfInput),
        };

        let identifiers: IdentifierList = *IdentifierList::new(tokens)?;
        expect_token!(tokens, In);
        let set_identifier: Identifier = *Identifier::new(tokens)?;

        Ok(Box::new(Self {
            assertion_type,
            identifiers,
            set_identifier,
        }))
    }

    pub fn visit<'a, 'b: 'a>(self, scope: &'a mut Scope) -> Result<(), NodeVisitationError> {
        todo!();
    }
}

pub struct AssertionElementDeclarationList {
    declarations: Vec<AssertionElementDeclaration>,
}

impl AssertionElementDeclarationList {

    pub fn new<'a, T: Iterator<Item = Token>>(tokens: &'a mut Peekable<T>) -> Result<Box<Self>, NodeParseError> {

        let mut declarations = Vec::new();

        loop {
            match tokens.peek() {
                Some(&Token {type_: TokenType::ForAll | TokenType::ThereEx, ..}) => {
                    declarations.push(*AssertionElementDeclaration::new(tokens)?);
                },
                _ => break
            };
        };

        Ok(Box::new(Self {
            declarations,
        }))
    }

    pub fn visit<'a, 'b: 'a>(self, scope: &'a mut Scope) -> Result<(), NodeVisitationError> {
        todo!();
    }
}

