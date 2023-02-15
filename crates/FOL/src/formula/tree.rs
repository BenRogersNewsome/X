use crate::{Element, PredicateNode, FunctionNode};

pub struct FreeVariable {
    _id: u64
}

impl FreeVariable {
    pub fn new() -> Self {
        Self { _id: rand::random() }
    }
}

pub enum FormulaTerm<E> {
    Function(FunctionNode<E>, Vec<FormulaTerm<E>>),
    Constant(E),
    Free(FreeVariable),
}

pub enum AtomicFormula<E> {
    Equality(FormulaTerm<E>, FormulaTerm<E>),
    Predicate(PredicateNode<E>, Vec<FormulaTerm<E>>),
    True,
    False,
}

/// A formula in clause-normal form, meaning that all quantifiers have been
/// removed.
pub enum CnfFormula<E> {
    Atomic(AtomicFormula<E>),
    Conjunction(Box<Conjunction<E>>),
    Disjunction(Box<Disjunction<E>>),
    Implication(Box<Implication<E>>),
    Biconditional(Box<Biconditional<E>>),
    Negation(Box<Negation<E>>),
}

pub struct Conjunction<E> {
    pub left: CnfFormula<E>,
    pub right: CnfFormula<E>,
}

pub struct Disjunction<E> {
    pub left: CnfFormula<E>,
    pub right: CnfFormula<E>,
}

pub struct Implication<E> {
    pub left: CnfFormula<E>,
    pub right: CnfFormula<E>,
}

pub struct Biconditional<E> {
    pub left: CnfFormula<E>,
    pub right: CnfFormula<E>,
}

pub struct Negation<E> {
    pub right: CnfFormula<E>,
}