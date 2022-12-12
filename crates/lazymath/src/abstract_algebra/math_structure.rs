pub mod future;
use future::FutureValue;

use std::{ops::Deref, rc::Rc, iter::zip};

use zsft::{Set, BinaryOperation, BinaryOperationDefinition, SetElement};
use crate::core::{Identity, IdentityElement, IdentityTerm, IdentityExpression};


pub enum StructBinding {
    Element(Rc<SetElement>),
    Operation(Rc<BinaryOperation>),
    Set(Rc<Set>),
}


/// A concrete instance of a math structure, such as a group or a field, usually created by calling `instantiate` on a
/// `MathStructure`.
/// 
/// A MathStructureInstance (MSI) is a implemented as a set, together with a set of operations and identities which define the
/// action of those operations on the set.
pub struct MathStructureInstance {
    pub underlying_set: Rc<Set>,
    pub over_structures: Vec<Rc<MathStructureInstance>>,

    pub bindings: Vec<StructBinding>,

    pub identities: Vec<Identity>,
    pub instance_of: Rc<MathStructure>,
}

impl Deref for MathStructureInstance {
    type Target = Set;

    fn deref(&self) -> &Self::Target {
        &self.underlying_set
    }
}


/// A definition for a binary operation attached to a mathematical structure, with each set given in terms of an index.
/// 
/// 0 corresponds to the underlying set of the current mathematical structure, with every positive index, `i`, corresponding
/// to the underlying set of the structure at the `i-1`th index the `over_structures` array.
/// A x B -> C\
#[derive(PartialEq, Eq, Debug)]
pub struct AbstractBinaryOperationDefinition{
    left: usize,
    right: usize,
    output: usize,
}

impl AbstractBinaryOperationDefinition {

    pub fn new(left: usize, right: usize, output: usize) -> Self {
        Self {
            left,
            right,
            output,
        }
    }

    pub fn to_binary_operation(&self, sets: &Vec<Rc<Set>>) -> Rc<BinaryOperation> {
        BinaryOperation::new(
            BinaryOperationDefinition(
                sets[self.left].clone(),
                sets[self.right].clone(),
                sets[self.output].clone(),
            )
        )
    }
}

/// A specification of the elements which are to be used in the `identity_definition` array, where each index refers to the
/// set which the element is contained within.
#[derive(Debug, PartialEq, Eq)]
pub enum IdentityDefinitionElement {
    ForAll(FutureValue<SetElement>),
    Bound(FutureValue<SetElement>),
}

impl IdentityDefinitionElement {

    pub fn to_identity_element(&self) -> IdentityElement {
        match self {
            IdentityDefinitionElement::ForAll(i) =>
                IdentityElement::ForAll(
                    i.get().unwrap()
                ),
            IdentityDefinitionElement::Bound(i) =>
                IdentityElement::ForOne(
                    i.get().unwrap()
                ),
        }
    }

}

#[derive(Debug, PartialEq, Eq)]
pub enum IdentityExpressionDefinitionTerm {
    Element(IdentityDefinitionElement),
    BinaryOperation(FutureValue<BinaryOperation>),
}

#[derive(PartialEq, Eq, Debug)]
pub struct IdentityExpressionDefinition {
    definition: Vec<IdentityExpressionDefinitionTerm>,
}

impl IdentityExpressionDefinition {

    pub fn new(definition: Vec<IdentityExpressionDefinitionTerm>) -> Self {
        Self {
            definition,
        }
    }

    pub fn to_identity_expression(&self) -> IdentityExpression {
        let mut expression: IdentityExpression = vec![];

        for term in &self.definition {
            match term {
                IdentityExpressionDefinitionTerm::BinaryOperation(i) => {
                    expression.push(
                        IdentityTerm::BinaryOperation(
                            i.get().unwrap()
                        )
                    );
                },
                IdentityExpressionDefinitionTerm::Element(i) => {
                    expression.push(
                        IdentityTerm::Element(
                            i.to_identity_element()
                        )
                    );
                },
            };
        }

        expression
    }

}

#[derive(PartialEq, Eq, Debug)]
pub struct IdentityDefinition {
    left: IdentityExpressionDefinition,
    right: IdentityExpressionDefinition,
}

impl IdentityDefinition {

    pub fn new(left: IdentityExpressionDefinition, right: IdentityExpressionDefinition) -> Self {
        Self {
            left,
            right,
        }
    }

    pub fn to_identity(&self) -> Identity {
        Identity(
            self.left.to_identity_expression(),
            self.right.to_identity_expression(),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FutureStructBinding {
    Operation(FutureValue<BinaryOperation>),
    Element(FutureValue<SetElement>),
    Set(FutureValue<Set>),
}

impl FutureStructBinding {

    pub fn reify(&self) -> StructBinding {
        match self {
            Self::Element(e) => StructBinding::Element(e.reify()),
            Self::Operation(o) => StructBinding::Operation(o.reify()),
            Self::Set(s) => StructBinding::Set(s.reify()),
        }
    }
}

#[derive(Debug)]
pub enum MathStructureInstantiationError {
    IncorrectNumberOfArguments(usize, usize),  // (expected, found)
    StructureOfWrongType(usize, Rc<MathStructure>, Rc<MathStructure>)  // (position, expected, found)
}

/// An abstract definition of a math structure
#[derive(Debug, PartialEq, Eq)]
pub struct MathStructure {
    pub future_set: FutureValue<Set>,
    pub over_structures: Vec<Rc<MathStructure>>,
    pub future_bindings: Vec<FutureStructBinding>,
    pub future_internals: Vec<FutureValue<SetElement>>,

    pub identity_definitions: Vec<IdentityDefinition>,
}

impl MathStructure {

    pub fn new(
        future_set: FutureValue<Set>,
        over_structures: Vec<Rc<MathStructure>>,
        future_bindings: Vec<FutureStructBinding>,
        future_internals: Vec<FutureValue<SetElement>>,
        identity_definitions: Vec<IdentityDefinition>,
    ) -> Self {
        Self {
            future_set,
            over_structures,
            future_bindings,
            future_internals,
            identity_definitions,
        }
    }

    /// Create an instance of a math structure, specified over the structures in `over_structures`. `over_structures` is an array
    /// of existing structure instances, which are instances of the structures specified in `MathStructure.over_structures`
    /// respectively.
    pub fn instantiate(self: &Rc<Self>, over_structures: Vec<Rc<MathStructureInstance>>) -> Result<MathStructureInstance, MathStructureInstantiationError> {

        self.validate_over_structures(&over_structures)?;

        let underlying_set = self.future_set.reify();

        let bindings: Vec<StructBinding> =
            self.future_bindings
                .iter()
                .map(|future_binding| {
                    future_binding.reify()
                })
                .collect();
        
        let _: Vec<Rc<SetElement>> =
            self.future_internals
                .iter()
                .map(|future_binding| {
                    future_binding.reify()
                })
                .collect();

        let identities: Vec<Identity> =
            self.identity_definitions
                .iter()
                .map(|id| id.to_identity())
                .collect();
        
        Ok(MathStructureInstance {
            bindings,
            underlying_set,
            over_structures,
            identities,
            instance_of: self.clone(),
        })
    }

    fn validate_over_structures(self: &Rc<Self>, over_structures: &Vec<Rc<MathStructureInstance>>) -> Result<(), MathStructureInstantiationError> {
        if !over_structures.len() == self.over_structures.len() {
            return Err(
                MathStructureInstantiationError::IncorrectNumberOfArguments(
                    self.over_structures.len(), over_structures.len()
                )
            );
        };

        for (i, (over_structure_instance, over_structure)) in zip(over_structures.iter(), self.over_structures.iter()).enumerate() {
            if over_structure_instance.instance_of != over_structure.clone() {
                return Err(
                    MathStructureInstantiationError::StructureOfWrongType(
                        i,
                        over_structure.clone(),
                        over_structure_instance.instance_of.clone()
                    )
                );
            }
        };

        Ok(())
    }

}