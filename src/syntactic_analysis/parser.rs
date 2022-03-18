use crate::lang::syntax::Node;
use crate::core::Stream;
use crate::lexical_analysis::tokens::Token;

pub fn parse_next_token(tokens: &mut dyn Stream<Token>) -> () {

    match tokens.next() {
        
        _ => panic!("Unexpected Token")
    }

}