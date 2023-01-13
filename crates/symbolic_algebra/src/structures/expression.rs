use std::hash::{Hasher, Hash};
use std::collections::hash_map::DefaultHasher;

use solar_bt::{apply, MatcherResult, Node};
use zsft::{SetElement, BinaryOperation, UnaryOperation};
use crate::manipulation::Manipulatable;
use crate::simplification::Simplifiable;

use super::identity::Identity;

use super::token_tree::VecTree;


pub type Expression<'a> = VecTree<&'a SetElement<'a>, &'a BinaryOperation, &'a UnaryOperation>;

pub enum ExpressionManipulationError {
    Error,
}

impl<'a> Manipulatable<'a> for Expression<'a> {
    type Identity = Identity<'a>;
    type Instruction = (usize, &'a Identity<'a>);
    type Error = ExpressionManipulationError;

    fn manipulate(&self, _instruction: &'a Self::Instruction) -> Result<Option<Self>, Self::Error> {
        todo!();
        // let results = apply(

        // )

        // let maybe_result = replace_identity(
        //     self,
        //     instruction.1.into(),
        //     instruction.0,
        // )?;
        
        // Ok(maybe_result)
    }

    fn try_manipulate(&self, identity: &'a Self::Identity) -> Result<Vec<(Self, Self::Instruction)>, &'static str> {

        let match_results: Vec<MatcherResult<Self>> = apply(
            &identity.left,
            self,
        );

        let replacement_results: Vec<(Self, Self::Instruction)> = match_results.into_iter()
            .map(|match_result| {
                (
                    match_result.apply_replacement(&identity.right),
                    (match_result.position(), identity),
                )
            }).collect();

        Ok(replacement_results)
    }
}

impl<'a> Simplifiable<'a> for Expression<'a> {
    fn simplicity(&self) -> usize {
        self.nodes.len()
    }

    fn uuid(&self) -> u64 {
        self.nodes.iter()
            .map(|token| {
                match token {
                    Node::Unary(x) => x.id(),
                    Node::Binary(x) => x.id(),
                    Node::Leaf(x) => x.id(),
                }
            }).sum()
    }
}

/// Create expressions, similar to how vec! works, only `Leaf()`, `Binary()` and `Unary()` may be omitted from the vec members, as they will be added automatically.
/// When code is compiled with the -O flag, this macro leads to no runtime performance impact.
#[macro_export]
macro_rules! expression {
 ($($x:expr),+ $(,)?) => {{
     use $crate::structures::{Expression, Element, BinaryOperator};
     use solar_bt::TreeNode;

     pub trait Wrapped {
         fn get_wrapped(self) -> TreeNode<Expression>;
     }

     impl Wrapped for Element {
         #[inline(always)]
         fn get_wrapped(self) -> TreeNode<Expression> {
             return Node::Leaf(self)
         }
     }

     impl Wrapped for BinaryOperator {
         #[inline(always)]
         fn get_wrapped(self) -> TreeNode<Expression> {
             return Node::Internal(self)
         }
     }
     Expression::new(vec![$(($x).get_wrapped()),+])
 }}
}


// #[cfg(test)]
// mod test_macros {

//     use crate::structures::{Element, BinaryOperator};
//     use solar_bt::Node;

//     #[test]
//     fn test_create_expression() {
//         let plus = BinaryOperator::new(b'+');
//         let a = Element::new(b"a");
//         let b = Element::new(b"b");

//         let expression = expression![
//             plus,
//             a.clone(),
//             b.clone(),
//         ];

//         assert_eq!(*expression, vec![
//             Node::Binary(plus),
//             Node::Leaf(a),
//             Node::Leaf(b),
//         ]);
//     }
// }

// #[cfg(test)]
// mod test_manipulation {

//     use crate::structures::{Element, BinaryOperator, Identity, Expression};
//     use crate::manipulation::Manipulatable;

//     use crate::structures::identity::identity_expression;

//     #[test]
//     fn test_apply_manipulation_to_identity() {
//         let plus: BinaryOperator = BinaryOperator::new(b'+');
//         let times: BinaryOperator = BinaryOperator::new(b'*');
//         let a: Element = Element::new(b"a");
//         let b: Element = Element::new(b"b");
//         let c: Element = Element::new(b"c");

//         let unsimplified = expression![
//             plus,
//             times,
//             a.clone(),
//             b.clone(),
//             times,
//             a.clone(),
//             c.clone(),
//         ];

//         let simplified = expression![
//             times,
//             a.clone(),
//             plus,
//             b.clone(),
//             c.clone(),
//         ];

//         let identity = Identity (
//             identity_expression![
//                 plus,
//                 times,
//                 NodePattern::Subtree("x".to_owned()),
//                 NodePattern::Subtree("y".to_owned()),
//                 times,
//                 NodePattern::Subtree("x".to_owned()),
//                 NodePattern::Subtree("z".to_owned()),
//             ],
//             identity_expression![
//                 times,
//                 NodePattern::Subtree("x".to_owned()),
//                 plus,
//                 NodePattern::Subtree("y".to_owned()),
//                 NodePattern::Subtree("z".to_owned()),
//             ],
//         );

//         let try_simplified = unsimplified.try_manipulate(&identity).unwrap();

//         assert_eq!(try_simplified.len(), 1);
//         assert_eq!(try_simplified[0].0, simplified);
//     }

//     #[test]
//     fn test_apply_manipulation_in_middle_of_expression() {
//         let plus: BinaryOperator = BinaryOperator::new(b'+');
//         let times: BinaryOperator = BinaryOperator::new(b'*');
//         let a: Element = Element::new(b"a");
//         let b: Element = Element::new(b"b");
//         let c: Element = Element::new(b"c");
//         let d: Element = Element::new(b"d");
//         let e: Element = Element::new(b"e");
//         let f: Element = Element::new(b"f");

//         // *+ef+*a+cd*b+cd
//         let unsimplified = expression![
//             times,
//             plus,
//             e.clone(),
//             f.clone(),
//             plus,
//             times,
//             a.clone(),
//             plus,
//             c.clone(),
//             d.clone(),
//             times,
//             b.clone(),
//             plus,
//             c.clone(),
//             d.clone(),
//         ];

//         // *+ef+*+cda*b+cd
//         let simplified = expression![
//             times,
//             plus,
//             e.clone(),
//             f.clone(),
//             plus,
//             times,
//             plus,
//             c.clone(),
//             d.clone(),
//             a.clone(),
//             times,
//             b.clone(),
//             plus,
//             c.clone(),
//             d.clone(),
//         ];

//         let identity = Identity (
//             identity_expression![
//                 times,
//                 NodePattern::Subtree("x".to_owned()),
//                 NodePattern::Subtree("y".to_owned()),
//             ],
//             identity_expression![
//                 times,
//                 NodePattern::Subtree("y".to_owned()),
//                 NodePattern::Subtree("x".to_owned()),
//             ],
//         );

//         let try_simplified: Vec<Expression> = 
//             unsimplified
//             .try_manipulate(&identity)
//             .unwrap()
//             .into_iter()
//             .map(|(a, _)|a)
//             .collect();

//         assert!(try_simplified.contains(&simplified));
//     }
// }


// #[cfg(test)]
// mod test_simplification {
    
//     use crate::structures::{Element, BinaryOperator, Identity};
//     use crate::simplification::simplify;

//     use crate::structures::identity::identity_expression;

//     fn init() {
//         let _ = env_logger::builder().is_test(true).try_init();
//     }

    
//     #[test]
//     fn test_factorise_single_bracket() {
//         let plus: BinaryOperator = BinaryOperator::new(b'+');
//         let times: BinaryOperator = BinaryOperator::new(b'*');
//         let a: Element = Element::new(b"a");
//         let b: Element = Element::new(b"b");
//         let c: Element = Element::new(b"c");

//         let multiplication_over_addition = Identity (
//             identity_expression![
//                 plus,
//                 times,
//                 NodePattern::Subtree("x".to_owned()),
//                 NodePattern::Subtree("y".to_owned()),
//                 times,
//                 NodePattern::Subtree("x".to_owned()),
//                 NodePattern::Subtree("z".to_owned()),
//             ],
//             identity_expression![
//                 times,
//                 NodePattern::Subtree("x".to_owned()),
//                 plus,
//                 NodePattern::Subtree("y".to_owned()),
//                 NodePattern::Subtree("z".to_owned()),
//             ],
//         );

//         let expression = expression![
//             plus,
//             times,
//             a.clone(),
//             b.clone(),
//             times,
//             a.clone(),
//             c.clone(),
//         ];
        

//         let expected_simplified = expression![
//             times,
//             a.clone(),
//             plus,
//             b.clone(),
//             c.clone(),
//         ];
    
//         let simplified = simplify(expression, &[multiplication_over_addition]).unwrap();

//         assert_eq!(simplified, expected_simplified);
//     }

//     #[test]
//     fn test_factorise_double_bracket() {
//         let plus: BinaryOperator = BinaryOperator::new(b'+');
//         let times: BinaryOperator = BinaryOperator::new(b'*');
//         let a: Element = Element::new(b"a");
//         let b: Element = Element::new(b"b");
//         let c: Element = Element::new(b"c");
//         let d: Element = Element::new(b"d");

//         let multiplication_over_addition = Identity (
//             identity_expression![
//                 plus,
//                 times,
//                 NodePattern::Subtree("x".to_owned()),
//                 NodePattern::Subtree("y".to_owned()),
//                 times,
//                 NodePattern::Subtree("x".to_owned()),
//                 NodePattern::Subtree("z".to_owned()),
//             ],
//             identity_expression![
//                 times,
//                 NodePattern::Subtree("x".to_owned()),
//                 plus,
//                 NodePattern::Subtree("y".to_owned()),
//                 NodePattern::Subtree("z".to_owned()),
//             ],
//         );

//         let commutativity_multiplication = Identity (
//             identity_expression![
//                 times,
//                 NodePattern::Subtree("x".to_owned()),
//                 NodePattern::Subtree("y".to_owned()),
//             ],
//             identity_expression![
//                 times,
//                 NodePattern::Subtree("y".to_owned()),
//                 NodePattern::Subtree("x".to_owned()),
//             ],
//         );

//         let expression = expression![
//             plus,
//             plus,
//             times,
//             a.clone(),
//             c.clone(),
//             times,
//             a.clone(),
//             d.clone(),
//             plus,
//             times,
//             b.clone(),
//             c.clone(),
//             times,
//             b.clone(),
//             d.clone(),
//         ];
        

//         let expected_simplified = expression![
//             times,
//             plus,
//             a.clone(),
//             b.clone(),
//             plus,
//             c.clone(),
//             d.clone(),
//         ];
    
//         let simplified = 
//             simplify(
//                 expression,
//                 &[multiplication_over_addition, commutativity_multiplication]
//             ).unwrap();

//         assert_eq!(simplified, expected_simplified);
//     }

//     #[test]
//     fn test_factorise_triple_bracket() {
//         init();
//         let plus: BinaryOperator = BinaryOperator::new(b'+');
//         let times: BinaryOperator = BinaryOperator::new(b'*');
//         let a: Element = Element::new(b"a");
//         let b: Element = Element::new(b"b");
//         let c: Element = Element::new(b"c");
//         let d: Element = Element::new(b"d");
//         let e: Element = Element::new(b"e");
//         let f: Element = Element::new(b"f");

//         let multiplication_over_addition = Identity (
//             identity_expression![
//                 plus,
//                 times,
//                 NodePattern::Subtree("x".to_owned()),
//                 NodePattern::Subtree("y".to_owned()),
//                 times,
//                 NodePattern::Subtree("x".to_owned()),
//                 NodePattern::Subtree("z".to_owned()),
//             ],
//             identity_expression![
//                 times,
//                 NodePattern::Subtree("x".to_owned()),
//                 plus,
//                 NodePattern::Subtree("y".to_owned()),
//                 NodePattern::Subtree("z".to_owned()),
//             ],
//         );

//         let multiplication_over_addition_2 = Identity (
//             identity_expression![
//                 plus,
//                 times,
//                 NodePattern::Subtree("y".to_owned()),
//                 NodePattern::Subtree("x".to_owned()),
//                 times,
//                 NodePattern::Subtree("z".to_owned()),
//                 NodePattern::Subtree("x".to_owned()),
//             ],
//             identity_expression![
//                 times,
//                 plus,
//                 NodePattern::Subtree("y".to_owned()),
//                 NodePattern::Subtree("z".to_owned()),
//                 NodePattern::Subtree("x".to_owned()),
//             ],
//         );

//         let expression = expression![
//             plus,
//             plus,
//             plus,
//             times,
//             times,
//             a.clone(),
//             c.clone(),
//             e.clone(),
//             times,
//             times,
//             a.clone(),
//             c.clone(),
//             f.clone(),
//             plus,
//             times,
//             times,
//             a.clone(),
//             d.clone(),
//             e.clone(),
//             times,
//             times,
//             a.clone(),
//             d.clone(),
//             f.clone(),
//             plus,
//             plus,
//             times,
//             times,
//             b.clone(),
//             c.clone(),
//             e.clone(),
//             times,
//             times,
//             b.clone(),
//             c.clone(),
//             f.clone(),
//             plus,
//             times,
//             times,
//             b.clone(),
//             d.clone(),
//             e.clone(),
//             times,
//             times,
//             b.clone(),
//             d.clone(),
//             f.clone(),
//         ];
        

//         let expected_simplified = expression![
//             times,
//             times,
//             plus,
//             a.clone(),
//             b.clone(),
//             plus,
//             c.clone(),
//             d.clone(),
//             plus,
//             e.clone(),
//             f.clone(),
//         ];
        
        
//         let simplified = 
//         simplify(
//             expression,
//             &[multiplication_over_addition, multiplication_over_addition_2]
//         ).unwrap();

//         assert_eq!(simplified, expected_simplified);
//     }

//     #[test]
//     fn test_simplify_with_expansion_first(){
//         let plus: BinaryOperator = BinaryOperator::new(b'+');
//         let times: BinaryOperator = BinaryOperator::new(b'*');
//         let a: Element = Element::new(b"a");
//         let b: Element = Element::new(b"b");
//         let c: Element = Element::new(b"c");
//         let d: Element = Element::new(b"d");
//         let e: Element = Element::new(b"e");

//         // xy + xz = x(y+z)
//         let multiplication_over_addition = Identity (
//             identity_expression![
//                 plus,
//                 times,
//                 NodePattern::Subtree("x".to_owned()),
//                 NodePattern::Subtree("y".to_owned()),
//                 times,
//                 NodePattern::Subtree("x".to_owned()),
//                 NodePattern::Subtree("z".to_owned()),
//             ],
//             identity_expression![
//                 times,
//                 NodePattern::Subtree("x".to_owned()),
//                 plus,
//                 NodePattern::Subtree("y".to_owned()),
//                 NodePattern::Subtree("z".to_owned()),
//             ],
//         );

//         // ab = d
//         let ab_to_d = Identity(
//             identity_expression![
//                 times,
//                 NodePattern::Literal(a.clone()),
//                 NodePattern::Literal(b.clone()),
//             ],
//             identity_expression![
//                 NodePattern::Literal(d.clone()),
//             ]
//         );

//         // ac = e
//         let ac_to_e = Identity(
//             identity_expression![
//                 times,
//                 NodePattern::Literal(a.clone()),
//                 NodePattern::Literal(c.clone()),
//             ],
//             identity_expression![
//                 NodePattern::Literal(e.clone()),
//             ]
//         );

//         // a(b+c)
//         let expression = expression![
//             times,
//             a.clone(),
//             plus,
//             b.clone(),
//             c.clone(),
//         ];

//         // d + e
//         let expected_simplified = expression![
//             plus,
//             d.clone(),
//             e.clone(),
//         ];

//         let identities = [multiplication_over_addition.clone(), ab_to_d.clone(), multiplication_over_addition.invert(), ac_to_e];

//         let simplified = 
//         simplify(
//             expression,
//             &identities,
//         ).unwrap();

//         assert_eq!(simplified, expected_simplified);
//     }
// }