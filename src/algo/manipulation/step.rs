use std::rc::Rc;
use crate::algo::structures::{Expression, Identity};

pub struct Simplification<'a> {
    pub identity: &'a Identity,
    pub direction: bool, // true == right, false == left
    pub simplicity: usize,
    pub parent: Rc<Step<'a>>,
}

impl Simplification<'_> {

    pub fn backtrack<'a>(&'a self, positions: usize) -> Option<&Step<'a>> {
        let mut current = &self.parent;
        for _ in 1..positions {
            match &**current {
                Step::Root(_) => {
                    return None;
                },
                Step::Simplification(simp) => {
                    current = &simp.parent;
                }
            }
        };
        return Some(&current);
    }

}

pub struct Root<'a> {
    pub tree: &'a Expression
}

pub enum Step<'a> {
    Simplification(Simplification<'a>),
    Root(Root<'a>),
}