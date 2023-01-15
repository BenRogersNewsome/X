use std::{rc::Rc, fmt::Debug, cell::UnsafeCell};

use enum_dispatch::enum_dispatch;
use crate::{item::Item, logic::{NumBound, LBool, Number}, SetElement, NumRange, NumRangeBoundary};
use self::union::IsUnionOf;


mod anonymous;
// mod difference;
// use difference::DifferenceSet;
// mod intersection;
// use intersection::IntersectionSet;
mod empty;
mod has_set_element;
mod has_size;
mod union;
mod with_items;
mod without_items;

pub use has_set_element::HasSetElement;
pub use has_size::HasSize;
pub use union::UnionSet;
pub use with_items::WithItems;
pub use without_items::WithoutItems;

#[derive(Debug)]
pub struct Set {
    _raw: Rc<UnsafeCell<SetType>>,
    _uid: u64,
}

impl Clone for Set {
    fn clone(&self) -> Self {
        Self {
            _raw: self._raw.clone(),
            _uid: self._uid,
        }
    }
}

impl PartialEq for Set {
    fn eq(&self, other: &Self) -> bool {
        self._uid == other._uid
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Eq for Set {}

/// This closed set of functions is ultimately safe as we never give a direct reference to the internal set.
impl Set {

    #[inline]
    pub fn anonymous() -> Self {
        Self::new(SetType::Anonymous(()))
    }

    pub fn new(set: SetType) -> Self {
        Self {
            _raw: Rc::new(UnsafeCell::new(set)),
            _uid: rand::random(),
        }
    }
    
    pub fn replace<Callback>(&self, creator: Callback) where Callback: FnOnce(SetType) -> SetType {
        // SAFETY: We wrap the old inner with the new inner so that there is no
        // duplication. We use a creator callback so that we never expose a
        // dangling reference to external code.
        // TODO: Is this safe with the bound as an FnOnce?
        unsafe {
            let inner_mut: &mut SetType = &mut *self._raw.get();
            let inner: SetType = std::ptr::read(inner_mut);
            let new: SetType = creator(inner);
            std::ptr::write(inner_mut, new);
        }
    }

    pub fn uid(&self) -> u64 {
        self._uid
    }

    unsafe fn get_inner_set(&self) -> &SetType {
        &*self._raw.get()
    }

    #[inline]
    pub fn contains(&self, item: Item) -> LBool {
        self.contains_(&item, &mut Vec::new())
    }

    pub(crate) fn contains_(&self, item: &crate::item::Item, signature: &mut Vec<u64>) -> LBool {
        let inner_set: &SetType = unsafe { self.get_inner_set() };
        inner_set.contains(item, signature)
    }

    #[inline]
    pub fn known_elements(&self) -> Box<dyn Iterator<Item = Item> + '_> {
        self.known_elements_(&mut Vec::new())
    }

    pub(crate) fn known_elements_(&self, signature: &mut Vec<u64>) -> Box<dyn Iterator<Item = Item> + '_> {
        let inner_set: &SetType = unsafe { self.get_inner_set() };
        inner_set.known_elements(signature)
    }

    #[inline]
    pub fn known__non_elements(&self) -> Box<dyn Iterator<Item = Item> + '_> {
        self.known_non_elements_(&mut Vec::new())
    }

    pub(crate) fn known_non_elements_(&self, signature: &mut Vec<u64>) -> Box<dyn Iterator<Item = Item> + '_> {
        let inner_set: &SetType = unsafe { self.get_inner_set() };
        inner_set.known_non_elements(signature)
    }

    #[inline]
    pub fn size(&self) -> NumBound<Number> {
        self.size_(&mut Vec::new())
    }

    pub(crate) fn size_(&self, signature: &mut Vec<u64>) -> NumBound<Number> {
        let inner_set: &SetType = unsafe { self.get_inner_set() };
        inner_set.size(signature)
    }

    pub(crate) fn contains_set_element_(&self, element: &SetElement,signature: &mut Vec<u64>) -> LBool {
        let inner_set: &SetType = unsafe { self.get_inner_set() };
        inner_set.contains_set_element(self, element, signature)
    }
}

#[enum_dispatch]
pub trait SetLayer where Self: Debug {
    fn contains(&self, item: &crate::item::Item, signature: &mut Vec<u64>) -> LBool;
    fn known_elements(&self, signature: &mut Vec<u64>) -> Box<dyn Iterator<Item=Item> + '_>;
    fn known_non_elements(&self, signature: &mut Vec<u64>) -> Box<dyn Iterator<Item=Item> + '_>;
    fn size(&self, signature: &mut Vec<u64>) -> NumBound<Number>;
    fn contains_set_element(&self, set: &Set, element: &SetElement, signature: &mut Vec<u64>) -> LBool;
}

impl SetLayer for Box<dyn SetLayer> {
    #[inline]
    fn contains(&self,item: &crate::item::Item,signature: &mut Vec<u64>) -> LBool {
        self.as_ref().contains(item, signature)
    }
    
    #[inline]
    fn known_elements(&self,signature: &mut Vec<u64>) -> Box<dyn Iterator<Item = Item> +'_> {
        self.as_ref().known_elements(signature)
    }

    #[inline]
    fn known_non_elements(&self,signature: &mut Vec<u64>) -> Box<dyn Iterator<Item = Item> +'_> {
        self.as_ref().known_non_elements(signature)
    }

    #[inline]
    fn size(&self,signature: &mut Vec<u64>) -> NumBound<Number> {
        self.as_ref().size(signature)
    }

    fn contains_set_element(&self,set: &Set,element: &SetElement,signature: &mut Vec<u64>) -> LBool {
        self.as_ref().contains_set_element(set, element, signature)
    }
}
impl SetLayer for () {
    fn contains(&self, _item: &Item, _signature: &mut Vec<u64>) -> LBool {
        LBool::Unknown
    }

    fn known_elements(&self, _signature: &mut Vec<u64>) -> Box<dyn Iterator<Item = Item> +'_> {
        Box::new(std::iter::empty())
    }

    #[inline]
    fn known_non_elements(&self, _signature: &mut Vec<u64>) -> Box<dyn Iterator<Item = Item> +'_> {
        Box::new(std::iter::empty())
    }

    #[inline]
    fn size(&self, _signature: &mut Vec<u64>) -> NumBound<Number> {
        use NumRangeBoundary::*;
        NumBound::Range(NumRange(Unbounded, Unbounded))
    }

    fn contains_set_element(&self, _set: &Set, _element: &SetElement, _signature: &mut Vec<u64>) -> LBool {
        LBool::Unknown
    }
}

#[enum_dispatch(SetLayer)]
#[derive(Debug)]
pub enum SetType {
    Anonymous(()),
    HasSetElement(HasSetElement),
    HasSize(HasSize),
    Union(UnionSet),
    UnionOf(IsUnionOf),
    WithItems(WithItems),
    WithoutItems(WithoutItems),

    // Offers extensibility without compromising the performance of the core
    // types by forcing dynamic dispatch
    Any(Box<dyn SetLayer>),
}

// impl Set {

//     pub fn anonymous() -> Self {
//         Self::new(SetDefinition::FromElements(Vec::new(), true))
//     }

//     /// A utility function for creating the union of 2 sets.
//     pub fn union(set_1: &Set, set_2: &Set) -> Self {
//         Self::new(SetDefinition::Union(set_1.clone(), set_2.clone()))
//     }

//     /// A utility function for creating the intersection of 2 sets.
//     pub fn intersection(set_1: &Set, set_2: &Set) -> Self {
//         Self::new(SetDefinition::Intersection(set_1.clone(), set_2.clone()))
//     }

//     /// A utility function for creating the difference of 2 sets.
//     pub fn difference(set_1: &Set, set_2: &Set) -> Self {
//         Self::new(SetDefinition::Difference(set_1.clone(), set_2.clone()))
//     }

//     /// A utility function for creating a set from a list of set elements.
//     pub fn from_elements(set_elements: Vec<&SetElement>) -> Self {
//         Self::new(SetDefinition::FromElements(
//             set_elements
//                 .into_iter()
//                 .map(|x| x.clone())
//                 .collect(),
//             false,
//         ))
//     }

//     pub fn contains(self: &Self, set_element: &SetElement) -> LBool {
//         match &self.0 {
//             SetDefinition::Union(set_1, set_2) => {
//                 set_1.contains(set_element) | set_2.contains(set_element)
//             },
//             SetDefinition::Intersection(set_1, set_2) => {
//                 set_1.contains(set_element) & set_2.contains(set_element)
//             },
//             SetDefinition::Difference(set_1, set_2) => {
//                 set_1.contains(set_element) & !set_2.contains(set_element)
//             },
//             SetDefinition::FromElements(elements, anon) => {
//                 if elements.contains(&set_element) {
//                     True
//                 } else if *anon {
//                     match ***set_element {
//                         // Needed to prevent infinite recursion when asking if an anonymous element is a member of an anonymous set.
//                         SetElementDefinition::Anonymous => Unknown,
//                         _ => set_element.in_set(self),
//                     }
//                 } else {
//                     False
//                 }
//             }, 
//         }
//     }

//     pub fn add_element(&mut self, set_element: &mut SetElement) -> Result<(), ()> {

//         match (self.0, set_element.0) {
//             (SetDefinition::Anonymous, SetElementDefinition::Anonymous) => {

//             },
//             _ => panic!(),
//         };

//         Ok(())
//     }
// }

// impl Debug for Set {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let addr: *const RawSet = &*self._raw;
//         f.write_str(&format!("{:?}", addr))
//     }
// }


// /// Take the intersection  of two sets
// /// ```
// /// let S = Set::anonymous()
// /// let T = Set::anonymous()
// /// let P = S & T
// /// ```
// impl<'a> BitAnd for &'a Set {
//     type Output = Set;
//     fn bitand(self, rhs: Self) -> Self::Output {
//         Set::intersection(self, rhs)
//     }
// }

// /// Take the union of two sets
// /// ```
// /// let S = Set::anonymous()
// /// let T = Set::anonymous()
// /// let P = S + T
// /// ```
// impl<'a> Add for &'a Set {
//     type Output = Set;
//     fn add(self, rhs: Self) -> Self::Output {
//         Set::union(self, rhs)
//     }
// }

// /// Take the difference of two sets
// /// ```
// /// let S = Set::anonymous()
// /// let T = Set::anonymous()
// /// let P = S - T
// /// ```
// impl<'a> Sub for &'a Set {
//     type Output = Set;
//     fn sub(self, rhs: Self) -> Self::Output {
//         Set::difference(self, rhs)
//     }
// }

// impl PartialEq for Set {
//     fn eq(&self, other: &Self) -> bool {
//         self._raw._literally_equal(&other._raw)
//     }

//     fn ne(&self, other: &Self) -> bool {
//         !self._raw._literally_equal(&other._raw)
//     }
// }

// impl Eq for Set { }

// #[derive(PartialEq, Clone, Eq, Debug)]
// pub enum SetDefinition {
//     FromElements(Vec<SetElement>, bool),

//     Union(Set, Set),
//     Intersection(Set, Set),
//     Difference(Set, Set),
// }

// #[cfg(test)]
// mod test_set_equality {
//     use super::*;

//     #[test]
//     fn test_reflexive_equality_of_sets() {
//         let set_a = Set::anonymous();
//         let set_b = Set::anonymous();
//         let set_c = Set::union(&set_a, &set_b);
//         let set_d = Set::intersection(&set_a, &set_b);
//         let set_e = Set::difference(&set_a, &set_b);

//         assert_eq!(set_a, set_a);
//         assert_eq!(set_b, set_b);
//         assert_eq!(set_c, set_c);
//         assert_eq!(set_d, set_d);
//         assert_eq!(set_e, set_e);
//     }

//     #[test]
//     fn test_non_equality_of_anonymous_sets() {
//         let set_a = Set::anonymous();
//         let set_b = Set::anonymous();

//         assert_ne!(set_a, set_b);
//     }

//     #[test]
//     fn test_equality_of_union_sets() {
//         let set_a = Set::anonymous();
//         let set_b = Set::anonymous();
//         let set_c = Set::union(&set_a, &set_b);
//         let set_d = Set::union(&set_a, &set_b);
//         let set_e = Set::union(&set_c, &set_b);

//         assert_ne!(set_a, set_c);
//         assert_ne!(set_a, set_d);
//         assert_ne!(set_b, set_c);
//         assert_ne!(set_b, set_d);
//         assert_eq!(set_c, set_d);
//         assert_ne!(set_c, set_e);
//         assert_ne!(set_d, set_e);
//     }

//     #[test]
//     fn test_equality_of_intersection_sets() {
//         let set_a = Set::anonymous();
//         let set_b = Set::anonymous();
//         let set_c = Set::intersection(&set_a, &set_b);
//         let set_d = Set::intersection(&set_a, &set_b);
//         let set_e = Set::intersection(&set_c, &set_b);

//         assert_ne!(set_a, set_c);
//         assert_ne!(set_a, set_d);
//         assert_ne!(set_b, set_c);
//         assert_ne!(set_b, set_d);
//         assert_eq!(set_c, set_d);
//         assert_ne!(set_c, set_e);
//         assert_ne!(set_d, set_e);
//     }

//     #[test]
//     fn test_equality_of_difference_sets() {
//         let set_a = Set::anonymous();
//         let set_b = Set::anonymous();
//         let set_c = Set::difference(&set_a, &set_b);
//         let set_d = Set::difference(&set_a, &set_b);
//         let set_e = Set::difference(&set_c, &set_b);

//         assert_ne!(set_a, set_c);
//         assert_ne!(set_a, set_d);
//         assert_ne!(set_b, set_c);
//         assert_ne!(set_b, set_d);
//         assert_eq!(set_c, set_d);
//         assert_ne!(set_c, set_e);
//         assert_ne!(set_d, set_e);
//     }

//     #[test]
//     fn test_non_equality_of_different_set_definitions() {
//         let set_a = Set::anonymous();
//         let set_b = Set::anonymous();
//         let set_c = Set::union(&set_a, &set_b);
//         let set_d = Set::intersection(&set_a, &set_b);
//         let set_e = Set::difference(&set_a, &set_b);

//         assert_ne!(set_a, set_b);
//         assert_ne!(set_c, set_d);
//         assert_ne!(set_c, set_e);
//         assert_ne!(set_d, set_e);
//     }
// }

// #[cfg(test)]
// mod test_set_membership {

//     use super::*;

//     #[test]
//     fn test_direct_membership() {

//         let set_a = Set::anonymous();
//         let element_a = SetElement::element_of(&set_a);
//         let set_b = Set::anonymous();

//         assert!(*set_a.contains(&element_a));
//         assert!(*!set_b.contains(&element_a));
//     }

//     #[test]
//     fn test_union_membership() {

//         let set_a = Set::anonymous();
//         let element_a = SetElement::element_of(&set_a);
//         let set_b = Set::anonymous();

//         let set_c = Set::union(&set_a, &set_b);
        
//         assert!(*set_a.contains(&element_a));
//         assert!(*!set_b.contains(&element_a));
//         assert!(*set_c.contains(&element_a));
//     }

//     #[test]
//     fn test_union_membership_recursive() {

//         let set_a = Set::anonymous();
//         let element_a = SetElement::element_of(&set_a);
//         let set_b = Set::anonymous();
        
//         let set_c = Set::union(&set_a, &set_b);
//         let set_d = Set::union(&set_c, &set_b);

//         assert!(*set_a.contains(&element_a));
//         assert!(*!set_b.contains(&element_a));
//         assert!(*set_c.contains(&element_a));
//         assert!(*set_d.contains(&element_a));
//     }

//     #[test]
//     fn test_intersection_membership() {

//         let set_a = Set::anonymous();
//         let element_a = SetElement::element_of(&set_a);
//         let set_b = Set::anonymous();

//         let set_c = Set::union(&set_a, &set_b); // a is in C
//         let set_d = Set::intersection(&set_a, &set_c); // a is in D

//         assert!(*set_a.contains(&element_a));
//         assert!(*!set_b.contains(&element_a));
//         assert!(*set_c.contains(&element_a));
//         assert!(*set_d.contains(&element_a));
//     }

//     #[test]
//     fn test_difference_membership() {

//         let set_a = Set::anonymous();
//         let element_a = SetElement::element_of(&set_a);
//         let set_b = Set::anonymous();

//         let set_c = Set::difference(&set_a, &set_b); // a is in C
//         let set_d = Set::difference(&set_a, &set_c); // a not in D

//         assert!(*set_a.contains(&element_a));
//         assert!(*!set_b.contains(&element_a));
//         assert!(*set_c.contains(&element_a));
//         assert!(*!set_d.contains(&element_a));
//     }

//     #[test]
//     fn test_from_elements_membership() {
//         let element_a = SetElement::anonymous();
//         let element_b = SetElement::anonymous();
//         let element_c = SetElement::anonymous();

//         let set_a = Set::from_elements(vec![
//             &element_a,
//             &element_b,
//             &element_c,
//         ]);
//         let set_b = Set::from_elements(vec![]);

//         assert!(*set_a.contains(&element_a));
//         assert!(*set_a.contains(&element_b));
//         assert!(*set_a.contains(&element_c));
//         assert!(*!set_b.contains(&element_a));
//         assert!(*!set_b.contains(&element_b));
//         assert!(*!set_b.contains(&element_c));
//     }

//     #[test]
//     fn test_not_infinite_recursion_in_anonymous_membership() {
//         use super::super::set_element::SetElement;
//         let set_a = Set::anonymous();
//         let element_a = SetElement::anonymous();

//         assert!(*!set_a.contains(&element_a));
//     }
// }