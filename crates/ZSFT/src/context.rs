// use std::{collections::HashMap, fmt::Debug};
// use super::{
//     operation::{BinaryOperation, UnaryOperation, BinaryOperationDefinition, UnaryOperationDefinition},
//     set::{Set, SetDefinition},
//     set_element::{SetElement, SetElementDefinition},
// };

// enum ContextElementDefinitionTag {
//     Unary,
//     Binary,
//     Set,
//     SetElement,
// }

// #[derive(Debug, PartialEq, Eq)]
// enum ContextElement {
//     UnaryOperation(UnaryOperation),
//     BinaryOperation(BinaryOperation),
//     Set(Set),
//     SetElement(SetElement),
// }

// pub struct Context { }

// #[derive(Debug, PartialEq, Eq)]
// pub enum CreationResult<T> {
//     AlreadyExists(ContextElement),
//     Created(T),
//     Invalid,
// }

// impl<T> CreationResult<T> {
//     pub fn unwrap(self) -> T {
//         match self {
//             Self::Created(x) => x,
//             _ => panic!(),
//         }
//     }
// }

// impl Context {

//     pub fn new() -> Self {
//         Self { }
//     }

//     pub fn create_set(&mut self, label: u32, definition: SetDefinition) -> CreationResult<Set> {        
//         CreationResult::Created(Set::new(definition))
//     }

//     /// Create a binary operation, with signature X x Y -> Z
//     pub fn create_binary_operation(
//         &mut self,
//         label: u32,
//         x: Set,
//         y: Set,
//         z: Set,
//     ) -> CreationResult<BinaryOperation> {

//         if let Some(x) = self._get_if_existing(label) {
//             return CreationResult::AlreadyExists(x);
//         };

//         match (self.elements.contains_key(&x), self.elements.contains_key(&y), self.elements.contains_key(&z)) {
//             (true, true, true) => {
//                 self.elements.insert(
//                     label,
//                     ContextElementDefinition{ binary: BinaryOperationDefinition::new(x, y, z)},
//                 );
        
//                 CreationResult::Created(BinaryOperation::new(label))
//             },
//             (false, _, _) => {
//                 CreationResult::Invalid
//             },
//             (_, false, _) => {
//                 CreationResult::Invalid
//             }
//             (_, _, false) => {
//                 CreationResult::Invalid
//             },
//         }
//     }

//     /// Create a unary operation, with signature X -> Y
//     pub fn create_unary_operation(
//         &mut self,
//         label: u32,
//         x: Set,
//         y: Set,
//     ) -> CreationResult<UnaryOperation> {
//         if let Some(x) = self._get_if_existing(label) {
//             return CreationResult::AlreadyExists(x);
//         };

//         match (self.elements.contains_key(&x), self.elements.contains_key(&y)) {
//             (true, true) => {
//                 self.elements.insert(
//                     label,
//                     ContextElementDefinition{ unary: UnaryOperationDefinition::new(x, y)},
//                 );
        
//                 CreationResult::Created(UnaryOperation::new(label))
//             },
//             (false, _) => {
//                 CreationResult::Invalid
//             },
//             (_, false) => {
//                 CreationResult::Invalid
//             }
//         }
//     }

//     fn create_set_element(
//         &mut self,
//         label: u32,
//         definition: SetElementDefinition
//     ) -> CreationResult<SetElement> {
        
//         if let Some(x) = self._get_if_existing(label) {
//             return CreationResult::AlreadyExists(x);
//         };
        
//         self.elements.insert(label, ContextElementDefinition{ setElement: definition });

//         CreationResult::Created(SetElement::new(label))
//     }

//     pub fn create_element_in_set(
//         &mut self,
//         label: u32,
//         set: Set
//     ) -> CreationResult<SetElement> {
//         self.create_set_element(label, SetElementDefinition::MemberOf(set))
//     }

//     // pub fn create_element_from_binary_operation(&mut self, label: u8, operation: BinaryOperation, a: SetElement, b: SetElement) -> CreationResult<SetElement, SetElement> {

//     //     let maybe_op_def = self.operations.get(operation);

//     //     match maybe_op_def {
//     //         Some(op_def) => {
//     //             if op_def
//     //         },
//     //         None => CreationResult::Invalid,
//     //     }

//     //     self._create_set_element(label, SetElementDefinition::BinaryOperation(operation, a, b))
//     // }

//     // pub fn _set_contains(&self, set_def: SetDefinition, member: SetElement) -> bool {

//     // }

//     pub fn set_contains(&self, set: Set, member: SetElement) -> bool {
        
//         let set_definition = unsafe {
//             match self.elements.get(&set) {
//                 Some(ContextElementDefinition { set: def }) => def,
//                 None => panic!(),
//             }
//         };

//         match set_definition {
//             SetDefinition::Union(a, b) => self._in_either(*a, *b, member),
//             SetDefinition::Intersection(a, b) => self._in_both(*a, *b, member),
//             SetDefinition::Difference(a, b) => {
//                 self.set_contains(*a, member) && !self.set_contains(*b, member)
//             },
//             SetDefinition::None => self._has_direct_membership(set, member),
//         }
//     }

//     fn _has_direct_membership(&self, set: Set, member: SetElement) -> bool {

//         let set_element_definition = unsafe {
//             match self.elements.get(&member) {
//                 Some(ContextElementDefinition { setElement: def }) => def,
//                 None => panic!(),
//             }
//         };

//         match set_element_definition {
//             SetElementDefinition::MemberOf(s) => { set == *s },
//             SetElementDefinition::BinaryOperation(op, a, b) => {
//                 false
//             },
//             _ => false,
//         }
//     }

//     fn _in_both(&self, set_1: Set, set_2: Set, member: SetElement) -> bool {
//         self.set_contains(set_1, member) && self.set_contains(set_2, member)
//     }

//     fn _in_either(&self, set_1: Set, set_2: Set, member: SetElement) -> bool {
//         self.set_contains(set_1, member) || self.set_contains(set_2, member)
//     }

    
// }

// // #[cfg(test)]
// // mod test_create_sets {
    
// //     use super::*;

// //     #[test]
// //     fn test_create_set(){
// //         let mut context = Context::new();

// //         let set_a = context.create_set(b'A', SetDefinition::None);
// //         assert_eq!(set_a, CreationResult::Created(Set::new(b'A')));
// //     }

// //     #[test]
// //     fn test_cant_create_same_set_twice(){
// //         let mut context = Context::new();

// //         let set_a = context.create_set(b'A', SetDefinition::None);
// //         assert_eq!(set_a, CreationResult::Created(Set::new(b'A')));

// //         let set_a_again = context.create_set(b'A', SetDefinition::None);
// //         assert_eq!(set_a_again, CreationResult::AlreadyExists(set_a.unwrap()));
// //     }
// // }

// // #[cfg(test)]
// // mod test_set_membership {

// //     use super::*;

// //     #[test]
// //     fn test_direct_membership() {
// //         let mut context = Context::new();

// //         let set_a = context.create_set(b'A', SetDefinition::None).unwrap();
// //         let element_a = context.create_element_in_set(b'a', set_a).unwrap();
// //         let set_b = context.create_set(b'B', SetDefinition::None).unwrap();

// //         assert!(context.set_contains(set_a, element_a));
// //         assert!(!context.set_contains(set_b, element_a));
// //     }

// //     #[test]
// //     fn test_union_membership() {
// //         let mut context = Context::new();

// //         let set_a = context.create_set(b'A', SetDefinition::None).unwrap();
// //         let element_a = context.create_element_in_set(b'a', set_a).unwrap();
// //         let set_b = context.create_set(b'B', SetDefinition::None).unwrap();

// //         let set_c = context.union(b'C', set_a, set_b).unwrap();

// //         assert!(context.set_contains(set_c, element_a))
// //     }

// //     #[test]
// //     fn test_union_membership_recursive() {
// //         let mut context = Context::new();

// //         let set_a = context.create_set(b'A', SetDefinition::None).unwrap();
// //         let element_a = context.create_element_in_set(b'a', set_a).unwrap();
// //         let set_b = context.create_set(b'B', SetDefinition::None).unwrap();

// //         let set_c = context.union(b'C', set_a, set_b).unwrap();
// //         let set_d = context.union(b'D', set_c, set_b).unwrap();

// //         assert!(context.set_contains(set_d, element_a))
// //     }

// //     #[test]
// //     fn test_intersection_membership() {
// //         let mut context = Context::new();

// //         let set_a = context.create_set(b'A', SetDefinition::None).unwrap();
// //         let element_a = context.create_element_in_set(b'a', set_a).unwrap();
// //         let set_b = context.create_set(b'B', SetDefinition::None).unwrap();

// //         let set_c = context.union(b'C', set_a, set_b).unwrap(); // a is in C
// //         let set_d = context.intersection(b'D', set_a, set_c).unwrap(); // a is in D

// //         assert!(context.set_contains(set_d, element_a))
// //     }

// //     #[test]
// //     fn test_difference_membership() {
// //         let mut context = Context::new();

// //         let set_a = context.create_set(b'A', SetDefinition::None).unwrap();
// //         let element_a = context.create_element_in_set(b'a', set_a).unwrap();
// //         let set_b = context.create_set(b'B', SetDefinition::None).unwrap();

// //         let set_c = context.difference(b'C', set_a, set_b).unwrap(); // a is in C

// //         let set_d = context.difference(b'D', set_a, set_c).unwrap(); // a not in D

// //         assert!(context.set_contains(set_a, element_a));
// //         assert!(!context.set_contains(set_d, element_a));
// //     }
// // }