
use std::ops::Deref;

use super::{set::Set, operation::{BinaryOperation, UnaryOperation}};


#[derive(Clone, Debug)]
pub struct SetElement<'a>(SetElementDefinition<'a>);

impl PartialEq for SetElement<'_>{
    
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

impl Eq for SetElement<'_> { }

impl<'a> Deref for SetElement<'a> {
    type Target = SetElementDefinition<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub enum OperationApplicationError {
    InvalidArguments,
}

impl<'a> SetElement<'a> {
    pub fn new() -> Self {
        Self(SetElementDefinition::Anonymous)
    }

    pub fn element_of<'b: 'a>(set: &'b Set) -> Self {
        Self(SetElementDefinition::MemberOf(set))
    }

    pub fn from_binary_operation<'c: 'a, 'b: 'a>(op: &'c BinaryOperation<'a>, a: &'b SetElement<'b>, b: &'b SetElement<'b>) -> Result<Self, OperationApplicationError> {

        if !op.0.contains(&a) || !op.1.contains(&b) {
            return Err(OperationApplicationError::InvalidArguments);
        };

        Ok(Self(SetElementDefinition::BinaryOperation(op, a, b)))
    }

    pub(super) fn in_set(&self, set: &Set<'a>) -> bool {
        match self.0 {
            SetElementDefinition::Anonymous => set.contains(self),
            SetElementDefinition::MemberOf(s) => set == s,
            SetElementDefinition::BinaryOperation(o, _, _) => 
                set.contains(&Self::element_of(o.2)),
            SetElementDefinition::UnaryOperation(o, _) => 
                set.contains(&Self::element_of(o.1)),
        }
    }

    fn _literally_equal(&self, other: &SetElement) -> bool {
        let addr_self: *const SetElement = self;
        let addr_other: *const SetElement = other;

        addr_self == addr_other
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SetElementDefinition<'a> {
    Anonymous,
    MemberOf(&'a Set<'a>),
    BinaryOperation(&'a BinaryOperation<'a>, &'a SetElement<'a>, &'a SetElement<'a>),
    UnaryOperation(&'a UnaryOperation<'a>, &'a SetElement<'a>),
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