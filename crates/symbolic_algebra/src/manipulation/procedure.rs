use crate::structures::ExpressionPattern;

/// A statement of how an expression might be simplified, expressed as one or more patterns followed by a general replacement.
/// 
/// A 'procedure' is a looser statement than an identity: Where an identity is a statement of equality between two patterns, a procedure is a
/// many-to-one relation of pattern to replacement which can be used to rapidly simplify complex expressions.
/// 
/// Computationally, a procedure can be thought of as a pre-compilation of a number of base identities into larger and more useful replacements.
pub struct Procedure<'a> {
    left: &'a ExpressionPattern,
    right: &'a ExpressionPattern,
}

impl<'a> Procedure<'a> {

    pub fn new(left: &'a ExpressionPattern, right: &'a ExpressionPattern) -> Self {
        Procedure { left, right }
    }

    pub fn new_from_many(lefts: Vec<&'a ExpressionPattern>, right: &'a ExpressionPattern) -> Vec<Self> {
        lefts.iter().map(|left| {
            Procedure { left, right }
        }).collect()
    }

    pub fn value(&self) -> usize {
        1
    }
}

pub type A<'a> = Vec<Procedure<'a>>;