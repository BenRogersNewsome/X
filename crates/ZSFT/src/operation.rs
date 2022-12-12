use rc_wrap::rc_wrap;
use std::ops::Deref;

use super::set::Set;


#[rc_wrap(
    #[derive(Debug, PartialEq, Eq)]
    pub BinaryOperation
)]
#[derive(Debug, PartialEq, Eq)]
pub struct RawBinaryOperation(BinaryOperationDefinition);

impl RawBinaryOperation {
    pub fn new(definition: BinaryOperationDefinition) -> Self {
        Self(definition)
    }
}

impl<'a> Deref for RawBinaryOperation {
    type Target = BinaryOperationDefinition;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl BinaryOperation {
    pub fn new(definition: BinaryOperationDefinition) -> Self {
        new_binary_operation!(definition)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BinaryOperationDefinition(pub Set, pub Set, pub Set);  // (A, B, C) ==> A x B -> C

impl BinaryOperationDefinition {
    pub fn new(x: &Set, y: &Set, z: &Set) -> Self {
        Self(x.clone(), y.clone(), z.clone())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UnaryOperation(UnaryOperationDefinition);

impl<'a> UnaryOperation {
    pub fn new(definition: UnaryOperationDefinition) -> Self {
        Self(definition)
    }
}

impl<'a> Deref for UnaryOperation {
    type Target = UnaryOperationDefinition;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UnaryOperationDefinition(pub Set, pub Set);  // (A, B) ==> A -> B

impl<'a> UnaryOperationDefinition {
    pub fn new(x: &Set, y: &Set) -> Self {
        Self(x.clone(), y.clone())
    }
}