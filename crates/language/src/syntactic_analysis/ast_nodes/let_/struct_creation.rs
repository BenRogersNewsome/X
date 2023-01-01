use std::iter::{Peekable, Iterator};
use std::vec;

use crate::lexical_analysis::{Token, TokenType};
use crate::syntactic_analysis::ast::{NodeParseError, NodeVisitationError};
use crate::scope::{Scope, ScopedItem};
use crate::syntactic_analysis::ast_nodes::common::{SetLiteral, StructSignature, IdentifierList};
use crate::syntactic_analysis::ast_nodes::{token_of_type, optional_token};

use super::super::Identifier;


/// let (F; +, -, 0, 1) bea Field
pub struct StructCreation {
    struct_name: Option<Identifier>,
    set_literal: SetLiteral,
}

impl StructCreation {
    pub fn new<'a, T: Iterator<Item = Token>>(tokens: &'a mut Peekable<T>) -> Result<Box<Self>, NodeParseError> {
        let (struct_name, set_literal): (Option<Identifier>, SetLiteral) = match tokens.peek() {
            Some(&token_of_type!(LeftBrace)) => (None, *SetLiteral::new(tokens)?),
            Some(&token_of_type!(Identifier(_))) => {
                let identifier = *Identifier::new(tokens)?;
                optional_token!(tokens, Of);
                let set_literal = *SetLiteral::new(tokens)?;
                (Some(identifier), set_literal)
            },
            Some(x) => return Err(NodeParseError::UnexpectedToken(x.to_owned(), vec![TokenType::LeftBrace, TokenType::Identifier(vec![])])),
            None => return Err(NodeParseError::UnexpectedEndOfInput),
        };

        Ok(Box::new(
            Self {
                set_literal,
                struct_name,
            }
        ))
    }

    pub fn visit<'a, 'b: 'a>(self, scope: &'a mut Scope, signature: StructSignature) -> Result<(), NodeVisitationError> {

        if let Some(struct_identifier) = self.struct_name {
            let structure = match scope.get(&struct_identifier.lexeme) {
                Some(ScopedItem::Structure(s)) => s.instantiate(vec![]).unwrap(),
                Some(x) => return Err(NodeVisitationError::TokenOfWrongType(struct_identifier.lexeme, x.to_owned())),
                None => return Err(NodeVisitationError::CantResolveToken(String::from_utf8_lossy(&struct_identifier.lexeme).to_string()))
            };
            signature.bind_struct_to_scope(structure, scope)?;
            Ok(())
        }else {
            Err(NodeVisitationError::Custom("Struct signatures can only be bound to struct instances."))
        }
    }

    pub fn visit_as_set(self, scope: &mut Scope, identifiers: IdentifierList) -> Result<(), NodeVisitationError> {

    }
}