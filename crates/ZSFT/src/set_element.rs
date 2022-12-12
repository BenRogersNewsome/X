use rc_wrap::rc_wrap;
use std::ops::Deref;

use super::{set::Set, operation::{BinaryOperation, UnaryOperation}};


#[rc_wrap(
    #[derive(Debug, PartialEq, Eq)]
    pub SetElement
)]
#[derive(Debug)]
pub struct RawSetElement(SetElementDefinition);

impl PartialEq for RawSetElement {
    
    fn eq(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (SetElementDefinition::Anonymous, SetElementDefinition::Anonymous) => self._literally_equal(other),
            _ => self.0 == other.0,
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Eq for RawSetElement { }

impl Deref for RawSetElement {
    type Target = SetElementDefinition;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl RawSetElement {

    pub(self) fn new(set_element_definition: SetElementDefinition) -> Self {
        Self(set_element_definition)
    }

    fn _literally_equal(&self, other: &RawSetElement) -> bool {
        let addr_self: *const Self = self;
        let addr_other: *const Self = other;

        addr_self == addr_other
    }
}

#[derive(Debug)]
pub enum OperationApplicationError {
    InvalidArguments,
}

impl<'a> SetElement {
    pub fn new() -> Self {
        new_set_element![SetElementDefinition::Anonymous]
    }

    pub fn element_of(set: &Set) -> Self {
        new_set_element![SetElementDefinition::MemberOf(set.clone())]
    }

    pub fn from_binary_operation(op: &BinaryOperation, a: &SetElement, b: &SetElement) -> Result<Self, OperationApplicationError> {

        if !op.0.contains(&a) || !op.1.contains(&b) {
            return Err(OperationApplicationError::InvalidArguments);
        };

        Ok(new_set_element![SetElementDefinition::BinaryOperation(op.clone(), a.clone(), b.clone())])
    }

    pub(super) fn in_set(self: &SetElement, set: &Set) -> bool {
        match &self.0 {
            SetElementDefinition::Anonymous => set.contains(self),
            SetElementDefinition::MemberOf(s) => set == s,
            SetElementDefinition::BinaryOperation(o, _, _) => 
                set.contains(&Self::element_of(&o.2)),
            SetElementDefinition::UnaryOperation(o, _) => 
                set.contains(&Self::element_of(&o.1)),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SetElementDefinition {
    Anonymous,
    MemberOf(Set),
    BinaryOperation(BinaryOperation, SetElement, SetElement),
    UnaryOperation(UnaryOperation, SetElement),
}

#[cfg(test)]
mod test_set_element_equality {
    use super::super::operation::BinaryOperationDefinition;

    use super::*;

    #[test]
    fn test_set_element_from_operation_equality() {
        let set_a = Set::new();
        let set_b = Set::new();

        let set_c = Set::new();

        let op = BinaryOperation::new(
            BinaryOperationDefinition::new(&set_a, &set_b, &set_c)
        );

        let a = SetElement::element_of(&set_a);
        let b = SetElement::element_of(&set_b);
        let c = SetElement::from_binary_operation(&op, &a, &b).unwrap();
        let d = SetElement::from_binary_operation(&op, &a, &b).unwrap();

        assert_eq!(c, d);
        assert_ne!(a, c);
    }
}

#[cfg(test)]
mod test_set_element_membership{

    use super::super::operation::BinaryOperationDefinition;

    use super::*;

    #[test]
    fn test_set_element_from_operation_membership() {

        let set_a = Set::new();
        let set_b = Set::new();
        let set_c = Set::new();

        let op = BinaryOperation::new(
            BinaryOperationDefinition::new(&set_a, &set_b, &set_c)
        );
        
        let a = SetElement::element_of(&set_a);
        let b = SetElement::element_of(&set_b);

        let c = SetElement::from_binary_operation(&op, &a, &b).unwrap();

        assert!(set_a.contains(&a));
        assert!(set_b.contains(&b));
        assert!(!set_c.contains(&b));
        assert!(set_c.contains(&c));
        assert!(!set_c.contains(&a));
    }
}