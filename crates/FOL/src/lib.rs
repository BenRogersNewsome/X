//! An implementation of first-order logic.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

// mod formula;

mod primitives;
pub use primitives::{AssertionResponse, ElementValue, TruthValue};

/// Utils for manipulating FOL on a syntactic level.
///
/// Contains a grammar for FOL and utility functions for converting general
/// FOL statements into:
/// - Normal Form (PNF)
/// - Skolem Normal Form (SNF)
/// - Clause Normal Form (CNF)
#[cfg_attr(docsrs, doc(cfg(feature = "syntax")))]
#[cfg(feature = "syntax")]
pub mod syntax;

/// Framework for constructing semantic statements in first order logic, from
/// predicates and functions.
///
/// Internally, the semantics module uses an in-memory graph to keep track of
/// all predicates and functions which have been previously defined, and the
/// relationships between them. The graph can be extended, to include custom
/// functions and predicates, using the `Predicate` and `Function` traits.
#[cfg_attr(docsrs, doc(cfg(feature = "semantics")))]
#[cfg(feature = "semantics")]
pub mod semantics;
