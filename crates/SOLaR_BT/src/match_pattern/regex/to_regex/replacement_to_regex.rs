use std::slice::Iter;
use super::super::TreeParseError;
use crate::{
    tree::{Node, TreeNode, Tree},
    pattern::PatternLeaf
};
use super::tree_to_regex::internal_to_regex;

pub fn replacement_to_regex<
        L: TreeNode,
        I: TreeNode,
        P: Tree<L=PatternLeaf<L>, I=I>
    >(pattern: &P, wildcards: &[Vec<u8>]) -> Result<String, TreeParseError> {
        let result = _replacement_to_regex::<L, I, P>(&mut pattern.iter(), wildcards)?;

        Ok(result)
} 

/// Convert an identity object into it's pre-traversal string representation, using regex pattern matching.
fn _replacement_to_regex<
        L: TreeNode,
        I: TreeNode,
        P: Tree<L=PatternLeaf<L>, I=I>
    >(pattern: &mut Iter<Node<P::L, P::I>>, wildcards: &[Vec<u8>]) -> Result<String, TreeParseError>{

        Ok(match pattern.next() {

            Some(Node::Leaf(PatternLeaf::Literal(e))) => e.to_string(),

            Some(Node::Leaf(PatternLeaf::Subtree(s))) => {
                let mut result = String::new();
                match wildcards.iter().position(|r| r == s.as_bytes()) {
                    Some(index) => {
                        result.push_str(&format!(r"{{{{g{index}}}}}"));
                    },
                    None => return Err(TreeParseError::InvalidWildcard)
                };
                result
            }
            Some(Node::Internal(o)) => {
                let left_subtree_string = _replacement_to_regex::<L, I, P>(pattern, wildcards)?;
                let right_subtree_string = _replacement_to_regex::<L, I, P>(pattern, wildcards)?;
                internal_to_regex::<P>(o, &left_subtree_string, &right_subtree_string, false)
            },

            None => panic!()
        })
}

// #[cfg(test)]
// mod tests {

//     use crate::identity_expression;
//     use crate::algo::core::trees::Tree;
//     use super::replacement_to_regex;
//     use super::super::super::TreeParseError;

//     #[test]
//     fn test_replacement_to_string(){
//         let replacement = identity_expression![
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
//         let element_x = b"x".to_vec();
//         let result = replacement_to_regex(&mut replacement.iter(), &vec![
//             element_x,
//         ]).unwrap();

//         assert_eq!(result, r"\*\(a\)\($1\)");
//     }

//     #[test]
//     fn test_replacement_to_string_with_unknown_wildcard(){
//         let replacement = identity_expression![
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

//         let return_val = replacement_to_regex(&mut replacement.iter(), &vec![]);
        
//         assert_eq!(return_val, Err(TreeParseError::InvalidWildcard))
//     }
// }