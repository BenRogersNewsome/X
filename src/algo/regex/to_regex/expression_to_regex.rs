use super::operator_to_regex;
use super::super::TreeParseError;
use crate::algo::structures::{Element, Operator, Node, TreeNode};
use std::slice::Iter;

/// Convert a tree object into it's pre-traversal string representation

pub fn expression_to_regex(expression: &mut Iter<Node<Element, Operator>>) -> Result<String, TreeParseError> {
    Ok(match expression.next() {

        Some(Node::Leaf(e)) => e.to_string(),

        Some(Node::Internal(o)) => {
            let left_subtree_string = expression_to_regex(expression)?;
            let right_subtree_string= expression_to_regex(expression)?;
            let result = operator_to_regex(&o, &left_subtree_string, &right_subtree_string, false);
            result
        },

        None => panic!()
    })
}

#[cfg(test)]
mod tests {

    use crate::expression;
    use crate::algo::structures::{Expression, Tree};
    use super::expression_to_regex;

    #[test]
    fn test_expression_to_regex_1() {
        let expression: Expression = expression! [
            Operator {
                label: b'*',
            },
            Element {
                label: b"a".to_vec(),
            },
            Element {
                label: b"b".to_vec(),
            },
        ];

        let result = expression_to_regex(&mut expression.iter()).unwrap();
        assert_eq!(result.as_str(), r"\*\(a\)\(b\)");
    }

    #[test]
    fn test_expression_to_regex_2() {
        let expression: Expression = expression! [
            Operator {
                label: b'*',
            },
            Operator {
                label: b'+',
            },
            Operator {
                label: b'*',
            },
            Element {
                label: b"a".to_vec(),
            },
            Element {
                label: b"b".to_vec(),
            },
            Operator {
                label: b'+',
            },
            Element {
                label: b"c".to_vec(),
            },
            Element {
                label: b"d".to_vec(),
            },
            Element {
                label: b"a".to_vec(),
            },
        ];

        let result = expression_to_regex(&mut expression.iter()).unwrap();
        assert_eq!(result.as_str(), r"\*\(\+\(\*\(a\)\(b\)\)\(\+\(c\)\(d\)\)\)\(a\)");
    }
}