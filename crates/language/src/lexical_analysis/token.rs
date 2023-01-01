
/// A struct representing lexical unit of <lang_name> code.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub type_: TokenType,
    pub rank: u8,
    pub position: (usize, usize) // (line, col)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    Colon, SemiColon, Equality, Comma, Turnstile,

    Close,  // ..
    Spread, // ...
    
    Question,
    
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    LeftBox, RightBox,
    
    RightArrow,
    
    Newline,
    
    // Keywords
    Let, Over, Struct, In, Id, Where, Create, ForAll, ThereEx, Bea, Be, Of,
    
    Identifier(Vec<u8>), Symbol(MathOperatorSymbols),
    
    EOF,
}

#[cfg(test)]
impl TokenType {
    pub fn id(lexeme: &[u8]) -> Self {
        Self::Identifier(lexeme.to_vec())
    }

    pub fn op(op: MathOperatorSymbols) -> Self {
        Self::Symbol(op)
    }
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum MathOperatorSymbols {
    Star, Plus, Minus, FSlash, Caret, Bang, Dot, Wedge,
    
    Del,
}

impl MathOperatorSymbols {

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Self::Del => b"@".to_vec(),
            Self::Star => b"*".to_vec(),
            Self::Plus => b"+".to_vec(),
            Self::Minus => b"-".to_vec(),
            Self::FSlash => b"/".to_vec(),
            Self::Caret => b"^".to_vec(),
            Self::Bang => b"!".to_vec(),
            Self::Dot => b".".to_vec(),
            Self::Wedge => b"/\\".to_vec(),
        }
    }
}