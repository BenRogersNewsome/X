use crate::LBool;

use super::Set;

#[derive(Debug)]
pub struct IntersectionSet<'a> {
    uid: u64,
    left: &'a dyn Set,
    right: &'a dyn Set,
}

impl<'a> Set for IntersectionSet<'a> {
    fn contains(&self, item: &crate::item::Item) -> LBool {
        self.left.contains(item) & self.right.contains(item)
    }

    fn uid(&self) -> u64 {
        
    }
}

impl<'a> IntersectionSet<'a> {
    pub fn of(left: &'a dyn Set, right: &'a dyn Set) -> Self {
        Self {
            left,
            right,
        }
    }
}