use std::iter::{Peekable, Iterator};
use std::vec;

use zsft::SetElement;

use crate::lang::tokens::Token;
use crate::syntactic_analysis::ast::{NodeParseError, NodeVisitationError};
use crate::scope::{Scope, ScopedItem};

use super::super::{
    Identifier,
};


pub struct ElementCreation {
    symbols: Vec<Identifier>,
    definition: ElementCreationDefinition
}

pub enum ElementCreationDefinition {
    InSet(ElementCreationInSet),
    // AsExpression(ElementCreationAsExpression),
}

impl ElementCreation {
    pub fn new<'a, T: Iterator<Item = Token>>(tokens: &'a mut Peekable<T>) -> Result<Box<Self>, NodeParseError> {
        let mut symbols = vec![];

        symbols.push(*Identifier::new(tokens)?);

        while tokens.peek() == Some(&Token::Comma) {
            tokens.next();
            symbols.push(*Identifier::new(tokens)?);
        };

        let definition = match tokens.next() {
            Some(Token::In) => ElementCreationDefinition::InSet(*ElementCreationInSet::new(tokens)?),
            // Some(Token::Equality) => ElementCreationDefinition::AsExpression(*ElementCreationAsExpression::new(tokens)?),
            Some(x) => return Err(NodeParseError::UnexpectedToken(x, vec![Token::In, Token::Equality])),
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
            // ElementCreationDefinition::AsExpression(expr) => expr.visit(self.symbols, scope),
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

// pub struct ElementCreationAsExpression {
//     expression: MathExpression,
// }

// impl ElementCreationAsExpression {
//     pub fn new<'a, T: Iterator<Item = Token>>(tokens: &'a mut Peekable<T>) -> Result<Box<Self>, NodeParseError> {
//         return Ok(Box::new(Self {
//             expression: *MathExpression::new(tokens)?,
//         }))
//     }
    
//     pub fn visit<'a, 'b: 'a>(self, symbols: Vec<Identifier>, scope: &'a mut Scope) -> Result<(), ()> {
//         for symbol in symbols {
//             match register.add(
//                 &symbol.lexeme,
//                 Registerable::Expression(self.expression.)
//             ) {
//                 Ok(()) => {},
//                 Err(()) => return Err(()),
//             }
//         };

//         Ok(())
//     }
// }