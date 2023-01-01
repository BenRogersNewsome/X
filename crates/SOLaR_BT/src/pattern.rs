use crate::Tree;

pub trait NodeSpecification<NodeT> where Self: PartialEq + Clone {
    fn is_match(&self, node: &NodeT) -> bool;
}

#[derive(Clone, Debug, PartialEq)]
pub enum LeafPattern<NodeSpec> {
    Specification(NodeSpec),
    Subtree,
    SubtreeCallback(usize),
}

/// A marker trait for trees which can also function as patterns for another base tree, `T`.
pub trait PatternTree<T: Tree> where Self : Tree<
    Leaf = LeafPattern<Self::LeafSpec>,
    Binary = Self::BinarySpec,
    Unary = Self::UnarySpec,
>{
    type LeafSpec: NodeSpecification<T::Leaf>;
    type BinarySpec: NodeSpecification<T::Binary>;
    type UnarySpec: NodeSpecification<T::Unary>;
}

/// Auto implement PatternTree for all trees which have the correct nodes to be patterns.
impl<
    T: Tree,
    PT: Tree,
    LeafSpec: NodeSpecification<T::Leaf>,
    BinarySpec: NodeSpecification<T::Binary>,
    UnarySpec: NodeSpecification<T::Unary>,
> PatternTree<T> for PT where PT: Tree<
    Leaf = LeafPattern<LeafSpec>,
    Binary=BinarySpec,
    Unary=UnarySpec,
> {
    type LeafSpec = LeafSpec;
    type BinarySpec = BinarySpec;
    type UnarySpec = UnarySpec;
}