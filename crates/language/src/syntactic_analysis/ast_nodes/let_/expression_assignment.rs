use std::{iter::Peekable, rc::Rc};

use crate::{syntactic_analysis::{ast_nodes::{MathExpression, common::IdentifierList}, ast::{NodeParseError, NodeVisitationError}}, lexical_analysis::Token, Scope, ScopedItem};

pub struct ElementCreationAsExpression {
    expression: MathExpression,
}

impl ElementCreationAsExpression {
    pub fn new<'a, T: Iterator<Item = Token>>(tokens: &'a mut Peekable<T>) -> Result<Box<Self>, NodeParseError> {
        return Ok(Box::new(Self {
            expression: *MathExpression::new(tokens)?,
        }))
    }
    
    pub fn visit<'a, 'b: 'a>(self, elements: IdentifierList, scope: &'a mut Scope) -> Result<(), NodeVisitationError> {
        for element in elements.identifiers {

            let expression: lazymath::core::Expression = self.expression.clone().into_expression(&scope)?;

            match scope.add(
                element.lexeme,
                ScopedItem::Expression(Rc::new(expression))
            ) {
                Ok(()) => {},
                Err(x) => return Err(NodeVisitationError::ItemAlreadyExists(x.to_owned())),
            }
        };

        Ok(())
    }
}