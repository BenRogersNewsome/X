
use zsft::{SetElement, BinaryOperation};


#[derive(Clone)]
pub enum IdentityElement {
    ForAll(SetElement),
    ForOne(SetElement),
}

pub enum IdentityTerm{
    Element(IdentityElement),
    BinaryOperation(BinaryOperation),
}

pub type IdentityExpression = Vec<IdentityTerm>;

pub struct Identity (pub IdentityExpression, pub IdentityExpression);