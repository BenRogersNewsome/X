use crate::algo::trees::{Tree, Node};
use super::nodes::{BinaryOperator, IdentityLeafNode};
use crate::algo::patterns::{PatternLeaf};
use super::nodes::Element;

#[derive(Clone, Debug)]
pub struct ExpressionPattern{
    tokens: Vec<Node<PatternLeaf<Element>, BinaryOperator>>,
}

impl Tree for ExpressionPattern {
    type L = IdentityLeafNode;
    type I = BinaryOperator;

    fn new(tokens: Vec<Node<IdentityLeafNode, BinaryOperator>>) -> Self {
        Self {
            tokens
        }
    }

    #[inline(always)]
    fn tokens(&self) -> &Vec<Node<IdentityLeafNode, BinaryOperator>> {
        &self.tokens
    }

    #[inline(always)]
    fn tokens_mut(&mut self) -> &mut Vec<Node<IdentityLeafNode, BinaryOperator>> {
        &mut self.tokens
    }

    #[inline(always)]
    fn into_tokens(self) -> Vec<Node<IdentityLeafNode, BinaryOperator>> {
        self.tokens
    }
}

impl<Idx> std::ops::Index<Idx> for ExpressionPattern where Idx: std::slice::SliceIndex<[Node<IdentityLeafNode, BinaryOperator>]> {
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.tokens.index(index)
    }
}

#[derive(Debug)]
pub struct Identity(pub ExpressionPattern, pub ExpressionPattern);

impl Identity {
    pub fn invert(&self) -> Self {
        Identity(self.1.clone(), self.0.clone())
    }
}

#[macro_export]
macro_rules! identity_expression {
    ($($x:expr),+ $(,)?) => {{
        use $crate::algo::structures::{ExpressionPattern, IdentityLeafNode, IdentityLeafNode::*, BinaryOperator, Element};
        use $crate::algo::trees::{Tree, Node};

        pub trait Wrapped {
            fn to_wrapped(self) -> Node<IdentityLeafNode, BinaryOperator>;
        }

        impl Wrapped for IdentityLeafNode {
            #[inline(always)]
            fn to_wrapped(self) -> Node<IdentityLeafNode, BinaryOperator> {
                return Node::Leaf(self)
            }
        }

        impl Wrapped for BinaryOperator {
            #[inline(always)]
            fn to_wrapped(self) -> Node<IdentityLeafNode, BinaryOperator> {
                return Node::Internal(self)
            }
        }
        ExpressionPattern::new(vec![$($x.to_wrapped()),+])
    }}
}

/// When compiled with the -O flag, this macro leads to no runtime performance impact
#[macro_export]
macro_rules! identity {
    ($($left:expr),+ $(,)?; $($right:expr),+ $(,)?) => {{
        use crate::identity_expression;
        Identity(
            identity_expression![$($left),+],
            identity_expression![$($right),+],
        )
    }}
}