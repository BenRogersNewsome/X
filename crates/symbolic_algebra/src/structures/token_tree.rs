use std::ops::Deref;
use solar_bt::{Tree, Node, TreeNode};

/// An structure for storing algebraic expressions in their pre-traversal representation.
///
/// Implements the tree trait in SOLaRBT for pattern matching.
#[derive(Clone, PartialEq, Debug)]
pub struct TokenTree<L: TreeNode, I: TreeNode> {
    pub tokens: Vec<Node<L, I>>,
}

impl<L: TreeNode, I: TreeNode> TokenTree<L, I> {

    pub fn new(tokens: Vec<Node<L, I>>) -> Self {
        Self {
            tokens
        }
    }

}


impl<L: TreeNode, I: TreeNode> ToString for TokenTree<L, I> {
    fn to_string(&self) -> String {
        let mut string_rep = String::new();

        for token in &self.tokens {
            match token {
                Node::Leaf(e) => string_rep.push_str(&e.to_string()),
                Node::Internal(o) => string_rep.push_str(&o.to_string()),
            };
        }

        string_rep
    }
}


impl<L: TreeNode, I: TreeNode> Deref for TokenTree<L, I> {
    type Target = Vec<Node<L, I>>;

    fn deref(&self) -> &Self::Target {
        &self.tokens
    }
}

impl<L: TreeNode, I: TreeNode> Tree for TokenTree<L, I> {
    type L = L;
    type I = I;

    fn new(tokens: Vec<Node<L, I>>) -> Self {
        Self {
            tokens
        }
    }

    #[inline(always)]
    fn tokens(&self) -> &Vec<Node<L, I>> {
        &self.tokens
    }

    #[inline(always)]
    fn tokens_mut(&mut self) -> &mut Vec<Node<L, I>> {
        &mut self.tokens
    }

    #[inline(always)]
    fn into_tokens(self) -> Vec<Node<L, I>> {
        self.tokens
    }
}