use crate::tree::TreeNode;

#[derive(Clone, Debug, PartialEq)]
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