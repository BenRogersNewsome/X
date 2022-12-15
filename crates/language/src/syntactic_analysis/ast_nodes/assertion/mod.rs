use std::iter::Peekable;

use zsft::{Set, SetElement};

use crate::{syntactic_analysis::ast::{NodeParseError, NodeVisitationError}, Scope, ScopedItem, lexical_analysis::Token};

use super::{Identifier, expect_token};

pub enum AssertionType {
    In(Identifier),
}

pub struct Assertion {
    item: Identifier,
    assertion: AssertionType,
}

impl Assertion {
    
    pub fn new<T: Iterator<Item=Token>>(tokens: &mut Peekable<T>) -> Result<Box<Self>, NodeParseError> {
        let item: Identifier = *Identifier::new(tokens)?;

        let assertion = expect_token!(tokens, In, Self::_in_assertion(tokens)?);

        Ok(Box::new(Self {
            item,
            assertion
        }))
    }

    fn _in_assertion<T: Iterator<Item=Token>>(tokens: &mut Peekable<T>) -> Result<AssertionType, NodeParseError> {
        Ok(AssertionType::In(*Identifier::new(tokens)?))
    }

    pub fn visit(self, scope: &mut Scope) -> Result<(), NodeVisitationError> {
        match &self.assertion {
            AssertionType::In(i) => self._visit_in_assertion(scope, i),
        }
    }

    fn _visit_in_assertion(&self, scope: &mut Scope, set_ident: &Identifier) -> Result<(), NodeVisitationError> {

        let set: &Set = match scope.get(&set_ident.lexeme) {
            Some(ScopedItem::Set(set)) => set,
            Some(x) => return Err(NodeVisitationError::UnexpectedRegisteredItem(x.to_owned())),
            None => return Err(NodeVisitationError::CantResolveToken(set_ident.clone().into()))
        };

        let set_element: SetElement = match scope.get(&self.item.lexeme) {
            Some(ScopedItem::SetElement(element)) => element.clone(),
            Some(ScopedItem::Expression(expr)) => expr.to_set_element(),
            Some(x) => return Err(NodeVisitationError::UnexpectedRegisteredItem(x.to_owned())),
            None => return Err(NodeVisitationError::CantResolveToken(self.item.clone().into()))
        };

        if !set.contains(&set_element) {
            panic!("Assertion failed")
        };

        Ok(())
    }
}