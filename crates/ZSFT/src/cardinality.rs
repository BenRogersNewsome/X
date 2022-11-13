use std::rc::Rc;

#[derive(PartialEq)]
pub enum Aleph {
    Null,
    N(AlephN),
}

#[derive(PartialEq)]
pub struct AlephN {
    greater_than: Rc<Aleph>,
    less_than: Option<Rc<Aleph>>,
}

#[derive(PartialEq)]
pub enum Cardinality {
    Finite(usize),
    Infinite(Aleph)
}