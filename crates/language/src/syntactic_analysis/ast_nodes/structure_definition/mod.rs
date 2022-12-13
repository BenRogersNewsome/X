
mod operation_definition;
mod set_membership_definition;
mod struct_signature;

use lazymath::abstract_algebra::MathStructure;
use set_membership_definition::SetMembershipDefinition;
use struct_signature::StructSignature;

use std::iter::Iterator;
use std::iter::Peekable;
use std::rc::Rc;

use crate::lang::tokens::Token;
use crate::scope::Scope;
use crate::syntactic_analysis::ast::NodeParseError;
use crate::syntactic_analysis::ast::NodeVisitationError;
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
            if let Some(Token::Newline) = tokens.peek() {
                tokens.next();
                break;
            };
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


#[cfg(test)]
mod tests {

    use crate::lang::tokens::{Token, MathOperatorSymbols};
    use super::*;

    #[test]
    fn test_create_struct_from_tokens() {

        let tokens = [
            // <struct> Field { 
            Token::id(b"Field"), Token::LeftBrace, 
                // F;
                Token::id(b"F"), Token::SemiColon,
                // + : F + F -> F,
                Token::op(MathOperatorSymbols::Plus), Token::Colon, Token::id(b"F"), Token::op(MathOperatorSymbols::Plus),
                Token::id(b"F"), Token::RightArrow, Token::id(b"F"), Token::Comma, Token::Newline,
                // * : F * F -> F,
                Token::op(MathOperatorSymbols::Star), Token::Colon, Token::id(b"F"), Token::op(MathOperatorSymbols::Star),
                Token::id(b"F"), Token::RightArrow, Token::id(b"F"), Token::Comma, Token::Newline,
            // }
            Token::RightBrace, Token::Newline,
            // where forall a,b in F:
            Token::Where, Token::ForAll, Token::id(b"a"), Token::Comma, Token::id(b"b"),
            Token::In, Token::id(b"F"), Token::Colon, Token::Newline,
            //     a + b = b + a
            Token::id(b"a"), Token::op(MathOperatorSymbols::Plus), Token::id(b"b"), Token::Equality,
            Token::id(b"b"), Token::op(MathOperatorSymbols::Plus), Token::id(b"a"),  Token::Newline,
            
            Token::Newline,
        ];

        let _struct_def = *StructDefinition::new(&mut tokens.into_iter().peekable()).unwrap();

    }
}