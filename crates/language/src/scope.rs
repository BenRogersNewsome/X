use std::{collections::HashMap, rc::Rc};
use zsft::{Set, SetElement, BinaryOperation};
use lazymath::core::Expression;
use lazymath::abstract_algebra::{MathStructure, FutureValue, FutureStructBinding};

#[derive(PartialEq, Debug)]
pub enum ScopedItem {
    Set(Set),
    SetElement(SetElement),
    BinaryOperation(BinaryOperation),
    Expression(Rc<Expression>),
    Structure(Rc<MathStructure>),

    FutureSet(FutureValue<Set>),
    FutureSetElement(FutureValue<SetElement>),
    FutureBoundSetElement(FutureValue<SetElement>),
    FutureBinaryOperation(FutureValue<BinaryOperation>),
    FutureStructBinding(FutureStructBinding),
}

impl ScopedItem {

    pub fn to_owned(&self) -> Self {
        match self {
            Self::Set(x) => Self::Set(x.clone()),
            Self::SetElement(x) => Self::SetElement(x.clone()),
            Self::BinaryOperation(x) => Self::BinaryOperation(x.clone()),
            Self::Expression(x) => Self::Expression(x.clone()),
            Self::Structure(x) => Self::Structure(x.clone()),
            Self::FutureSet(x) => Self::FutureSet(x.clone()),
            Self::FutureSetElement(x) => Self::FutureSetElement(x.clone()),
            Self::FutureBoundSetElement(x) => Self::FutureBoundSetElement(x.clone()),
            Self::FutureBinaryOperation(x) => Self::FutureBinaryOperation(x.clone()),
            Self::FutureStructBinding(x) => Self::FutureStructBinding(x.clone()),
        }
    }
}

pub struct Scope {
    register: HashMap<Vec<u8>, ScopedItem>,
}

impl Scope {

    pub fn new() -> Self {
        Self {
            register: HashMap::new(),
        }
    }

    /// Add element to the register. Returns `Err(())` if element already exists, and Ok(()) if otherwise.
    pub fn add(&mut self, identifier: Vec<u8>, item: ScopedItem) -> Result<(), &ScopedItem> {
        if self.register.contains_key(&identifier) {
            return Err(self.register.get(&identifier).unwrap())
        }else{
            self.register.insert(identifier, item);
            return Ok(())
        }
    }

    pub fn get(&self, identifier: &[u8]) -> Option<&ScopedItem> {
        match self.register.get(&identifier.to_vec()) {
            None => None,
            Some(x) => Some(x),
        }
    }

}