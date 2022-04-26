use onig::{Regex, RegexOptions, Syntax};
use handlebars::Handlebars;
use serde_json::{Value};

use crate::algo::{
    regex::{
        Regexable
    },
    structures::{
        Expression, Identity,
    },
};

pub fn replace_identity<'a>(expression: &Expression, identity: &Identity) -> Result<Vec<Expression>, &'static str> {

    let expression_string = expression.to_regex().unwrap();
    let (pattern_string, replacement_string) = identity.to_regex().unwrap();
    let re = Regex::with_options(
        &pattern_string,
        RegexOptions::REGEX_OPTION_DONT_CAPTURE_GROUP,
        Syntax::default(),
    ).unwrap();

    let captures_iter = re.captures_iter(&expression_string);
    let mut new_tree_strings = vec![];

    for caps in captures_iter {
        let root_cap_pos = caps.pos(0).unwrap();
        let mut new_tree_string = String::new();
        new_tree_string.push_str(
            &expression_string[..root_cap_pos.0]
        );
        let mut data = String::new();
        data.push_str("{\n");
        for (i, cap) in caps.iter().enumerate(){
            if i != 0 {
                data.push_str(&format!(r#""g{}": "{}""#, i-1, cap.unwrap()));
                if i < caps.len() - 1 {
                    data.push(',');
                }
                data.push('\n');
            }
        }
        data.push_str("}");
        let reg = Handlebars::new();
        let val: Value = serde_json::from_str(&data).unwrap();
        new_tree_string.push_str(
            &reg.render_template(&replacement_string, &val).unwrap()
        );
        new_tree_string.push_str(
            &expression_string[root_cap_pos.1..]
        );

        new_tree_strings.push(new_tree_string);
    };  

    let new_trees = new_tree_strings.iter().map(|s| Expression::from_regex(s).unwrap()).collect();
    Ok(new_trees)
}

// #[cfg(test)]
// mod tests {
//     use crate::algo::{trees::{node::{ Operator, Element}, identity::{IdentityToken, IdentityExpression}}, regex::from_string::tree_from_string};
//     use super::replace_identity;

//     #[test]
//     fn test_replace_identity() {
        
//         let tree = tree_from_string(r"+(*(a)(+(c)(d)))(+(*(b)(c))(*(b)(d)))").unwrap();
//         let pattern: IdentityExpression = vec![
//             IdentityToken::Operator(Operator {
//                 label: b'*'
//             }),
//             IdentityToken::Subtree(Element {
//                 label: b"x".to_vec(),
//             }),
//             IdentityToken::Subtree(Element {
//                 label: b"y".to_vec(),
//             }),
//         ];

//         let replacement: IdentityExpression = vec![
//             IdentityToken::Operator(Operator {
//                 label: b'*'
//             }),
//             IdentityToken::Subtree(Element {
//                 label: b"y".to_vec(),
//             }),
//             IdentityToken::Subtree(Element {
//                 label: b"x".to_vec(),
//             }),
//         ];

//         let after = replace_identity(&tree, &(pattern, replacement)).unwrap();
//         assert_eq!(after[0], tree_from_string(r"+(*(+(c)(d))(a))(+(*(b)(c))(*(b)(d)))").unwrap());
//     }

// }