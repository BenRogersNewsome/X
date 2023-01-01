use crate::Tree;

#[derive(Clone, Debug, PartialEq)]
pub enum LeafReplacement<T: Tree> {
    Literal(T),
    Captured(usize),
}