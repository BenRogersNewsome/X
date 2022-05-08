use std::slice::Iter;
use crate::tree::{Node, Tree, TreeNode};

pub fn tree_to_regex<T: Tree>(tree: T) -> Result<String, &'static str> {
    _tree_to_regex::<T>(&mut tree.iter())
}

fn _tree_to_regex<T: Tree>(tree: &mut Iter<Node<T::L, T::I>>) -> Result<String, &'static str> {
    Ok(match tree.next() {

        Some(Node::Leaf(e)) => e.uid(),

        Some(Node::Internal(o)) => {
            let left_subtree_string = _tree_to_regex::<T>(tree)?;
            let right_subtree_string= _tree_to_regex::<T>(tree)?;
            let result = internal_to_regex::<T>(o, &left_subtree_string, &right_subtree_string, false);
            result
        },

        None => panic!()
    })
}

pub fn internal_to_regex<T: Tree>(internal_node: &T::I, left: &String, right: &String, regex: bool) -> String {
    let mut result = String::new();

    let escape = |_result: &mut String| {
        if regex { 
            _result.push_str(r"\");
        };
    };
    escape(&mut result);
    result.push_str(&internal_node.to_string());

    escape(&mut result);
    result.push_str(r"(");
    result.push_str(left);
    escape(&mut result);
    result.push_str(r")");

    escape(&mut result);
    result.push_str(r"(");
    result.push_str(right);
    escape(&mut result);
    result.push_str(r")");

    return result;
}

