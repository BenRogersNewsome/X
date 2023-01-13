use std::ops::Deref;
use solar_bt::{LeafPattern, NodeSpecification, TreeNode, LeafReplacement, Tree};
use super::expression::Expression;

use zsft::{BinaryOperation, SetElement, UnaryOperation};

use super::token_tree::VecTree;

#[derive(PartialEq, Clone, Debug)]
pub enum ElementPattern<'a> {
    Literal(&'a SetElement<'a>),
}

impl<'a> NodeSpecification<&'a SetElement<'a>> for ElementPattern<'a> {
    fn is_match<N: Deref<Target=&'a SetElement<'a>>>(&self, node: N) -> bool {
        match self {
            Self::Literal(literal) => *literal == *node
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum OperatorPattern<T> {
    Literal(T),
}

impl<'a> NodeSpecification<&'a BinaryOperation> for OperatorPattern<&BinaryOperation> {
    fn is_match<N: Deref<Target=&'a BinaryOperation>>(&self, node: N) -> bool {
        match self {
            Self::Literal(literal) => *literal == *node
        }
    }
}

impl<'a> NodeSpecification<&'a UnaryOperation> for OperatorPattern<&UnaryOperation> {
    fn is_match<N: Deref<Target=&'a UnaryOperation>>(&self, node: N) -> bool {
        match self {
            Self::Literal(literal) => *literal == *node
        }
    }
}

pub type ExpressionPattern<'a> = VecTree<LeafPattern<
    ElementPattern<'a>>,
    OperatorPattern<&'a BinaryOperation>,
    OperatorPattern<&'a UnaryOperation>
>;

impl<'a, Idx> std::ops::Index<Idx> for ExpressionPattern<'a> where Idx: std::slice::SliceIndex<[TreeNode<ExpressionPattern<'a>>]> {
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.nodes.index(index)
    }
}

pub type ExpressionReplacement<'a> = VecTree<LeafReplacement<Expression<'a>>, <Expression<'a> as Tree>::Binary, <Expression<'a> as Tree>::Unary>;

/// An identity contains two patterns, one is the matcher and the other the replacement, respectively.
///
/// Identities are assumed to be one way.
#[derive(Debug, Clone)]
pub struct Identity<'a> {
    pub left: ExpressionPattern<'a>,
    pub right: ExpressionReplacement<'a>,
}
