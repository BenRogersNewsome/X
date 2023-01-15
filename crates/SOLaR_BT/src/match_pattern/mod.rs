use std::slice::Iter;

use crate::{pattern::PatternTree, Node, TreeNode};

use self::active_pattern::ContinuePatternResult;

use super::Tree;

mod capture;
pub use capture::Capture;


mod active_pattern;
use active_pattern::ActivePattern;
pub use active_pattern::MatcherResult;


pub fn apply<
    'a, 'b,
    T: Tree + 'b,
    P: PatternTree<T> + 'a
>(
    pattern: &'a P,
    tree: &'b T,
) -> Vec<MatcherResult<'b, T>> where 'b: 'a {

    let pattern_tokens: Iter<'a, TreeNode<P>> = pattern.iter();
    let mut active_patterns: Vec<ActivePattern<'a, 'b, T, P>> = vec![];
    let mut results: Vec<MatcherResult<'b, T>> = Vec::new();

    for (position, tree_token) in tree.iter().enumerate() {
        active_patterns.push(ActivePattern::new(pattern_tokens.clone(), position, tree));

        active_patterns = active_patterns.into_iter().filter_map(|active_pattern| {
            match active_pattern.continue_pattern(tree_token) {
                ContinuePatternResult::End => None,
                ContinuePatternResult::Continue(active_pattern) => Some(active_pattern),
                ContinuePatternResult::Complete(result) => {
                    results.push(result);
                    None
                },
            }
        }).collect();
    };

    results

}