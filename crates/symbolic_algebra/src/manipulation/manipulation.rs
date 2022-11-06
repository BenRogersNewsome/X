use std::rc::Rc;

use super::{step::Step, Manipulatable};

/// A manipulation is a record of a change to an expression
pub struct Manipulation<'a, T: Manipulatable<'a>> {
    pub parent: Rc<Step<'a, T>>,
    pub instruction: T::Instruction,
}