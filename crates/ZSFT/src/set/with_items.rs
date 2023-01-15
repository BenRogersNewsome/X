use crate::{LBool, item::Item, Set, logic::{NumBound, AssertionResponse, Number}, SetType, NumRange, NumRangeBoundary};

use super::SetLayer;

#[derive(Debug)]
pub struct WithItems {
    items: Vec<Item>,
    underlying_set: Box<SetType>,
    num_known_elements: usize,
}

impl WithItems {
    pub fn assert_on<'a>(base_set: &'a Set, items: Vec<&'a Item>) -> AssertionResponse {
        let mut items_to_add: Vec<Item> = Vec::with_capacity(items.len());
        for item in items {
            match base_set.contains_(&item, &mut Vec::new()) {
                LBool::False => return AssertionResponse::AssertionInvalid,
                LBool::True => {
                    // Don't create a redundant assertion if we already know
                    // that the item is in the set.
                    continue;
                },
                LBool::Unknown => {
                    items_to_add.push(item.clone());
                },
            }
        };

        if items_to_add.len() == 0 {
            return AssertionResponse::RedundantAssertion;
        };

        let num_known_elements_beneath: usize = base_set.known_elements_(&mut Vec::new()).count();
        let num_known_elements: usize = num_known_elements_beneath + items_to_add.len();

        // match (base_set.size(&mut Vec::new()), num_known_elements) {
        //     ( NumBound::Eq(a), b ) if b > a => {return AssertionResponse::AssertionInvalid;},
        //     ( NumBound::Lt(a), b ) if b >= a => {return AssertionResponse::AssertionInvalid;}
        //     _ => {},
        // };

        base_set.replace(|inner| {
            SetType::WithItems(Self {
                items: items_to_add,
                underlying_set: Box::new(inner),
                num_known_elements,
            })
        });
        AssertionResponse::AssertionMade
    }
}

impl SetLayer for WithItems {
    fn contains(&self, item: &crate::item::Item, signature: &mut Vec<u64>) -> LBool {

        use NumBound::*;
        use Number::*;

        let underlying_contains: LBool = self.underlying_set.contains(&item, signature);
        if underlying_contains == LBool::True {
            return self.contains(&item, signature);
        };

        if let Eq(Ordinal(s)) = self.size(signature) {
            if s == self.num_known_elements {
                return LBool::from(self.known_elements(signature).any(|i|i==*item))
            }
        };
        LBool::from(self.items.contains(item)) | self.underlying_set.contains(item, signature)
    }

    fn known_elements(&self, signature: &mut Vec<u64>) -> Box<dyn Iterator<Item=Item> + '_> {
        Box::new(self.underlying_set.known_elements(signature).chain(self.items.iter().cloned()))
    }

    #[inline]
    fn known_non_elements(&self, signature: &mut Vec<u64>) -> Box<dyn Iterator<Item=Item> + '_> {
        self.underlying_set.known_non_elements(signature)
    }

    fn size(&self, signature: &mut Vec<u64>) -> NumBound<Number> {
        match self.underlying_set.size(signature) {
            NumBound::Eq(n) => NumBound::Eq(n),
            NumBound::Range(r) => (
                r & NumRange(
                    NumRangeBoundary::Closed(Number::Ordinal(self.num_known_elements)),
                    NumRangeBoundary::Unbounded
                )
            ).unwrap(),
        }
    }

    #[inline]
    fn contains_set_element(&self,set: &Set,element: &crate::SetElement,signature: &mut Vec<u64>) -> LBool {
        self.underlying_set.contains_set_element(set, element, signature)
    }
}