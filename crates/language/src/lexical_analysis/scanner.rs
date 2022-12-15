use std::cell::RefCell;
use std::iter::Peekable;
use std::rc::Rc;
use crate::input::InputStream;
use super::token_stream::TokenStream;
use super::token::{
    Token,
    TokenType,
    TokenType::*,
    MathOperatorSymbols,
};

pub fn get_tokens(stream: InputStream) -> Peekable<TokenStream>{

    let stream_mut = Rc::new(RefCell::new(stream));

    let next: Box<dyn Fn() -> Option<Token>> = Box::new(move|| {
        let mut mut_ref = stream_mut.borrow_mut();
        match scan_for_token(&mut mut_ref) {
            Token { type_: TokenType::EOF, position: _, rank: _ } => None,
            x => Some(x),
        }
    });
    
    TokenStream::new(next)
}

fn scan_for_token<'a>(stream: &'a mut InputStream) -> Token {
    skip_whitespace(stream);
    match_non_whitespace_token(stream)
}

fn skip_whitespace(stream: &mut InputStream) {
    while stream.peek() == Some(b' '){
        stream.skip();
    }
}

fn match_non_whitespace_token<'a>(stream: &'a mut InputStream) -> Token {

    let current_char = match stream.next() {
        Some(x) => x,
        None => return Token {
            type_: TokenType::EOF,
            rank: 0,
            position: stream.get_position(),
        },
    };

    if let Some((token_type, rank)) = single_character_token(current_char, stream.peek(), stream.ppeek()) {
        for _ in 1..rank {
            stream.next();
        };
        stream.get();
        return Token {
            type_: token_type,
            rank,
            position: stream.get_position(),
        };
    };

    if let Some((token_type, rank)) = match_identifier(current_char, stream) {
        return Token {
            type_: token_type,
            rank,
            position: stream.get_position(),
        }
    };

    panic!("Can't parse {:?}", String::from_utf8_lossy(stream.get()))
}

fn match_identifier(character: u8, stream: &mut InputStream) -> Option<(TokenType, u8)> {

    if !is_alpha_num(character){
        return None;
    }

    while is_alpha_num_or_underscore(stream.peek()) {
        stream.next();
    }

    let lexeme = stream.get();

    let rank = lexeme.len() as u8;

    let token = match keyword(lexeme) {
        Some(x) => x,
        None => TokenType::Identifier(lexeme.to_vec()),
    };

    Some((token, rank))
}

fn is_alpha_num(byte: u8) -> bool {
    (byte >= b'A' && byte <= b'Z') || (byte >= b'a' && byte <= b'z') || ( byte >= b'0' && byte <= b'9')
}

fn is_underscore(byte: u8) -> bool {
    byte == b'_'
}

fn is_alpha_num_or_underscore(byte: Option<u8>) -> bool {
    match byte {
        Some(x) => is_alpha_num(x) || is_underscore(x),
        None => false,
    }
}

fn single_character_token(current: u8, next: Option<u8>, nnext: Option<u8>) -> Option<(TokenType, u8)> {
    match (current, next, nnext) {
        // Logical operators
        (b'?', _, _) => Some((Question, 1)),

        (b'=', _, _) => Some((Equality, 1)),
        (b':', _, _) => Some((Colon, 1)),
        (b';', _, _) => Some((SemiColon, 1)),
        (b',', _, _) => Some((Comma, 1)),

        (b'|', Some(b'-'), _) => Some((Turnstile, 2)),
        (b'(', Some(b'-'), _) => Some((In, 2)),
        (b'\\', Some(b'-'), Some(b'/')) => Some((ForAll, 3)),

        // Braces
        (b'(', _, _) => Some((LeftParen, 1)),
        (b')', _, _) => Some((RightParen, 1)),
        (b'{', _, _) => Some((LeftBrace, 1)),
        (b'}', _, _) => Some((RightBrace, 1)),
        (b'[', _, _) => Some((LeftBox, 1)),
        (b']', _, _) => Some((RightBox, 1)),

        // Symbols
        (b'+', _, _) => Some((Symbol(MathOperatorSymbols::Plus), 1)),
        (b'/', Some(b'\\'), _) => Some((Symbol(MathOperatorSymbols::Wedge), 2)),
        (b'/', _, _) => Some((Symbol(MathOperatorSymbols::FSlash), 1)),
        (b'*', _, _) => Some((Symbol(MathOperatorSymbols::Star), 1)),
        (b'-', Some(b'>'), _) => Some((RightArrow, 2)),
        (b'-', _, _) => Some((Symbol(MathOperatorSymbols::Minus), 1)),
        (b'^', _, _) => Some((Symbol(MathOperatorSymbols::Caret), 1)),
        (b'.', _, _) => Some((Symbol(MathOperatorSymbols::Dot), 1)),
        (b'!', _, _) => Some((Symbol(MathOperatorSymbols::Bang), 1)),
        (b'@', _, _) => Some((Symbol(MathOperatorSymbols::Del), 1)),
        
        // Whitespace
        (b'\n', _, _) => Some((Newline, 1)),

        _ => None,
    }
}

fn keyword(bytes: &[u8]) -> Option<TokenType> {
    match bytes {
        b"assert" => Some(Turnstile),
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


// #[cfg(test)]
// mod test_scanner {

//     use crate::input::InputStream;
//     use crate::lexical_analysis::{TokenType, MathOperatorSymbols};
//     use super::*;


//     #[test]
//     fn test_get_tokens() {
//         let input = b"let in where over struct id create abc +-*/.!^?=:,(){}[]\n".to_vec();
//         let input_stream = InputStream::new(&input);
//         let tokens: Vec<Token> = get_tokens(input_stream).collect();

//         assert_eq!(tokens, vec![
//             TokenType::Let,
//             TokenType::In,
//             TokenType::Where,
//             TokenType::Over,
//             TokenType::Struct,
//             TokenType::Id,
//             TokenType::Create,
//             TokenType::Identifier(b"abc".to_vec()),
//             TokenType::Symbol(MathOperatorSymbols::Plus),
//             TokenType::Symbol(MathOperatorSymbols::Minus),
//             TokenType::Symbol(MathOperatorSymbols::Star),
//             TokenType::Symbol(MathOperatorSymbols::FSlash),
//             TokenType::Symbol(MathOperatorSymbols::Dot),
//             TokenType::Symbol(MathOperatorSymbols::Bang),
//             TokenType::Symbol(MathOperatorSymbols::Caret),
//             TokenType::Question,
//             TokenType::Equality,
//             TokenType::Colon,
//             TokenType::Comma,
//             TokenType::LeftParen,
//             TokenType::RightParen,
//             TokenType::LeftBrace,
//             TokenType::RightBrace,
//             TokenType::LeftBox,
//             TokenType::RightBox,
//             TokenType::Newline,
//         ]);
//     }
// }