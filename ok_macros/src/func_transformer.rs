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
        Level::Off => quote! {LevelFilter::OFF},
        Level::Error => quote! {LevelFilter::ERROR},
        Level::Warning => quote! {LevelFilter::WARN},
        Level::Info => quote! {LevelFilter::INFO},
        Level::Debug => quote! {LevelFilter::DEBUG},
        Level::Trace => quote! {LevelFilter::TRACE},
    };

    let body = &item_fn.block;

    let result = quote! {
      #[allow(unused_must_use, unreachable_code)]
      #( #attrs )*
      #is_async fn #fn_name #generics(#inputs) #output
      #where_clause {
        use okstd::oklog::setup_logging;
        use tracing::level_filters::LevelFilter;
        setup_logging(#level_token);
        return #body
    }

    };

    TokenStream::from(result)
}
