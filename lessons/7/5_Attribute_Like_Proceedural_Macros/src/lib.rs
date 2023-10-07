#![allow(unused)]

use darling::{ast::NestedMeta, FromMeta};
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, parse_quote, ItemFn};

#[derive(FromMeta)]
struct MacroArgs {
    #[darling(default)]
    verbose: bool,
}

#[proc_macro_attribute]
pub fn log_call(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(darling::Error::from(e).write_errors()),
    };
    let mut input = parse_macro_input!(input as ItemFn);

    let args = match MacroArgs::from_list(&args) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    impl_log_call(&args, &mut input)
}

fn impl_log_call(args: &MacroArgs, input: &mut ItemFn) -> TokenStream {
    let fn_name = &input.sig.ident;

    input.block.stmts.insert(
        0,
        parse_quote! {
            println!("[Info] calling {}", stringify!(#fn_name));
        },
    );

    input.into_token_stream().into()
}
