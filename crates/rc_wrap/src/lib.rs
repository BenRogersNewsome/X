extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, Ident, ItemStruct, Visibility, parse::Parse, Attribute};

struct RcWrapAttributes {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
}

impl Parse for RcWrapAttributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {

        let attrs = input.call(Attribute::parse_outer)?;
        let vis = Visibility::parse(input)?;
        let ident = Ident::parse(input)?;

        Ok(Self {
            attrs,
            vis,
            ident,
        })
    }
}

fn to_snake_case(input: String) -> String {
    let mut result: String = String::new();
    let mut chars = input.chars();

    match chars.next() {
        Some(c) => result.push_str(&c.to_lowercase().to_string()),
        None => { return result },
    };

    while let Some(c) = chars.next() {
        if c.is_uppercase() {
            result.push('_');
            result.push_str(&c.to_lowercase().to_string());
        }else{
            result.push(c)
        };
    };

    result
}

#[proc_macro_attribute]
pub fn rc_wrap(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast: ItemStruct = parse_macro_input!(item as ItemStruct);
    let name: Ident = (&ast.ident).clone();

    let parsed_attr: RcWrapAttributes = parse_macro_input!(attr as RcWrapAttributes);
    let wrapped_name = parsed_attr.ident;
    let wrapped_vis = parsed_attr.vis;
    let wrapped_attrs = parsed_attr.attrs;

    let new_macro_name = Ident::new(&format!("new_{}", to_snake_case(wrapped_name.to_string())), name.span());
    
    quote! { 
        #ast

        #(#wrapped_attrs)*
        #[derive(Clone)]
        #wrapped_vis struct #wrapped_name {
            _raw: std::rc::Rc<#name>,
        }

        impl std::ops::Deref for #wrapped_name {
            type Target = #name;

            fn deref(&self) -> &Self::Target {
                &self._raw.deref()
            }
        }

        ///
        /// ```
        ///     let set = new_set!()
        /// ```
        macro_rules! #new_macro_name {
            ( $($arg:expr),* ) => {
                #wrapped_name {
                    _raw: std::rc::Rc::new(#name::new($($arg),*)),
                }
            };
        }

    }.into()
}

