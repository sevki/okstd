use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, ItemFn};

pub fn transform_function(
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
    let asyncness: &Option<syn::token::Async> = &item_fn.sig.asyncness;
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
    let mut olf_fnc: ItemFn = parse_macro_input!(old_fn as ItemFn);
    olf_fnc.sig.ident = old_ident.clone();

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
      #asyncness fn #fn_name #generics(#inputs) #output 
      #where_clause {
        setup_logging(#level_token);
        return #body
    }

    };

    TokenStream::from(result)
}


/// setup panic hook for crashdu.mp
/// similar to log, we just want to setup the panic hook
/// and keep the function as is
/// 
/// # Example
/// 
/// ```rust,notest
/// #[okstd::crashdump]
/// fn does_something() {
///    // do something
/// }
///```
/// to 
/// ```rust,notest
/// fn does_something() {
///   setup_panic_hook();
///  // do something
/// }
/// ```
pub fn setup_panic_hook(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    

    let mut item_fn = parse_macro_input!(input as ItemFn);

    let fn_name = &item_fn.clone().sig.ident;

    let attrs: &Vec<syn::Attribute> = &item_fn.attrs;
    let asyncness: &Option<syn::token::Async> = &item_fn.sig.asyncness;
    let generics: &syn::Generics = &item_fn.sig.generics;
    let inputs: &Punctuated<syn::FnArg, syn::token::Comma> = &item_fn.sig.inputs;
    
    let output: &syn::ReturnType = &item_fn.sig.output;
    let where_clause: &Option<syn::WhereClause> = &item_fn.sig.generics.where_clause;

    let orig_ident = item_fn.sig.ident.clone();
    // Rename the `test` function to `old_test`
    let new_name = format!("__panic_hook_{}", orig_ident);
    let old_ident = syn::Ident::new(new_name.as_str(), item_fn.sig.ident.span());
    item_fn.sig.ident = old_ident.clone();

    let body = &item_fn.block;

    let result = quote! {
      #[allow(unused_must_use)]
      #( #attrs )*
      #asyncness fn #fn_name #generics(#inputs) #output 
      #where_clause {
        std::panic::set_hook(Box::new(panic_hook));
        return #body
    }

    };

    TokenStream::from(result)
}