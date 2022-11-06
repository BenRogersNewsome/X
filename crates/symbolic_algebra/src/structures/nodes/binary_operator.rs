use solar_bt::{TreeNode, ParsableTreeNode};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct BinaryOperator {
    pub label: u8
}

impl BinaryOperator {
    pub fn new(label: u8) -> Self {
        Self {
            label
        }
    }
}

impl TreeNode for BinaryOperator {
    fn to_string(&self) -> String {
        String::from_utf8([self.label].to_vec()).unwrap()
    }
}

impl ParsableTreeNode for BinaryOperator {
    fn from_string(id: &str) -> Result<Self, &'static str> {
        let bytes = id.as_bytes();

        if bytes.len() != 1 {
            return Err("")
        }else{
            return Ok(Self {
                label: bytes[0]
            })
        }
    }
}