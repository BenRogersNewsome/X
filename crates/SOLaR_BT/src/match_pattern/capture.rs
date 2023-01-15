use std::iter::Cloned;

use crate::TreeNode;
use super::Node;
use super::Tree;


pub struct Capture<'a, T: Tree> {
    captured: Vec<&'a TreeNode<T>>,
    position: usize,
    polarity: isize,
}

impl<'a, T: Tree> Capture<'a, T> {

    #[inline]
    pub fn new(position: usize) -> Self {
        Self {
            captured: Vec::new(),
            position,
            polarity: 1,
        }
    }

    #[inline]
    pub fn captured_nodes<'b>(&'b self) -> Cloned<Cloned<std::slice::Iter<'b, &'a TreeNode<T>>>> {
        self.captured.iter().cloned().cloned()
    }

    /// Adds a token to the captured subtree and returns true if the subtree is complete.
    pub fn add_token(&mut self, node: &'a TreeNode<T>) -> bool {
        match node {
            &Node::Leaf(_) => self.polarity -= 1,
            &Node::Binary(_) => self.polarity += 1,
            &Node::Unary(_) => {},
        };

        self.captured.push(node);

        self.complete()
    }

    #[inline]
    pub fn complete(&self) -> bool {
        assert!(self.polarity >= 0);
        self.polarity == 0
    }
}