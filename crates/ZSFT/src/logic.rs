mod l_bool;
mod num_val;
mod number;
mod traits;

pub use num_val::{NumBound, Number};
pub use l_bool::LBool;

#[derive(Debug, PartialEq, Eq)]
pub enum AssertionResponse {
    AssertionMade,
    RedundantAssertion,
    AssertionInvalid,
}

