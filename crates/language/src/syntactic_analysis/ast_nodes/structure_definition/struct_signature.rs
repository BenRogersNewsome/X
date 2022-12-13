use std::iter::Iterator;
use std::iter::Peekable;

use lazymath::abstract_algebra::FutureValue;
use lazymath::abstract_algebra::FutureStructBinding;
use zsft::BinaryOperation;
use zsft::Set;
use zsft::SetElement;

use crate::lang::tokens::Token;
use crate::scope::Scope;
use crate::syntactic_analysis::ast::NodeParseError;
use crate::syntactic_analysis::ast::NodeVisitationError;
use crate::syntactic_analysis::ast_nodes::expect_token;
use crate::syntactic_analysis::ast_nodes::skip_whitespace;

use super::Identifier;
use super::operation_definition::OperationDefinition;


pub enum StructBinding {
    Operation(OperationDefinition),
    Element(Identifier),
}


///
/// # Examples
/// 
/// {
///     F;
///     + : F + F -> F,
///     * : F * F -> F,
///     0,
///     1,
/// }
pub struct StructSignature {
    pub name: Identifier,
    pub bindings: Vec<StructBinding>,
}

impl StructSignature {

    pub fn new<'a, T: Iterator<Item = Token>>(tokens: &'a mut Peekable<T>) -> Result<Box<Self>, NodeParseError> {
        expect_token!(tokens, LeftBrace);
        skip_whitespace!(tokens);
        
        let name = *Identifier::new(tokens)?;
        expect_token!(tokens, SemiColon);
        skip_whitespace!(tokens);

        let mut bindings: Vec<StructBinding> = vec![];
        
        loop {
            skip_whitespace!(tokens);
            bindings.push(
                match tokens.peek() {
                    Some(&Token::Symbol(_)) => StructBinding::Operation(*OperationDefinition::new(tokens)?),
                    Some(&Token::Identifier(_)) => StructBinding::Element(*Identifier::new(tokens)?),
                    Some(x) => return Err(NodeParseError::UnexpectedToken(x.to_owned(), vec![Token::Comma, Token::RightBrace])),
                    None => return Err(NodeParseError::UnexpectedEndOfInput),
                }
            );

            match tokens.next() {
                Some(Token::Comma) => {
                    skip_whitespace!(tokens);
                    if let Some(&Token::RightBrace) = tokens.peek() {
                        tokens.next();
                        break;
                    };
                },
                Some(Token::RightBrace) => break,
                Some(x) => return Err(NodeParseError::UnexpectedToken(x, vec![Token::Comma, Token::RightBrace])),
                None => return Err(NodeParseError::UnexpectedEndOfInput),
            }
        };

        Ok(Box::new(
            Self {
                name,
                bindings,
            }
        ))
    }

    pub fn get_bindings(self, scope: &mut Scope) -> Result<(Vec<FutureStructBinding>, FutureValue<Set>), NodeVisitationError> {

        let future_set: FutureValue<Set> = FutureValue::new(Box::new(|| {
            Set::anonymous()
        }));

        scope.add(self.name.lexeme.to_vec(), crate::ScopedItem::FutureSet(future_set.clone())).unwrap();

        let mut bindings: Vec<FutureStructBinding>  = vec![FutureStructBinding::Set(future_set.clone())];

        for binding in self.bindings.into_iter() {
            bindings.push(match binding {
                StructBinding::Element(e) => {

                    let future_set_clone = future_set.clone();
                    let future_struct_binding = FutureStructBinding::Element(FutureValue::new(
                        Box::new(move || {
                            SetElement::element_of(&future_set_clone.get().unwrap())
                        }))
                    );
                    
                    scope.add(e.lexeme.to_vec(), crate::scope::ScopedItem::FutureStructBinding(future_struct_binding.clone())).unwrap();

                    future_struct_binding
                },
                StructBinding::Operation(o) => {
                    let future_set_clone = future_set.clone();
                    let future_struct_binding = FutureStructBinding::Operation(
                        FutureValue::new(Box::new(move || {
                            if o.right != None {
                                BinaryOperation::from_signature(
                                    &future_set_clone.clone().get().unwrap(),
                                    &future_set_clone.clone().get().unwrap(),
                                    &future_set_clone.get().unwrap(),
                                )
                            }else {
                                todo!("Unary operations")
                            }
                        }))
                    );

                    scope.add(o.name.symbol.to_bytes(), crate::scope::ScopedItem::FutureStructBinding( future_struct_binding.clone())).unwrap();

                    future_struct_binding
                },
            })
        };

        Ok((bindings, future_set))
    }
}

// #[cfg(test)]
// mod tests {

//     use crate::lang::tokens::MathOperatorSymbols;
//     use super::*;

//     #[test]
//     fn test_create_struct_signature() {

//         let tokens = [
//             Token::LeftParen, Token::Identifier(b"F".to_vec()), Token::Comma, Token::Symbol(MathOperatorSymbols::Plus),
//             Token::Comma, Token::Symbol(MathOperatorSymbols::Star), Token::RightParen, Token::Newline,
//         ];

//         let struct_signature = StructSignature::new(&mut tokens.into_iter().peekable()).unwrap();

//         assert_eq!(struct_signature.name.lexeme, b"F".to_vec());
//         assert_eq!(struct_signature.operations.len(), 2);
//         assert_eq!(struct_signature.operations[0].symbol, MathOperatorSymbols::Plus);
//         assert_eq!(struct_signature.operations[1].symbol, MathOperatorSymbols::Star);
//     }
// }