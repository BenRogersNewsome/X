use std::iter::Peekable;

use crate::lang::tokens::Token;
use crate::syntactic_analysis::ast::NodeParseError;
use super::MathExpression;
use super::super::Identifier;

pub fn primary<'a, T: Iterator<Item = Token>>(tokens: &'a mut Peekable<T>) -> Result<Box<MathExpression>, NodeParseError> {

    if let Some(&Token::LeftParen) = tokens.peek() {
        tokens.next();
        let expression = MathExpression::new(tokens)?;
    
        match tokens.next() {
            Some(Token::RightParen) => return Ok(expression),
            Some(x) => return Err(NodeParseError::UnexpectedToken(x, vec![Token::RightParen])),
            None => return Err(NodeParseError::UnexpectedEndOfInput),
        };
    }else {
        return Ok(
            Box::new(
                MathExpression::Identifier(*Identifier::new(tokens)?)
            )
        )
    }
    
}