use core::slice::Iter;

pub trait TreeNode {
    fn to_string(&self) -> String;
}

#[derive(Debug, PartialEq, Clone)]
pub enum Node<L: TreeNode, I: TreeNode> {
    Leaf(L),
    Internal(I),
}

pub trait Tree where Self: Sized {
    type L: TreeNode;
    type I: TreeNode;

    fn new(tokens: Vec<Node<Self::L, Self::I>>) -> Self;

    fn tokens(&self) -> &Vec<Node<Self::L, Self::I>>;
    fn tokens_mut(&mut self) -> &mut Vec<Node<Self::L, Self::I>>;
    fn into_tokens(self) -> Vec<Node<Self::L, Self::I>>;

    fn iter(&self) -> Iter<Node<Self::L, Self::I>> {
        self.tokens().iter()
    }

    /// Join two trees together with an internal node
    fn join(operator: Self::I, tree_1: Self, tree_2: Self) -> Self {
        let mut tokens: Vec<Node<Self::L, Self::I>> = vec![
            Node::Internal(operator),
        ];
        tokens.append(&mut tree_1.into_tokens());
        tokens.append(&mut tree_2.into_tokens());
        Self::new(tokens)
    }

    /// Lower is better
    fn simplicity(&self) -> usize {
        self.tokens().len()
    }
}