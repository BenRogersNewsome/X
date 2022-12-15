use std::iter::Iterator;

use crate::syntactic_analysis::NonTerminalNode;
use crate::lang::tokens::Token;

use super::Node;
use super::Identifier;
use super::equation::Equation;

pub struct Definition {
    free_var: Identifier,
    constraint: Equation,
}

impl Node for Definition {

    fn new<'a>(tokens: &'a mut dyn Iterator<Item=Token>) -> Result<Box<Self>> {
        
    }

    fn to_str(&self) -> String {
        String::from("DEFINITION")
    }
}

impl NonTerminalNode<'_> for Definition {

    fn as_vec(&self) -> Vec<& dyn Node> {
        todo!()
    }
}