use std::iter::{Peekable, Iterator};
use std::rc::Rc;
use std::vec;

use zsft::SetElement;

use crate::lexical_analysis::TokenType;
use crate::lexical_analysis::Token;
use crate::syntactic_analysis::ast::{NodeParseError, NodeVisitationError};
use crate::scope::{Scope, ScopedItem};
use crate::syntactic_analysis::ast_nodes::{MathExpression, do_while_token};

use super::super::{
    Identifier,
};


pub struct ElementCreation {
    symbols: Vec<Identifier>,
    definition: ElementCreationDefinition
}

pub enum ElementCreationDefinition {
    InSet(ElementCreationInSet),
    AsExpression(ElementCreationAsExpression),
}

impl ElementCreation {
    pub fn new<'a, T: Iterator<Item = Token>>(tokens: &'a mut Peekable<T>) -> Result<Box<Self>, NodeParseError> {
        let mut symbols = vec![];

        symbols.push(*Identifier::new(tokens)?);

        do_while_token!(tokens, Comma, 
            symbols.push(*Identifier::new(tokens)?)
        );

        let definition = match tokens.next() {
            Some(Token { type_: TokenType::In, ..}) => ElementCreationDefinition::InSet(*ElementCreationInSet::new(tokens)?),
            Some(Token { type_: TokenType::Equality, ..}) => ElementCreationDefinition::AsExpression(*ElementCreationAsExpression::new(tokens)?),
            Some(x) => return Err(NodeParseError::UnexpectedToken(x, vec![TokenType::In, TokenType::Equality])),
            None => return Err(NodeParseError::UnexpectedEndOfInput),
        };

        Ok(Box::new(Self {
            symbols,
            definition,
        }))
    }

    pub fn visit<'a, 'b: 'a>(self, scope: &'a mut Scope) -> Result<(), NodeVisitationError> {
        match self.definition {
            ElementCreationDefinition::InSet(s) => s.visit(self.symbols, scope),
            ElementCreationDefinition::AsExpression(expr) => expr.visit(self.symbols, scope),
        }
    }
}

pub struct ElementCreationInSet {
    containing_set: Identifier,
}

impl ElementCreationInSet {
    pub fn new<'a, T: Iterator<Item = Token>>(tokens: &'a mut Peekable<T>) -> Result<Box<Self>, NodeParseError> {
        let containing_set = *Identifier::new(tokens)?;

        return Ok(Box::new(Self {
            containing_set,
        }))
    }
    
    pub fn visit<'a, 'b: 'a>(self, symbols: Vec<Identifier>, scope: &'a mut Scope) -> Result<(), NodeVisitationError> {
        let containing_set = match scope.get(&self.containing_set.lexeme) {
            Some(x) => match &*x {
                ScopedItem::Set(set) => set.clone(),
                x => return Err(NodeVisitationError::UnexpectedRegisteredItem(x.to_owned())),
            },
            None => return Err(NodeVisitationError::CantResolveToken(String::from_utf8_lossy(&self.containing_set.lexeme).to_string())),
        };

        for symbol in symbols {
            match scope.add(
                symbol.lexeme.to_vec(),
                ScopedItem::SetElement(SetElement::element_of(&containing_set))
            ) {
                Ok(()) => {},
                Err(x) => return Err(NodeVisitationError::ItemAlreadyExists(x.to_owned())),
            }
        };

        Ok(())
    }
}

pub struct ElementCreationAsExpression {
    expression: MathExpression,
}

impl ElementCreationAsExpression {
    pub fn new<'a, T: Iterator<Item = Token>>(tokens: &'a mut Peekable<T>) -> Result<Box<Self>, NodeParseError> {
        return Ok(Box::new(Self {
            expression: *MathExpression::new(tokens)?,
        }))
    }
    
    pub fn visit<'a, 'b: 'a>(self, symbols: Vec<Identifier>, scope: &'a mut Scope) -> Result<(), NodeVisitationError> {
        for symbol in symbols {

            let expression: lazymath::core::Expression = self.expression.clone().into_expression(&scope)?;

            match scope.add(
                symbol.lexeme,
                ScopedItem::Expression(Rc::new(expression))
            ) {
                Ok(()) => {},
                Err(x) => return Err(NodeVisitationError::ItemAlreadyExists(x.to_owned())),
            }
        };

        Ok(())
    }
}