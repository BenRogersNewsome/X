use crate::LBool;
use super::SetHandle;

#[derive(Debug)]
pub struct DifferenceSet {
    uid: u64,
    left: SetHandle,
    right: SetHandle,
}

impl DifferenceSet {
    pub fn contains(&self, item: &crate::item::Item) -> LBool {
        self.left.contains(item) & !self.right.contains(item)
    }

    pub fn uid(&self) -> u64 {
        self.uid
    }
    
    pub fn of(left: S, right: S) -> Self {
        // XXX
        Self {
            uid: 1,
            left,
            right,
        }
    }
}