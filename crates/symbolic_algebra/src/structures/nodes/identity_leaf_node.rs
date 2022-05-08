use crate::algo::trees::TreeNode;
use super::element::Element;

#[derive(PartialEq, Debug, Clone)]
pub enum IdentityLeafNode {
    Element(Element),
    Subtree(Element)
}

impl TreeNode for IdentityLeafNode {
    fn to_string(&self) -> String {
        match self {
            Self::Element(e) | Self::Subtree(e) => e.to_string()
        }
    }
}