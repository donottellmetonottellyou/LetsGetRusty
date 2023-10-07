use darling::{ast::NestedMeta, FromMeta};
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, parse_quote, FnArg, Ident, ItemFn, Pat, Stmt};

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

    if args.verbose {
        let fn_args = extract_arg_names(input);
        let statements = generate_verbose_log(fn_name, fn_args);
        input.block.stmts.splice(0..0, statements);
    } else {
        input.block.stmts.insert(
            0,
            parse_quote! {
                println!("[Info] calling {}", stringify!(#fn_name));
            },
        );
    }

    input.into_token_stream().into()
}

fn extract_arg_names(func: &ItemFn) -> Vec<&Ident> {
    func.sig
        .inputs
        .iter()
        .filter_map(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                if let Pat::Ident(pat) = &(*pat_type.pat) {
                    return Some(&pat.ident);
                }
            }

            None
        })
        .collect()
}

fn generate_verbose_log(fn_name: &Ident, fn_args: Vec<&Ident>) -> Vec<Stmt> {
    let mut statements = vec![parse_quote! {
        print!("[Info] calling {} | ", stringify!(#fn_name));
    }];

    for arg in fn_args {
        statements.push(parse_quote! {
            print!("{} = {:?} ", stringify!(#arg), #arg);
        });
    }

    statements.push(parse_quote! {
        println!();
    });

    statements
}
