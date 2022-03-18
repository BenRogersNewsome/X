use super::Node;
use super::NonTerminalNode;

use super::Identifier;
use super::Definition;

use crate::lexical_analysis::tokens::Token;
use crate::lexical_analysis::tokens::TokenType;
use crate::core::Stream;

pub struct Let {
    symbols: Vec<Identifier>,
    containing_set: Identifier,
    definitions: Vec<Definition>,
}

impl Node for Let {
    fn to_str(&self) -> String {
        "LET"
    }

    fn new(tokens: &'a mut dyn Stream<Token>) -> Result<Box<Self>> {
        let mut symbols = vec![];

        match Identifier::new(tokens) {
            Ok(x) => symbols.push(x),
            Err() => panic!(),
        };

        while tokens.peek().token_type == TokenType::Comma {
            tokens.next();
            match Identifier::new(tokens) {
                Ok(x) => symbols.push(x),
                Err() => panic!(),
            };
        }

        match tokens.next() {
            Token {lexeme: _, token_type: TokenType::In} => (),
            _ => panic!(),
        };

        let containing_set = match Identifier::new(tokens) {
            Ok(x) => x,
            Err() => panic!(),
        };

        return Ok(Box::new(Let {
            symbols,
            containing_set,
            definitions: vec![],
        }))
    }
}

impl NonTerminalNode<'_> for Let {
    

    fn as_vec(&self) -> Vec<& dyn Node> {
        todo!()
    }
}