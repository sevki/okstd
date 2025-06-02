#![recursion_limit = "128"]

use {
    proc_macro::TokenStream,
    quote::quote,
    syn::{parse_macro_input, ItemFn, Pat}, // Added Pat here
};

mod func_transformer;
mod impls;

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut user_main_fn = parse_macro_input!(item as ItemFn);

    // Check if the function is async
    if user_main_fn.sig.asyncness.is_none() {
        panic!("The `main` function must be async");
    }

    // Preserve original signature details
    let original_inputs = &user_main_fn.sig.inputs;
    let original_output = &user_main_fn.sig.output;
    let original_generics = &user_main_fn.sig.generics;
    let original_where_clause = &user_main_fn.sig.generics.where_clause;

    // Create argument list for calling the old function
    let mut call_args = Vec::new();
    for arg in original_inputs.iter() {
        if let syn::FnArg::Typed(pat_type) = arg {
            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                call_args.push(&pat_ident.ident);
            } else {
                // This case might need more robust handling for complex patterns,
                // but for typical main functions, ident patterns are common.
                panic!("Unsupported argument pattern in main function. Only simple identifiers are supported.");
            }
        } else {
            // self arguments are not typical for a main function.
            panic!("Unsupported 'self' argument in main function.");
        }
    }

    // Rename the original main function
    let old_main_ident = syn::Ident::new("old_main", user_main_fn.sig.ident.span());
    user_main_fn.sig.ident = old_main_ident.clone(); // user_main_fn is now the "old_main"

    // Generate the new main function wrapper
    // The new main function should be synchronous, and its signature
    // (inputs, output, generics, where_clause) should match the original async function.
    let new_main_fn = quote! {
        fn main #original_generics (#original_inputs) #original_output #original_where_clause {
            // Using tokio::runtime::Runtime directly.
            // For more complex scenarios, tokio::runtime::Builder could be used:
            // let runtime = tokio::runtime::Builder::new_multi_thread()
            //     .worker_threads(num_cpus::get()) // Example: use num_cpus
            //     .enable_all()
            //     .build()
            //     .unwrap();
            let runtime = tokio::runtime::Runtime::new().unwrap();
            runtime.block_on(#old_main_ident(#(#call_args),*))
        }
    };

    // Combine the renamed original function and the new main wrapper
    let output = quote! {
        #user_main_fn // This is the original function, renamed to old_main and still async
        #new_main_fn  // This is the new synchronous main that calls old_main
    };

    output.into()
}

// test, just like main, but for tests.
#[proc_macro_attribute]
pub fn test(args: TokenStream, input: TokenStream) -> TokenStream {
    func_transformer::test(args, input)
}

#[proc_macro_attribute]
pub fn log(args: TokenStream, input: TokenStream) -> TokenStream {
    func_transformer::setup_logging(args, input)
}

#[proc_macro_attribute]
pub fn impls(args: TokenStream, input: TokenStream) -> TokenStream {
    impls::impls(args, input)
}
