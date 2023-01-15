use crate::{SetType, logic::{AssertionResponse, Number, NumBound}, Set, Item, LBool};

use super::SetLayer;

#[derive(Debug)]
pub struct HasSize {
    size: NumBound<Number>,
    underlying_set: Box<SetType>,
}

impl HasSize {

    pub fn assert_on(size: NumBound<Number>, base_set: &Set) -> AssertionResponse {
        
        let existing_size: NumBound<Number> = base_set.size_(&mut Vec::new());

        let new_size: NumBound<Number> = match existing_size & size {
            None => return AssertionResponse::AssertionInvalid,
            Some(x) if x == existing_size => return AssertionResponse::RedundantAssertion,
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
        use NumBound::*;
        use Number::*;

        let underlying_contains: LBool = self.underlying_set.contains(&item, signature);
        if underlying_contains == LBool::True {
            return LBool::True;
        };

        let known_elements: Vec<Item> = self.known_elements(signature).collect();
        if let Eq(Ordinal(s)) = self.size {
            if s == known_elements.len() {
                return LBool::from(known_elements.contains(item))
            }
        };
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

    #[inline]
    fn contains_set_element(&self, set: &Set,element: &crate::SetElement,signature: &mut Vec<u64>) -> crate::LBool {
        self.underlying_set.contains_set_element(set, element, signature)
    }
}