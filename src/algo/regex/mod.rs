use super::structures::{Expression, Identity};


pub mod to_regex {

    use crate::algo::structures::{Expression, Identity, Tree};

    use super::TreeParseError;

    mod operator_to_regex;
    mod expression_to_regex;
    mod pattern_to_regex;
    mod replacement_to_regex;
    mod regex;

    use operator_to_regex::operator_to_regex;
    use expression_to_regex::expression_to_regex;
    use pattern_to_regex::pattern_to_regex;
    use replacement_to_regex::replacement_to_regex;

    pub fn expression(expression: &Expression) -> Result<String, TreeParseError> {
        expression_to_regex(&mut expression.iter())
    }

    pub fn identity(identity: &Identity) -> Result<(String, String), TreeParseError> {
        let mut wildcards = vec![];
        let pattern_string = pattern_to_regex(&mut identity.0.iter(), &mut wildcards)?;
        let replacement_string = replacement_to_regex(&mut identity.1.iter(), &wildcards)?;
        Ok((pattern_string, replacement_string))
    }
}

#[derive(Debug, PartialEq)]
pub enum RegexParseError {
    UnableToParseRegex,
}

#[derive(Debug, PartialEq)]
pub enum TreeParseError {
    UnableToParseTree,
    InvalidWildcard,
}

mod from_regex {
    mod expression_from_regex;

    pub use expression_from_regex::expression_from_regex as expression;
}


pub trait Regexable {

    type Output;

    fn to_regex(&self) -> Result<Self::Output, TreeParseError>;
    fn from_regex<'a>(regex: &'a str) -> Result<Self, RegexParseError> where Self: Sized;
}

impl Regexable for Expression {

    type Output = String;

    fn to_regex(&self) -> Result<String, TreeParseError> {
        Ok(to_regex::expression(&self)?)
    }

    fn from_regex<'a>(regex: &'a str) -> Result<Self, RegexParseError> {
        Ok(from_regex::expression(&regex)?)
    }
}

impl Regexable for Identity {

    type Output = (String, String);

    fn to_regex(&self) -> Result<(String, String), TreeParseError> {
        Ok(to_regex::identity(&self)?)
    }

    fn from_regex<'a>(_: &'a str) -> Result<Self, RegexParseError> {
        panic!() // Calling this an antipattern would be kind
    }
}