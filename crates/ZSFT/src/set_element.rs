use std::{fmt::Debug, rc::Rc, cell::UnsafeCell};
use enum_dispatch::enum_dispatch;
use crate::BinaryOperation;

use super::{set::Set, LBool};

mod in_set;
mod is_operation;
pub use in_set::InSet;
pub use is_operation::IsBinaryOperation;

#[derive(Debug)]
pub struct SetElement {
    id: u64,
    raw: Rc<UnsafeCell<SetElementType>>,
}

impl SetElement {

    #[inline]
    pub fn anonymous() -> Self {
        Self::new(SetElementType::Anonymous(()))
    }

    pub fn element_of(set: &Set) -> Self {
        let new_element = Self::anonymous();
        InSet::assert_on(&new_element, set);
        new_element
    }

    #[inline]
    pub fn from_binary_operation(op: &BinaryOperation, left: &SetElement, right: &SetElement) -> Option<Self> {
        IsBinaryOperation::new(
            op,
            left,
            right
        ).map(|op| {
            Self::new(SetElementType::BinaryOperation(op))
        })
    }

    pub fn new(type_: SetElementType) -> Self {
        Self {
            id: rand::random(),
            raw: Rc::new(UnsafeCell::new(type_)),
        }
    }
    
    pub fn replace<Callback>(&self, creator: Callback) where Callback: FnOnce(SetElementType) -> SetElementType {
        // SAFETY: We wrap the old inner with the new inner so that there is no
        // duplication. We use a creator callback so that we never expose a
        // dangling reference to external code.
        // TODO: Is this safe with the bound as an FnOnce?
        unsafe {
            let inner_mut: &mut SetElementType = &mut *self.raw.get();
            let inner: SetElementType = std::ptr::read(inner_mut);
            let new: SetElementType = creator(inner);
            std::ptr::write(inner_mut, new);
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    #[inline]
    unsafe fn get_inner_set(&self) -> &SetElementType {
        &*self.raw.get()
    }

    pub fn get(&self) -> SetElementType {
        unsafe {
            (*self.raw.get()).clone()
        }
    }

    pub fn in_set(&self, set: &Set) -> LBool {
        self.in_set_(set, &mut Vec::new())
    }

    pub(crate) fn in_set_(&self, set: &Set, signature: &mut Vec<u64>) -> LBool {
        let inner = unsafe { self.get_inner_set() };
        inner.in_set(self, set, signature)
    }
}


impl<'a> PartialEq for SetElement {
    
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl<'a> Eq for SetElement { }

impl Clone for SetElement {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            raw: self.raw.clone(),
        }
    }
}

// impl<'a> SetElement<'a> {

//     pub fn alias_of(set_element: &'a SetElement<'a>) -> Self {
//         Self::new(SetElementDefinition::Alias(set_element))
//     }

//     pub fn element_of(set: &Set) -> Self {
//         Self(SetElementDefinition::ElementOf(set.clone()), rand::random())
//     }

//     #[inline]
//     pub fn new(def: SetElementDefinition<'a>) -> Self {
//         Self(def, rand::random())
//     }

//     pub fn from_binary_operation(op: &'a BinaryOperation, a: &'a SetElement<'a>, b: &'a SetElement<'a>) -> Result<Self, OperationApplicationError> {
//         if *!a.in_set(&op.0.0) | *!b.in_set(&op.0.1) {
//             return Err(OperationApplicationError::InvalidArguments);
//         };

//         Ok(Self::new(SetElementDefinition::BinaryOperation(op, a, b)),)
//     }

//     pub fn from_unary_operation(op: &'a UnaryOperation, a: &'a SetElement<'a>) -> Result<Self, OperationApplicationError> {
//         if *!a.in_set(&op.0.0) {
//             return Err(OperationApplicationError::InvalidArguments);
//         };

//         Ok(Self::new(SetElementDefinition::UnaryOperation(op, a)))
//     }

//     pub(super) fn in_set(self: &'a SetElement<'a>, set: &'a Set) -> LBool {
//         match &self.0 {
//             SetElementDefinition::Alias(e) => e.in_set(set),
//             SetElementDefinition::ElementOf(s) => LBool::from(set == s),
//             SetElementDefinition::BinaryOperation(o, _, _) => 
//                 Self::element_of(&o.2).in_set(set),
//             SetElementDefinition::UnaryOperation(o, _) => 
//                 Self::element_of(&o.1).in_set(set),
//         }
//     }

//     pub fn id(&self) -> u64 {
//         self.1
//     }
// }

#[enum_dispatch]
pub trait SetElementLayer where Self: Clone {
    fn in_set(&self, element: &SetElement, set: &Set, signature: &mut Vec<u64>) -> LBool;
}

impl SetElementLayer for () {
    fn in_set(&self,element: &SetElement,set: &Set,signature: &mut Vec<u64>) -> LBool {
        LBool::Unknown
    }
}

#[enum_dispatch(SetElementLayer)]
#[derive(Debug, Clone)]
pub enum SetElementType {
    Anonymous(()),
    InSet(InSet),
    BinaryOperation(IsBinaryOperation)
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