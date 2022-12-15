use solar_bt::{TreeNode, ParsableTreeNode};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Element {
    pub label: Vec<u8>
}

impl Element {
    pub fn new(label: &[u8]) -> Self {
        Self {
            label: label.to_owned(),
        }
    }
}

impl TreeNode for Element {
    fn to_string(&self) -> String {
        String::from_utf8(self.label.to_vec()).unwrap()
    }
}

impl ParsableTreeNode for Element {
    fn from_string(id: &str) -> Result<Self, &'static str> {
        return Ok(Self {
            label: id.as_bytes().to_owned(),
        })
    }
}