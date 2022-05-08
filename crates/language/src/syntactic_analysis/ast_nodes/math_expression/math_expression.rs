use super::super::{Identifier};
use super::super::Node;
use super::{PostfixUnary, InfixBinary, match_postfix_unary_operator, binary_operator, primary};
use crate::core::{Stream, pipe};
use crate::lexical_analysis::tokens::Token;

pub enum MathExpression {
    Identifier(Identifier),
    InfixBinary(InfixBinary),
    PostfixUnary(PostfixUnary),
}

impl Node for MathExpression {
    fn new(tokens: &'a mut dyn Stream<Token>) -> Result<Box<Self>> {
        let piped = pipe(
            [
                binary_operator(),
                match_postfix_unary_operator,
            ]
        );

        return piped(tokens, primary);
    }
}

/*
 * term = (term, ({'+'} | {'-'}), comma) | comma;
 * comma = (comma, {','}, dot) | dot
 * dot = (dot, {'.'}, factor) | factor
 * factor = (factor, ({'*'} | {'/'}), power) | power;
 * power = (power, {'^'}, unary) | unary
 *
 * unary = (operator, unary) | primary;
 *
 * primary = symbol | "(", expression, ")";
 */