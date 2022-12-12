use input::InputStream;

use lexical_analysis::scanner::get_tokens;
use syntactic_analysis::Ast;

mod core;
mod lang;
mod input;
mod lexical_analysis;
mod syntactic_analysis;
mod scope;

pub use scope::{Scope, ScopedItem};

pub fn run<'a>(source_code: &[u8]) -> Scope {
    let input = InputStream::new(source_code);
    let tokens = get_tokens(input);

    let ast = Ast::new(&mut tokens.peekable()).unwrap();

    let mut scope = Scope::new();

    ast.visit(&mut scope).unwrap();

    scope
}