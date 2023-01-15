use std::{collections::HashMap, rc::Rc};
use zsft::{Set, SetElement, BinaryOperation, Item};
use lazymath::core::Expression;
use lazymath::abstract_algebra::{MathStructure, FutureValue, FutureStructBinding};

#[derive(PartialEq, Debug)]
pub enum ScopedItem {
    Item(Item),
    Set(Set),
    SetElement(SetElement),
    BinaryOperation(BinaryOperation),
    Expression(Rc<Expression>),
    Structure(Rc<MathStructure>),

    FutureItem(FutureValue<Item>),
    FutureSet(FutureValue<Set>),
    FutureSetElement(FutureValue<SetElement>),
    FutureBoundSetElement(FutureValue<SetElement>),
    FutureBinaryOperation(FutureValue<BinaryOperation>),
    FutureStructBinding(FutureStructBinding),
}

impl ScopedItem {

    pub fn to_owned(&self) -> Self {
        match self {
            Self::Item(x) => Self::Item(x.clone()),
            Self::Set(x) => Self::Set(x.clone()),
            Self::SetElement(x) => Self::SetElement(x.clone()),
            Self::BinaryOperation(x) => Self::BinaryOperation(x.clone()),
            Self::Expression(x) => Self::Expression(x.clone()),
            Self::Structure(x) => Self::Structure(x.clone()),
            Self::FutureItem(x) => Self::FutureItem(x.clone()),
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

    pub fn contains_key(&self, identifier: &[u8]) -> bool {
        self.register.contains_key(identifier)
    }

}

pub struct InteriorScope<'a> {
    parent_scope: &'a mut Scope,
    register: HashMap<Vec<u8>, ScopedItem>,
}

impl<'a> InteriorScope<'a> {

    pub fn from(parent_scope: &'a mut Scope) -> Self {
        Self {
            parent_scope,
            register: HashMap::new(),
        }
    }

    /// Add element to the register. Returns `Err(())` if element already exists, and Ok(()) if otherwise.
    pub fn add(&mut self, identifier: Vec<u8>, item: ScopedItem) -> Result<(), &ScopedItem> {
        if self.register.contains_key(&identifier) || self.parent_scope.contains_key(&identifier) {
            return Err(self.register.get(&identifier).unwrap())
        }else{
            self.register.insert(identifier, item);
            return Ok(())
        }
    }

    pub fn get(&self, identifier: &[u8]) -> Option<&ScopedItem> {
        if let Some(x) = self.register.get(&identifier.to_vec()) {
            Some(x)
        } else if let Some(x) = self.parent_scope.get(&identifier) {
            Some(x)
        } else {
            None
        }
    }

}