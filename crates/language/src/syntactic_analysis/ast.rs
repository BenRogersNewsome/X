
use crate::{lexical_analysis::TokenType, scope::ScopedItem, lexical_analysis::Token};

#[derive(Debug)]
pub enum NodeParseError {
    UnexpectedEndOfInput,
    UnexpectedToken(Token, Vec<TokenType>),
}

#[derive(Debug)]
pub enum NodeVisitationError {
    UnexpectedRegisteredItem(ScopedItem),
    RegisteredItemNotFound,
    ItemAlreadyExists(ScopedItem),

    ReDeclaredScopedVariable(Vec<u8>),
    CantResolveToken(String),
    TokenOfWrongType(Vec<u8>, ScopedItem),

    Custom(&'static str),
}

// pub trait Node {
//     fn try_new<'a>(tokens: &'a mut Peekable<dyn Iterator<Item = Token>>) -> Result<Option<Box<Self>>, NodeParseError>;
//     fn new<'a>(tokens: &'a mut Peekable<dyn Iterator<Item = Token>>) -> Result<Box<Self>, NodeParseError> {
//         match Self::try_new(tokens)? {
//             Ok(None) => Err(NodeParseError::ExpectedNodeNotPresent),
//             Some(x) => Ok(x),
//         }
//     }
//     fn to_str(&self) -> String;
// }