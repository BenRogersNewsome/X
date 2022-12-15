use solar_bt::{TreeNode};
use super::binary_operator::BinaryOperator;
use super::chain_operator::ChainOperator;

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