use super::super::tree::TreeNode;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Element {
    pub label: Vec<u8>
}

impl TreeNode for Element {
    fn to_string(&self) -> String {
        String::from_utf8(self.label.to_vec()).unwrap()
    }
}