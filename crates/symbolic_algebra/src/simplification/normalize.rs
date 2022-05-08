use crate::algo::structures::{Expression, Algebra, Tree, TreeNode, Node, BinaryOperator, Element};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChainOperator {
    pub label: u8,
    pub order: usize,
}

impl TreeNode for ChainOperator {
    fn to_string(&self) -> String {
        String::from_utf8([self.label].to_vec()).unwrap()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Operator {
    Binary(BinaryOperator),
    Chain(ChainOperator),
}

impl TreeNode for Operator {
    fn to_string(&self) -> String {
        match self {
            Self::Binary(x) => x.to_string(),
            Self::Chain(x) => x.to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct NormalizedExpression {
    tokens: Vec<Node<Element, Operator>>,
}

impl Tree for NormalizedExpression {
    type L = Element;
    type I = Operator;

    fn new(tokens: Vec<Node<Element, Operator>>) -> Self {
        NormalizedExpression {
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
}

fn normalize(expression: &Expression, algebra: &Algebra) -> NormalizedExpression {
    
}