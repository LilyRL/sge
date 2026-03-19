use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as Ts2};
use quote::quote;
use syn::{Data, DeriveInput, Fields, LitStr, parse_macro_input};

/// Derives automatic `From<T>` implementations for each variant of a tuple enum.
///
/// # Example
/// ```
/// #[derive(Union)]
/// enum MyUnion {
///     String(String),
///     Number(i32),
///     Custom(MyCustomType),
/// }
/// ```
#[proc_macro_derive(Union)]
pub fn derive_union(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    let enum_name = &input.ident;

    let variants = match &input.data {
        Data::Enum(data_enum) => &data_enum.variants,
        _ => panic!("Union can only be derived for enums"),
    };

    let mut from_impls = quote! {};

    for variant in variants {
        let variant_name = &variant.ident;

        if let Fields::Unnamed(fields) = &variant.fields
            && fields.unnamed.len() == 1
        {
            let ty = &fields.unnamed[0].ty;

            from_impls = quote! {
                #from_impls
                impl From<#ty> for #enum_name {
                    fn from(value: #ty) -> Self {
                        Self::#variant_name(value)
                    }
                }
            };
        }
    }

    from_impls.into()
}

/// Derives `From<T>`, `Display`, and `Error` trait implementations for an error enum.
///
/// # Example
/// ```
/// #[derive(ErrorUnion)]
/// enum MyError {
///     Io(std::io::Error),
///     Parse(ParseError),
///     Custom(MyCustomError),
/// }
/// ```
#[proc_macro_derive(ErrorUnion)]
pub fn derive_error_union(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    let enum_name = &input.ident;

    let variants = match &input.data {
        Data::Enum(data_enum) => &data_enum.variants,
        _ => panic!("ErrorUnion can only be derived for enums"),
    };

    let mut from_impls = quote! {};
    let mut match_entries: Vec<Ts2> = vec![];

    for variant in variants {
        let variant_name = &variant.ident;

        if let Fields::Unnamed(fields) = &variant.fields
            && fields.unnamed.len() == 1
        {
            let ty = &fields.unnamed[0].ty;

            from_impls = quote! {
                #from_impls
                impl From<#ty> for #enum_name {
                    fn from(value: #ty) -> Self {
                        Self::#variant_name(value)
                    }
                }
            };

            let name_str = LitStr::new(&variant_name.to_string(), Span::call_site());
            match_entries.push(quote! {
                Self::#variant_name(e) => write!(f, "{}: {e}", #name_str)
            });
        }
    }

    quote! {
        #from_impls

        impl ::std::fmt::Display for #enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::result::Result<(), ::std::fmt::Error> {
                match self {
                    #(#match_entries),*
                }
            }
        }

        impl ::std::error::Error for #enum_name {}
    }
    .into()
}
