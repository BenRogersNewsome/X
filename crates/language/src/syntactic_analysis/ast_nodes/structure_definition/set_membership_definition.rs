use std::iter::Iterator;
use std::iter::Peekable;

use lazymath::abstract_algebra::FutureValue;
use zsft::SetElement;

use crate::lang::tokens::Token;
use crate::scope::Scope;
use crate::scope::ScopedItem;
use crate::syntactic_analysis::ast::NodeParseError;
use crate::syntactic_analysis::ast::NodeVisitationError;
use crate::syntactic_analysis::ast_nodes::expect_token;

use super::Identifier;


pub struct SetMembershipDefinition {
    pub members: Vec<Identifier>,
    pub set: Identifier,
}

impl SetMembershipDefinition {

    pub fn new<'a, T: Iterator<Item = Token>>(tokens: &'a mut Peekable<T>) -> Result<Box<Self>, NodeParseError> {
        
        expect_token!(tokens, ForAll);

        let mut members = vec![];

        let set = loop {
            members.push(*Identifier::new(tokens)?);

            if let Some(Token::In) = tokens.peek() {
                tokens.next();
                break *Identifier::new(tokens)?
            }else{
                expect_token!(tokens, Comma);
            }
        };

        Ok(Box::new(Self {
            members,
            set,
        }))
    }

    pub fn get_internal_future_bindings(self, scope: &mut Scope) -> Result< Vec<FutureValue<SetElement>>, NodeVisitationError> {
        
        let mut internals: Vec<FutureValue<SetElement>> = Vec::new();

        let parent_set = match scope.get(&self.set.lexeme) {
            Some(ScopedItem::FutureSet(s)) => s.clone(),
            Some(x) => return Err(NodeVisitationError::TokenOfWrongType(self.set.lexeme.to_vec(), x.to_owned())),
            _ => return Err(NodeVisitationError::CantResolveToken(String::from_utf8_lossy(&self.set.lexeme).to_string())),
        };

        for member_id in self.members.into_iter() {
            if scope.get(&member_id.lexeme) != None {
                return Err(NodeVisitationError::ReDeclaredScopedVariable(member_id.lexeme));
            };
            let cloned_parent_set = parent_set.clone();
            let future_set_element: FutureValue<SetElement> = FutureValue::new(Box::new(move || {
                SetElement::element_of(cloned_parent_set.clone().get().unwrap())
            }));

            scope.add(member_id.lexeme.to_vec(), ScopedItem::FutureSetElement(future_set_element.clone())).unwrap();

            internals.push(future_set_element);
        };

        Ok(internals)

    }
}

// #[cfg(test)]
// mod tests {

//     use super::*;

//     #[test]
//     fn test_create_set_membership_definition_forall() {

//         let tokens = [
//             Token::ForAll, Token::Identifier(b"a".to_vec()), Token::Comma, Token::Identifier(b"b".to_vec()), Token::In,
//             Token::Identifier(b"F".to_vec()),
//         ];

//         let set_membership_definition = SetMembershipDefinition::new(&mut tokens.into_iter().peekable()).unwrap();

//         assert_eq!(set_membership_definition.relationship, SetMembershipRelationship::ForAll);
//         assert_eq!(set_membership_definition.members.len(), 2);
//         assert_eq!(set_membership_definition.members[0].lexeme, b"a".to_vec());
//         assert_eq!(set_membership_definition.members[1].lexeme, b"b".to_vec());
//         assert_eq!(set_membership_definition.set.lexeme, b"F".to_vec());
//     }

//     #[test]
//     fn test_create_set_membership_definition_therex() {

//         let tokens = [
//             Token::ThereEx, Token::Identifier(b"0".to_vec()), Token::Comma, Token::Identifier(b"1".to_vec()), Token::In,
//             Token::Identifier(b"V".to_vec()),
//         ];

//         let set_membership_definition = SetMembershipDefinition::new(&mut tokens.into_iter().peekable()).unwrap();

//         assert_eq!(set_membership_definition.relationship, SetMembershipRelationship::ThereEx);
//         assert_eq!(set_membership_definition.members.len(), 2);
//         assert_eq!(set_membership_definition.members[0].lexeme, b"0".to_vec());
//         assert_eq!(set_membership_definition.members[1].lexeme, b"1".to_vec());
//         assert_eq!(set_membership_definition.set.lexeme, b"V".to_vec());
//     }
// }