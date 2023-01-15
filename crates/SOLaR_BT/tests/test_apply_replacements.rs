extern crate solar_bt;
mod test_utils;

use solar_bt::{Node, Tree, LeafPattern, apply, LeafReplacement};

use test_utils::{
    BinaryOperator,
    Equation,
    Element,
    IdentityPattern,
    IdentityReplacement,
};

#[test]
fn test_replace_commutativity_with_subtree() {

    let initial_equation = Equation::new(vec![
        Node::Binary(BinaryOperator::Add),
        Node::Binary(BinaryOperator::Add),
        Node::Leaf(Element { label: b"a".to_vec() }),
        Node::Leaf(Element { label: b"b".to_vec() }),
        Node::Binary(BinaryOperator::Add),
        Node::Leaf(Element { label: b"c".to_vec() }),
        Node::Leaf(Element { label: b"d".to_vec() }),
    ]);

    let commutativity_identity_left = IdentityPattern::new(vec![
        Node::Binary(BinaryOperator::Add),
        Node::Leaf(LeafPattern::Subtree),
        Node::Leaf(LeafPattern::Subtree),
    ]);

    let commutativity_identity_right = IdentityReplacement::new(vec![
        Node::Binary(BinaryOperator::Add),
        Node::Leaf(LeafReplacement::Captured(1)),
        Node::Leaf(LeafReplacement::Captured(0)),
    ]);

    let match_results = apply(&commutativity_identity_left, &initial_equation);
    assert_eq!(match_results.len(), 3);
    
    let new_equations: Vec<_> = 
        match_results.into_iter()
        .map(|mr|mr.apply_replacement(&commutativity_identity_right))
        .collect();
    
    assert!(new_equations.contains(&Equation::new(vec![
        Node::Binary(BinaryOperator::Add),
        Node::Binary(BinaryOperator::Add),
        Node::Leaf(Element { label: b"b".to_vec() }),
        Node::Leaf(Element { label: b"a".to_vec() }),
        Node::Binary(BinaryOperator::Add),
        Node::Leaf(Element { label: b"c".to_vec() }),
        Node::Leaf(Element { label: b"d".to_vec() }),
    ])));

    assert!(new_equations.contains(&Equation::new(vec![
        Node::Binary(BinaryOperator::Add),
        Node::Binary(BinaryOperator::Add),
        Node::Leaf(Element { label: b"c".to_vec() }),
        Node::Leaf(Element { label: b"d".to_vec() }),
        Node::Binary(BinaryOperator::Add),
        Node::Leaf(Element { label: b"a".to_vec() }),
        Node::Leaf(Element { label: b"b".to_vec() }),
    ])));

    assert!(new_equations.contains(&Equation::new(vec![
        Node::Binary(BinaryOperator::Add),
        Node::Binary(BinaryOperator::Add),
        Node::Leaf(Element { label: b"a".to_vec() }),
        Node::Leaf(Element { label: b"b".to_vec() }),
        Node::Binary(BinaryOperator::Add),
        Node::Leaf(Element { label: b"d".to_vec() }),
        Node::Leaf(Element { label: b"c".to_vec() }),
    ])));

}