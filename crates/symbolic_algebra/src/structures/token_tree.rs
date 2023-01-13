use std::{ops::Deref, fmt::Debug};
use solar_bt::{Tree, TreeNode};

/// An structure for storing algebraic expressions in their pre-traversal representation.
///
/// Implements the tree trait in SOLaRBT for pattern matching.
#[derive(Clone, PartialEq, Debug)]
pub struct VecTree<Leaf: PartialEq + Clone + Debug, Binary: PartialEq + Clone + Debug, Unary: PartialEq + Clone + Debug> {
    pub nodes: Vec<TreeNode<Self>>,
}

impl<Leaf: PartialEq + Clone + Debug, Binary: PartialEq + Clone + Debug, Unary: PartialEq + Clone + Debug> Deref for VecTree<Leaf, Binary, Unary> {
    type Target = Vec<TreeNode<Self>>;

    fn deref(&self) -> &Self::Target {
        &self.nodes
    }
}

impl<Leaf: PartialEq + Clone + Debug, Binary: PartialEq + Clone + Debug, Unary: PartialEq + Clone + Debug> IntoIterator for VecTree<Leaf, Binary, Unary> {
    type Item = TreeNode<Self>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.nodes.into_iter()
    }
}

impl<Leaf: PartialEq + Clone + Debug, Binary: PartialEq + Clone + Debug, Unary: PartialEq + Clone + Debug> Tree for VecTree<Leaf, Binary, Unary> {
    type Leaf = Leaf;
    type Binary = Binary;
    type Unary = Unary;

    fn new<I: IntoIterator<Item = TreeNode<Self>>>(nodes: I) -> Self {
        Self {
            nodes: nodes.into_iter().collect(),
        }
    }

    fn iter<'a>(&'a self) -> std::slice::Iter<TreeNode<Self>> {
        self.nodes.iter()
    }
}