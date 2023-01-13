use crate::{LBool, item::Item, Set, logic::{NumBound, AssertionResponse, Number}};

use super::SetLayer;

#[derive(Debug)]
pub struct WithoutItems {
    items: Vec<Item>,
    underlying_set: Box<dyn SetLayer>,
}

impl WithoutItems {
    pub fn assert_on(base_set: &Set, items: Vec<Item>) -> AssertionResponse {
        use AssertionResponse::*;
        for item in &items {
            if base_set.contains(item, &mut Vec::new()) == LBool::True {
                return AssertionInvalid;
            };
        }
        base_set.replace(|inner| {
            crate::SetType::WithoutItems(Self {
                items,
                underlying_set: Box::new(inner),
            })
        });
        return AssertionMade
    }
}

impl SetLayer for WithoutItems {
    fn contains(&self, item: &crate::item::Item, signature: &mut Vec<u64>) -> LBool {
        if self.items.contains(item) {
            return LBool::False;
        } else {
            return self.underlying_set.contains(item, signature);
        }
    }

    #[inline]
    fn known_elements(&self, signature: &mut Vec<u64>) -> Box<dyn Iterator<Item=Item> + '_> {
        self.underlying_set.known_elements(signature)
    }

    fn known_non_elements(&self, signature: &mut Vec<u64>) -> Box<dyn Iterator<Item=Item> + '_> {
        Box::new(
            self.underlying_set.known_non_elements(signature).chain(self.items.iter().cloned())
        )
    }

    #[inline]
    fn size(&self, signature: &mut Vec<u64>) -> NumBound<Number> {
        self.underlying_set.size(signature)
    }
}