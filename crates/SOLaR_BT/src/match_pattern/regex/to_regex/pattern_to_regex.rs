use std::slice::Iter;
use super::super::TreeParseError;
use crate::{
    tree::{Node, TreeNode, Tree},
    pattern::PatternLeaf,
};
use super::tree_to_regex::internal_to_regex;


pub fn pattern_to_regex<
        L: TreeNode,
        I: TreeNode,
        P: Tree<L=PatternLeaf<L>, I=I>
    >(pattern: &P) -> Result<(String, Vec<Vec<u8>>), TreeParseError> {
    
        let mut wildcards = vec![];
        let result = _pattern_to_regex::<L, I, P>(&mut pattern.iter(), &mut wildcards)?;

        let lookahead_result = format!("(?=(?<a>{}))", result);

        Ok((lookahead_result, wildcards))
} 

/// Convert an identity object into it's pre-traversal string representation, using regex pattern matching.
fn _pattern_to_regex<
    L: TreeNode,
    I: TreeNode,
    P: Tree<L=PatternLeaf<L>, I=I>>(pattern: &mut Iter<Node<P::L, P::I>>, wildcards: &mut Vec<Vec<u8>>) -> Result<String, TreeParseError> {

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
                let left_subtree_string = _pattern_to_regex::<L, I, P>(pattern, wildcards)?;
                let right_subtree_string = _pattern_to_regex::<L, I, P>(pattern, wildcards)?;
                internal_to_regex::<P>(o, &left_subtree_string, &right_subtree_string, true)
            },

            None => panic!()
        })
}