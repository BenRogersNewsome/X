use crate::{syntax::{Implication, GenericFormula, Disjunction, Negation, Conjunction, Biconditional, Existential, Universal, GenericAtomicFormula}};

pub struct ClauseNormalForm {
    sets: Vec<Vec<GenericAtomicFormula>>,
}

impl ClauseNormalForm {
    fn collect_into_sets(f: GenericFormula) {
        match f {
            GenericFormula::
        }
    }
}

impl From<GenericFormula> for ClauseNormalForm {
    fn from(f: GenericFormula) -> Self {
        let conjunctive_normal_form = f.to_cnf();
        
    }
}

impl GenericFormula {
    pub fn to_cnf(self) -> ClauseNormalForm {
        let snf = self.to_snf();
        let conjunctive_normal_form = self.clausify();
    }
}


trait ToConjunctiveNormalForm {
    type Output;
    fn clausify(self) -> Self::Output;
}

impl ToConjunctiveNormalForm for GenericFormula {
    type Output = Self;

    fn clausify(self) -> Self::Output {
        match self {
            Self::Atomic(x) => x.into(),
            Self::Biconditional(x) => x.clausify(),
            Self::Conjunction(x) => x.clausify(),
            Self::Disjunction(x) => x.clausify(),
            Self::Existential(x) => x.clausify(),
            Self::Implication(x) => x.clausify(),
            Self::Negation(x) => x.clausify(),
            Self::Universal(x) => x.clausify(),
        }
    }
}

/// Implication : P -> Q ~= (!P) \/ Q
impl ToConjunctiveNormalForm for Implication<GenericFormula, GenericFormula> {
    type Output = GenericFormula;

    fn clausify(self) -> Self::Output {
        Disjunction {
            left: Negation {
                right: self.left.clausify(),
            }.clausify(),
            right: self.right.clausify(),
        }.clausify()
    }
}

/// Equivalence : P <-> Q ~= (P -> Q) /\ (Q -> P)
impl ToConjunctiveNormalForm for Biconditional<GenericFormula, GenericFormula> {
    type Output = GenericFormula;

    fn clausify(self) -> Self::Output {
        Conjunction {
            left: Implication {
                left: self.left.clone().clausify(),
                right: self.right.clone().clausify(),
            }.clausify(),
            right: Implication {
                left: self.right.clausify(),
                right: self.left.clausify(),
            }.clausify(),
        }.clausify()
    }
}

impl ToConjunctiveNormalForm for Negation<GenericFormula> {
    type Output = GenericFormula;

    fn clausify(self) -> Self::Output {
        match self.right {

            // De Morgan 1
            // !(P \/ Q) ~= !P /\ !Q
            GenericFormula::Disjunction(c) => {
                Conjunction {
                    left: Negation {
                        right: c.left.clausify(),
                    }.clausify(),
                    right: Negation {
                        right: c.right.clausify(),
                    }.clausify(),
                }.clausify()
            },

            // De Morgan 2
            // !(P /\ Q) ~= !P \/ !Q
            GenericFormula::Conjunction(c) => {
                Disjunction {
                    left: Negation {
                        right: c.left.clausify(),
                    }.clausify(),
                    right: Negation {
                        right: c.right.clausify(),
                    }.clausify(),
                }.clausify()
            },

            // Double Negation
            // !!P ~= P
            GenericFormula::Negation(n) => {
                n.right.clausify()
            }

            x => Negation {right: x.clausify()}.into(),
        }
    }
}

impl ToConjunctiveNormalForm for Disjunction<GenericFormula, GenericFormula> {
    type Output = GenericFormula;

    // Distributivity Disjunction : P \/ (Q /\ X) ∼= (P \/ Q) /\ (P \/ X) 
    fn clausify(self) -> Self::Output {

        match (self.left, self.right) {
            (GenericFormula::Conjunction(c1), x) |
            (x, GenericFormula::Conjunction(c1)) => {
                Conjunction {
                    left: Disjunction {
                        left: x.clausify(),
                        right: c1.left.clausify(),
                    }.into(),
                    right: Disjunction {
                        left: x.clausify(),
                        right: c1.right.clausify(),
                    }.into()
                }.into()
            },

            (GenericFormula::Conjunction(c1), GenericFormula::Conjunction(c2)) => {
                Conjunction {
                    left: Conjunction {
                        left: Disjunction {
                            left: c1.left.clausify(),
                            right: c2.left.clausify(),
                        }.into(),
                        right: Disjunction {
                            left: c1.left.clausify(),
                            right: c2.right.clausify(),
                        }.into()
                    }.into(),
                    right: Conjunction {
                        left: Disjunction {
                            left: c1.right.clausify(),
                            right: c2.left.clausify(),
                        }.into(),
                        right: Disjunction {
                            left: c1.right.clausify(),
                            right: c2.right.clausify(),
                        }.into()
                    }.into()
                }.into()
            },

            (left, right) => {
                Disjunction {
                    left: left.clausify(),
                    right: right.clausify(),
                }.into()
            }

        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Trivial Impls
////////////////////////////////////////////////////////////////////////////////

impl ToConjunctiveNormalForm for Conjunction<GenericFormula, GenericFormula> {
    type Output = GenericFormula;

    fn clausify(self) -> Self::Output {
        Conjunction {
            left: self.left.clausify(),
            right: self.right.clausify(),
        }.into()
    }
}

impl ToConjunctiveNormalForm for Existential<GenericFormula> {
    type Output = GenericFormula;

    fn clausify(self) -> Self::Output {
        Existential {
            left: self.left,
            right: self.right.clausify(),
        }.into()
    }
}

impl ToConjunctiveNormalForm for Universal<GenericFormula> {
    type Output = GenericFormula;

    fn clausify(self) -> Self::Output {
        Universal {
            left: self.left,
            right: self.right.clausify(),
        }.into()
    }
}