mod input;
mod lang;
mod lexical_analysis;
mod syntactic_analysis;
mod core;

use input::InputStream;
use lexical_analysis::token_stream::TokenStream;
use lexical_analysis::scanner::get_tokens;

use lang::syntax::NonTerminal;

use std::str;

use crate::core::Stream;
use crate::lang::tokens::TokenType;

fn main() {
    
    let source: &[u8] = "let v, u in ColumnVectors".as_bytes();

    println!("{:?}", str::from_utf8(source));

    let mut input_stream = InputStream::new(source);
    let tokens = get_tokens(&mut input_stream);
    let mut token_stream = TokenStream::new(&tokens);

    let mut root = lang::syntax::Root::new(&mut token_stream);

    for child in root.children {
        print!("{}", child.to_str(0))
    }
}