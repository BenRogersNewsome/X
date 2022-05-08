use crate::algo::structures::Expression;

use super::manipulation::Manipulation;

pub enum Step<'a> {
    Expression(Expression),
    Manipulation(Manipulation<'a>),
}

