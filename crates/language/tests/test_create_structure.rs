extern crate language;

use language::run;

mod common;
use common::expect_scoped_item_to_be;


#[test]
fn test_create_structure() {
    let scope = run(br"
        struct Field {
            F;
            + : F + F -> F,
            * : F * F -> F,
            0,
            1,
        } where \-/ a, b (- F:
            a + b = b + a
            a * b = b * a
            a * 1 = a
            a + 0 = a
        
        let (F; *, +, 0, 1) bea Field
        let x, y in F
        let z = x + y

        |- z (- F
        assert z in F
    ");

    expect_scoped_item_to_be!(scope, b"Field", Structure);
    let set_f = expect_scoped_item_to_be!(scope, b"F", Set);
    expect_scoped_item_to_be!(scope, b"+", BinaryOperation);
    expect_scoped_item_to_be!(scope, b"*", BinaryOperation);
    let elem_0 = expect_scoped_item_to_be!(scope, b"0", SetElement);
    expect_scoped_item_to_be!(scope, b"1", SetElement);

    assert!(set_f.contains(&elem_0));
    
    expect_scoped_item_to_be!(scope, b"y", SetElement);
    let elem_x = expect_scoped_item_to_be!(scope, b"x", SetElement);
    assert!(set_f.contains(&elem_x));

    let expr_z = expect_scoped_item_to_be!(scope, b"z", Expression);
    let elem_z = expr_z.to_set_element();

    assert!(set_f.contains(&elem_z));
}