extern crate language;

use language::run;

mod common;
use common::expect_scoped_item_to_be;

use zsft::LBool::*;

///
/// 
/// 
#[test]
fn test_simplify_expression() {

    let scope = run(br"
        let S be {...}

        def * : S * S -> S
        def + : S + S -> S

        |- \-/ a, b, c (- S {
            a*(b+c) = a*b + a*c
        }

        let a, b, c (- S
        
        let d = a*b + a*c
    ");

    
}