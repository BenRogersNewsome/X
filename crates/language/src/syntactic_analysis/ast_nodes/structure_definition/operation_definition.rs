use std::iter::{Iterator, Peekable};

use crate::lexical_analysis::TokenType;
use crate::lexical_analysis::Token;
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

        let right = if let Some(&Token { type_: TokenType::Symbol(_), ..}) = tokens.peek(){
            tokens.next();
            Some(*Identifier::new(tokens)?)
        }else{
            None
        };
        
        match tokens.next() {
            Some(Token { type_: TokenType::RightArrow, ..}) => {},
            Some(x) => return Err(NodeParseError::UnexpectedToken(x, vec![TokenType::RightArrow])),
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

#[cfg(test_)]
mod tests {

    use crate::lang::tokens::MathOperatorSymbols;
    use super::*;

    #[test]
    fn test_create_operation_definition() {

        let tokens = [
            TokenType::Symbol(MathOperatorSymbols::Plus), TokenType::Colon, TokenType::Identifier(b"F".to_vec()), TokenType::Symbol(MathOperatorSymbols::Plus), TokenType::Identifier(b"V".to_vec()), TokenType::RightArrow,  TokenType::Identifier(b"V".to_vec()),
            TokenType::Newline,
        ];

        let operation_definition = *OperationDefinition::new(&mut tokens.into_iter().peekable()).unwrap();

        assert_eq!(operation_definition.name.symbol, MathOperatorSymbols::Plus);
        assert_eq!(operation_definition.left.lexeme, b"F".to_vec());
        assert_eq!(operation_definition.right.unwrap().lexeme, b"V".to_vec());
        assert_eq!(operation_definition.output.lexeme, b"V".to_vec());
    }
}