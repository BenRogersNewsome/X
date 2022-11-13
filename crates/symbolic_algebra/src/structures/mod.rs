mod nodes {
    mod binary_operator;
    mod element;
    
    pub use element::Element;
    pub use binary_operator::BinaryOperator;
}

mod sets {
    mod cardinality;
    mod operation;
    mod set;
    mod set_element;
    mod context;
}

use env_logger;

mod token_tree;

pub mod algebras;
#[macro_use]
mod expression;
#[macro_use]
mod identity;
mod algebra;

pub use algebra::Algebra;
pub use expression::Expression;
pub use identity::{Identity, ExpressionPattern};
pub use nodes::{Element, BinaryOperator};