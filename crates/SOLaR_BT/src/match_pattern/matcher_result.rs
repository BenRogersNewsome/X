// use crate::{ReplacementElement, Node};

// use super::Tree;
// use super::capture::Capture;

// pub enum MatchTreeNode<'a, T: Tree> {
//     Literal(&'a Node<T>),
//     Capture(Capture<'a, T>),
// }

// /// A tree holding the matched subtree of the pattern in the host tree.
// pub struct MatchTree<'a, T: Tree + 'a> {
//     nodes: Vec<TreeNode<Self>>,
// }

// impl<'a, T: Tree + 'a> MatchTree<'a, T> {
//     pub fn len(&self) -> usize {
//         self.nodes.len()
//     }
// }

// impl<'a, T: Tree + 'a> Tree for MatchTree<'a, T> {
//     type Leaf = MatchTreeNode<'a, T>;
//     type Binary = T::Binary;
//     type Unary = T::Unary;

//     fn new<I: IntoIterator<Item = TreeNode<Self>>>(nodes: I) -> Self {
//         Self {
//             nodes: nodes.into_iter().collect()
//         }
//     }

//     fn iter<'b>(&'b self) -> std::slice::Iter<TreeNode<Self>> {
//         self.nodes.iter()
//     }
// }

// /// The result of a match of a pattern on a tree
// pub struct MatcherResult<'a, T: Tree> {
//     original_tree: &'a T,
//     start_position: usize,
//     end_position: usize,
//     matched_subtree: MatchTree<'a, T>,
//     captures: Vec<Capture<'a, T>>,
// }

// impl<'a, T: Tree> MatcherResult<'a, T> {

//     #[inline]
//     pub fn new(
//         original_tree: &'a T,
//         start_position: usize,
//         end_position: usize,
//         matched_subtree: MatchTree<'a, T>,
//         captures: Vec<&'a Capture<'a, T>>,
//     ) -> Self {
//         Self {
//             original_tree,
//             start_position,
//             end_position,
//             matched_subtree,
//             captures,
//         }
//     }

//     pub fn apply_replacement<
//         ReplacementTree: Tree<
//             Leaf = ReplacementElement<T>,
//             Binary = T::Binary,
//             Unary = T::Unary
//         >
//     >(&self, replacement: ReplacementTree) -> T where T: Clone, T::Binary: Clone, T::Unary: Clone, T::Leaf: Clone {

//         let mut resultant_tree_nodes = 
//             self.original_tree.clone().iter()
//                 .cloned()
//                 .take(self.start_position)
//                 .chain(
//                     replacement.iter().flat_map::<Box<dyn Iterator<Item = Node<T>>>, _>(|replacement_tree_node| {
//                         match *replacement_tree_node {
//                             Node::Binary(b) =>
//                                 Box::new(std::iter::once(Node::Binary(b))),
//                             Node::Unary(u) =>
//                                 Box::new(std::iter::once(Node::Unary(u))),
//                             Node::Leaf(ReplacementElement::Literal(literal_tree)) =>
//                                 Box::new(literal_tree.iter().cloned()),
//                             Node::Leaf(ReplacementElement::Captured(c)) => 
//                                 Box::new(self.captures[c].captured_nodes().cloned()),
//                         }
//                     })
//                 )
//                 .chain(
//                     self.original_tree.iter()
//                         .cloned()
//                         .skip(self.end_position)
//                 );
        
//         return T::new(resultant_tree_nodes)
//     }

// }