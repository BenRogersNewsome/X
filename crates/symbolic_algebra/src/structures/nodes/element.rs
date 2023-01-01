
#[derive(PartialEq, Eq, Debug, Clone, Hash)]
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
