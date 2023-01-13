extern crate language;

use language::run;

mod common;
use common::expect_scoped_item_to_be;

use zsft::LBool::*;


#[test]
fn test_create_set() {

    let scope = run(br"
        let S be {...}
        let a, b in S

        let T be {a}
        let P be {b, ...}

        def + : S + S -> S

        let c = a + b

        |- c (- S
        |- b (- T
    ");

    let set_s = expect_scoped_item_to_be!(scope, b"S", Set);
    let set_t = expect_scoped_item_to_be!(scope, b"T", Set);
    let set_p = expect_scoped_item_to_be!(scope, b"P", Set);
    let elem_a = expect_scoped_item_to_be!(scope, b"a", SetElement);
    let elem_b = expect_scoped_item_to_be!(scope, b"b", SetElement);
    expect_scoped_item_to_be!(scope, b"+", BinaryOperation);
    expect_scoped_item_to_be!(scope, b"c", Expression);

    assert_eq!(set_s.contains(elem_a), True);
    assert_eq!(set_s.contains(elem_b), True);

    assert_eq!(set_t.contains(elem_a), True);
    assert_eq!(set_t.contains(elem_b), False);

    assert_eq!(set_p.contains(elem_b), True);
    assert_eq!(set_p.contains(elem_a), Unknown);
}