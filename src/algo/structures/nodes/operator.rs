use super::super::tree::TreeNode;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Operator {
    pub label: u8
}

impl TreeNode for Operator {
    fn to_string(&self) -> String {
        String::from_utf8([self.label].to_vec()).unwrap()
    }
}