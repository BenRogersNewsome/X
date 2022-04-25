use std::slice::Iter;

use crate::algo::structures::{
    Operator, Node,
    IdentityLeafNode,
    Element, TreeNode
};

#[derive(Debug)]
pub enum FormatError {

}

pub fn format_tree<T: TreeNode>(tree: &mut Iter<Node<T, Operator>>) -> Result<String, FormatError> {
    Ok(match tree.next() {

        Some(Node::Leaf(e)) => e.to_string(),

        Some(Node::Internal(o)) => {
            let left_subtree_string = format_tree(tree)?;
            let right_subtree_string = format_tree(tree)?;
            let result = operator_to_string(&o, &left_subtree_string, &right_subtree_string);
            result
        },

        None => panic!("Unexpected end to tree"),
    })
}

pub fn format_expression(expression: &mut Iter<Node<Element, Operator>>) -> Result<String, FormatError> {
    format_tree::<Element>(expression)
}

pub fn format_pattern(pattern: &mut Iter<Node<IdentityLeafNode, Operator>>) -> Result<String, FormatError> {
    format_tree::<IdentityLeafNode>(pattern)
}

fn operator_to_string(operator: &Operator, left_subtree: &str, right_subtree: &str) -> String {
    format!("({} {} {})", left_subtree, operator.to_string(), right_subtree)
}