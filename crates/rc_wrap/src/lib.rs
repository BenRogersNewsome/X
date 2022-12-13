use proc_macro::TokenStream;

mod rc_wrap;

#[proc_macro_attribute]
pub fn rc_wrap(attr: TokenStream, item: TokenStream) -> TokenStream {
    rc_wrap::rc_wrap(attr, item)
}