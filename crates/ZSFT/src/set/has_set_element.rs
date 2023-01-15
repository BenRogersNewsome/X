use crate::{set_element::SetElement, Set};
use super::{SetType, SetLayer};

#[derive(Debug)]
pub struct HasSetElement {
    base_set: Box<SetType>,
    has_element: SetElement,
    signed: u64,
}

impl HasSetElement {
    pub(crate) fn new(base_set: SetType, has_element: &SetElement, signed: u64) -> Self {
        Self {
            base_set: Box::new(base_set),
            has_element: has_element.clone(),
            signed,
        }
    }
}

impl SetLayer for HasSetElement {
    #[inline]
    fn contains(&self,item: &crate::item::Item,signature: &mut Vec<u64>) -> crate::LBool {
        self.base_set.contains(item, signature)
    }

    #[inline]
    fn known_elements(&self,signature: &mut Vec<u64>) -> Box<dyn Iterator<Item = crate::Item> +'_> {
        self.base_set.known_elements(signature)
    }

    #[inline]
    fn known_non_elements(&self,signature: &mut Vec<u64>) -> Box<dyn Iterator<Item = crate::Item> +'_> {
        self.base_set.known_non_elements(signature)
    }

    #[inline]
    fn size(&self,signature: &mut Vec<u64>) -> crate::logic::NumBound<crate::logic::Number> {
        self.base_set.size(signature)
    }
    
    fn contains_set_element(&self, set: &Set, element: &SetElement, signature: &mut Vec<u64>) -> crate::LBool {
        if signature.contains(&self.signed) {
            return self.base_set.contains_set_element(set, element, signature);
        };
        signature.push(self.signed);

        self.base_set.contains_set_element(set, element, signature) |
        self.has_element.in_set_(set, signature)
    }
}