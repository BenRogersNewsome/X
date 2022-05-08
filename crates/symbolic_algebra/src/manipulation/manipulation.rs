use std::rc::Rc;
use crate::algo::structures::Identity;

use super::step::Step;

/// A manipulation is a record of a change to an expression
pub struct Manipulation<'a> {
    parent: Rc<Step<'a>>,
    identity: &'a Identity,
    position: usize,
}