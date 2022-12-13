use std::iter::{Iterator, Peekable};

use crate::lang::tokens::Token;
use crate::syntactic_analysis::ast::NodeParseError;
use crate::syntactic_analysis::ast_nodes::expect_token;

use super::super::Symbol;
use super::super::identifier::Identifier;

pub struct OperationDefinition {
    pub name: Symbol,
    pub left: Identifier,
    pub right: Option<Identifier>,
    pub output: Identifier,
}

impl OperationDefinition {

    pub fn new<'a, T: Iterator<Item = Token>>(tokens: &'a mut Peekable<T>) -> Result<Box<Self>, NodeParseError> {
        let name = *Symbol::new(tokens)?;

        expect_token!(tokens, Colon);

        let left = *Identifier::new(tokens)?;

        let right = if let Some(Token::Symbol(_)) = tokens.peek(){
            tokens.next();
            Some(*Identifier::new(tokens)?)
        }else{
            None
        };
        
        match tokens.next() {
            Some(Token::RightArrow) => {},
            Some(x) => return Err(NodeParseError::UnexpectedToken(x, vec![Token::RightArrow])),
            None => return Err(NodeParseError::UnexpectedEndOfInput),
        };

        let output = *Identifier::new(tokens)?;

        Ok(Box::new(Self {
            name,
            left,
            right,
            output,
        }))
        
    }
}

#[cfg(test)]
mod tests {

    use crate::lang::tokens::MathOperatorSymbols;
    use super::*;

    #[test]
    fn test_create_operation_definition() {

        let tokens = [
            Token::Symbol(MathOperatorSymbols::Plus), Token::Colon, Token::Identifier(b"F".to_vec()), Token::Symbol(MathOperatorSymbols::Plus), Token::Identifier(b"V".to_vec()), Token::RightArrow,  Token::Identifier(b"V".to_vec()),
            Token::Newline,
        ];

        let operation_definition = *OperationDefinition::new(&mut tokens.into_iter().peekable()).unwrap();

        assert_eq!(operation_definition.name.symbol, MathOperatorSymbols::Plus);
        assert_eq!(operation_definition.left.lexeme, b"F".to_vec());
        assert_eq!(operation_definition.right.unwrap().lexeme, b"V".to_vec());
        assert_eq!(operation_definition.output.lexeme, b"V".to_vec());
    }
}