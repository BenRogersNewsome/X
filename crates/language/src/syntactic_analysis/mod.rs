pub mod ast_nodes;
mod ast;

use crate::{lexical_analysis::TokenType, scope::Scope, lexical_analysis::Token};

use ast_nodes::{Let, StructDefinition, Assertion};

use self::ast::NodeVisitationError;

enum TopLevelNode {
    Let(Let),
    Struct(StructDefinition),
    Assertion(Assertion),
}

pub struct Ast(Vec<TopLevelNode>);

impl Ast {
    pub fn new<'a, T: Iterator<Item = Token>>(tokens: &'a mut std::iter::Peekable<T>) -> Result<Box<Self>, ast::NodeParseError> {
        let mut nodes = vec![];
        while tokens.peek() != None {
            match tokens.next() {
                Some(Token { type_: TokenType::Turnstile, ..}) => nodes.push(TopLevelNode::Assertion(*Assertion::new(tokens)?)),
                Some(Token { type_: TokenType::Let, ..}) => nodes.push(TopLevelNode::Let(*Let::new(tokens)?)),
                Some(Token { type_: TokenType::Struct, ..}) => nodes.push(TopLevelNode::Struct(*StructDefinition::new(tokens)?)),
                Some(Token { type_: TokenType::Newline, ..}) => {},
                _ => panic!(),
            }
        }
        Ok(Box::new(Self(nodes)))
    }

    pub fn visit<'a>(self, scope: &'a mut Scope) -> Result<(), NodeVisitationError> {
        for node in self.0 {
            match node {
                TopLevelNode::Assertion(x) => x.visit(scope)?,
                TopLevelNode::Let(x) => x.visit(scope)?,
                TopLevelNode::Struct(x) => x.visit(scope)?,
            };
        };
        Ok(())
    }
}