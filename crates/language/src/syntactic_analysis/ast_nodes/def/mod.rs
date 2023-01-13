use std::iter::Peekable;

use zsft::{Set, BinaryOperation};

use crate::{lexical_analysis::{Token, TokenType, MathOperatorSymbols}, syntactic_analysis::{ast::{NodeParseError, NodeVisitationError}}, Scope, ScopedItem};

use super::{Identifier, expect_token};

///
/// `def + : F + F -> F`
pub struct Def {
    identifier: MathOperatorSymbols,
    left: Identifier,
    right: Option<Identifier>,
    result: Identifier,
}

impl Def {

    pub fn new<'a, T: Iterator<Item = Token>>(tokens: &'a mut Peekable<T>) -> Result<Box<Self>, NodeParseError> {
        let identifier = match tokens.next() {
            Some(Token {type_: TokenType::Symbol(x), ..}) => x,
            Some(x) => return Err(NodeParseError::UnexpectedToken(x, vec![TokenType::Symbol(MathOperatorSymbols::Bang)])),
            None => return Err(NodeParseError::UnexpectedEndOfInput),
        };

        expect_token!(tokens, Colon);

        let left: Identifier = *Identifier::new(tokens)?;

        let right: Option<Identifier> = match tokens.peek() {
            Some(Token { type_: TokenType::RightArrow, .. }) => None,
            _ => {

                match tokens.next() {
                    Some(Token {type_: TokenType::Symbol(_), ..}) => {},
                    Some(x) => return Err(NodeParseError::UnexpectedToken(x, vec![])),
                    None => return Err(NodeParseError::UnexpectedEndOfInput),
                };

                Some(*Identifier::new(tokens)?)
            },
        };

        expect_token!(tokens, RightArrow);

        let result = *Identifier::new(tokens)?;

        Ok(
            Box::new(
                Self {
                    identifier,
                    left,
                    right,
                    result,
                }
            )
        )
    }

    pub fn visit<'a, 'b: 'a>(self, scope: &'a mut Scope) -> Result<(), NodeVisitationError> {

        let left_set: Set = match scope.get(&self.left.lexeme) {
            Some(ScopedItem::Set(x)) => x.clone(),
            Some(x) => return Err(NodeVisitationError::UnexpectedRegisteredItem(x.to_owned())),
            None => return Err(NodeVisitationError::CantResolveToken(String::from_utf8(self.left.lexeme).unwrap())),
        };

        let result_set: Set = match scope.get(&self.result.lexeme) {
            Some(ScopedItem::Set(x)) => x.clone(),
            Some(x) => return Err(NodeVisitationError::UnexpectedRegisteredItem(x.to_owned())),
            None => return Err(NodeVisitationError::CantResolveToken(String::from_utf8(self.left.lexeme).unwrap())),
        };

        if let Some(right) = self.right {
            let right_set: Set = match scope.get(&right.lexeme) {
                Some(ScopedItem::Set(x)) => x.clone(),
                Some(x) => return Err(NodeVisitationError::UnexpectedRegisteredItem(x.to_owned())),
                None => return Err(NodeVisitationError::CantResolveToken(String::from_utf8(self.left.lexeme).unwrap())),
            };

            match scope.add(
                self.identifier.to_bytes(),
                ScopedItem::BinaryOperation(BinaryOperation::from_signature(
                    &left_set,
                    &right_set,
                    &result_set,
                ))
            ) {
                Ok(()) => Ok(()),
                Err(x) => Err(NodeVisitationError::ItemAlreadyExists(x.to_owned())),
            }
        }else{
            todo!();
        }
    }

}