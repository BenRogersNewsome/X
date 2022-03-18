use crate::lexical_analysis::tokens::Token;
use crate::core::Stream;


pub trait NonTerminalNode<'a> {
    fn as_vec(&self) -> Vec<& dyn Node>;

    // fn next(&self, tokens: dyn Stream<Token>) -> Option<Box<Vec<dyn Walkable>>>;
}

pub trait Node {
    fn new(tokens: &'a mut dyn Stream<Token>) -> Result<Box<Self>>;
    fn to_str(&self) -> String;
}

fn binary(token: &mut dyn Stream<Token>) -> Result<Box<dyn Node>> {
    
}