use std::rc::Rc;
use crate::algo::structures::{Expression, Identity};

#[derive(Clone)]
pub struct Simplification<'a> {
    pub identity: &'a Identity,
    pub direction: bool, // true == right, false == left
    pub position: usize,
    pub simplicity: usize,
    pub parent: Rc<Step<'a>>,
}

impl Simplification<'_> {

    pub fn backtrack<'a>(&'a self, positions: usize) -> Option<Rc<Step<'a>>> {
        if positions <= 1 {
            let simp = self.clone();
            return Some(Rc::new(Step::Simplification(simp)));
        }
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
        return Some(Rc::clone(current));
    }

}

pub struct Root<'a> {
    pub tree: &'a Expression
}

pub enum Step<'a> {
    Simplification(Simplification<'a>),
    Root(Root<'a>),
}