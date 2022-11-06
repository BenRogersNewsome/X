use core::slice::Iter;

/// Represents a node of a tree. All trees are composed of TreeNodes
pub trait TreeNode {
    fn to_string(&self) -> String;

    fn uid(&self) -> String {
        self.to_string()
    }
}

/// If a tree is to be constructable from a string, it needs to be composed of elements which implement `ParsableTreeNode`
pub trait ParsableTreeNode where Self: TreeNode + Sized {
    fn from_string(id: &str) -> Result<Self, &'static str>;
}

/// All tree nodes are wrapped in the `Node` type to express whether the node is a leaf or branch node
#[derive(Debug, PartialEq, Clone)]
pub enum Node<L: TreeNode, I: TreeNode> {
    Leaf(L),
    Internal(I),
}

/// 
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
        let mut tokens: Vec<Node<Self::L, Self::I>> = Vec::with_capacity(1 + tree_1.tokens().len() + tree_2.tokens().len());
        
        tokens.push(Node::Internal(operator));
        tokens.append(&mut tree_1.into_tokens());
        tokens.append(&mut tree_2.into_tokens());
        Self::new(tokens)
    }

    /// Lower is better
    fn simplicity(&self) -> usize {
        self.tokens().len()
    }
}

pub trait ParsableTree where Self: Tree {
    type L: ParsableTreeNode;
    type I: ParsableTreeNode;
}

impl<L, B, T> ParsableTree for T where T: Tree<L=L, I=B>, L: ParsableTreeNode, B: ParsableTreeNode {
    type L = L;
    type I = B;
}



#[cfg(test)]
mod tests {

    use super::{Tree, Node, TreeNode};

    impl TreeNode for String {
        fn to_string(&self) -> String {
            self.clone()
        }
    }

    struct TestTree {
        tokens: Vec<Node<String, String>>,
    }

    impl Tree for TestTree {
        type L = String;
        type I = String;

        fn new(tokens: Vec<Node<String, String>>) -> Self {
            Self {
                tokens,
            }
        }

        fn tokens(&self) -> &Vec<Node<Self::L, Self::I>> {
            &self.tokens
        }

        fn tokens_mut<'a>(&'a mut self) -> &'a mut Vec<Node<Self::L, Self::I>> {
            &mut self.tokens
        }

        fn into_tokens(self) -> Vec<Node<Self::L, Self::I>> {
            self.tokens
        }
    }

    #[test]
    fn test_join(){
        let tree_1 = TestTree::new(vec![
            Node::Internal(String::from("+")),
            Node::Leaf(String::from("a")),
            Node::Leaf(String::from("b")),
        ]);

        let tree_2 = TestTree::new(vec![
            Node::Internal(String::from("*")),
            Node::Leaf(String::from("c")),
            Node::Leaf(String::from("d")),
        ]);

        let joined = TestTree::join(String::from("-"), tree_1, tree_2);

        assert_eq!(joined.tokens, vec![
            Node::Internal(String::from("-")),
            Node::Internal(String::from("+")),
            Node::Leaf(String::from("a")),
            Node::Leaf(String::from("b")),
            Node::Internal(String::from("*")),
            Node::Leaf(String::from("c")),
            Node::Leaf(String::from("d")),
        ]);
    }

    #[test]
    fn test_simplicity(){
        let tree = TestTree::new(vec![
            Node::Internal(String::from("+")),
            Node::Leaf(String::from("a")),
            Node::Leaf(String::from("b")),
        ]);

        assert_eq!(tree.simplicity(), 3);
    }

    #[test]
    fn test_iter(){
        let tree = TestTree::new(vec![
            Node::Internal(String::from("+")),
            Node::Leaf(String::from("a")),
            Node::Leaf(String::from("b")),
        ]);
        
        assert!(tree.iter().eq(tree.tokens.iter()));
    }
}