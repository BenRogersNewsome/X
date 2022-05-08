use crate::algo::{
    structures::{
        Expression, Identity
    },
};
use super::regina::Span;


pub fn simplify<'a>(expression: &'a Expression, algebra: Vec<Identity>) -> Result<Expression, &'static str> {

    let span = Span::init(expression, &algebra, true);

    let result = span.saturate().simplest();

    if result.len() == 0 {
        return Err("")
    }else{
        Ok(
            result[0].current.clone()
        )
    }
}

// fn find_most_alike_original<'a>(strands: Vec<Strand<'a>>, original: &'a Expression) -> Vec<Strand<'a>> {
//     strands.into_iter().fold(vec![], |mut a, b| {
//         if a.len() == 0 {
//             return vec![b];
//         }
//         let sim_a = similarity(&a[0].current, original);
//         let sim_b = similarity(&b.current, original);
//         if sim_a > sim_b {
//             return a
//         }else if sim_a == sim_b {
//             a.push(b);
//             return a;
//         }else{
//             return vec![b];
//         }
//     })
// }

#[cfg(test)]
mod tests {

    use crate::algo::{
        structures::{
            Expression,
            algebras::associative_commutative_algebra,
        },
        regex::Regexable, display::Display
    };

    use super::simplify;

    #[test]
    fn test_simplify() {

        // (ab + ac)d
        let exp: Expression = Expression::from_regex(r"*(+(*(a)(b))(*(a)(c)))(d)").unwrap();

        let result = simplify(&exp, associative_commutative_algebra()).unwrap();

        //(a(b+c))d
        assert_eq!(result, Expression::from_regex(r"*(*(a)(+(b)(c)))(d)").unwrap());
    }

    #[test]
    fn test_simplify_2() {
        // (ac + ad) + (bc + bd)
        let tree: Expression = Expression::from_regex(r"+(+(*(a)(c))(*(a)(d)))(+(*(b)(c))(*(b)(d)))").unwrap();
        let result = simplify(
            &tree,
            associative_commutative_algebra(),
        ).unwrap();
        println!("{}", result.format());
        //(a+b)(c+d)
        assert_eq!(result, Expression::from_regex(r"*(+(a)(b))(+(c)(d))").unwrap());
    }

    #[test]
    fn test_simplify_3() {
        // (ac + ad) + (bc + bd)
        let tree: Expression = Expression::from_regex(r"+(+(+(*(a)(*(c)(e)))(*(a)(*(c)(f))))(+(*(a)(*(d)(e)))(*(a)(*(d)(f)))))(+(+(*(b)(*(c)(e)))(*(b)(*(c)(f))))(+(*(b)(*(d)(e)))(*(b)(*(d)(f)))))").unwrap();
        let result = simplify(
            &tree,
            associative_commutative_algebra(),
        ).unwrap();
        println!("{}", result.format());
        //(a+b)(c+d)
        assert_eq!(result, Expression::from_regex(r"*(+(a)(b))(+(c)(d))").unwrap());
    }
}