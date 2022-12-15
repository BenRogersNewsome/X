extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{self, parse_macro_input, Ident, ItemStruct, Visibility, parse::Parse, Attribute, Fields, FieldsNamed, punctuated::Punctuated, token::Comma, FieldsUnnamed, Type};

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

pub fn rc_wrap(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast: ItemStruct = parse_macro_input!(item as ItemStruct);
    let name: Ident = (&ast.ident).clone();

    let parsed_attr: RcWrapAttributes = parse_macro_input!(attr as RcWrapAttributes);
    let wrapped_name = parsed_attr.ident;
    let wrapped_vis = parsed_attr.vis;
    let wrapped_attrs = parsed_attr.attrs;

    let new_function: proc_macro2::TokenStream = construct_new_function(&name, &ast.fields);
    
    quote! { 
        #ast

        #(#wrapped_attrs)*
        #[derive(Clone)]
        #wrapped_vis struct #wrapped_name {
            _raw: std::rc::Rc<#name>,
        }

        impl #wrapped_name {
            #new_function
        }

        impl std::ops::Deref for #wrapped_name {
            type Target = #name;

            fn deref(&self) -> &Self::Target {
                &self._raw.deref()
            }
        }

    }.into()
}

fn construct_new_function(name: &Ident, fields: &Fields) -> proc_macro2::TokenStream {
    match fields {
        Fields::Named(named) => construct_new_function_from_named_fields(name, named),
        Fields::Unnamed(unnamed) => construct_new_function_from_unnamed_fields(name, unnamed),
        Fields::Unit => {
            todo!("Unit structs rc_wrap")
        },
    }
}

fn construct_new_function_from_named_fields(name: &Ident, fields: &FieldsNamed) -> proc_macro2::TokenStream {

    let new_function_args = &fields.named;

    let struct_fields_without_types: Punctuated<&Ident, Comma> =
        Punctuated::from_iter(
                new_function_args.iter().map(|field| field.ident.as_ref().unwrap())
        );

    quote!{
        pub fn new(#new_function_args) -> Self {
            Self {
                _raw: std::rc::Rc::new(#name {
                    #struct_fields_without_types
                })
            }
        }
    }
}

fn construct_new_function_from_unnamed_fields(name: &Ident, fields: &FieldsUnnamed) -> proc_macro2::TokenStream {

    let new_function_args: Vec<Ident> = fields.unnamed
        .iter()
        .enumerate()
        .map(|(i, _)| {
            Ident::new(&format!("item_{}", i), Span::call_site())
        })
        .collect();

    let new_function_args_types: Vec<Type> = fields.unnamed
        .iter()
        .map(|field| {
            field.ty.clone()
        })
        .collect();

    quote!{
        pub fn new(#(#new_function_args: #new_function_args_types),*) -> Self {
            Self {
                _raw: std::rc::Rc::new(#name (
                    #(#new_function_args),*
                ))
            }
        }
    }
}