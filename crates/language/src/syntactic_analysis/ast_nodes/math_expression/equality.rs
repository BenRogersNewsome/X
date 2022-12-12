
use std::iter::Peekable;

use crate::{lang::tokens::Token, syntactic_analysis::{ast::NodeParseError, ast_nodes::expect_token}};

use super::MathExpression;

#[derive(Debug, PartialEq, Eq)]
pub struct Equality {
    left: Box<MathExpression>,
    right: Box<MathExpression>,
}

impl Equality {
    pub fn new<'a, T: Iterator<Item = Token>>(tokens: &'a mut Peekable<T>) -> Result<Box<Self>, NodeParseError> {
        let left = MathExpression::new(tokens)?;

        expect_token!(tokens, Equality);

        let right = MathExpression::new(tokens)?;
        Ok(Box::new(
            Equality {
                left,
                right,
            }
        ))
    }
}