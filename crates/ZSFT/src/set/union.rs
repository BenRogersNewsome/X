use crate::{LBool, item::Item, logic::{NumBound, Number}, SetType, NumRange};
use super::{Set, SetLayer};

#[derive(Debug)]
pub struct UnionSet {
    left: Set,
    right: Set,
    signed: u64,
}

impl UnionSet {
    pub fn of<'b>(left: &'b Set, right: &'b Set) -> Set {
        let signed: u64 = rand::random();
        
        let union_set: Set = Set::new(SetType::Union(Self {
            left: left.clone(),
            right: right.clone(),
            signed,
        }));

        left.replace(|inner| {
            SetType::UnionOf(IsUnionOf {
                union_set: union_set.clone(),
                union_with: right.clone(),
                underlying_self: Box::new(inner),
                signed,
            })
        });

        right.replace(|inner| {
            SetType::UnionOf(IsUnionOf {
                union_set: union_set.clone(),
                union_with: left.clone(),
                underlying_self: Box::new(inner),
                signed,
            })
        });

        union_set
    }

    #[inline]
    fn _naive_contains(&self, _: &crate::item::Item, _: &mut Vec<u64>) -> LBool {
        LBool::Unknown
    }
}

impl SetLayer for UnionSet {
    fn contains(&self, item: &crate::item::Item, signature: &mut Vec<u64>) -> LBool {
        if signature.contains(&self.signed) {
            self._naive_contains(item, signature)
        } else {
            signature.push(self.signed);
            self.left.contains_(item, signature) | self.right.contains_(item, signature)
        }
    }

    fn known_elements(&self, signature: &mut Vec<u64>) -> Box<dyn Iterator<Item=Item> + '_> {
        Box::new(
            self.left.known_elements_(signature).chain(self.right.known_elements_(signature))
        )
    }

    fn known_non_elements(&self, signature: &mut Vec<u64>) -> Box<dyn Iterator<Item=Item> + '_> {
        Box::new(
            self.left.known_non_elements_(signature).chain(self.right.known_non_elements_(signature))
        )
    }

    fn size(&self, signature: &mut Vec<u64>) -> NumBound<Number> {
        use crate::logic::NumRangeBoundary::*;
        if signature.contains(&self.signed) {
            NumBound::Range(NumRange(Unbounded, Unbounded))
        }else{
            signature.push(self.signed);
            self.left.size_(signature) + self.right.size_(signature)
        }

    }

    fn contains_set_element(&self,set: &Set,element: &crate::SetElement, signature: &mut Vec<u64>) -> LBool {
        if signature.contains(&self.signed) {
            element.in_set_(set, signature)
        } else {
            signature.push(self.signed);

            element.in_set_(set, signature) |
            self.left.contains_set_element_(element, signature) |
            self.right.contains_set_element_(element, signature)
        }
    }
}

#[derive(Debug)]
pub struct IsUnionOf {
    union_set: Set,
    union_with: Set,
    underlying_self: Box<SetType>,
    signed: u64,
}

impl IsUnionOf {
    fn _naive_contains(&self, item: &crate::item::Item, signature: &mut Vec<u64>) -> LBool {
        self.underlying_self.contains(item, signature)
    }
}

impl SetLayer for IsUnionOf {
    fn contains(&self, item: &crate::item::Item, signature: &mut Vec<u64>) -> LBool {
        if signature.contains(&self.signed) {
            return self._naive_contains(item, signature);
        }else{
            signature.push(self.signed);
        };

        let union_set_contains = self.union_set.contains_(item, signature);
        let union_with_contains = self.union_with.contains_(item, signature);

        if union_set_contains == LBool::False {
            LBool::False
        } else if (union_set_contains == LBool::True) && (union_with_contains == LBool::False) {
            LBool::True
        } else {
            self.underlying_self.contains(item, signature)
        }
    }

    #[inline]
    fn known_elements(&self, signature: &mut Vec<u64>) -> Box<dyn Iterator<Item=Item> + '_> {
        self.underlying_self.known_elements(signature)
    }

    fn known_non_elements(&self, signature: &mut Vec<u64>) -> Box<dyn Iterator<Item=Item> + '_> {
        Box::new(
            self.underlying_self.known_elements(signature).chain(
                self.union_set.known_non_elements_(signature)
            )
        )
    }

    fn size(&self, signature: &mut Vec<u64>) -> NumBound<Number> {
        use NumBound::*;
        todo!();
        // match (
        //     self.underlying_self.size(signature),
        //     self.union_set.size(signature),
        //     self.union_with.size(signature)
        // ) {
        //     (Eq(n), _, _) => Eq(n),
        //     // (Gt(a), Gt(b), Lt(c)) |
        //     // (Gt(a), Eq(b), Lt(c)) |
        //     // (Gt(a), Gt(b), Lt(c))
        // }
        
    }

    fn contains_set_element(&self,set: &Set,element: &crate::SetElement,signature: &mut Vec<u64>) -> LBool {
        if signature.contains(&self.signed) {
            element.in_set_(set, signature)
        }else{
            let union_set_contains = self.union_set.contains_set_element_(element, signature);
            let union_with_contains = self.union_with.contains_set_element_(element, signature);

            if union_set_contains == LBool::False {
                LBool::False
            } else if (union_set_contains == LBool::True) && (union_with_contains == LBool::False) {
                LBool::True
            } else {
                self.underlying_self.contains_set_element(set, element, signature)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Set, set::{without_items::WithoutItems, with_items::WithItems}, item::Item, logic::AssertionResponse};

    use super::UnionSet;
    use AssertionResponse::*;
    use crate::logic::LBool::*;

    #[test]
    fn test_reflexive_union_assertions_from_union_member() {
        let set_a = Set::new(crate::SetType::Anonymous(()));
        let set_b = Set::new(crate::SetType::Anonymous(()));

        // |- C = A u B
        let set_c = UnionSet::of(&set_a, &set_b);

        // |- a !(- C
        let item_a = Item::new();
        assert_eq!(WithoutItems::assert_on(&set_c, vec![&item_a]), AssertionMade);

        // a (- A ?
        // This assertion should fail because:
        //  - a is not in the union of A and B,
        //  - Therefore a cannot be in either A or B individually
        assert_eq!(set_a.contains(item_a), False)
    }

    #[test]
    fn test_reflexive_union_assertions_from_union_result() {
        let set_a = Set::new(crate::SetType::Anonymous(()));
        let set_b = Set::new(crate::SetType::Anonymous(()));

        // |- a (- A
        let item_a = Item::new();
        assert_eq!(WithItems::assert_on(&set_a, vec![&item_a]), AssertionMade);

        // C = A u B
        let set_c = UnionSet::of(&set_a, &set_b);

        // a (- C ?
        // This assertion should pass because:
        //  - a is in A
        //  - C = A u B
        //  - Therefore a is in C
        assert_eq!(set_c.contains(item_a), True)
    }
}