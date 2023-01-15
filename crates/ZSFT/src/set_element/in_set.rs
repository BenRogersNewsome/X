use crate::{set::{Set, HasSetElement}, SetElement, logic::AssertionResponse, LBool};
use super::{SetElementLayer, SetElementType};

#[derive(Debug, Clone)]
pub struct InSet {
    member_of: Set,
    signed: u64,
    underlying_element: Box<SetElementType>,
}

impl InSet {
    pub fn assert_on(element: &SetElement, set: &Set) -> AssertionResponse {
        match set.contains_set_element_(element, &mut Vec::new()) {
            LBool::True => return AssertionResponse::RedundantAssertion,
            LBool::False => return AssertionResponse::AssertionInvalid,
            _ => {
                let signed: u64 = rand::random();
                let set_cloned = set.clone();
                element.replace(move |inner| {
                    SetElementType::InSet(Self {
                        member_of: set_cloned,
                        signed,
                        underlying_element: Box::new(inner),
                    })
                });

                set.replace(move |inner| {
                    crate::SetType::HasSetElement(HasSetElement::new(
                        inner,
                        &element,
                        signed,
                    ))
                });
                AssertionResponse::AssertionMade
            }
        }
    }
}

impl SetElementLayer for InSet {

    fn in_set(&self, element: &SetElement, set: &Set, signature: &mut Vec<u64>) -> crate::LBool {
        if signature.contains(&self.signed) {
            self.underlying_element.in_set(element, set, signature)
        }else{
            signature.push(self.signed);

            LBool::from(self.member_of == *set) |
            self.underlying_element.in_set(element, set, signature) |
            self.member_of.contains_set_element_(element, signature)
        }
    }
}