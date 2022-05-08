use crate::tree::{TreeNode, Tree};

pub enum PatternLeaf<L: TreeNode> {
    Literal(L),
    Subtree(String),
}

impl<L: TreeNode> TreeNode for PatternLeaf<L> {

    fn to_string(&self) -> String {
        match self {
            PatternLeaf::Literal(l) => {
                l.to_string()
            },
            PatternLeaf::Subtree(s) => {
                String::from(s)
            },
        }
    }

}

/// This is a 'trait alias'
pub trait Pattern where Self: Tree<L=PatternLeaf<Self::PL>, I=Self::PI> {
    type PL: TreeNode;
    type PI: TreeNode;
}
impl<T, L: TreeNode, I: TreeNode> Pattern for T where T: Tree<L=PatternLeaf<L>, I=I> + ?Sized {
    type PL = L;
    type PI = I;
}