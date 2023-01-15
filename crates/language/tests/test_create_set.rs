extern crate language;

use language::run;

mod common;
use common::expect_scoped_item_to_be;

use zsft::LBool::*;


#[test]
fn test_create_set() {

    let scope = run(br"
        let S be {...}
        let x, y in S

        let a, b

        let T be {a}
        let P be {b, ...}

        def + : S + S -> S

        |- a (- T
    ");

    let set_s = expect_scoped_item_to_be!(scope, b"S", Set);
    let set_t = expect_scoped_item_to_be!(scope, b"T", Set);
    let set_p = expect_scoped_item_to_be!(scope, b"P", Set);
    let elem_x = expect_scoped_item_to_be!(scope, b"x", SetElement);
    let elem_y = expect_scoped_item_to_be!(scope, b"y", SetElement);
    let item_a = expect_scoped_item_to_be!(scope, b"a", Item);
    let item_b = expect_scoped_item_to_be!(scope, b"b", Item);

    assert_eq!(elem_x.in_set(set_s), True);
    assert_eq!(elem_y.in_set(set_s), True);

    assert_eq!(set_t.contains(*item_a), True);
    assert_eq!(set_t.contains(*item_b), False);

    assert_eq!(set_p.contains(*item_b), True);
    assert_eq!(set_p.contains(*item_a), Unknown);
}