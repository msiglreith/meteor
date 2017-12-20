#![feature(proc_macro)]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate synom;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate rustfmt_nightly as rustfmt;

use proc_macro::TokenStream;
use proc_macro2::Term;
use syn::*;
use quote::Tokens;
use quote::ToTokens;
use std::str;

#[proc_macro_attribute]
pub fn staged(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let mut item = match syn::parse_str::<Item>(&input) {
        Ok(item) => item,
        Err(err) => panic!("{:?}", err),
    };

    let (mut item, block) = match item {
        Item::Fn(fn_item) => {
            if let Unsafety::Unsafe(_) = fn_item.unsafety {
                panic!("Function can't be unsafe.");
            }
            if let Constness::Const(_) = fn_item.constness {
                panic!("Function can't be const.");
            }
            if fn_item.abi.is_some() {
                panic!("Invalid function abi.");
            }
            if !fn_item.decl.generics.params.is_empty() {
                panic!("Generic types are not supported.");
            }

            let block = virtualize(&fn_item.block);
            (fn_item, block)
        }
        _ => panic!("Unexpected item kind, expected function: {:?}", item),
    };

    item.vis = syn::Visibility::Inherited(syn::VisInherited { });

    let mut outer_attrs = item.attrs.clone();

    outer_attrs.push(syn::Attribute {
        style: syn::AttrStyle::Outer,
        pound_token: syn::tokens::Pound([Span::default()]),
        bracket_token: syn::tokens::Bracket(Span::default()),
        path: syn::Path {
           leading_colon: None,
           segments: vec![
                syn::PathSegment {
                   ident: syn::Ident::new(
                       Term::intern("proc_macro"),
                       Span::default(),
                   ),
                   parameters: syn::PathParameters::None,
               }
           ].into(),
        },
        tts: vec![],
        is_sugared_doc: false,
    });

    let ident = item.ident.clone();
    let outer_ident = item.ident.clone();
    let gen_ident = syn::Ident::new(Term::intern(&(item.ident.as_ref().to_owned() + "_gen")), Span::default());

    let fn_inputs = &item.decl.inputs;
    let func = quote!{ | #fn_inputs | #block };

    // println!("{:?}", func.to_string());

    let outer_item = quote! {
        #(#outer_attrs)*
        pub fn #outer_ident (input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            let generator = #func;

            // let gen_ident = quote! { #gen_ident }.to_string();
            let gen_fn = generator();
            let input = input.to_string();
            let fn_impl = format!("
                {{
                    || {{
                        {}
                    }}
                }}
            ", gen_fn.to_string());
            // panic!("{:?}", fn_impl.to_string());
            fn_impl.to_string().parse().unwrap()
        }
    };

    println!("Generated: {}", formatting(&outer_item.to_string()));
    outer_item.to_string().parse().unwrap()
}

fn formatting(input: &str) -> String {
    let string = input.to_string();
    let config = rustfmt::config::Config::default();
    let res = rustfmt::format_input::<Vec<u8>>(
        rustfmt::Input::Text(string),
        &config,
        None,
    );

    match res {
        Ok((summary, file_map, report)) => {
            file_map[0].1.to_string()
        }
        Err(err) => {
            println!("{:?}", err);
            "".into()
        }
    }
}

fn virtualize(block: &syn::Block) -> Tokens {
    let mut tokens = Tokens::new();
    block.virtualize(&mut tokens);

    let virtualized = quote! {
        {
        let mut __tokens = ::quote::Tokens::new();
        #tokens
        __tokens
        }
    };

    let wrapper_tokens = quote! {
        fn foo() {
        let mut __tokens = ::quote::Tokens::new();
        #tokens
        __tokens
        }
    };

    println!("virtualized: {}", formatting(&wrapper_tokens.to_string()));
    virtualized
}
