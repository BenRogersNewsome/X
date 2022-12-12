pub mod ast_nodes;
mod ast;

use crate::{lang::tokens::Token, scope::Scope};

use ast_nodes::Let;
use ast_nodes::StructDefinition;

use self::ast::NodeVisitationError;

enum TopLevelNode {
    Let(Let),
    Struct(StructDefinition)
}

pub struct Ast(Vec<TopLevelNode>);

impl Ast {
    pub fn new<'a, T: Iterator<Item = crate::lang::tokens::Token>>(tokens: &'a mut std::iter::Peekable<T>) -> Result<Box<Self>, ast::NodeParseError> {
        let mut nodes = vec![];
        while tokens.peek() != None {
            match tokens.next() {
                Some(Token::Let) => nodes.push(TopLevelNode::Let(*Let::new(tokens)?)),
                Some(Token::Struct) => nodes.push(TopLevelNode::Struct(*StructDefinition::new(tokens)?)),
                Some(Token::Newline) => {},
                _ => panic!(),
            }
        }
        Ok(Box::new(Self(nodes)))
    }

    fn to_str(&self) -> String {
        todo!()
    }

    pub fn visit<'a>(self, scope: &'a mut Scope) -> Result<(), NodeVisitationError> {
        for node in self.0 {
            match node {
                TopLevelNode::Let(x) => x.visit(scope)?,
                TopLevelNode::Struct(x) => x.visit(scope)?,
            };
        };
        Ok(())
    }
}