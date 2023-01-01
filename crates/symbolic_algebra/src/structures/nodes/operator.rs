
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct Operator {
    pub label: u8
}

impl Operator {
    pub fn new(label: u8) -> Self {
        Self {
            label
        }
    }
}
