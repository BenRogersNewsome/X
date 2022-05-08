mod regina {
    mod span;
    mod strand;
    mod step;
    mod history;

    pub use span::Span;
}

mod expand;
mod simplify;

pub use simplify::simplify;
pub use expand::expand;