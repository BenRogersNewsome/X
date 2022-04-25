use super::tree::{Tree, Node};
use super::nodes::{Operator, IdentityLeafNode};

pub struct IdentityExpression{
    tokens: Vec<Node<IdentityLeafNode, Operator>>,
}

impl Tree for IdentityExpression {
    type L = IdentityLeafNode;
    type I = Operator;

    fn new(tokens: Vec<Node<IdentityLeafNode, Operator>>) -> Self {
        IdentityExpression {
            tokens
        }
    }

    #[inline(always)]
    fn tokens(&self) -> &Vec<Node<IdentityLeafNode, Operator>> {
        &self.tokens
    }

    #[inline(always)]
    fn tokens_mut(&mut self) -> &mut Vec<Node<IdentityLeafNode, Operator>> {
        &mut self.tokens
    }

    #[inline(always)]
    fn into_tokens(self) -> Vec<Node<IdentityLeafNode, Operator>> {
        self.tokens
    }
}

impl<Idx> std::ops::Index<Idx> for IdentityExpression where Idx: std::slice::SliceIndex<[Node<IdentityLeafNode, Operator>]> {
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.tokens.index(index)
    }
}

pub struct Identity(pub IdentityExpression, pub IdentityExpression);

#[macro_export]
macro_rules! identity_expression {
    ($($x:expr),+ $(,)?) => {{
        use $crate::algo::structures::{Tree, IdentityExpression, Node, IdentityLeafNode, IdentityLeafNode::*, Operator, Element};
    
        pub trait Wrapped {
            fn to_wrapped(self) -> Node<IdentityLeafNode, Operator>;
        }

        impl Wrapped for IdentityLeafNode {
            #[inline(always)]
            fn to_wrapped(self) -> Node<IdentityLeafNode, Operator> {
                return Node::Leaf(self)
            }
        }

        impl Wrapped for Operator {
            #[inline(always)]
            fn to_wrapped(self) -> Node<IdentityLeafNode, Operator> {
                return Node::Internal(self)
            }
        }
        IdentityExpression::new(vec![$($x.to_wrapped()),+])
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