use std::iter::{Peekable, zip};

use lazymath::abstract_algebra::{MathStructureInstance, StructBinding};

use crate::{syntactic_analysis::{ast_nodes::{Identifier, expect_token}, ast::{NodeParseError, NodeVisitationError}}, lexical_analysis::{TokenType, MathOperatorSymbols}, Scope, ScopedItem, lexical_analysis::Token};

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
                Some(Token{ type_: TokenType::SemiColon, ..}) => Self {
                    identity,
                    bindings: Self::_scan_bindings(tokens)?,
                },
                Some(Token{ type_: TokenType::RightParen, ..}) => Self {
                    identity,
                    bindings: vec![],
                },
                Some(x) => return Err(NodeParseError::UnexpectedToken(x, vec![TokenType::SemiColon, TokenType::RightParen])),
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
                    try_add_to_scope![scope, to.lexeme, ScopedItem::Item(from)]; },
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
                Some(Token { type_: TokenType::Comma, .. }) => {
                    if let Some(&Token { type_: TokenType::RightBrace, ..}) = tokens.peek() {
                        break;
                    };
                    continue;
                },
                Some(Token { type_: TokenType::RightParen, ..}) => break,
                Some(x) => return Err(NodeParseError::UnexpectedToken(x, vec![TokenType::Comma, TokenType::RightParen])),
                None => return Err(NodeParseError::UnexpectedEndOfInput),
            };
        };

        Ok(bindings)
    }

    fn _scan_for_binding<T: Iterator<Item=Token>>(tokens: &mut Peekable<T>) -> Result<StructSignatureBinding, NodeParseError> {
        Ok(
            match tokens.next() {
                Some(Token { type_: TokenType::Symbol(s), .. }) => StructSignatureBinding::Operation(s),
                Some(Token { type_: TokenType::Identifier(i), ..}) => StructSignatureBinding::Element(Identifier::from_lexeme(i)),
                Some(x) => return Err(NodeParseError::UnexpectedToken(x, vec![TokenType::Symbol(MathOperatorSymbols::Bang), TokenType::Identifier(vec![])])),
                None => return Err(NodeParseError::UnexpectedEndOfInput),
            }
        )
    }

}