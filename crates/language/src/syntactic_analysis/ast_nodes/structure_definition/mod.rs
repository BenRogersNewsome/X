
mod operation_definition;
mod set_membership_definition;
mod struct_signature;

use lazymath::abstract_algebra::MathStructure;
use set_membership_definition::SetMembershipDefinition;
use struct_signature::StructSignature;

use std::iter::Iterator;
use std::iter::Peekable;
use std::rc::Rc;

use crate::lexical_analysis::Token;
use crate::scope::Scope;
use crate::syntactic_analysis::ast::NodeParseError;
use crate::syntactic_analysis::ast::NodeVisitationError;
use crate::syntactic_analysis::ast_nodes::break_on_token;
use crate::syntactic_analysis::ast_nodes::{expect_token, optional_token};

use super::Identifier;
use super::math_expression::Equality;

/// 
/// struct Field {
///     F;
///     + : F + F -> F,
///     * : F * F -> F,
///     0,
///     1,
/// } where for all a, b in F:
///     a + b = b + a
///     a * b = b * a
///     a * 1 = a
///     a + 0 = a
///     ...
/// 
/// struct VectorSpace over Field(F, _, _, 0, 1) {
///     V,
///     + : V + V -> V,
///     . : V . V -> F,
/// } where forall a, b, c in V:
///     a + b = b + a
///     a + (b + c) = (a + b) + c
///     ...
/// 
/// let (F; +, *, 0, 1) be a Field
/// let (V; +, .) be a VectorSpace over F
pub struct StructDefinition {
    pub name: Identifier,
    pub signature: StructSignature,
    pub constraint_variables: SetMembershipDefinition,
    pub constraints: Vec<Equality>,
}

impl StructDefinition {

    pub fn new<'a, T: Iterator<Item = Token>>(tokens: &'a mut Peekable<T>) -> Result<Box<Self>, NodeParseError> {
        let name = *Identifier::new(tokens)?;
        let signature = *StructSignature::new(tokens)?;

        optional_token!(tokens, Newline);
        expect_token!(tokens, Where);

        let constraint_variables = *SetMembershipDefinition::new(tokens)?;

        expect_token!(tokens, Colon);
        expect_token!(tokens, Newline);

        let mut constraints = vec![];
        loop {
            break_on_token!(tokens, Newline);
            constraints.push(*Equality::new(tokens)?);
            expect_token!(tokens, Newline);
        };

        Ok(Box::new(Self {
            name,
            signature,
            constraint_variables,
            constraints,
        }))

    }

    pub fn visit(self, scope: &mut Scope) -> Result<(), NodeVisitationError> {
        
        let mut local_scope = Scope::new();

        let (bindings, future_set) = self.signature.get_bindings(&mut local_scope)?;

        let internals = self.constraint_variables.get_internal_future_bindings(&mut local_scope)?;

        if let Err(item) = scope.add(
            self.name.lexeme.to_vec(), 
            crate::scope::ScopedItem::Structure(Rc::new(MathStructure::new(
                future_set,
                vec![],
                bindings,
                internals,
                vec![],
            )))
        ) {
            return Err(NodeVisitationError::ItemAlreadyExists(item.to_owned()))
        }

        Ok(())
    }
}


#[cfg(test_)]
mod tests {

    use crate::lexical_analysis::{TokenType, MathOperatorSymbols};
    use super::*;

    #[test]
    fn test_create_struct_from_tokens() {

        let tokens = [
            // <struct> Field { 
            TokenType::id(b"Field"), TokenType::LeftBrace, 
                // F;
                TokenType::id(b"F"), TokenType::SemiColon,
                // + : F + F -> F,
                TokenType::op(MathOperatorSymbols::Plus), TokenType::Colon, TokenType::id(b"F"), TokenType::op(MathOperatorSymbols::Plus),
                TokenType::id(b"F"), TokenType::RightArrow, TokenType::id(b"F"), TokenType::Comma, TokenType::Newline,
                // * : F * F -> F,
                TokenType::op(MathOperatorSymbols::Star), TokenType::Colon, TokenType::id(b"F"), TokenType::op(MathOperatorSymbols::Star),
                TokenType::id(b"F"), TokenType::RightArrow, TokenType::id(b"F"), TokenType::Comma, TokenType::Newline,
            // }
            TokenType::RightBrace, TokenType::Newline,
            // where forall a,b in F:
            TokenType::Where, TokenType::ForAll, TokenType::id(b"a"), TokenType::Comma, TokenType::id(b"b"),
            TokenType::In, TokenType::id(b"F"), TokenType::Colon, TokenType::Newline,
            //     a + b = b + a
            TokenType::id(b"a"), TokenType::op(MathOperatorSymbols::Plus), TokenType::id(b"b"), TokenType::Equality,
            TokenType::id(b"b"), TokenType::op(MathOperatorSymbols::Plus), TokenType::id(b"a"),  TokenType::Newline,
            
            TokenType::Newline,
        ];

        let _struct_def = *StructDefinition::new(&mut tokens.into_iter().peekable()).unwrap();

    }
}