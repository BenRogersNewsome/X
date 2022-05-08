use std::slice::Iter;
use super::super::TreeParseError;
use crate::{
    tree::{Node, TreeNode},
    pattern::{Pattern, PatternLeaf},
};
use super::tree_to_regex::internal_to_regex;


pub fn pattern_to_regex<P: Pattern>(pattern: &P) -> Result<(String, Vec<Vec<u8>>), TreeParseError> {
    let mut wildcards = vec![];
    let result = _pattern_to_regex::<P>(&mut pattern.iter(), &mut wildcards)?;

    Ok((result, wildcards))
} 

/// Convert an identity object into it's pre-traversal string representation, using regex pattern matching.
fn _pattern_to_regex<'a, P: Pattern>(pattern: &mut Iter<Node<P::L, P::I>>, wildcards: &mut Vec<Vec<u8>>) -> Result<String, TreeParseError>{

    Ok(match pattern.next() {

        Some(Node::Leaf(PatternLeaf::Literal(e))) => e.to_string(),

        Some(Node::Leaf(PatternLeaf::Subtree(s))) => {
            let mut result = String::new();
            match wildcards.iter().position(|r| r == s.as_bytes()) {
                Some(index) => {
                    result.push_str(&format!(r"\k<g{index}>"));
                },
                None => {
                    let index = wildcards.len();
                    result.push_str(&super::regex::subtree_regex(format!(r"g{index}")));
                    wildcards.push(s.as_bytes().to_vec());
                }
            };
            result
        },

        Some(Node::Internal(o)) => {
            let left_subtree_string = _pattern_to_regex::<P>(pattern, wildcards)?;
            let right_subtree_string = _pattern_to_regex::<P>(pattern, wildcards)?;
            internal_to_regex::<P>(o, &left_subtree_string, &right_subtree_string, true)
        },

        None => panic!()
    })
}

// #[cfg(test)]
// mod tests {

//     use crate::identity_expression;
//     use crate::algo::core::trees::Tree;
//     use crate::algo::structures::ExpressionPattern;
//     use super::pattern_to_regex;

//     #[test]
//     fn test_pattern_to_string_1() {
//         let pattern: ExpressionPattern = identity_expression![
//             BinaryOperator {
//                 label: b'*',
//             },
//             Element(Element {
//                 label: b"a".to_vec(),
//             }),
//             Subtree(Element {
//                 label: b"x".to_vec(),
//             })
//         ];

//         let result= pattern_to_regex(&mut pattern.iter(), &mut vec![]).unwrap();
//         assert_eq!(result.as_str(), r"\*\(a\)\((.+)\)");
//     }

//     #[test]
//     fn test_pattern_to_string_2() {
//         let pattern: ExpressionPattern = identity_expression![
//             BinaryOperator {
//                 label: b'*',
//             },
//             Subtree(Element {
//                 label: b"x".to_vec(),
//             }),
//             BinaryOperator {
//                 label: b'+',
//             },
//             Element(Element {
//                 label: b"a".to_vec(),
//             }),
//             Subtree(Element {
//                 label: b"x".to_vec(),
//             }),
//         ];

//         let result = pattern_to_regex(&mut pattern.iter(), &mut vec![]).unwrap();
//         assert_eq!(result.as_str(), r"\*\((.+)\)\(\+\(a\)\(\1\)\)");
//     }
// }