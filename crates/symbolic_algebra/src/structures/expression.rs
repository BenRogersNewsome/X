use crate::algo::trees::{Tree, Node};
use super::nodes::{Element, BinaryOperator};

#[derive(Clone, PartialEq, Debug)]
pub struct Expression {
    tokens: Vec<Node<Element, BinaryOperator>>,
}

impl Tree for Expression {
    type L = Element;
    type I = BinaryOperator;

    fn new(tokens: Vec<Node<Element, BinaryOperator>>) -> Self {
        Expression {
            tokens
        }
    }

    #[inline(always)]
    fn tokens(&self) -> &Vec<Node<Element, BinaryOperator>> {
        &self.tokens
    }

    #[inline(always)]
    fn tokens_mut(&mut self) -> &mut Vec<Node<Element, BinaryOperator>> {
        &mut self.tokens
    }

    #[inline(always)]
    fn into_tokens(self) -> Vec<Node<Element, BinaryOperator>> {
        self.tokens
    }
}

impl<Idx> std::ops::Index<Idx> for Expression where Idx: std::slice::SliceIndex<[Node<Element, BinaryOperator>]> {
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.tokens.index(index)
    }
}

/// When compiled with the -O flag, this macro leads to no runtime performance impact
#[macro_export]
macro_rules! expression {
    ($($x:expr),+ $(,)?) => {{
        use $crate::algo::structures::{Expression, Element, BinaryOperator};
        use $crate::algo::trees::{Tree, Node};
    
        pub trait Wrapped {
            fn get_wrapped(self) -> Node<Element, BinaryOperator>;
        }

        impl Wrapped for Element {
            #[inline(always)]
            fn get_wrapped(self) -> Node<Element, BinaryOperator> {
                return Node::Leaf(self)
            }
        }

        impl Wrapped for BinaryOperator {
            #[inline(always)]
            fn get_wrapped(self) -> Node<Element, BinaryOperator> {
                return Node::Internal(self)
            }
        }
        Expression::new(vec![$(($x).get_wrapped()),+])
    }}
}