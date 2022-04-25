use crate::algo::structures::{IdentityLeafNode, Operator, Node, TreeNode};
use std::slice::Iter;
use super::operator_to_regex;
use super::super::TreeParseError;

/// Convert an replacement object into it's pre-traversal string representation, using regex replacement syntax
pub fn replacement_to_regex<'a>(identity: &mut Iter<Node<IdentityLeafNode, Operator>>, wildcards: &Vec<Vec<u8>>) -> Result<String, TreeParseError>{
    Ok(match identity.next() {

        Some(Node::Leaf(IdentityLeafNode::Element(e))) => e.to_string(),

        Some(Node::Leaf(IdentityLeafNode::Subtree(s))) => {
            let mut result = String::new();
            match wildcards.iter().position(|r| r == &s.label) {
                Some(index) => {
                    result.push_str(&format!(r"{{{{g{index}}}}}"));
                },
                None => return Err(TreeParseError::InvalidWildcard)
            };
            result
        }
        Some(Node::Internal(o)) => {
            let left_subtree_string = replacement_to_regex(identity, wildcards)?;
            let right_subtree_string = replacement_to_regex(identity, wildcards)?;
            let result = operator_to_regex(o, &left_subtree_string, &right_subtree_string, false);
            result
        },

        None => panic!()
    })
}

#[cfg(test)]
mod tests {

    use crate::identity_expression;
    use crate::algo::structures::{Tree};
    use super::replacement_to_regex;
    use super::super::super::TreeParseError;

    #[test]
    fn test_replacement_to_string(){
        let replacement = identity_expression![
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
        let element_x = b"x".to_vec();
        let result = replacement_to_regex(&mut replacement.iter(), &vec![
            element_x,
        ]).unwrap();

        assert_eq!(result, r"\*\(a\)\($1\)");
    }

    #[test]
    fn test_replacement_to_string_with_unknown_wildcard(){
        let replacement = identity_expression![
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

        let return_val = replacement_to_regex(&mut replacement.iter(), &vec![]);
        
        assert_eq!(return_val, Err(TreeParseError::InvalidWildcard))
    }
}