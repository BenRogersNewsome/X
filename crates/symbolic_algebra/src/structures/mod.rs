mod nodes {
    mod binary_operator;
    mod element;
    mod identity_leaf_node;
    
    pub use element::Element;
    pub use identity_leaf_node::IdentityLeafNode;
    pub use binary_operator::BinaryOperator;
}

pub mod algebras;
mod expression;
mod identity;
mod algebra;

// pub use algebra::Algebra;
pub use expression::Expression;
pub use identity::{Identity, ExpressionPattern};
pub use nodes::{Element, BinaryOperator, IdentityLeafNode};
pub use algebra::Algebra;