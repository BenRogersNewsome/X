// use crate::{LBool, item::Item, Set, SetType};

// #[derive(Debug)]
// pub struct Empty {
//     underlying_set: Box<SetType>,
// }

// impl Empty {
//     pub fn contains(&self, item: &crate::item::Item, signature: &mut Vec<u64>) -> LBool {
//         LBool::False
//     }

//     pub fn assert_on(base_set: &Set) -> bool {
//         for item in &items {
//             if base_set.contains(item, &mut Vec::new()) == LBool::True {
//                 return false;
//             };
//         }
//         base_set.replace(|inner| {
//             SetType::WithoutItems(Self {
//                 items,
//                 underlying_set: Box::new(inner),
//             })
//         });
//         return true
//     }
// }