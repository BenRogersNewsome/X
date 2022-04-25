use super::{
    identity::Identity,
};

use crate::identity;

type Algebra = Vec<Identity>;

pub fn associative_commutative_algebra<'a>() -> Algebra {

    let multiplication_distributes_over_addition: Identity = identity!(
        Operator {
            label: b'+'
        },
        Operator {
            label: b'*'
        },
        Subtree(Element {
            label: b"x".to_vec(),
        }),
        Subtree(Element {
            label: b"y".to_vec(),
        }),
        Operator {
            label: b'*'
        },
        Subtree(Element {
            label: b"x".to_vec(),
        }),
        Subtree(Element {
            label: b"z".to_vec(),
        }),
        ;
        
        Operator {
            label: b'*'
        },
        Subtree(Element {
            label: b"x".to_vec(),
        }),
        Operator {
            label: b'+'
        },
        Subtree(Element {
            label: b"y".to_vec(),
        }),
        Subtree(Element {
            label: b"z".to_vec(),
        }),
    );

    let commutative_multiplication: Identity = identity!(
        Operator {
            label: b'*'
        },
        Subtree(Element {
            label: b"x".to_vec(),
        }),
        Subtree(Element {
            label: b"y".to_vec(),
        }),
        ; 
        Operator {
            label: b'*'
        },
        Subtree(Element {
            label: b"y".to_vec(),
        }),
        Subtree(Element {
            label: b"x".to_vec(),
        }),
    );

    let associative_addition: Identity = identity!(

            Operator {
                label: b'+'
            },
            Operator {
                label: b'+'
            },
            Subtree(Element {
                label: b"a".to_vec(),
            }),
            Subtree(Element {
                label: b"b".to_vec(),
            }),
            Subtree(Element {
                label: b"c".to_vec(),
            }),
            ;

            Operator {
                label: b'+'
            },
            Subtree(Element {
                label: b"a".to_vec(),
            }),
            Operator {
                label: b'+'
            },
            Subtree(Element {
                label: b"b".to_vec(),
            }),
            Subtree(Element {
                label: b"c".to_vec(),
            }),
    );

    let associative_multiplication: Identity = identity!(

            Operator {
                label: b'*'
            },
            Operator {
                label: b'*'
            },
            Subtree(Element {
                label: b"a".to_vec(),
            }),
            Subtree(Element {
                label: b"b".to_vec(),
            }),
            Subtree(Element {
                label: b"c".to_vec(),
            }),
        ;
            Operator {
                label: b'*'
            },
            Subtree(Element {
                label: b"a".to_vec(),
            }),
            Operator {
                label: b'*'
            },
            Subtree(Element {
                label: b"b".to_vec(),
            }),
            Subtree(Element {
                label: b"c".to_vec(),
            }),
    );

    vec![
            associative_addition,
            associative_multiplication,
            commutative_multiplication,
            multiplication_distributes_over_addition,
        ]
}