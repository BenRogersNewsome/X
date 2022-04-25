use super::operator_to_regex;
use crate::algo::structures::{TreeNode, Node, IdentityLeafNode, Operator};
use std::slice::Iter;
use super::super::TreeParseError;
use super::regex;

/// Convert an identity object into it's pre-traversal string representation, using regex pattern matching.
pub fn pattern_to_regex<'a>(identity: &mut Iter<Node<IdentityLeafNode, Operator>>, wildcards: &mut Vec<Vec<u8>>) -> Result<String, TreeParseError>{

    Ok(match identity.next() {

        Some(Node::Leaf(IdentityLeafNode::Element(e))) => e.to_string(),

        Some(Node::Leaf(IdentityLeafNode::Subtree(s))) => {
            let mut result = String::new();
            match wildcards.iter().position(|r| r == &s.label) {
                Some(index) => {
                    result.push_str(&format!(r"\k<g{index}>"));
                },
                None => {
                    let index = wildcards.len();
                    result.push_str(&regex::subtree_regex(format!(r"g{index}")));
                    wildcards.push(s.label.clone());
                }
            };
            result
        },

        Some(Node::Internal(o)) => {
            let left_subtree_string = pattern_to_regex(identity, wildcards)?;
            let right_subtree_string = pattern_to_regex(identity, wildcards)?;
            operator_to_regex(o, &left_subtree_string, &right_subtree_string, true)
        },

        None => panic!()
    })
}

#[cfg(test)]
mod tests {

    use crate::identity_expression;
    use crate::algo::structures::{
        IdentityExpression, Tree
    };
    use super::pattern_to_regex;

    #[test]
    fn test_pattern_to_string_1() {
        let pattern: IdentityExpression = identity_expression![
            Operator {
                label: b'*',
            },
            Element(Element {
                label: b"a".to_vec(),
            }),
            Subtree(Element {
                label: b"x".to_vec(),
            })
        ];

        let result= pattern_to_regex(&mut pattern.iter(), &mut vec![]).unwrap();
        assert_eq!(result.as_str(), r"\*\(a\)\((.+)\)");
    }

    #[test]
    fn test_pattern_to_string_2() {
        let pattern: IdentityExpression = identity_expression![
            Operator {
                label: b'*',
            },
            Subtree(Element {
                label: b"x".to_vec(),
            }),
            Operator {
                label: b'+',
            },
            Element(Element {
                label: b"a".to_vec(),
            }),
            Subtree(Element {
                label: b"x".to_vec(),
            }),
        ];

        let result = pattern_to_regex(&mut pattern.iter(), &mut vec![]).unwrap();
        assert_eq!(result.as_str(), r"\*\((.+)\)\(\+\(a\)\(\1\)\)");
    }
}