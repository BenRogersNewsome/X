pub mod to_regex {
    mod tree_to_regex;
    mod pattern_to_regex;
    mod replacement_to_regex;
    mod regex;
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
    mod tree_from_regex;

    pub use tree_from_regex::tree_from_regex;
}


pub trait Regexable {

    type Output;

    fn to_regex(&self) -> Result<Self::Output, TreeParseError>;
    fn from_regex<'a>(regex: &'a str) -> Result<Self, RegexParseError> where Self: Sized;
}