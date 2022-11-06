use lazy_static::lazy_static;
use onig::{Regex, RegexOptions, Syntax};
use super::super::RegexParseError;
use crate::tree::{Tree, ParsableTreeNode, Node};

const GLOBAL_REGEX: &str = r"^(?<op>[+*])\((?<one>[+*]\(\g<one>\)\(\g<one>\)|[a-z])\)\((?<two>[+*]\(\g<two>\)\(\g<two>\)|[a-z])\)$|^(?<el>[a-z])$";  // PITA

pub fn tree_from_regex<T: Tree>(expression_string: &str) -> Result<T, RegexParseError> where T::I: ParsableTreeNode, T::L: ParsableTreeNode {
    lazy_static! {
        static ref GLOBAL_PATTERN: Regex = Regex::with_options(
            GLOBAL_REGEX,
            RegexOptions::REGEX_OPTION_DONT_CAPTURE_GROUP,
            Syntax::default(),
        ).unwrap();
    }
    let caps = GLOBAL_PATTERN.captures(expression_string).unwrap();
    if let Some(x) = caps.at(1) {       
        let left = tree_from_regex::<T>(caps.at(2).unwrap()).unwrap(); // Push on the left subtree
        let right = tree_from_regex::<T>(caps.at(3).unwrap()).unwrap(); // Push on the right subtree
        
        let first = T::join(
            T::I::from_string(x).unwrap(),
            left,
            right
        );

        return Ok(first);
    };

    if let Some(x) = caps.at(4) {
        let first = T::new(vec![
            Node::Leaf(T::L::from_string(x).unwrap()),
        ]);

        return Ok(first);
    };

    Err(RegexParseError::UnableToParseRegex)
}

// #[cfg(test)]
// mod tests {

//     use super::expression_from_regex;
//     use crate::expression;

//     #[test]
//     fn test_expression_from_string(){

//         let tree_string = r"+(a)(b)";

//         let tree = expression_from_regex(tree_string).unwrap();
//         assert_eq!(tree, expression![
//             Operator {
//                 label: b'+',
//             },
//             Element {
//                 label: b"a".to_vec(),
//             },
//             Element {
//                 label: b"b".to_vec(),
//             },
//         ]);
//     }

//     #[test]
//     fn test_tree_from_string_2(){

//         let tree_string = r"*(a)(+(b)(c))";

//         let tree = expression_from_regex(tree_string).unwrap();
//         assert_eq!(tree, expression![
//             Operator {
//                 label: b'*',
//             },
//             Element {
//                 label: b"a".to_vec(),
//             },
//             Operator {
//                 label: b'+',
//             },
//             Element {
//                 label: b"b".to_vec(),
//             },
//             Element {
//                 label: b"c".to_vec(),
//             },
//         ]);
//     }

//     #[test]
//     fn test_tree_from_string_3(){

//         let tree_string = r"+(*(a)(b))(*(a)(c))";

//         let tree = expression_from_regex(tree_string).unwrap();
//         assert_eq!(tree, expression![
//             Operator {
//                 label: b'+',
//             },
//             Operator {
//                 label: b'*',
//             },
//             Element {
//                 label: b"a".to_vec(),
//             },
//             Element {
//                 label: b"b".to_vec(),
//             },
//             Operator {
//                 label: b'*',
//             },
//             Element {
//                 label: b"a".to_vec(),
//             },
//             Element {
//                 label: b"c".to_vec(),
//             },
//         ]);
//     }
// }