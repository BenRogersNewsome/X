use super::tokens::*;
use crate::lang::characters::*;
use crate::input::InputStream;

pub fn get_tokens(stream: &mut InputStream) -> Vec<Token> {

    let mut tokens: Vec<Token> = vec![];

    while match tokens.last() {
        None => true,
        Some(x) => match x.token_type {
            TokenType::EOF => false,
            _ => true,
        }
    } {
        tokens.push(scan_for_token(stream));
    }
    
    return tokens;
}

pub fn scan_for_token<'a>(stream: &'a mut InputStream) -> Token {
    skip_whitespace(stream);
    match_non_whitespace_token(stream)
}

pub fn skip_whitespace(stream: &mut InputStream) {
    while !stream.is_end() && stream.peek() == b' '{
        stream.skip();
    }
}

pub fn match_non_whitespace_token<'a>(stream: &'a mut InputStream) -> Token {

    if stream.is_end() {
        return Token {
            token_type: TokenType::EOF,
            lexeme: vec![b' '],
        }
    }

    let current_char = stream.next();

    match match_single_character_token(current_char, stream) {
        Some(x) => return x,
        None => ()
    };

    match match_identifier(current_char, stream) {
        Some(x) => return x,
        None => ()
    };

    panic!()
}

fn match_single_character_token(character: u8, stream: &mut InputStream) -> Option<Token> {
    match single_character_token(character) {
        Some(x) => Some(Token {
            token_type: x,
            lexeme: stream.get().to_vec(),
        }),
        None => None,
    }
}

fn match_identifier(character: u8, stream: &mut InputStream) -> Option<Token> {

    if !is_alpha(character){
        return None;
    }

    while !stream.is_end() && is_alpha_or_underscore(stream.peek()) {
        stream.next();
    }

    let lexeme = stream.get();

    let token_type = match keyword(lexeme) {
        Some(x) => x,
        None => TokenType::Identifier,
    };

    Some(Token {
        token_type,
        lexeme: lexeme.to_vec(),
    })
}