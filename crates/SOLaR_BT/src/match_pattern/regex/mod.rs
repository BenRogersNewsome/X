mod to_regex {
    pub mod tree_to_regex;
    pub mod pattern_to_regex;
    pub mod replacement_to_regex;
    mod regex;
}

pub use to_regex::tree_to_regex::tree_to_regex;
pub use to_regex::pattern_to_regex::pattern_to_regex;
pub use to_regex::replacement_to_regex::replacement_to_regex;

mod from_regex {
    pub mod tree_from_regex;
}

pub use from_regex::tree_from_regex::tree_from_regex;

#[derive(Debug, PartialEq)]
pub enum RegexParseError {
    UnableToParseRegex,
}

#[derive(Debug, PartialEq)]
pub enum TreeParseError {
    InvalidWildcard,
}