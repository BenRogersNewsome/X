use solar_bt::{Node, PatternLeaf};
use super::nodes::BinaryOperator;
use super::nodes::Element;

use super::token_tree::TokenTree;

pub type ExpressionPattern = TokenTree<PatternLeaf<Element>, BinaryOperator>;

impl<Idx> std::ops::Index<Idx> for ExpressionPattern where Idx: std::slice::SliceIndex<[Node<PatternLeaf<Element>, BinaryOperator>]> {
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.tokens.index(index)
    }
}

/// An identity contains two patterns, one is the matcher and the other the replacement, respectively.
///
/// Identities are assumed to be reversible.
#[derive(Debug, Clone)]
pub struct Identity(pub ExpressionPattern, pub ExpressionPattern);

impl Identity {
    pub fn invert(&self) -> Self {
        Identity(self.1.clone(), self.0.clone())
    }
}

impl ToString for Identity {
    fn to_string(&self) -> String {
        let left: String = self.0.to_string();
        let right: String = self.1.to_string();
        format!("{} = {}", left, right)
    }
}

impl Into<(ExpressionPattern, ExpressionPattern)> for Identity {
    fn into(self) -> (ExpressionPattern, ExpressionPattern) {
        (self.0, self.1)
    }
}

impl<'a> Into<(&'a ExpressionPattern, &'a ExpressionPattern)> for &'a Identity {
    fn into(self) -> (&'a ExpressionPattern, &'a ExpressionPattern) {
        (&self.0, &self.1)
    }
}

/// Create identity_expressions, similar to how vec! works, only `Leaf()` and `Internal()` may be ommitted from the vec members, as they will be added automatically.
/// When code is compiled with the -O flag, this macro leads to no runtime performance impact.
#[macro_export]
macro_rules! identity_expression {
    ($($x:expr),+ $(,)?) => {{
        use $crate::structures::{ExpressionPattern, Element, BinaryOperator};
        use solar_bt::{Node, PatternLeaf};

        pub trait Wrapped {
            fn get_wrapped(self) -> Node<PatternLeaf<Element>, BinaryOperator>;
        }

        impl Wrapped for PatternLeaf<Element> {
            #[inline(always)]
            fn get_wrapped(self) -> Node<PatternLeaf<Element>, BinaryOperator> {
                return Node::Leaf(self)
            }
        }

        impl Wrapped for BinaryOperator {
            #[inline(always)]
            fn get_wrapped(self) -> Node<PatternLeaf<Element>, BinaryOperator> {
                return Node::Internal(self)
            }
        }
        ExpressionPattern::new(vec![$(($x).get_wrapped()),+])
    }}
}

pub(crate) use identity_expression;


#[cfg(test)]
mod test_macros {

    use crate::structures::{Element, BinaryOperator};
    use solar_bt::{Node, PatternLeaf};

    #[test]
    fn test_create_identity_expression() {
        let plus = BinaryOperator::new(b'+');
        let a = Element::new(b"a");
        let b = String::from("b");

        let expression = identity_expression![
            plus,
            PatternLeaf::Literal(a.clone()),
            PatternLeaf::Subtree(b.clone()),
        ];

        assert_eq!(*expression, vec![
            Node::Internal(plus),
            Node::Leaf(PatternLeaf::Literal(a)),
            Node::Leaf(PatternLeaf::Subtree(b)),
        ]);
    }
}