use crate::algo::structures::{Operator, Expression, Tree};
use crate::expression;
use lazy_static::lazy_static;
use onig::{Regex, RegexOptions, Syntax};
use super::super::RegexParseError;

const GLOBAL_REGEX: &'static str = r"^(?<op>[+*])\((?<one>[+*]\(\g<one>\)\(\g<one>\)|[a-z])\)\((?<two>[+*]\(\g<two>\)\(\g<two>\)|[a-z])\)$|^(?<el>[a-z])$";  // PITA

impl Expression {
    pub fn from_string() -> String {
        String::new()
    }
}

pub fn expression_from_regex<'a>(expression_string: &'a str) -> Result<Expression, RegexParseError>{
    lazy_static! {
        static ref GLOBAL_PATTERN: Regex = Regex::with_options(
            GLOBAL_REGEX,
            RegexOptions::REGEX_OPTION_DONT_CAPTURE_GROUP,
            Syntax::default(),
        ).unwrap();
    }

    let caps = GLOBAL_PATTERN.captures(expression_string).unwrap();
    match caps.at(1) {
        Some(x) => {
            
            let left = expression_from_regex(caps.at(2).unwrap()).unwrap(); // Push on the left subtree
            let right = expression_from_regex(caps.at(3).unwrap()).unwrap(); // Push on the right subtree
            
            let first = Expression::join(
                Operator {
                    label: x.as_bytes()[0],
                },
                left,
                right
            );

            return Ok(first);
        },
        None => {},
    };

    match caps.at(4) {
        Some(x) => {
            let first = expression![
                Element {
                    label: x.as_bytes().to_vec(),
                }
            ];

            return Ok(first);
        },
        None => {},
    };

    return Err(RegexParseError::UnableToParseRegex);
}

// #[cfg(test)]
// mod tests {

//     use super::expression_from_regex;
//     use crate::expression;

//     #[test]
//     fn test_expression_from_string(){

//         let tree_string = r"+(a)(b)";

//         let tree = expression_from_regex(tree_string).unwrap();
//         assert_eq!(tree, expression![
//             Operator {
//                 label: b'+',
//             },
//             Element {
//                 label: b"a".to_vec(),
//             },
//             Element {
//                 label: b"b".to_vec(),
//             },
//         ]);
//     }

//     #[test]
//     fn test_tree_from_string_2(){

//         let tree_string = r"*(a)(+(b)(c))";

//         let tree = expression_from_regex(tree_string).unwrap();
//         assert_eq!(tree, expression![
//             Operator {
//                 label: b'*',
//             },
//             Element {
//                 label: b"a".to_vec(),
//             },
//             Operator {
//                 label: b'+',
//             },
//             Element {
//                 label: b"b".to_vec(),
//             },
//             Element {
//                 label: b"c".to_vec(),
//             },
//         ]);
//     }

//     #[test]
//     fn test_tree_from_string_3(){

//         let tree_string = r"+(*(a)(b))(*(a)(c))";

//         let tree = expression_from_regex(tree_string).unwrap();
//         assert_eq!(tree, expression![
//             Operator {
//                 label: b'+',
//             },
//             Operator {
//                 label: b'*',
//             },
//             Element {
//                 label: b"a".to_vec(),
//             },
//             Element {
//                 label: b"b".to_vec(),
//             },
//             Operator {
//                 label: b'*',
//             },
//             Element {
//                 label: b"a".to_vec(),
//             },
//             Element {
//                 label: b"c".to_vec(),
//             },
//         ]);
//     }
// }