use std::{iter::Peekable, slice::Iter, vec::IntoIter, fmt::Debug};

use crate::{pattern::{NodeSpecification, PatternTree, LeafPattern}, Node, TreeNode, Capture};

use super::Tree;

use crate::LeafReplacement;

#[derive(Clone, PartialEq, Debug)]
pub enum MatchTreeNode<'a, T: Tree> {
    Literal(&'a TreeNode<T>),
    Capture(usize),
}

/// A tree holding the matched subtree of the pattern in the host tree.
#[derive(Clone, PartialEq, Debug)]
pub struct MatchTree<'a, T: Tree + 'a> {
    nodes: Vec<TreeNode<Self>>,
}

impl<'a, T: Tree + 'a> MatchTree<'a, T> {
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

impl<'a, T: Tree + 'a> IntoIterator for MatchTree<'a, T> {
    type Item = TreeNode<Self>;
    type IntoIter = IntoIter<Self::Item>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.nodes.into_iter()
    }

}

impl<'a, T: Tree + 'a> Tree for MatchTree<'a, T> {
    type Leaf = MatchTreeNode<'a, T>;
    type Binary = T::Binary;
    type Unary = T::Unary;

    fn new<I: IntoIterator<Item = TreeNode<Self>>>(nodes: I) -> Self {
        Self {
            nodes: nodes.into_iter().collect()
        }
    }

    fn iter<'b>(&'b self) -> std::slice::Iter<TreeNode<Self>> {
        self.nodes.iter()
    }
}

/// The result of a match of a pattern on a tree
pub struct MatcherResult<'a, T: Tree> {
    original_tree: &'a T,
    start_position: usize,
    end_position: usize,
    matched_subtree: MatchTree<'a, T>,
    captures: Vec<Capture<'a, T>>,
}

impl<'a, T: Tree> MatcherResult<'a, T> {

    #[inline]
    pub fn new(
        original_tree: &'a T,
        start_position: usize,
        end_position: usize,
        matched_subtree: MatchTree<'a, T>,
        captures: Vec<Capture<'a, T>>,
    ) -> Self {
        Self {
            original_tree,
            start_position,
            end_position,
            matched_subtree,
            captures,
        }
    }

    pub fn apply_replacement<
        ReplacementTree: Tree<
            Leaf = LeafReplacement<T>,
            Binary = T::Binary,
            Unary = T::Unary
        >
    >(&self, replacement: &ReplacementTree) -> T where T: Clone, T::Binary: Clone, T::Unary: Clone, T::Leaf: Clone {

        let resultant_tree_nodes = 
            self.original_tree.iter().take(self.start_position).cloned()
                .chain(
                    replacement.iter().flat_map::<Box<dyn Iterator<Item = TreeNode<T>>>, _>(|replacement_tree_node: &TreeNode<ReplacementTree>| {
                        match replacement_tree_node {
                            Node::Binary(b) =>
                                Box::new(std::iter::once(Node::Binary(b.clone()))),
                            Node::Unary(u) =>
                                Box::new(std::iter::once(Node::Unary(u.clone()))),
                            Node::Leaf(LeafReplacement::Literal(literal_tree)) =>
                                Box::new(literal_tree.clone().into_iter()),
                            Node::Leaf(LeafReplacement::Captured(c)) => 
                                Box::new(self.captures[*c].captured_nodes()),
                        }
                    })
                )
                .chain(
                    self.original_tree.iter().skip(self.end_position).cloned()
                );
        
        T::new(resultant_tree_nodes)
    }

    pub fn position(&self) -> usize {
        self.start_position
    }

}

pub enum ContinuePatternResult<
    'a, 'b: 'a,
    T: Tree + 'b,
    P: PatternTree<T> + 'a
> {
    Complete(MatcherResult<'b, T>),
    Continue(ActivePattern<'a, 'b, T, P>),
    End,
}

pub struct ActivePattern<
    'a, 'b: 'a,
    T: Tree + 'b,
    P: PatternTree<T> + 'a
> {
    original_tree: &'b T,
    pattern: Peekable<Iter<'a, TreeNode<P>>>,
    captures: Vec<Capture<'b, T>>,
    current_callback: Capture<'b, T>,
    start_position: usize,
    current_position: usize,
    matched_segments: Vec<TreeNode<MatchTree<'b, T>>>,
}

impl<
    'a, 'b: 'a,
    T: Tree + 'b,
    P: PatternTree<T> + 'a
> ActivePattern<'a, 'b, T, P> {

    pub fn new(pattern: Iter<'a, TreeNode<P>>, position: usize, original_tree: &'b T) -> Self {
        Self {
            original_tree,
            pattern: pattern.peekable(),
            captures: Vec::new(),
            current_callback: Capture::new(0),
            start_position: position,
            current_position: position,
            matched_segments: Vec::new(),
        }
    }

    pub fn continue_pattern(mut self, tree_token: &'b TreeNode<T>) -> ContinuePatternResult<'a, 'b, T, P> {
        self.current_position += 1;
        return match (self.pattern.peek(), tree_token) {
    
            (Some(Node::Leaf(LeafPattern::Specification(spec))), Node::Leaf(x)) => {
                if !spec.is_match(x) {
                    return ContinuePatternResult::End;
                }
                self.try_move_to_next_pattern_token()
            },
    
            (Some(Node::Binary(pb)), Node::Binary(b)) if pb.is_match(b) => {
                self.try_move_to_next_pattern_token()
            },

            (Some(Node::Unary(pu)), Node::Unary(u)) if pu.is_match(u) => {
                self.try_move_to_next_pattern_token()
            },
    
            (Some(Node::Leaf(LeafPattern::Subtree)), _) => {
                if self.captures.last_mut().expect("Must have added capture before assignment").add_token(tree_token) {
                    return self.try_move_to_next_pattern_token();
                }
                ContinuePatternResult::Continue(self)
            },

            (Some(Node::Leaf(LeafPattern::SubtreeCallback(subtree_index))), _) => {
                if self.current_callback.add_token(tree_token) {

                    let previous_capture: Vec<_> = self.captures[*subtree_index].captured_nodes().collect();
                    let this_capture: Vec<_> = self.current_callback.captured_nodes().collect();
                    if previous_capture == this_capture {
                        return self.try_move_to_next_pattern_token();
                    } else {
                        return ContinuePatternResult::End;
                    }
                }
                ContinuePatternResult::Continue(self)
            },

            (None, _) => {
                unreachable!("Should have caught this in `try_move_to_next_pattern_token`");
            },

            _ => {
                ContinuePatternResult::End
            }
        };
    }

    fn to_match_result(self) -> MatcherResult<'b, T> {
        MatcherResult::new(
            self.original_tree,
            self.start_position,
            self.current_position,
            MatchTree::new(self.matched_segments),
            self.captures
        )
    }

    /// Move to the next token and return a `ContinuePatternResult` based on whether there is a token or not.
    fn try_move_to_next_pattern_token(mut self) -> ContinuePatternResult<'a, 'b, T, P> {
        self.pattern.next();
        match self.pattern.peek() {
            Some(Node::Leaf(LeafPattern::Subtree)) => {
                let new_capture = Capture::new(self.current_position);
                self.captures.push(new_capture);
                self.matched_segments.push(Node::Leaf(MatchTreeNode::Capture(self.captures.len() - 1)));
                ContinuePatternResult::Continue(self)
            },
            Some(Node::Leaf(LeafPattern::SubtreeCallback(_))) => {
                self.current_callback = Capture::new(self.current_position);
                ContinuePatternResult::Continue(self)
            }
            None => ContinuePatternResult::Complete(self.to_match_result()),
            _ => ContinuePatternResult::Continue(self),
        }
    }

}