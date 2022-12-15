use zsft::{BinaryOperation, SetElement};

#[derive(PartialEq, Eq, Debug)]
pub enum ExpressionTerm {
    Element(SetElement),
    BinaryOperation(BinaryOperation),
}

/// A mathematical expression, e.g. `a+2*b`, expressed as a tree in it's pre-traversal representation.
#[derive(PartialEq, Eq, Debug)]
pub struct Expression(Vec<ExpressionTerm>);

impl Expression {
    pub fn new(expression: Vec<ExpressionTerm>) -> Self {
        Self(expression)
    }

    pub fn to_set_element(&self) -> SetElement {
        let mut elements = self.0.iter();

        Self::_to_set_element(&mut elements)
    }

    fn _to_set_element<'a, T: Iterator<Item=&'a ExpressionTerm>>(elements: &mut T) -> SetElement {
        match elements.next() {
            Some(ExpressionTerm::Element(e)) => e.clone(),
            Some(ExpressionTerm::BinaryOperation(b)) => {
                let left = Self::_to_set_element(elements);
                let right = Self::_to_set_element(elements);
                SetElement::from_binary_operation(b, &left,&right).unwrap()
            },
            None => panic!("Unexpected end of input"),
        }
    }
}

pub struct Equality {
    pub left: Expression,
    pub right: Expression,
}