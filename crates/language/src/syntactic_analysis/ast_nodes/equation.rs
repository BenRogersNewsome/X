use std::iter::Iterator;

use crate::syntactic_analysis::NonTerminalNode;
use crate::lang::tokens::Token;

use super::Node;
use super::MathExpression;

pub struct Equation {
    left: MathExpression,
    right: MathExpression,
}

impl Node for Equation {

    fn new<'a>(tokens: &'a mut dyn Iterator<Item=Token>) -> Result<Box<Self>> {
        
    }

    fn to_str(&self) -> String {
        String::from("DEFINITION")
    }
}

impl NonTerminalNode<'_> for Equation {

    fn as_vec(&self) -> Vec<& dyn Node> {
        todo!()
    }
}