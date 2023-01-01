extern crate solar_bt;
mod test_utils;

use solar_bt::{Node, Tree, PatternLeafElement, apply, LeafReplacement};

use test_utils::{
    BinaryOperator,
    Equation,
    Element,
    ElementSpec,
    IdentityPattern,
    IdentityReplacement,
    UnaryOperator,
};

#[test]
fn test_replace_commutativity_with_subtree() {

    let initial_equation = Equation::new(vec![
        Node::Binary(BinaryOperator::Add),
        Node::Leaf(Element { label: b"a".to_vec() }),
        Node::Leaf(Element { label: b"b".to_vec() }),
    ]);

    let commutativity_identity_left = IdentityPattern::new(vec![
        Node::Binary(BinaryOperator::Add),
        Node::Leaf(PatternLeafElement::Subtree(String::from("a"))),
        Node::Leaf(PatternLeafElement::Subtree(String::from("b"))),
    ]);

    let commutativity_identity_right = IdentityReplacement::new(vec![
        Node::Binary(BinaryOperator::Add),
        Node::Leaf(LeafReplacement::Captured(1)),
        Node::Leaf(LeafReplacement::Captured(0)),
    ]);

    let match_results = apply(&commutativity_identity_left, &initial_equation);
    assert_eq!(match_results.len(), 1);
    let match_result = match_results.first().unwrap();

    let final_equation = match_result.apply_replacement(commutativity_identity_right);

    assert_eq!(final_equation, Equation::new(vec![
        Node::Binary(BinaryOperator::Add),
        Node::Leaf(Element { label: b"b".to_vec() }),
        Node::Leaf(Element { label: b"a".to_vec() }),
    ]));

}