use super::MathExpression;
use super::super::Identifier;
use crate::core::Stream;
use crate::lexical_analysis::tokens::Token;
use crate::lexical_analysis::tokens::TokenType;

pub fn primary(tokens: &mut dyn Stream<Token>) {
    match tokens.peek().token_type {
        TokenType::Identifier => return Ok(Identifier::new(tokens)),
        TokenType::LeftParen => tokens.next(),
        _ => return Err()
    };

    let expression = MathExpression::new(tokens);
    
    match tokens.peek().token_type {
        TokenType::RightParen => return Ok(expression),
        _ => return Err()
    };
}