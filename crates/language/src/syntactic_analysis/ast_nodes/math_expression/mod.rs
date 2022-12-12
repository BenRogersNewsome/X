use std::iter::Peekable;

use lazymath::abstract_algebra::{IdentityDefinitionElement, FutureValue, IdentityExpressionDefinitionTerm, IdentityExpressionDefinition};
use lazymath::core::ExpressionTerm;
use zsft::{SetElement, BinaryOperation};

use crate::{lang::tokens::Token, syntactic_analysis::ast::NodeVisitationError};
use crate::syntactic_analysis::ast::NodeParseError;
use crate::scope::{Scope, ScopedItem};

mod infix_binary;
mod primary;
mod equality;
pub use equality::Equality;

use self::infix_binary::minus;

use super::Identifier;
use infix_binary::InfixBinary;
// pub use postfix_unary::{PostfixUnary, match_postfix_unary_operator};
pub use primary::primary;

#[derive(Debug, PartialEq, Eq)]
pub enum MathExpression {
    Identifier(Identifier),
    InfixBinary(InfixBinary),
}

///
/// term = (term, ({'+'} | {'-'}), comma) | comma;
/// dot = (dot, {'.'}, factor) | factor
/// factor = (factor, ({'*'} | {'/'}), power) | power;
/// power = (power, {'^'}, unary) | unary
/// unary = (operator, unary) | primary;
/// primary = symbol | "(", expression, ")";
///
impl MathExpression {

    /// Create a new math expression from an iterator of tokens.
    pub fn new<'a, T: Iterator<Item = Token>>(tokens: &'a mut Peekable<T>) -> Result<Box<Self>, NodeParseError> {
        minus(tokens)
    }

    /// Convert the AST node into a lazymath expression, consuming the node in the process.
    pub fn into_expression<'a>(self, scope: &'a mut Scope) -> Result<lazymath::core::Expression, NodeVisitationError> {
        match self {
            Self::Identifier(id) => Self::_identifier_to_expression(scope, id),
            Self::InfixBinary(ib) => Self::_infix_binary_to_expression(scope, ib),
        }
    }

    fn _identifier_to_expression<'a>(scope: &'a mut Scope, id: Identifier) -> Result<lazymath::core::Expression, NodeVisitationError>  {
        let set_element: SetElement = match scope.get(&id.lexeme) {
            Some(rc) => match rc {
                ScopedItem::SetElement(s) => s.clone(),
                o => return Err(NodeVisitationError::UnexpectedRegisteredItem(o.to_owned())),
            },
            None => return Err(NodeVisitationError::RegisteredItemNotFound),
        };

        return Ok(vec![ExpressionTerm::Element(set_element)])
    }

    fn _infix_binary_to_expression<'a>(scope: &'a mut Scope, ib: InfixBinary) -> Result<lazymath::core::Expression, NodeVisitationError>  {
        let operation: BinaryOperation = match scope.get(&ib.operator.to_bytes()) {
            Some(rc) => match rc {
                ScopedItem::BinaryOperation(b) => b.clone(),
                o => return Err(NodeVisitationError::UnexpectedRegisteredItem(o.to_owned())),
            },
            None => return Err(NodeVisitationError::RegisteredItemNotFound),
        };

        return Ok(vec![ExpressionTerm::BinaryOperation(operation)])
    }

    pub fn into_future_expression(self, scope: &mut Scope) -> Result<lazymath::abstract_algebra::IdentityExpressionDefinition, NodeVisitationError> {
        Ok(IdentityExpressionDefinition::new(self._into_future_expression(scope)?))
    }

    fn _into_future_expression(self, scope: &mut Scope) -> Result<Vec<lazymath::abstract_algebra::IdentityExpressionDefinitionTerm>, NodeVisitationError> {
        match self {
            Self::Identifier(id) => Ok(
                vec![
                    IdentityExpressionDefinitionTerm::Element(Self::_identifier_to_future_expression(scope, id)?),
                ]
            ),
            Self::InfixBinary(ib) => Self::_infix_binary_to_future_expression(scope, ib),
        }
    }

    fn _identifier_to_future_expression<'a>(scope: &'a mut Scope, id: Identifier) -> Result<IdentityDefinitionElement, NodeVisitationError>  {
        let expression_term: IdentityDefinitionElement = match scope.get(&id.lexeme) {
            Some(rc) => match rc {
                ScopedItem::FutureSetElement(s) => IdentityDefinitionElement::ForAll(s.clone()),
                ScopedItem::FutureBoundSetElement(s) => IdentityDefinitionElement::Bound(s.clone()),
                o => return Err(NodeVisitationError::UnexpectedRegisteredItem(o.to_owned())),
            },
            None => return Err(NodeVisitationError::RegisteredItemNotFound),
        };

        return Ok(expression_term)
    }

    fn _infix_binary_to_future_expression<'a>(scope: &'a mut Scope, ib: InfixBinary) -> Result<Vec<lazymath::abstract_algebra::IdentityExpressionDefinitionTerm>, NodeVisitationError>  {
        let operation: FutureValue<BinaryOperation> = match scope.get(&ib.operator.to_bytes()) {
            Some(ScopedItem::FutureBinaryOperation(o)) => o.clone(),
            Some(x) => return Err(NodeVisitationError::UnexpectedRegisteredItem(x.to_owned())),
            None => return Err(NodeVisitationError::RegisteredItemNotFound),
        };

        let left = (*ib.left_operand)._into_future_expression(scope)?;
        let right = (*ib.right_operand)._into_future_expression(scope)?;

        let mut result = vec![IdentityExpressionDefinitionTerm::BinaryOperation(operation)];
        result.extend(left);
        result.extend(right);

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::lang::tokens::MathOperatorSymbols;
    use super::*;

    #[test]
    fn test_binary_operations() {
        let tokens = [
            Token::id(b"a"), Token::op(MathOperatorSymbols::Plus), Token::id(b"b"), Token::op(MathOperatorSymbols::Star),
            Token::id(b"c"),
        ];

        let math_expression = *MathExpression::new(&mut tokens.into_iter().peekable()).unwrap();

        assert_eq!(math_expression,
            MathExpression::InfixBinary(
                InfixBinary {
                    left_operand: Box::new(MathExpression::Identifier(Identifier { lexeme: b"a".to_vec() })),
                    operator: MathOperatorSymbols::Plus,
                    right_operand: Box::new(MathExpression::InfixBinary(
                        InfixBinary {
                            left_operand: Box::new(MathExpression::Identifier(Identifier { lexeme: b"b".to_vec() })),
                            operator: MathOperatorSymbols::Star,
                            right_operand: Box::new(MathExpression::Identifier(Identifier { lexeme: b"c".to_vec() }))
                        }
                    )),
                }
            )
        );

    }
}