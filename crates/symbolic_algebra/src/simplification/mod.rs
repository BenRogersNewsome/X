// mod regina {
//     mod span;
//     mod strand;
//     mod step;
//     mod history;

//     pub use span::Span;
// }

mod simplify;

mod winston;

use crate::manipulation::{Manipulatable, Strand};

pub use simplify::simplify;

pub trait Simplifiable<'a> where Self: Manipulatable<'a> {

    fn simplicity(&self) -> usize;
    fn uuid(&self) -> u64;
}

impl<'a, T: 'a> Simplifiable<'a> for Strand<'a, T> where T: Simplifiable<'a>, T::Identity: Into<String> {

    fn simplicity(&self) -> usize {
        self.current.simplicity()
    }

    fn uuid(&self) -> u64 {
        self.current.uuid()
    }

}