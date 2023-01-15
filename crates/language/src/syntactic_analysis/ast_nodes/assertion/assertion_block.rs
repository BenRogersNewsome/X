use std::iter::Peekable;

use crate::{syntactic_analysis::{ast_nodes::{MathExpression, expect_token}, ast::{NodeParseError, NodeVisitationError}}, lexical_analysis::{Token, TokenType}, Scope, scope::InteriorScope};

use super::assertion_element_declaration::AssertionElementDeclarationList;



/// An AST node for an assertion block, such as:
/// ```X
/// |- \-/ a, b (- S, -] 0, 1 (- S {
///     a + 0 = a
///     a * 1 = a
///     a + b = b + a    
/// }
/// ```
pub struct AssertionBlock {
    element_declarations: AssertionElementDeclarationList,
    identities: Vec<MathExpression>,
}

impl AssertionBlock {
    pub fn new<'a, T: Iterator<Item = Token>>(tokens: &'a mut Peekable<T>) -> Result<Box<Self>, NodeParseError> {

        let element_declarations: AssertionElementDeclarationList =
            *AssertionElementDeclarationList::new(tokens)?;
        
        expect_token!(tokens, LeftBrace);

        let mut identities = Vec::new();        
        loop {
            match tokens.peek() {
                Some(&Token {type_: TokenType::RightBrace, ..}) => {
                    tokens.next();
                    break;
                },
                _ => {
                    identities.push(*MathExpression::new(tokens)?);
                }
            };
        };
        
        Ok(Box::new(Self {
            element_declarations,
            identities,
        }))
    }

    pub fn visit<'a, 'b: 'a>(self, scope: &'a mut Scope) -> Result<(), NodeVisitationError> {
        let interior_scope = InteriorScope::from(scope);
    }
}