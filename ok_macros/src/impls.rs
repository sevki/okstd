// impls.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::Token;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

struct ImplsArgs {
    traits: Punctuated<syn::Path, Token![,]>,
}

impl Parse for ImplsArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(ImplsArgs {
            traits: input.parse_terminated(syn::Path::parse, Token![,])?,
        })
    }
}

pub fn impls(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as ImplsArgs);
    let input = parse_macro_input!(input as DeriveInput);

    let Data::Enum(data_enum) = input.data.clone() else {
        return quote! {
            compile_error!("impls can only be used on enums");
        }
        .into();
    };

    let trait_checks = args.traits.iter().collect::<Vec<_>>();

    let checks = data_enum.variants.iter().flat_map(|variant| {
        let variant_name = &variant.ident;
        match &variant.fields {
            Fields::Unnamed(fields) => {
                fields.unnamed.iter().flat_map(|field| {
                    let ty = &field.ty;
                    let trait_bounds = trait_checks.iter().map(|trait_path| {
                        quote! {
                            const _: () = {
                                trait AssertImpl where #ty: #trait_path {}
                            };
                        }
                    });
                    trait_bounds.collect::<Vec<_>>()
                }).collect::<Vec<_>>()
            },
            _ => {
                vec![quote! {
                    compile_error!(concat!("Variant ", stringify!(#variant_name), " must be a tuple variant"));
                }]
            }
        }
    });

    let expanded = quote! {
        #input

        #(#checks)*
    };

    expanded.into()
}
