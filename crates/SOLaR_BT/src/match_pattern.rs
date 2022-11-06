use onig::{Regex, RegexOptions, Syntax};
use handlebars::Handlebars;
use serde_json::Value;
use crate::{PatternLeaf, pattern};

use self::regex::{tree_to_regex, pattern_to_regex, tree_from_regex, replacement_to_regex};
use crate::tree::{ParsableTreeNode, Tree};

use onig::Captures;

mod regex;

#[derive(Debug)]
pub enum ReplacementError {

}

pub fn replace_identity<
        L: ParsableTreeNode,
        I: ParsableTreeNode,
        T: Tree<L=L, I=I>,
        P: Tree<L=PatternLeaf<L>, I=I>,
    >(
        tree: &T,
        identity: (&P, &P),
        position: usize,
        ) -> Result<Option<T>, ReplacementError> {

    let expression_string = tree_to_regex(tree).unwrap();
    let (pattern_string, wildcards) = pattern_to_regex(identity.0).unwrap();
    let replacement_string = replacement_to_regex(identity.1, &wildcards).unwrap();

    let re = Regex::with_options(
            &pattern_string,
    RegexOptions::REGEX_OPTION_DONT_CAPTURE_GROUP,
    Syntax::default(),
    ).unwrap();

    let mut captures_iter = re.captures_iter(&expression_string);

    match captures_iter.nth(position - 1) {
        Some(caps) => {
            let new_tree_string = _replace_capture(&caps, &expression_string, &replacement_string);
            let new_tree = tree_from_regex(&new_tree_string).unwrap();
            Ok(Some(new_tree))
        },
        None => Ok(None)
    }
}

pub fn try_replace_identity<
        L: ParsableTreeNode,
        I: ParsableTreeNode,
        T: Tree<L=L, I=I>,
        P: Tree<L=PatternLeaf<L>, I=I>,
    >(
        tree: &T,
        identity: (&P, &P),
    ) -> Result<Vec<(T, usize)>, &'static str> {

    let expression_string = tree_to_regex(tree).unwrap();
    let (pattern_string, wildcards) = pattern_to_regex(identity.0).unwrap();
    let replacement_string = replacement_to_regex(identity.1, &wildcards).unwrap();

    let re = Regex::new(
        &pattern_string,
    ).unwrap();

    let captures_iter: Vec<Captures> = re.captures_iter(&expression_string).collect();
    let mut new_tree_strings = vec![];
    
    for caps in captures_iter {
        new_tree_strings.push(
                (
                    _replace_capture(&caps, &expression_string, &replacement_string),
                    caps.offset(),
                )
        );
    };

    let new_trees: Vec<(T, usize)> = new_tree_strings.iter().map(|s| (tree_from_regex(&s.0).unwrap(), s.1)).collect();
    Ok(new_trees)
}


fn _replace_capture(
        caps: &Captures,
        expression_string: &str,
        replacement_string: &str,
    ) -> String {
    let (offset, end) = caps.pos(1).unwrap();
    let mut new_tree_string = String::new();
    new_tree_string.push_str(
            &expression_string[..offset]
    );
    let mut data = String::new();
    data.push_str("{\n");
    for (i, cap) in caps.iter().skip(2).enumerate(){
        data.push_str(&format!(r#""g{}": "{}""#, i, cap.unwrap()));
        if i < caps.len() - 3 {
            data.push(',');
        }
        data.push('\n');
    }
    data.push('}');
    let reg = Handlebars::new();
    let val: Value = serde_json::from_str(&data).unwrap();
    new_tree_string.push_str(
            &reg.render_template(replacement_string, &val).unwrap()
    );
    new_tree_string.push_str(
            &expression_string[end..]
    );
    new_tree_string
}


 #[cfg(test)]
mod tests {

    use crate::{Tree, Node, PatternLeaf};
    use crate::test_utils::{Equation, Element, Operator, Identity};

    use super::{replace_identity, try_replace_identity};

    #[test]
    fn test_replace_identity() {

        let tree = Equation::new(vec![
            Node::Internal(Operator::MULTIPLY),
            Node::Leaf(Element {
                label: b"a".to_vec(),
            }),
            Node::Internal(Operator::ADD),
            Node::Leaf(Element {
                label: b"b".to_vec(),
            }),
            Node::Leaf(Element {
                label: b"c".to_vec(),
            }),
        ]);

        let pattern = Identity::new(vec![
            Node::Internal(Operator::MULTIPLY),
            Node::Leaf(PatternLeaf::Subtree("x".to_string())),
            Node::Internal(Operator::ADD),
            Node::Leaf(PatternLeaf::Subtree("y".to_string())),
            Node::Leaf(PatternLeaf::Subtree("z".to_string())),
        ]);

        let replacement = Identity::new(vec![
            Node::Internal(Operator::ADD),
            Node::Internal(Operator::MULTIPLY),
            Node::Leaf(PatternLeaf::Subtree("x".to_string())),
            Node::Leaf(PatternLeaf::Subtree("y".to_string())),
            Node::Internal(Operator::MULTIPLY),
            Node::Leaf(PatternLeaf::Subtree("x".to_string())),
            Node::Leaf(PatternLeaf::Subtree("z".to_string())),
        ]);

        let after = replace_identity(&tree, (&pattern, &replacement), 1).unwrap().unwrap();
        assert_eq!(
            after,
            Equation::new(vec![
                Node::Internal(Operator::ADD),
                Node::Internal(Operator::MULTIPLY),
                Node::Leaf(Element {
                    label: b"a".to_vec(),
                }),
                Node::Leaf(Element {
                    label: b"b".to_vec(),
                }),
                Node::Internal(Operator::MULTIPLY),
                Node::Leaf(Element {
                    label: b"a".to_vec(),
                }),
                Node::Leaf(Element {
                    label: b"c".to_vec(),
                }),
            ]),
        );
    }

    #[test]
    fn test_try_replace_identity() {

        let tree = Equation::new(vec![
            Node::Internal(Operator::MULTIPLY),
            Node::Leaf(Element {
                label: b"a".to_vec(),
            }),
            Node::Internal(Operator::ADD),
            Node::Leaf(Element {
                label: b"b".to_vec(),
            }),
            Node::Leaf(Element {
                label: b"c".to_vec(),
            }),
        ]);

        let pattern = Identity::new(vec![
            Node::Internal(Operator::MULTIPLY),
            Node::Leaf(PatternLeaf::Subtree("x".to_string())),
            Node::Internal(Operator::ADD),
            Node::Leaf(PatternLeaf::Subtree("y".to_string())),
            Node::Leaf(PatternLeaf::Subtree("z".to_string())),
        ]);

        let replacement = Identity::new(vec![
            Node::Internal(Operator::ADD),
            Node::Internal(Operator::MULTIPLY),
            Node::Leaf(PatternLeaf::Subtree("x".to_string())),
            Node::Leaf(PatternLeaf::Subtree("y".to_string())),
            Node::Internal(Operator::MULTIPLY),
            Node::Leaf(PatternLeaf::Subtree("x".to_string())),
            Node::Leaf(PatternLeaf::Subtree("z".to_string())),
        ]);

        let after = try_replace_identity(&tree, (&pattern, &replacement)).unwrap();
        assert_eq!(
            after[0].0,
            Equation::new(vec![
                Node::Internal(Operator::ADD),
                Node::Internal(Operator::MULTIPLY),
                Node::Leaf(Element {
                    label: b"a".to_vec(),
                }),
                Node::Leaf(Element {
                    label: b"b".to_vec(),
                }),
                Node::Internal(Operator::MULTIPLY),
                Node::Leaf(Element {
                    label: b"a".to_vec(),
                }),
                Node::Leaf(Element {
                    label: b"c".to_vec(),
                }),
            ]),
        );
    }

    #[test]
    fn test_try_replace_identity_in_part_of_expression() {

        let tree = Equation::new(vec![
            Node::Internal(Operator::ADD),
            Node::Internal(Operator::ADD),
            Node::Internal(Operator::MULTIPLY),
            Node::Leaf(Element {
                label: b"a".to_vec(),
            }),
            Node::Leaf(Element {
                label: b"c".to_vec(),
            }),
            Node::Internal(Operator::MULTIPLY),
            Node::Leaf(Element {
                label: b"a".to_vec(),
            }),
            Node::Leaf(Element {
                label: b"d".to_vec(),
            }),
            Node::Internal(Operator::ADD),
            Node::Internal(Operator::MULTIPLY),
            Node::Leaf(Element {
                label: b"b".to_vec(),
            }),
            Node::Leaf(Element {
                label: b"c".to_vec(),
            }),
            Node::Internal(Operator::MULTIPLY),
            Node::Leaf(Element {
                label: b"b".to_vec(),
            }),
            Node::Leaf(Element {
                label: b"d".to_vec(),
            }),
        ]);

        let replacement = Identity::new(vec![
            Node::Internal(Operator::MULTIPLY),
            Node::Leaf(PatternLeaf::Subtree("x".to_string())),
            Node::Internal(Operator::ADD),
            Node::Leaf(PatternLeaf::Subtree("y".to_string())),
            Node::Leaf(PatternLeaf::Subtree("z".to_string())),
        ]);

        let pattern = Identity::new(vec![
            Node::Internal(Operator::ADD),
            Node::Internal(Operator::MULTIPLY),
            Node::Leaf(PatternLeaf::Subtree("x".to_string())),
            Node::Leaf(PatternLeaf::Subtree("y".to_string())),
            Node::Internal(Operator::MULTIPLY),
            Node::Leaf(PatternLeaf::Subtree("x".to_string())),
            Node::Leaf(PatternLeaf::Subtree("z".to_string())),
        ]);

        let after: Vec<Equation> = 
            try_replace_identity(&tree, (&pattern, &replacement))
                .unwrap()
                .into_iter()
                .map(|(a, _)|a)
                .collect();

        assert!(
            after.contains(
                &Equation::new(vec![
                    Node::Internal(Operator::ADD),
                    Node::Internal(Operator::MULTIPLY),
                    Node::Leaf(Element {
                        label: b"a".to_vec(),
                    }),
                    Node::Internal(Operator::ADD),
                    Node::Leaf(Element {
                        label: b"c".to_vec(),
                    }),
                    Node::Leaf(Element {
                        label: b"d".to_vec(),
                    }),
                    Node::Internal(Operator::ADD),
                    Node::Internal(Operator::MULTIPLY),
                    Node::Leaf(Element {
                        label: b"b".to_vec(),
                    }),
                    Node::Leaf(Element {
                        label: b"c".to_vec(),
                    }),
                    Node::Internal(Operator::MULTIPLY),
                    Node::Leaf(Element {
                        label: b"b".to_vec(),
                    }),
                    Node::Leaf(Element {
                        label: b"d".to_vec(),
                    }),
                ])
            )
        );

        assert!(
            after.contains(
                &Equation::new(vec![
                    Node::Internal(Operator::ADD),
                    Node::Internal(Operator::ADD),
                    Node::Internal(Operator::MULTIPLY),
                    Node::Leaf(Element {
                        label: b"a".to_vec(),
                    }),
                    Node::Leaf(Element {
                        label: b"c".to_vec(),
                    }),
                    Node::Internal(Operator::MULTIPLY),
                    Node::Leaf(Element {
                        label: b"a".to_vec(),
                    }),
                    Node::Leaf(Element {
                        label: b"d".to_vec(),
                    }),
                    Node::Internal(Operator::MULTIPLY),
                    Node::Leaf(Element {
                        label: b"b".to_vec(),
                    }),
                    Node::Internal(Operator::ADD),
                    Node::Leaf(Element {
                        label: b"c".to_vec(),
                    }),
                    Node::Leaf(Element {
                        label: b"d".to_vec(),
                    }),
                ])
            )
        );
    }

}
