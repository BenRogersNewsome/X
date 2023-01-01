/// If a tree is to be constructable from a string, it needs to be composed of elements which implement `ParsableTreeNode`
pub trait ParsableTreeNode where Self: Sized {
    fn from_string(id: &str) -> Result<Self, &'static str>;
}

/// All tree nodes are wrapped in the `_Node` type to express whether the node is a leaf or branch node
#[derive(Debug, PartialEq, Clone)]
pub enum Node<Leaf: Clone, Binary: PartialEq + Clone, Unary: PartialEq + Clone> {
    Leaf(Leaf),
    Binary(Binary),
    Unary(Unary),
}

/// A utility type for constructing the `_Node` type from a given tree.
pub type TreeNode<T> = Node<
    <T as Tree>::Leaf,
    <T as Tree>::Binary,
    <T as Tree>::Unary,
>;

/// 
pub trait Tree where Self: Sized + PartialEq + IntoIterator<Item = Node<Self::Leaf, Self::Binary, Self::Unary>> + Clone {
    type Leaf: PartialEq + Clone;
    type Binary: PartialEq + Clone;
    type Unary: PartialEq + Clone;

    fn new<I: IntoIterator<Item = TreeNode<Self>>>(nodes: I) -> Self;

    /// Join two trees together with an internal node
    fn join(operator: Self::Binary, tree_1: Self, tree_2: Self) -> Self {
        let nodes: Vec<TreeNode<Self>> = std::iter::once(Node::Binary(operator))
            .chain(tree_1.into_iter())
            .chain(tree_2.into_iter())
            .collect();
        Self::new(nodes)
    }

    fn iter<'b>(&'b self) -> std::slice::Iter<TreeNode<Self>>;
}


// #[cfg(test)]
// mod tests {

//     use super::{Tree, Node, TreeNode};

//     impl TreeNode for String {
//         fn to_string(&self) -> String {
//             self.clone()
//         }
//     }

//     struct TestTree {
//         tokens: Vec<Node<String, String>>,
//     }

//     impl Tree for TestTree {
//         type L = String;
//         type I = String;

//         fn new(tokens: Vec<Node<String, String>>) -> Self {
//             Self {
//                 tokens,
//             }
//         }

//         fn tokens(&self) -> &Vec<Node<Self::L, Self::I>> {
//             &self.tokens
//         }

//         fn tokens_mut<'a>(&'a mut self) -> &'a mut Vec<Node<Self::L, Self::I>> {
//             &mut self.tokens
//         }

//         fn into_tokens(self) -> Vec<Node<Self::L, Self::I>> {
//             self.tokens
//         }
//     }

//     #[test]
//     fn test_join(){
//         let tree_1 = TestTree::new(vec![
//             Node::Binary(String::from("+")),
//             Node::Leaf(String::from("a")),
//             Node::Leaf(String::from("b")),
//         ]);

//         let tree_2 = TestTree::new(vec![
//             Node::Binary(String::from("*")),
//             Node::Leaf(String::from("c")),
//             Node::Leaf(String::from("d")),
//         ]);

//         let joined = TestTree::join(String::from("-"), tree_1, tree_2);

//         assert_eq!(joined.tokens, vec![
//             Node::Binary(String::from("-")),
//             Node::Binary(String::from("+")),
//             Node::Leaf(String::from("a")),
//             Node::Leaf(String::from("b")),
//             Node::Binary(String::from("*")),
//             Node::Leaf(String::from("c")),
//             Node::Leaf(String::from("d")),
//         ]);
//     }

//     #[test]
//     fn test_simplicity(){
//         let tree = TestTree::new(vec![
//             Node::Binary(String::from("+")),
//             Node::Leaf(String::from("a")),
//             Node::Leaf(String::from("b")),
//         ]);

//         assert_eq!(tree.simplicity(), 3);
//     }

//     #[test]
//     fn test_iter(){
//         let tree = TestTree::new(vec![
//             Node::Binary(String::from("+")),
//             Node::Leaf(String::from("a")),
//             Node::Leaf(String::from("b")),
//         ]);
        
//         assert!(tree.iter().eq(tree.tokens.iter()));
//     }
// }