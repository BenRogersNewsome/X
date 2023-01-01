use std::iter::{Peekable, Iterator};

use zsft::SetElement;

use crate::lexical_analysis::Token;
use crate::syntactic_analysis::ast::{NodeParseError, NodeVisitationError};
use crate::scope::{Scope, ScopedItem};
use crate::syntactic_analysis::ast_nodes::common::IdentifierList;

use super::super::{
    Identifier,
};


pub struct ElementCreation {
    set_identifier: Identifier,
}

impl ElementCreation {
    pub fn new<'a, T: Iterator<Item = Token>>(tokens: &'a mut Peekable<T>) -> Result<Box<Self>, NodeParseError> {
        let set_identifier = *Identifier::new(tokens)?;

        Ok(Box::new(Self {
            set_identifier,
        }))
    }

    pub fn visit<'a, 'b: 'a>(self, scope: &'a mut Scope, elements: IdentifierList) -> Result<(), NodeVisitationError> {
        let containing_set = match scope.get(&self.set_identifier.lexeme) {
            Some(x) => match &*x {
                ScopedItem::Set(set) => set.clone(),
                x => return Err(NodeVisitationError::UnexpectedRegisteredItem(x.to_owned())),
            },
            None => return Err(NodeVisitationError::CantResolveToken(String::from_utf8_lossy(&self.set_identifier.lexeme).to_string())),
        };

        for element in elements.identifiers {
            match scope.add(
                element.lexeme.to_vec(),
                ScopedItem::SetElement(SetElement::element_of(&containing_set))
            ) {
                Ok(()) => {},
                Err(x) => return Err(NodeVisitationError::ItemAlreadyExists(x.to_owned())),
            }
        };

        Ok(())
    }
}

