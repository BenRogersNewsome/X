
use zsft::{SetElement, BinaryOperation, Item};


pub enum IdentityElement {
    ForAll(SetElement),
    ForOne(Item),
}

pub enum IdentityTerm {
    Element(IdentityElement),
    BinaryOperation(BinaryOperation),
}

pub type IdentityExpression = Vec<IdentityTerm>;

pub struct Identity (pub IdentityExpression, pub IdentityExpression);