#![recursion_limit = "128"]
extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;

use syn::{parse_macro_input, ItemFn};

mod func_transformer;

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);

    // Check if the function is async
    if input.sig.asyncness.is_none() {
        panic!("The `main` function must be async");
    }

    // Rename the `main` function to `old_main`
    let old_main_ident = syn::Ident::new("old_main", input.sig.ident.span());
    input.sig.ident = old_main_ident.clone();

    // Generate the new `main` function
    let new_main = quote! {
        fn main() {
            let rt = Runtimes::setup_runtimes().unwrap();
            rt.block_on(#old_main_ident())
        }
    };

    // Combine the input and the new `main` function
    let output = quote! {
        #input
        #new_main
    };

    output.into()
}

// test, just like main, but for tests.
#[proc_macro_attribute]
pub fn test(args: TokenStream, input: TokenStream) -> TokenStream {
    func_transformer::test(args, input)
}

#[proc_macro_attribute]
pub fn crashdump(args: TokenStream, input: TokenStream) -> TokenStream {
    func_transformer::setup_panic_hook(args, input)
}

#[proc_macro_attribute]
pub fn log(args: TokenStream, input: TokenStream) -> TokenStream {
    func_transformer::setup_logging(args, input)
}
