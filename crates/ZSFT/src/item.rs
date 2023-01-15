#[derive(Clone, Hash, Debug, Copy)]
pub struct Item {
    id: u128,
    type_: ItemType,
}

impl Item {
    pub fn new() -> Self {
        Self {
            id: rand::random(),
            type_: ItemType::Anonymous,
        }
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Eq for Item {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
enum ItemType {
    Anonymous,
}