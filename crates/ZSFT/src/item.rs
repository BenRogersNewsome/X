use std::{fmt::Debug, collections::hash_map::DefaultHasher, hash::{Hash, Hasher}, rc::Rc};


#[derive(Clone, PartialEq, Eq)]
pub struct Item {
    _type: Rc<ItemType>,
}

impl Item {
    pub fn new() -> Self {
        Self {
            _type: Rc::new(ItemType::Single),
        }
    }
}

pub enum ItemType {
    Single,
}

impl PartialEq for ItemType {
    
    fn eq(&self, other: &Self) -> bool {
        self._literally_equal(other)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Eq for ItemType { }

impl ItemType {
    fn _literally_equal(&self, other: &Self) -> bool {
        let addr_self: *const Self = self;
        let addr_other: *const Self = other;

        addr_self == addr_other
    }
}

impl Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let raw_pointer: *const ItemType = self._type.as_ref();

        let mut hasher = DefaultHasher::new();
        raw_pointer.hash(&mut hasher);

        f.write_str(&format!("{:?}", hasher.finish()))?;
        Ok(())
    }
}

impl Hash for Item {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let raw_pointer: *const ItemType = self._type.as_ref();
        raw_pointer.hash(state);
    }
}

#[cfg(test)]
mod test_item_equality {

}