use std::rc::Rc;

use zsft::{BinaryOperation, SetElement};

#[derive(PartialEq, Eq, Debug)]
pub enum ExpressionTerm {
    Element(Rc<SetElement>),
    BinaryOperation(Rc<BinaryOperation>),
}

/// A mathematical expression, e.g. `a+2*b`, expressed as a tree in it's pre-traversal representation.
pub type Expression = Vec<ExpressionTerm>;

pub struct Equality {
    pub left: Expression,
    pub right: Expression,
}