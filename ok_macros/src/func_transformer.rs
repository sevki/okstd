use {
    proc_macro::TokenStream,
    quote::quote,
    syn::{parse_macro_input, punctuated::Punctuated, ItemFn},
};

pub fn setup_logging(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let old_fn = input.clone();

    let mut level: Level = Level::Off;

    args.into_iter().for_each(|arg| {
        let arg = arg.to_string();
        match arg.as_str() {
            "debug" => level = Level::Debug,
            "info" => level = Level::Info,
            "critical" => level = Level::Critical,
            "warn" => level = Level::Warning,
            "error" => level = Level::Error,
            "trace" => level = Level::Trace,
            "off" => level = Level::Off,
            _ => panic!("Unknown log level: {}", arg),
        }
    });

    let mut item_fn = parse_macro_input!(input as ItemFn);

    let fn_name = &item_fn.clone().sig.ident;

    let attrs: &Vec<syn::Attribute> = &item_fn.attrs;
    let is_async: &Option<syn::token::Async> = &item_fn.sig.asyncness;
    let generics: &syn::Generics = &item_fn.sig.generics;
    let inputs: &Punctuated<syn::FnArg, syn::token::Comma> = &item_fn.sig.inputs;

    let output: &syn::ReturnType = &item_fn.sig.output;
    let where_clause: &Option<syn::WhereClause> = &item_fn.sig.generics.where_clause;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Level {
        /// Off
        Off,
        /// Critical
        Critical,
        /// Error
        Error,
        /// Warning
        Warning,
        /// Info
        Info,
        /// Debug
        Debug,
        /// Trace
        Trace,
    }

    let orig_ident = item_fn.sig.ident.clone();
    // Rename the `test` function to `old_test`
    let new_name = format!("__logging_{}", orig_ident);
    let old_ident = syn::Ident::new(new_name.as_str(), item_fn.sig.ident.span());
    item_fn.sig.ident = old_ident.clone();

    if level == Level::Off {
        return old_fn;
    }
    let mut old_func: ItemFn = parse_macro_input!(old_fn as ItemFn);
    old_func.sig.ident = old_ident.clone();

    let level_token = match level {
        Level::Off => quote! {},
        Level::Critical => quote! {LevelFilter::Critical},
        Level::Error => quote! {LevelFilter::Error},
        Level::Warning => quote! {LevelFilter::Warning},
        Level::Info => quote! {LevelFilter::Info},
        Level::Debug => quote! {LevelFilter::Debug},
        Level::Trace => quote! {LevelFilter::Trace},
    };

    let body = &item_fn.block;

    let result = quote! {
      #[allow(unused_must_use, unreachable_code)]
      #( #attrs )*
      #is_async fn #fn_name #generics(#inputs) #output
      #where_clause {
        setup_logging(#level_token);
        return #body
    }

    };

    TokenStream::from(result)
}

pub fn test(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item_fn = parse_macro_input!(input as ItemFn); // No 'mut' needed if not modifying item_fn directly before quote

    let fn_name = &item_fn.sig.ident; // Use item_fn directly

    // let _attrs: &Vec<syn::Attribute> = &item_fn.attrs; // Not used in the final quote for test
    let is_async: &Option<syn::token::Async> = &item_fn.sig.asyncness;
    let generics: &syn::Generics = &item_fn.sig.generics;
    let inputs: &Punctuated<syn::FnArg, syn::token::Comma> = &item_fn.sig.inputs;

    let output: &syn::ReturnType = &item_fn.sig.output;
    let where_clause: &Option<syn::WhereClause> = &item_fn.sig.generics.where_clause;

    // No renaming needed for the test function itself, the body is wrapped.
    // let orig_ident = item_fn.sig.ident.clone();
    // let new_name = format!("__test_{}", orig_ident);
    // let old_ident = syn::Ident::new(new_name.as_str(), item_fn.sig.ident.span());
    // item_fn.sig.ident = old_ident.clone(); // This would rename the function in `item_fn`

    let body = &item_fn.block;

    if is_async.is_none() {
        // For non-async tests, keep original attributes and signature, just inject the body.
        // The #[test] attribute is part of the output.
        let result = quote! {
          #[allow(unused_must_use)]
          #[test]
          fn #fn_name #generics(#inputs) #output
          #where_clause {
            #body
          }
        };
        return TokenStream::from(result);
    }

    // For async tests, wrap the body in a tokio runtime.
    // The #[test] attribute is part of the output.
    let result = quote! {
      #[allow(unused_must_use)]
      #[test]
      fn #fn_name #generics(#inputs) #output
      #where_clause {
        // Ensure tokio is a dependency in Cargo.toml of the crate using this macro.
        tokio::runtime::Runtime::new().unwrap().block_on(async #body)
      }
    };

    TokenStream::from(result)
}
