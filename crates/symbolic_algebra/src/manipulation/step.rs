use super::{manipulation::Manipulation, Manipulatable};

pub enum Step<'a, T: Manipulatable<'a>> {
    Expression(T),
    Manipulation(Manipulation<'a, T>),
}

