use crate::core::Stream;
use super::tokens::Token;
use super::tokens::TokenType;

use std::str;



impl Identifier {
    fn new(lexeme: Vec<u8>) -> Identifier {
        Identifier {
            lexeme,
        }
    }
}

impl Node for Identifier {

    fn to_str(&self, tabs: u8) -> String{
        let mut string = String::new();
        for i in 0..tabs {
            string.push_str("\t");
        };
        string.push_str(str::from_utf8(&self.lexeme).unwrap());
        string.push_str("\n");
        return string;
    }
}

impl Node for Definition {
    fn to_str(&self, tabs: u8) -> String {
        String::from("Definition")
    }
}
impl Node for Let {

    fn to_str(&self, tabs: u8) -> String {
        let mut string = String::new();
        for i in 0..tabs {
            string.push('\t');
        };
        string.push_str("Let");
        string.push('\n');

        for symbol in &self.symbols {
            string.push_str(&symbol.to_str(tabs + 1));
        }

        string.push_str(&self.containing_set.to_str(tabs+1));

        for definition in &self.definitions {
            string.push_str(&definition.to_str(tabs + 1));
        }

        return string;
    }

}

impl<'a> NonTerminal<'a> for Let {

    fn new(tokens: &'a mut dyn Stream<Token>) -> Box<Let> {

        
    }

    fn as_vec(&self) -> Vec<& dyn Node> {
        vec![]
        // self.symbols.append(self.containing_set).extend(self.definitions)
    }

}

impl<'a> NonTerminal<'a> for Root<'a> {

    fn new(tokens: &'a mut dyn Stream<Token>) -> Box<Root<'a>> {
        let mut children: Vec<Box<dyn Node>> = vec![];
        match &tokens.next().token_type {
            TokenType::Let => children.push(Let::new(tokens)),
            x => panic!("Unexpected token {:?}", x)
        };

        Box::new(
            Root { children }
        )
    }

    fn as_vec(&self) -> Vec<& dyn Node> {
        vec![]
    }
}