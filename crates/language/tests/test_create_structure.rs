extern crate language;

use language::{run, ScopedItem};

mod common;
use common::expect_scoped_item_to_be;


#[test]
fn test_create_structure() {
    let scope = run(b"
        struct Field {
            F;
            + : F + F -> F,
            * : F * F -> F,
            0,
            1,
        } where forall a, b in F:
            a + b = b + a
            a * b = b * a
            a * 1 = a
            a + 0 = a
    
        let (F; *, +, 0, 1) bea Field

        let x, y in F

        let z = x + y
    ");

    expect_scoped_item_to_be!(scope, b"Field", Structure);
    let set_f = expect_scoped_item_to_be!(scope, b"F", Set);
    expect_scoped_item_to_be!(scope, b"+", BinaryOperation);
    expect_scoped_item_to_be!(scope, b"*", BinaryOperation);
    expect_scoped_item_to_be!(scope, b"0", SetElement);
    expect_scoped_item_to_be!(scope, b"1", SetElement);
    
    expect_scoped_item_to_be!(scope, b"y", SetElement);
    let elem_x = expect_scoped_item_to_be!(scope, b"x", SetElement);
    assert!(set_f.contains(&elem_x));

    let _ = expect_scoped_item_to_be!(scope, b"z", Expression);
}