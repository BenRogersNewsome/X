use solar_bt::{LeafPattern, NodeSpecification, TreeNode, LeafReplacement, Tree};
use super::expression::Expression;

use zsft::{BinaryOperation, SetElement, UnaryOperation};

use super::token_tree::VecTree;

#[derive(PartialEq, Clone, Debug)]
pub enum ElementPattern {
    Literal(SetElement),
}

impl NodeSpecification<SetElement> for ElementPattern {
    fn is_match(&self, node: &SetElement) -> bool {
        match self {
            Self::Literal(literal) => literal == node
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum OperatorPattern<T> {
    Literal(T),
}

impl NodeSpecification<BinaryOperation> for OperatorPattern<BinaryOperation> {
    fn is_match(&self, node: &BinaryOperation) -> bool {
        match self {
            Self::Literal(literal) => literal == node
        }
    }
}

impl NodeSpecification<UnaryOperation> for OperatorPattern<UnaryOperation> {
    fn is_match(&self, node: &UnaryOperation) -> bool {
        match self {
            Self::Literal(literal) => literal == node
        }
    }
}

pub type ExpressionPattern = VecTree<LeafPattern<ElementPattern>, OperatorPattern<BinaryOperation>, OperatorPattern<UnaryOperation>>;

impl<Idx> std::ops::Index<Idx> for ExpressionPattern where Idx: std::slice::SliceIndex<[TreeNode<ExpressionPattern>]> {
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.nodes.index(index)
    }
}

pub type ExpressionReplacement = VecTree<LeafReplacement<Expression>, <Expression as Tree>::Binary, <Expression as Tree>::Unary>;

/// An identity contains two patterns, one is the matcher and the other the replacement, respectively.
///
/// Identities are assumed to be one way.
#[derive(Debug, Clone)]
pub struct Identity {
    pub left: ExpressionPattern,
    pub right: ExpressionReplacement,
}
