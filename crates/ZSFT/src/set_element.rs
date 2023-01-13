use std::{ops::Deref, fmt::Debug};

use crate::operation::BinaryOperationDefinition;

use super::{set::Set, operation::{BinaryOperation, UnaryOperation}, LBool};

pub struct SetElement<'a>(pub(crate) SetElementDefinition<'a>, u64);

impl<'a> PartialEq for SetElement<'a> {
    
    fn eq(&self, other: &Self) -> bool {
        self._literally_equal(other)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl<'a> Eq for SetElement<'a>{ }

impl<'a> Deref for SetElement<'a> {
    type Target = SetElementDefinition<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> SetElement<'a> {
    fn _literally_equal(&self, other: &'a SetElement<'a>) -> bool {
        let addr_self: *const Self = self;
        let addr_other: *const Self = other;

        addr_self == addr_other
    }
}

#[derive(Debug)]
pub enum OperationApplicationError {
    InvalidArguments,
}

impl<'a> Debug for SetElement<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!();
        f.write_str(&format!("SetElement"))?;
        Ok(())
    }
}

impl<'a> SetElement<'a> {

    pub fn element_of(set: &Set) -> Self {
        Self(SetElementDefinition::ElementOf(set.clone()), rand::random())
    }

    #[inline]
    pub fn new(def: SetElementDefinition<'a>) -> Self {
        Self(def, rand::random())
    }

    pub fn from_binary_operation(op: &'a BinaryOperation, a: &'a SetElement<'a>, b: &'a SetElement<'a>) -> Result<Self, OperationApplicationError> {
        if *!a.in_set(&op.0.0) | *!b.in_set(&op.0.1) {
            return Err(OperationApplicationError::InvalidArguments);
        };

        Ok(Self::new(SetElementDefinition::BinaryOperation(op, a, b)),)
    }

    pub fn from_unary_operation(op: &'a UnaryOperation, a: &'a SetElement<'a>) -> Result<Self, OperationApplicationError> {
        if *!a.in_set(&op.0.0) {
            return Err(OperationApplicationError::InvalidArguments);
        };

        Ok(Self::new(SetElementDefinition::UnaryOperation(op, a)))
    }

    pub(super) fn in_set(self: &'a SetElement<'a>, set: &'a Set) -> LBool {
        match &self.0 {
            SetElementDefinition::ElementOf(s) => LBool::from(set == s),
            SetElementDefinition::BinaryOperation(o, _, _) => 
                Self::element_of(&o.2).in_set(set),
            SetElementDefinition::UnaryOperation(o, _) => 
                Self::element_of(&o.1).in_set(set),
        }
    }

    pub fn id(&self) -> u64 {
        self.1
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SetElementDefinition<'a> {
    ElementOf(Set),
    BinaryOperation(&'a BinaryOperation, &'a SetElement<'a>, &'a SetElement<'a>),
    UnaryOperation(&'a UnaryOperation, &'a SetElement<'a>),
}

// #[cfg(test)]
// mod test_set_element_equality {
//     use super::super::operation::BinaryOperationDefinition;

//     use super::*;

//     #[test]
//     fn test_set_element_from_operation_equality() {
//         let set_a = Set::anonymous();
//         let set_b = Set::anonymous();

//         let set_c = Set::anonymous();

//         let op = BinaryOperation::new(
//             BinaryOperationDefinition::new(&set_a, &set_b, &set_c)
//         );

//         let a = SetElement::element_of(&set_a);
//         let b = SetElement::element_of(&set_b);
//         let c = SetElement::from_binary_operation(&op, &a, &b).unwrap();
//         let d = SetElement::from_binary_operation(&op, &a, &b).unwrap();

//         assert_eq!(c, d);
//         assert_ne!(a, c);
//     }
// }

// #[cfg(test)]
// mod test_set_element_membership{

//     use super::super::operation::BinaryOperationDefinition;

//     use super::*;

//     #[test]
//     fn test_set_element_from_operation_membership() {

//         let set_a = Set::anonymous();
//         let set_b = Set::anonymous();
//         let set_c = Set::anonymous();

//         let op = BinaryOperation::new(
//             BinaryOperationDefinition::new(&set_a, &set_b, &set_c)
//         );
        
//         let a = SetElement::element_of(&set_a);
//         let b = SetElement::element_of(&set_b);

//         let c = SetElement::from_binary_operation(&op, &a, &b).unwrap();

//         assert!(*set_a.contains(&a));
//         assert!(*set_b.contains(&b));
//         assert!(*!set_c.contains(&b));
//         assert!(*set_c.contains(&c));
//         assert!(*!set_c.contains(&a));
//     }
// }