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

}