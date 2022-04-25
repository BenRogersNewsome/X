use crate::algo::{
    display::similarity,
    structures::{
        Expression, Identity, Tree
    },
};

use super::strand::Strand;
use super::history::History;


pub fn simplify<'a>(expression: &'a Expression, algebra: Vec<Identity>) -> Result<Expression, &'static str> {
    let strands = vec![
        Strand::init(expression)
    ];

    let simplified = find_all(strands, &algebra);

    let simplest = find_simplest(simplified);

    let most_alike = find_most_alike_original(simplest, expression);

    match most_alike.len() {
        0 => { return Err("") },
        _ => { return Ok(most_alike[0].current.clone()) },
    }
}

fn find_all<'a>(strands: Vec<Strand<'a>>, algebra: &'a Vec<Identity>) -> Vec<Strand<'a>> {
    let mut history = History::new();
    let mut current_strands = strands;
    loop {
        let (new_strands, done) = iterate(current_strands, &mut history, &algebra);
        current_strands = new_strands;
        if done {
            break;
        }
    };
    current_strands
}

fn iterate<'a>(strands: Vec<Strand<'a>>, history: &mut History, algebra: &'a Vec<Identity>) -> (Vec<Strand<'a>>, bool) {
    let mut new_strands = vec![];
    let mut done = true;
    for strand in strands {
        let propagated = strand.propagate(algebra, history);
        match propagated {
            Ok(mut returned_strands) => {
                done = false;
                new_strands.append(&mut returned_strands)
            },
            Err(existing_strand) => {
                new_strands.push(existing_strand)
            }
        }
    };
    (new_strands, done)
}

fn find_simplest<'a>(strands: Vec<Strand<'a>>) -> Vec<Strand<'a>> {
    strands.into_iter().fold(vec![], |mut accum: Vec<Strand>, strand| {
        if accum.len() == 0 {
            return vec![strand]
        }
        if &strand.current.simplicity() < &accum[0].current.simplicity(){
            return vec![strand]
        }else if &strand.current.simplicity() == &accum[0].current.simplicity() {
            accum.push(strand);
            return accum;
        }else{
            return accum;
        };
    })
}

fn find_most_alike_original<'a>(strands: Vec<Strand<'a>>, original: &'a Expression) -> Vec<Strand<'a>> {
    strands.into_iter().fold(vec![], |mut a, b| {
        if a.len() == 0 {
            return vec![b];
        }
        let sim_a = similarity(&a[0].current, original);
        let sim_b = similarity(&b.current, original);
        if sim_a > sim_b {
            return a
        }else if sim_a == sim_b {
            a.push(b);
            return a;
        }else{
            return vec![b];
        }
    })
}

#[cfg(test)]
mod tests {

    use crate::algo::{
        structures::{
            Expression,
            algebras::associative_commutative_algebra,
        },
        regex::Regexable,
    };

    use super::simplify;

    #[test]
    fn test_simplify() {
        // Testing (ab+ac)d == (a(b+c))d
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
        //(a+b)(c+d)
        assert_eq!(result, Expression::from_regex(r"*(+(a)(b))(+(c)(d))").unwrap());
    }
}