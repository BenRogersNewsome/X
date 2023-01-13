extern crate symbolic_algebra;
use solar_bt::{Tree, Node, LeafPattern, LeafReplacement};
use symbolic_algebra::{Expression, ExpressionPattern, ExpressionReplacement, Identity, OperatorPattern, simplify, Simplifiable};
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

    let simplified = simplify(unsimplified, &vec![distributive_identity], 1).expect("Unexpected error simplifying expression");

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

#[test]
fn test_simplify_double_bracket() {

    // Expected steps:
    // axc + axd + bxc + bxd =
    // ax(c+d) + bxc + bxd =
    // ax(c+d) + bx(c+d) = 
    // (c+d)xa + (c+d)xb =
    // (c+d)(a+b)

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

    // (a+b)+c = a+(b+c)
    let associative_identity = Identity {
        left: ExpressionPattern::new(vec![
            Node::Binary(OperatorPattern::Literal(addition.clone())),
            Node::Binary(OperatorPattern::Literal(addition.clone())),
            Node::Leaf(LeafPattern::Subtree),
            Node::Leaf(LeafPattern::Subtree),
            Node::Leaf(LeafPattern::Subtree),
        ]),
        right: ExpressionReplacement::new(vec![
            Node::Binary(addition.clone()),
            Node::Leaf(LeafReplacement::Captured(0)),
            Node::Binary(addition.clone()),
            Node::Leaf(LeafReplacement::Captured(1)),
            Node::Leaf(LeafReplacement::Captured(2)),
        ]),
    };

    // ab = ba
    let commutative_identity = Identity {
        left: ExpressionPattern::new(vec![
            Node::Binary(OperatorPattern::Literal(multiplication.clone())),
            Node::Leaf(LeafPattern::Subtree),
            Node::Leaf(LeafPattern::Subtree),
        ]),
        right: ExpressionReplacement::new(vec![
            Node::Binary(multiplication.clone()),
            Node::Leaf(LeafReplacement::Captured(1)),
            Node::Leaf(LeafReplacement::Captured(0)),
        ]),
    };

    let elem_a = SetElement::element_of(&set_a);
    let elem_b = SetElement::element_of(&set_a);
    let elem_c = SetElement::element_of(&set_a);
    let elem_d = SetElement::element_of(&set_a);

    // ac + ad + bc + bd    (S15)
    let unsimplified = Expression::new(vec![
        Node::Binary(addition.clone()),
        Node::Binary(addition.clone()),
        Node::Binary(multiplication.clone()),
        Node::Leaf(elem_a.clone()),
        Node::Leaf(elem_c.clone()),
        Node::Binary(multiplication.clone()),
        Node::Leaf(elem_a.clone()),
        Node::Leaf(elem_d.clone()),
        Node::Binary(addition.clone()),
        Node::Binary(multiplication.clone()),
        Node::Leaf(elem_b.clone()),
        Node::Leaf(elem_c.clone()),
        Node::Binary(multiplication.clone()),
        Node::Leaf(elem_b.clone()),
        Node::Leaf(elem_d.clone()),
    ]);

    let simplified = simplify(
        unsimplified,
        &vec![
            distributive_identity,
            // This isn't actually needed for this simplification, but it will confuse the algorithm
            // so leaving it in for the test.
            associative_identity,
            commutative_identity,
        ],
        1,
    ).expect("Unexpected error simplifying expression");

    // Expect (a+b)x(c+d)   (S7)
    assert_eq!(simplified.simplicity(), 7);
}
