use std::iter::{Peekable, Iterator};
use std::vec;

use crate::lang::tokens::Token;
use crate::syntactic_analysis::ast::{NodeParseError, NodeVisitationError};
use crate::scope::{Scope, ScopedItem};
use crate::syntactic_analysis::ast_nodes::common::StructSignature;

use super::super::{
    Identifier,
    expect_token
};


/// let (F; +, -, 0, 1) bea Field
pub struct StructCreation {
    signature: StructSignature,
    structure: Identifier,
}

impl StructCreation {
    pub fn new<'a, T: Iterator<Item = Token>>(tokens: &'a mut Peekable<T>) -> Result<Box<Self>, NodeParseError> {
        let signature = StructSignature::new(tokens)?;
        expect_token!(tokens, Bea);
        let structure = *Identifier::new(tokens)?;
        expect_token!(tokens, Newline);

        Ok(Box::new(
            Self {
                signature,
                structure,
            }
        ))
    }

    pub fn visit<'a, 'b: 'a>(self, scope: &'a mut Scope) -> Result<(), NodeVisitationError> {

        let structure = match scope.get(&self.structure.lexeme) {
            Some(ScopedItem::Structure(s)) => s.instantiate(vec![]).unwrap(),
            Some(x) => return Err(NodeVisitationError::TokenOfWrongType(self.structure.lexeme, x.to_owned())),
            None => return Err(NodeVisitationError::CantResolveToken(String::from_utf8_lossy(&self.structure.lexeme).to_string()))
        };

        self.signature.bind_struct_to_scope(structure, scope)?;
        
        Ok(())
    }
}