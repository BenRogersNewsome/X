use crate::{Set, BinaryOperation, SetElement, LBool};

use super::SetElementLayer;


#[derive(Debug)]
pub enum OperationApplicationError {
    InvalidArguments,
}

#[derive(Debug, Clone)]
pub struct IsBinaryOperation {
    operation: BinaryOperation,
    left: SetElement,
    right: SetElement,
}

impl IsBinaryOperation {
    #[inline]
    pub fn new(operation: &BinaryOperation, left: &SetElement, right: &SetElement) -> Option<Self> {

        if left.in_set_(&operation.0.0, &mut Vec::new()) != LBool::True ||
            right.in_set_(&operation.0.1, &mut Vec::new()) != LBool::True
        {
            None
        } else {
            Some(Self {
                operation: operation.clone(),
                left: left.clone(),
                right: right.clone(),
            })
        }
    }
}

impl SetElementLayer for IsBinaryOperation {
    fn in_set(&self, element: &crate::SetElement, set: &Set,signature: &mut Vec<u64>) -> crate::LBool {
        if *set == self.operation.0.2 {
            LBool::True
        } else {
            set.contains_set_element_(element, signature)
        }
    }
}