extern crate symbolic_algebra;
use solar_bt::{Tree, Node, LeafPattern, LeafReplacement};
use symbolic_algebra::{Expression, ExpressionPattern, ExpressionReplacement, Identity, OperatorPattern, simplify};
use zsft::{BinaryOperation, SetElement, Set};

#[test]
fn test_simplify_one_step() {

    let set_a = Set::anonymous();

    let addition = BinaryOperation::from_signature(&set_a, &set_a, &set_a);
    let multiplication = BinaryOperation::from_signature(&set_a, &set_a, &set_a);

    // axb+axc = ax(b+c)
    let distributive_identity = Identity {
        left: ExpressionPattern::new(vec![
            Node::Binary(OperatorPattern::Literal(addition.clone())),
            Node::Binary(OperatorPattern::Literal(multiplication.clone())),
            Node::Leaf(LeafPattern::Subtree),
            Node::Leaf(LeafPattern::Subtree),
            Node::Binary(OperatorPattern::Literal(multiplication.clone())),
            Node::Leaf(LeafPattern::SubtreeCallback(0)),
            Node::Leaf(LeafPattern::Subtree),
        ]),
        right: ExpressionReplacement::new(vec![
            Node::Binary(multiplication.clone()),
            Node::Leaf(LeafReplacement::Captured(0)),
            Node::Binary(addition.clone()),
            Node::Leaf(LeafReplacement::Captured(1)),
            Node::Leaf(LeafReplacement::Captured(2)),
        ]),
    };

    let elem_a = SetElement::element_of(&set_a);
    let elem_b = SetElement::element_of(&set_a);
    let elem_c = SetElement::element_of(&set_a);

    // axb + axc
    let unsimplified = Expression::new(vec![
        Node::Binary(addition.clone()),
        Node::Binary(multiplication.clone()),
        Node::Leaf(elem_a.clone()),
        Node::Leaf(elem_b.clone()),
        Node::Binary(multiplication.clone()),
        Node::Leaf(elem_a.clone()),
        Node::Leaf(elem_c.clone()),
    ]);

    let simplified = simplify(unsimplified, &vec![distributive_identity]).expect("Unexpected error simplifying expression");

    // ax(b+c)
    let expected_simplified = Expression::new(vec![
        Node::Binary(multiplication.clone()),
        Node::Leaf(elem_a.clone()),
        Node::Binary(addition.clone()),
        Node::Leaf(elem_b.clone()),
        Node::Leaf(elem_c.clone()),
    ]);

    assert_eq!(simplified, expected_simplified);
}