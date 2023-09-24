use quote::quote;

use proc_macro::TokenStream;

#[proc_macro_derive(Log)]
pub fn log_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = ast.ident;

    quote! {
        impl Log for #name {
            fn info(&self, msg: &str) {
                println!("[Info] {}: {msg}", stringify!(#name));
            }

            fn warn(&self, msg: &str) {
                println!("[Warn] {}: {msg}", stringify!(#name));
            }

            fn error(&self, msg: &str) {
                println!("[Err] {}: {msg}", stringify!(#name));
            }
        }
    }
    .into()
}
