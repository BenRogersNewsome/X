use crate::algo::structures::Expression;

trait Equivalence {
    fn equivalent(&self, other: &Self) -> bool;
}

#[macro_export]
macro_rules! equivalent {
    ($left:expr; $right:expr) => {{
        $left.equivalent($right)
    }}
}

pub fn is_equivalent(left: &Expression, right: &Expression) -> Result<bool, &'static str> {

    

    return Err("");
}