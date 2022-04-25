use super::MathExpression;
use crate::core::Stream;
use crate::lexical_analysis::tokens::Token;
use crate::lexical_analysis::tokens::TokenType;
use crate::core::{pipe, NextFn, ParserFn};

// Binary operators in order of precedence
const BINARY_OPERATORS: [TokenType; 3] = [
    TokenType::Star,
    TokenType::Slash,
    TokenType::Plus,
];

pub struct InfixBinary {
    left_operand: MathExpression,
    operator: Token,
    right_operand: MathExpression,
}

pub fn binary_operator(){
    pipe(
        BINARY_OPERATORS.map(|token_type| generate_match_infix_binary_operator(token_type))
    );
}

fn generate_match_infix_binary_operator(token_type: TokenType) -> ParserFn<&mut dyn Stream<Token>, MathExpression> {
    |tokens: &mut dyn Stream<Token>, next: NextFn| -> MathExpression {
        let left_operand = next(tokens);

        let operator = match tokens.peek().token_type {
            x if matches!(x, token_type) => {tokens.next(); x},
            _ => return left_operand
        };

        let right_operand = next(tokens);

        InfixBinary {
            left_operand,
            right_operand,
            operator,
        }
    }
}



