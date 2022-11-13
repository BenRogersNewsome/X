use std::ops::Deref;

use super::set::Set;


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BinaryOperation<'a>(BinaryOperationDefinition<'a>);

impl<'a> BinaryOperation<'a> {
    pub fn new(definition: BinaryOperationDefinition<'a>) -> Self {
        Self(definition)
    }
}

impl<'a> Deref for BinaryOperation<'a> {
    type Target = BinaryOperationDefinition<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BinaryOperationDefinition<'a>(pub &'a Set<'a>, pub &'a Set<'a>, pub &'a Set<'a>);  // (A, B, C) ==> A x B -> C

impl<'a> BinaryOperationDefinition<'a> {
    pub fn new<'b: 'a>(x: &'b Set, y: &'b Set, z: &'b Set) -> Self {
        Self(x, y, z)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UnaryOperation<'a>(UnaryOperationDefinition<'a>);

impl<'a> UnaryOperation<'a> {
    pub fn new(definition: UnaryOperationDefinition<'a>) -> Self {
        Self(definition)
    }
}

impl<'a> Deref for UnaryOperation<'a> {
    type Target = UnaryOperationDefinition<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UnaryOperationDefinition<'a>(pub &'a Set<'a>, pub &'a Set<'a>);  // (A, B) ==> A -> B

impl<'a> UnaryOperationDefinition<'a> {
    pub fn new<'b: 'a>(x: &'b Set, y: &'b Set) -> Self {
        Self(x, y)
    }
}