use super::super::tree::TreeNode;

#[derive(PartialEq, Debug, Clone)]
pub enum Operator {
    Binary(BinaryOperator),
    Chain(ChainOperator),
}

impl TreeNode for Operator {
    fn to_string(&self) -> String {
        String::from_utf8([self.label].to_vec()).unwrap()
    }
}