use std::iter::Peekable;

use crate::lexical_analysis::MathOperatorSymbols;
use crate::lexical_analysis::Token;
use crate::syntactic_analysis::ast::NodeParseError;
use super::MathExpression;
use super::primary::primary;


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InfixBinary {
    pub left_operand: Box<MathExpression>,
    pub operator: MathOperatorSymbols,
    pub right_operand: Box<MathExpression>,
}


macro_rules! matcher_function {
    ($name:ident, $operator:expr, $next:ident) => {
        pub fn $name<'a, T: Iterator<Item = Token>>(tokens: &'a mut Peekable<T>) -> Result<Box<MathExpression>, NodeParseError> {
            let left_operand = $next(tokens)?;
        
            if let Some(x) = tokens.peek() {

                if x.type_ == crate::lexical_analysis::TokenType::Symbol($operator) {
                    tokens.next();
    
                    let right_operand = $next(tokens)?;
                    Ok(Box::new(
                        MathExpression::InfixBinary(InfixBinary {
                            left_operand,
                            operator: $operator,
                            right_operand,
                        }
                    )))
                }else{
                    Ok(left_operand)
                }
            }else{
                Err(NodeParseError::UnexpectedEndOfInput)
            }
        }
    };
}


matcher_function!(power, MathOperatorSymbols::Caret, primary);
matcher_function!(divide, MathOperatorSymbols::FSlash, power);
matcher_function!(times, MathOperatorSymbols::Star, divide);
matcher_function!(plus, MathOperatorSymbols::Plus, times);
matcher_function!(minus, MathOperatorSymbols::Minus, plus);