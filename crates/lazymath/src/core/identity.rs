use std::rc::Rc;

use zsft::{SetElement, BinaryOperation};


#[derive(Clone)]
pub enum IdentityElement {
    ForAll(Rc<SetElement>),
    ForOne(Rc<SetElement>),
}

pub enum IdentityTerm{
    Element(IdentityElement),
    BinaryOperation(Rc<BinaryOperation>),
}

pub type IdentityExpression = Vec<IdentityTerm>;

pub struct Identity (pub IdentityExpression, pub IdentityExpression);