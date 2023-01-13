use std::{ops::Deref, fmt::Debug};

use crate::Set;


pub struct BinaryOperation(pub(crate) BinaryOperationDefinition, u64);

impl PartialEq for BinaryOperation {
    
    fn eq(&self, other: &Self) -> bool {
        let raw_self: *const Self = self;
        let raw_other: *const Self = other;
        raw_self == raw_other
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Eq for BinaryOperation { }

impl BinaryOperation {
    pub fn new(definition: BinaryOperationDefinition) -> Self {
        Self(definition, rand::random())
    }
}

impl Deref for BinaryOperation {
    type Target = BinaryOperationDefinition;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Debug for BinaryOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!();
        Ok(())
    }
}

impl BinaryOperation {
    pub fn from_signature(a: &Set, b: &Set, c: &Set) -> Self {
        Self::new(BinaryOperationDefinition::new(a, b, c))
    }

    pub fn id(&self) -> u64 {
        self.1
    }
}

#[derive(Clone, Debug)]
pub struct BinaryOperationDefinition(pub(crate) Set, pub(crate) Set, pub(crate) Set);  // (A, B, C) ==> A x B -> C

impl BinaryOperationDefinition {
    pub fn new(a: &Set, b: &Set, c: &Set) -> Self {
        Self(a.clone(), b.clone(), c.clone())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UnaryOperation(pub(crate) UnaryOperationDefinition, u64);

impl UnaryOperation {
    pub fn new(definition: UnaryOperationDefinition) -> Self {
        Self(definition, rand::random())
    }

    pub fn from_signature(a: &Set, b: &Set) -> Self {
        Self::new(UnaryOperationDefinition::new(a, b))
    }

    pub fn id(&self) -> u64 {
        self.1
    }
}

impl Deref for UnaryOperation {
    type Target = UnaryOperationDefinition;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnaryOperationDefinition(pub(crate) Set, pub(crate) Set);  // (A, B) ==> A -> B

impl UnaryOperationDefinition {
    pub fn new(x: &Set, y: &Set) -> Self {
        Self(x.clone(), y.clone())
    }
}