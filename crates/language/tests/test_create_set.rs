extern crate language;

use language::run;

#[test]
fn test_create_set() {

    let register = run(b"
        let (A) be Set
        let a in A
    ");

    let maybe_set_a = register.get(b"A").unwrap();
    let elem_a = register.get(b"a").unwrap();

    let set_a = match maybe_set_a {
        Registerable::Set(s) => s,
        _ => panic!("Expected A to be a Set")
    };

    match elem_a {
        Registerable::SetElement(e) => assert!(set_a.contains(e)),
        _ => panic!("Expected a to be a set element")
    }
}