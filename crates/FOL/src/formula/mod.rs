mod tree;

pub use tree::{
    AtomicFormula,
    Biconditional,
    CnfFormula,
    Conjunction,
    Disjunction,
    FormulaTerm,
    FreeVariable,
    Implication,
    Negation,
};

use crate::PredicateNode;

fn _into_predicate_node<E>(formula: CnfFormula<E>) -> PredicateNode<E> {
    
}

impl<E> From<CnfFormula<E>> for PredicateNode<E> {

    fn from(formula: CnfFormula<E>) -> Self {
        _into_predicate_node(formula)    
    }

}