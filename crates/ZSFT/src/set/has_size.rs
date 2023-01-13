use crate::{SetType, logic::{AssertionResponse, Number, NumBound}, Set};

use super::SetLayer;

#[derive(Debug)]
pub struct HasSize {
    size: NumBound<Number>,
    underlying_set: Box<SetType>,
}

impl HasSize {

    pub fn assert_on(size: NumBound<Number>, base_set: &Set) -> AssertionResponse {
        
        let existing_size: NumBound<Number> = base_set.size(&mut Vec::new());

        let new_size: NumBound<Number> = match existing_size & size {
            None => return AssertionResponse::AssertionInvalid,
            Some(existing_size) => return AssertionResponse::RedundantAssertion,
            Some(x) => x,
        };

        base_set.replace(|old_layer| {
            SetType::HasSize(Self {
                size: new_size,
                underlying_set: Box::new(old_layer)
            })
        });

        AssertionResponse::AssertionMade
    }

}

impl SetLayer for HasSize {
    #[inline]
    fn contains(&self,item: &crate::item::Item,signature: &mut Vec<u64>) -> crate::LBool {
        self.underlying_set.contains(item, signature)
    }

    #[inline]
    fn known_elements(&self,signature: &mut Vec<u64>) -> Box<dyn Iterator<Item = crate::item::Item> +'_> {
        self.underlying_set.known_elements(signature)
    }

    #[inline]
    fn known_non_elements(&self,signature: &mut Vec<u64>) -> Box<dyn Iterator<Item = crate::item::Item> +'_> {
        self.underlying_set.known_non_elements(signature)
    }

    #[inline]
    fn size(&self, _signature: &mut Vec<u64>) -> NumBound<Number> {
        self.size
    }
}