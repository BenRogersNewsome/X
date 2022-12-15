use super::MathExpression;
use crate::core::Stream;
use crate::lexical_analysis::tokens::Token;
use crate::core::{NextFn};

pub struct PostfixUnary {
    operator: Token,
    operand: MathExpression,
}


pub fn match_postfix_unary_operator(tokens: &mut dyn Stream<Token>, next: NextFn) -> MathExpression {
        let operator = match tokens.peek().token_type {
            x if matches!(x, token_type) => {tokens.next(); x},
            _ => return next(tokens)
        };
        
        let operand = match_postfix_unary_operator(tokens, next);

        PostfixUnary {
            operator,
            operand,
        }

}
