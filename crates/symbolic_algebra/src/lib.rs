mod manipulation;
mod simplification;
mod structures;

pub use structures::{
    Algebra,
    Expression,
    ExpressionPattern,
    ExpressionReplacement,
    Identity,
    OperatorPattern,
};

pub use simplification::{simplify};