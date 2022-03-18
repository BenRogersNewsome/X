use crate::syntactic_analysis::NonTerminalNode;

use super::Node;
use super::Identifier;
use super::MathExpression;

pub struct Definition {
    left: Identifier,
    right: MathExpression,
}

impl Node for Definition {

    fn new(tokens: &'a mut dyn crate::core::Stream<crate::lexical_analysis::tokens::Token>) -> Result<Box<Self>> {
        todo!()
    }

    fn to_str(&self) -> String {
        "DEFINITION"
    }
}

impl NonTerminalNode<'_> for Definition {

    fn as_vec(&self) -> Vec<& dyn Node> {
        todo!()
    }
}