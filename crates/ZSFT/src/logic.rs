mod l_bool;
mod num_val;
mod number;
mod traits;

pub use num_val::{NumBound, Number, NumRange, NumRangeBoundary};
pub use l_bool::LBool;

#[derive(Debug, PartialEq, Eq)]
pub enum AssertionResponse {
    AssertionMade,
    RedundantAssertion,
    AssertionInvalid,
}

impl AssertionResponse {
    pub fn expect(&self) -> () {
        match self {
            AssertionResponse::AssertionInvalid => panic!("Error unwrapping AssertionResponse::AssertionInvalid"),
            _ => {},
        }
    }
}

