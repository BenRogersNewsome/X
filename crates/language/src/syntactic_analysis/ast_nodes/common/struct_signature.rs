use std::iter::{Peekable, zip};

use lazymath::abstract_algebra::{MathStructureInstance, StructBinding};

use crate::{syntactic_analysis::{ast_nodes::{Identifier, expect_token}, ast::{NodeParseError, NodeVisitationError}}, lang::tokens::{Token, MathOperatorSymbols}, Scope, ScopedItem};

pub enum StructSignatureBinding {
    Element(Identifier),
    Operation(MathOperatorSymbols),
}

///
/// (F; +, -, 0, 1)
pub struct StructSignature {
    pub identity: Identifier,
    pub bindings: Vec<StructSignatureBinding>,
}

macro_rules! try_add_to_scope {
    ($scope:ident, $identifier:expr, $item:expr) => {
        if let Err(x) = $scope.add($identifier, $item) {
            return Err(NodeVisitationError::ItemAlreadyExists(x.to_owned()));
        };
    };
}

impl StructSignature {

    pub fn new<T: Iterator<Item=Token>>(tokens: &mut Peekable<T>) -> Result<Self, NodeParseError> {
        expect_token!(tokens, LeftParen);
        let identity = *Identifier::new(tokens)?;

        Ok(
            match tokens.next() {
                Some(Token::SemiColon) => Self {
                    identity,
                    bindings: Self::_scan_bindings(tokens)?,
                },
                Some(Token::RightParen) => Self {
                    identity,
                    bindings: vec![],
                },
                Some(x) => return Err(NodeParseError::UnexpectedToken(x, vec![Token::SemiColon, Token::RightParen])),
                None => return Err(NodeParseError::UnexpectedEndOfInput),
            }
        )
    }

    pub fn bind_struct_to_scope(self, math_structure_instance: MathStructureInstance, scope: &mut Scope) -> Result<(), NodeVisitationError> {
        
        try_add_to_scope![scope, self.identity.lexeme, ScopedItem::Set(math_structure_instance.underlying_set)];

        if self.bindings.len() != math_structure_instance.bindings.len() - 1 {
            todo!("Return error");
        };

        for (bind_to, bind_from) in zip(self.bindings, math_structure_instance.bindings.into_iter().skip(1)) {
            match (bind_to, bind_from) {
                (StructSignatureBinding::Element(to), StructBinding::Element(from)) => { 
                    try_add_to_scope![scope, to.lexeme, ScopedItem::SetElement(from)]; },
                (StructSignatureBinding::Operation(to), StructBinding::Operation(from)) => { 
                    try_add_to_scope![scope, to.to_bytes(), ScopedItem::BinaryOperation(from)]; },
                _ => {todo!("Raise mismatch error")},
            }
        };

        Ok(())
    }

    fn _scan_bindings<T: Iterator<Item=Token>>(tokens: &mut Peekable<T>) -> Result<Vec<StructSignatureBinding>, NodeParseError> {
        let mut bindings: Vec<StructSignatureBinding> = vec![];
        loop {
            bindings.push(Self::_scan_for_binding(tokens)?);
            match tokens.next() {
                Some(Token::Comma) => {
                    if let Some(&Token::RightBrace) = tokens.peek() {
                        break;
                    };
                    continue;
                },
                Some(Token::RightParen) => break,
                Some(x) => return Err(NodeParseError::UnexpectedToken(x, vec![Token::Comma, Token::RightParen])),
                None => return Err(NodeParseError::UnexpectedEndOfInput),
            };
        };

        Ok(bindings)
    }

    fn _scan_for_binding<T: Iterator<Item=Token>>(tokens: &mut Peekable<T>) -> Result<StructSignatureBinding, NodeParseError> {
        Ok(
            match tokens.next() {
                Some(Token::Symbol(s)) => StructSignatureBinding::Operation(s),
                Some(Token::Identifier(i)) => StructSignatureBinding::Element(Identifier::from_lexeme(i)),
                Some(x) => return Err(NodeParseError::UnexpectedToken(x, vec![Token::Symbol(MathOperatorSymbols::Bang), Token::Identifier(vec![])])),
                None => return Err(NodeParseError::UnexpectedEndOfInput),
            }
        )
    }

}