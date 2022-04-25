use super::tree::{Tree, Node};
use super::nodes::{Element, Operator};

#[derive(Clone, PartialEq, Debug)]
pub struct Expression {
    tokens: Vec<Node<Element, Operator>>,
}

impl Tree for Expression {
    type L = Element;
    type I = Operator;

    fn new(tokens: Vec<Node<Element, Operator>>) -> Self {
        Expression {
            tokens
        }
    }

    #[inline(always)]
    fn tokens(&self) -> &Vec<Node<Element, Operator>> {
        &self.tokens
    }

    #[inline(always)]
    fn tokens_mut(&mut self) -> &mut Vec<Node<Element, Operator>> {
        &mut self.tokens
    }

    #[inline(always)]
    fn into_tokens(self) -> Vec<Node<Element, Operator>> {
        self.tokens
    }

    fn simplicity(&self) -> usize {
        return self.tokens.len()
    }
}

impl<Idx> std::ops::Index<Idx> for Expression where Idx: std::slice::SliceIndex<[Node<Element, Operator>]> {
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.tokens.index(index)
    }
}

/// When compiled with the -O flag, this macro leads to no runtime performance impact
#[macro_export]
macro_rules! expression {
    ($($x:expr),+ $(,)?) => {{
        use $crate::algo::structures::{Expression, Element, Operator, Node};
    
        pub trait Wrapped {
            fn get_wrapped(self) -> Node<Element, Operator>;
        }

        impl Wrapped for Element {
            #[inline(always)]
            fn get_wrapped(self) -> Node<Element, Operator> {
                return Node::Leaf(self)
            }
        }

        impl Wrapped for Operator {
            #[inline(always)]
            fn get_wrapped(self) -> Node<Element, Operator> {
                return Node::Internal(self)
            }
        }
        Expression::new(vec![$(($x).get_wrapped()),+])
    }}
}