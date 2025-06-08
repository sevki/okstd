#![recursion_limit = "128"]

use proc_macro::TokenStream;

extern crate tokio;

mod func_transformer;
mod impls;

#[proc_macro_attribute]
pub fn log(args: TokenStream, input: TokenStream) -> TokenStream {
    func_transformer::setup_logging(args, input)
}

#[proc_macro_attribute]
pub fn impls(args: TokenStream, input: TokenStream) -> TokenStream {
    impls::impls(args, input)
}
