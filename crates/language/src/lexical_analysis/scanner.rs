use std::iter::Peekable;
use std::sync::{Arc, Mutex};
use crate::lang::tokens::{Token, single_character_token, keyword};
use crate::lang::characters::*;
use crate::input::InputStream;
use super::token_stream::TokenStream;

pub fn get_tokens(stream: InputStream) -> Peekable<TokenStream>{

    // TODO: Fix this chicanery
    let stream_mut = Arc::new(Mutex::new(stream));

    let next: Box<dyn Fn() -> Option<Token>> = Box::new(move|| {
        let mut mut_ref = stream_mut.lock().unwrap();
        match scan_for_token(&mut mut_ref) {
            Token::EOF => None,
            x => Some(x),
        }
    });
    
    TokenStream::new(next)
}

pub fn scan_for_token<'a>(stream: &'a mut InputStream) -> Token {
    skip_whitespace(stream);
    match_non_whitespace_token(stream)
}

pub fn skip_whitespace(stream: &mut InputStream) {
    while stream.peek() == Some(b' '){
        stream.skip();
    }
}

pub fn match_non_whitespace_token<'a>(stream: &'a mut InputStream) -> Token {

    let current_char = match stream.next() {
        Some(x) => x,
        None => return Token::EOF,
    };

    match match_single_character_token(current_char, stream.peek()) {
        Some((x, used_next)) => {
            if used_next {
                stream.next();
            };
            stream.get();
            return x
        },
        None => ()
    };

    match match_identifier(current_char, stream) {
        Some(x) => return x,
        None => ()
    };

    panic!("Can't parse {:?}", String::from_utf8_lossy(stream.get()))
}

fn match_single_character_token(character: u8, next: Option<u8>) -> Option<(Token, bool)> {
    if let Some(x) = single_character_token(character, next) {
        return Some(x);
    };

    None
}

fn match_identifier(character: u8, stream: &mut InputStream) -> Option<Token> {

    if !is_alpha_num(character){
        return None;
    }

    while is_alpha_num_or_underscore(stream.peek()) {
        stream.next();
    }

    let lexeme = stream.get();

    let token = match keyword(lexeme) {
        Some(x) => x,
        None => Token::Identifier(lexeme.to_vec()),
    };

    Some(token)
}


#[cfg(test)]
mod test_scanner {

    use crate::input::InputStream;
    use crate::lang::tokens::{Token, MathOperatorSymbols};
    use super::*;


    #[test]
    fn test_get_tokens() {
        let input = b"let in where over struct id create abc +-*/.!^?=:,(){}[]\n".to_vec();
        let input_stream = InputStream::new(&input);
        let tokens: Vec<Token> = get_tokens(input_stream).collect();

        assert_eq!(tokens, vec![
            Token::Let,
            Token::In,
            Token::Where,
            Token::Over,
            Token::Struct,
            Token::Id,
            Token::Create,
            Token::Identifier(b"abc".to_vec()),
            Token::Symbol(MathOperatorSymbols::Plus),
            Token::Symbol(MathOperatorSymbols::Minus),
            Token::Symbol(MathOperatorSymbols::Star),
            Token::Symbol(MathOperatorSymbols::FSlash),
            Token::Symbol(MathOperatorSymbols::Dot),
            Token::Symbol(MathOperatorSymbols::Bang),
            Token::Symbol(MathOperatorSymbols::Caret),
            Token::Question,
            Token::Equality,
            Token::Colon,
            Token::Comma,
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::RightBrace,
            Token::LeftBox,
            Token::RightBox,
            Token::Newline,
        ]);
    }
}