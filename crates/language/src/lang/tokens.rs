#[derive(Debug, PartialEq, Clone, Eq)]
pub enum MathOperatorSymbols {
    Star, Plus, Minus, FSlash, Caret, Bang, Dot,
}

impl MathOperatorSymbols {

    pub fn to_bytes(&self) -> Vec<u8> {
        let byte = match self {
            Self::Star => b'*',
            Self::Plus => b'+',
            Self::Minus => b'-',
            Self::FSlash => b'/',
            Self::Caret => b'^',
            Self::Bang => b'!',
            Self::Dot => b'.',
        };
        vec![byte]
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Colon, SemiColon, Equality, Comma,

    Question,

    LeftParen, RightParen,
    LeftBrace, RightBrace,
    LeftBox, RightBox,

    RightArrow,

    Newline, LineBreak,

    // Keywords
    Let, Over, Struct, In, Id, Where, Create, ForAll, ThereEx, Bea,

    Identifier(Vec<u8>), Symbol(MathOperatorSymbols),

    EOF,
}

impl Token {

    #[cfg(test)]
    pub fn id(lexeme: &[u8]) -> Self {
        Self::Identifier(lexeme.to_vec())
    }

    #[cfg(test)]
    pub fn op(op: MathOperatorSymbols) -> Self {
        Self::Symbol(op)
    }
}

pub use self::mappings::single_character_token;
pub use self::mappings::keyword;

mod mappings {

    use super::Token::*;
    use super::MathOperatorSymbols;

    pub fn single_character_token(current: u8, next: Option<u8>) -> Option<(super::Token, bool)> {
        match current {
            // Logical operators
            b'?' => Some((Question, false)),

            b'=' => Some((Equality, false)),
            b':' => Some((Colon, false)),
            b';' => Some((SemiColon, false)),
            b',' => Some((Comma, false)),

            // Braces
            b'(' => Some((LeftParen, false)),
            b')' => Some((RightParen, false)),
            b'{' => Some((LeftBrace, false)),
            b'}' => Some((RightBrace, false)),
            b'[' => Some((LeftBox, false)),
            b']' => Some((RightBox, false)),

            // Symbols
            b'+' => Some((Symbol(MathOperatorSymbols::Plus), false)),
            b'/' => Some((Symbol(MathOperatorSymbols::FSlash), false)),
            b'*' => Some((Symbol(MathOperatorSymbols::Star), false)),
            b'-' => match next {
                Some(b'>') => {
                    Some((RightArrow, true))
                },
                _ => Some((Symbol(MathOperatorSymbols::Minus), false)),
            }
            b'^' => Some((Symbol(MathOperatorSymbols::Caret), false)),
            b'.' => Some((Symbol(MathOperatorSymbols::Dot), false)),
            b'!' => Some((Symbol(MathOperatorSymbols::Bang), false)),
            
            // Whitespace
            b'\n' => Some((Newline, false)),

            _ => None,
        }
    }

    pub fn keyword(bytes: &[u8]) -> Option<super::Token> {
        match bytes {
            b"create" => Some(Create),
            b"bea" => Some(Bea),
            b"forall" => Some(ForAll),
            b"id" => Some(Id),
            b"in" => Some(In),
            b"let" => Some(Let),
            b"over" => Some(Over),
            b"struct" => Some(Struct),
            b"therex" => Some(ThereEx),
            b"where" => Some(Where),

            _ => None,
        }
    }
}
