use crate::algo::trees::TreeNode;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BinaryOperator {
    pub label: u8
}

impl TreeNode for BinaryOperator {
    fn to_string(&self) -> String {
        String::from_utf8([self.label].to_vec()).unwrap()
    }
}