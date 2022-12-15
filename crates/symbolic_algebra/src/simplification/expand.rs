use crate::algo::{structures::{Expression, Identity}};
use super::regina::Span;

pub fn expand(expression: &Expression, algebra: &Vec<Identity>) -> Result<Expression, &'static str> {

    let span = Span::init(expression, &algebra, false);

    let result = span.saturate().simplest();

    if result.len() == 0 {
        return Err("")
    }else{
        Ok(
            result[0].current.clone()
        )
    }
}

#[cfg(test)]
mod tests {

    use crate::algo::{
        structures::{Expression, algebras::associative_commutative_algebra},
        regex::Regexable,
        display::Display,
    };

    use super::expand;

    #[test]
    fn test_expand_1() {
        let simple: Expression = Expression::from_regex(r"*(+(a)(b))(+(c)(d))").unwrap();

        let expanded = expand(&simple, &associative_commutative_algebra()).unwrap();
        println!("{}", expanded.format())
    }
}