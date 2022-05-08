use std::rc::Rc;
use crate::algo::structures::Expression;

use super::step::Step;

pub struct Strand<'a> {
    pub step: Rc<Step<'a>>,
    pub current: Expression,
}