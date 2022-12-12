extern crate language;

use language::{run, ScopedItem};


macro_rules! expect_scoped_item_to_be {
    ($scope:ident, $name:literal, $type:ident) => {
        match $scope.get($name).unwrap() {
            ScopedItem::$type(x) => x,
            _ => panic!("Expected \"F\" to be a Set"),
        }
    };
}


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
    ");

    expect_scoped_item_to_be!(scope, b"Field", Structure);
    let set_f = expect_scoped_item_to_be!(scope, b"F", Set);
    expect_scoped_item_to_be!(scope, b"+", BinaryOperation);
    expect_scoped_item_to_be!(scope, b"*", BinaryOperation);
    expect_scoped_item_to_be!(scope, b"0", SetElement);
    expect_scoped_item_to_be!(scope, b"1", SetElement);
    
    expect_scoped_item_to_be!(scope, b"y", SetElement);

    
    match scope.get(b"x").unwrap() {
        ScopedItem::SetElement(s) => {
            assert!(set_f.contains(&s));
        },
        _ => panic!("Expected x to be a SetElement")
    }
}