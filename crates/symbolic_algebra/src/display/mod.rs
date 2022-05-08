use crate::algo::structures::{Expression, Identity, Tree};

mod similarity;
mod format_tree;

pub use similarity::similarity;

pub trait Display {

    fn format(&self) -> String;

}

impl Display for Expression {

    fn format(&self) -> String {
        format_tree::format_expression(&mut self.iter()).unwrap()
    }

}

impl Display for Identity {

    fn format(&self) -> String {
        format!(
            "{} = {}",
            format_tree::format_pattern(&mut self.0.iter()).unwrap(),
            format_tree::format_pattern(&mut self.1.iter()).unwrap()
        )
    }

}