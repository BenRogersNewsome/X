#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Plus, Minus, Star, Slash, Dot,

    Bang, 

    Colon, Equality, Comma,

    Question,

    LeftParen, RightParen,
    LeftBrace, RightBrace,
    LeftBox, RightBox,

    Newline, 

    // Keywords
    Let, Over, Struct, In, Id, Where, Create,

    Identifier,

    EOF,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: Vec<u8>,
}

pub use self::mappings::single_character_token;
pub use self::mappings::keyword;

mod mappings {
    use super::TokenType::*;

    pub fn single_character_token(character: u8) -> Option<super::TokenType> {
        match character {

            // Mathematical Operators
            b'+' => Some(Plus),
            b'-' => Some(Minus),
            b'*' => Some(Star),
            b'/' => Some(Slash),
            b'.' => Some(Dot),
            b'!' => Some(Bang),
            
            // Logical operators
            b'?' => Some(Question),

            b'=' => Some(Equality),
            b':' => Some(Colon),
            b',' => Some(Comma),

            // Braces
            b'(' => Some(LeftParen),
            b')' => Some(RightParen),
            b'{' => Some(LeftBrace),
            b'}' => Some(RightBrace),
            b'[' => Some(LeftBox),
            b']' => Some(RightBox),
            
            // Whitespace
            b'\n' => Some(Newline),

            _ => None,
        }
    }

    pub fn keyword(bytes: &[u8]) -> Option<super::TokenType> {
        match bytes {
            b"let" => Some(Let),
            b"in" => Some(In),
            b"where" => Some(Where),
            b"over" => Some(Over),
            b"struct" => Some(Struct),
            b"id" => Some(Id),
            b"create" => Some(Create),

            _ => None,
        }
    }
}
