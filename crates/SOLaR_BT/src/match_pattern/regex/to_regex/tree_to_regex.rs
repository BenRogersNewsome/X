use std::slice::Iter;
use crate::{tree::{Node, Tree, TreeNode}, match_pattern::regex::TreeParseError};

pub fn tree_to_regex<T: Tree>(tree: &T) -> Result<String, TreeParseError> {
    _tree_to_regex::<T>(&mut tree.iter())
}

fn _tree_to_regex<T: Tree>(tree: &mut Iter<Node<T::L, T::I>>) -> Result<String, TreeParseError> {
    Ok(match tree.next() {

        Some(Node::Leaf(e)) => e.uid(),

        Some(Node::Internal(o)) => {
            let left_subtree_string = _tree_to_regex::<T>(tree)?;
            let right_subtree_string= _tree_to_regex::<T>(tree)?;
            internal_to_regex::<T>(o, &left_subtree_string, &right_subtree_string, false)
        },

        None => panic!()
    })
}

pub fn internal_to_regex<T: Tree>(internal_node: &T::I, left: &str, right: &str, regex: bool) -> String {
    let mut result = String::new();

    let escape = |_result: &mut String| {
        if regex { 
            _result.push('\\');
        };
    };
    escape(&mut result);
    result.push_str(&internal_node.to_string());

    escape(&mut result);
    result.push('(');
    result.push_str(left);
    escape(&mut result);
    result.push(')');

    escape(&mut result);
    result.push('(');
    result.push_str(right);
    escape(&mut result);
    result.push(')');

    result
}

