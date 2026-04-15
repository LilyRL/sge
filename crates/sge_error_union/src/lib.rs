use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as Ts2};
use quote::quote;
use syn::{
    Data, DeriveInput, Fields, Ident, ItemStruct, LitStr, PatType, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
};

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
pub fn derive_sge_error_union(tokens: TokenStream) -> TokenStream {
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

struct WrapperArgs {
    new_args: Option<Punctuated<PatType, Token![,]>>,
}

impl Parse for WrapperArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(WrapperArgs { new_args: None });
        }
        let name: Ident = input.parse()?;
        if name != "new" {
            return Err(syn::Error::new(name.span().into(), "expected `new(...)`"));
        }
        let content;
        syn::parenthesized!(content in input);
        let args = Punctuated::<PatType, Token![,]>::parse_terminated(&content)?;
        Ok(WrapperArgs {
            new_args: Some(args),
        })
    }
}

#[proc_macro_attribute]
pub fn wrapper(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as WrapperArgs);
    let input = parse_macro_input!(item as ItemStruct);

    let struct_name = &input.ident;

    let inner_type = match &input.fields {
        Fields::Unnamed(fields) if fields.unnamed.len() == 1 => &fields.unnamed.first().unwrap().ty,
        _ => {
            return syn::Error::new_spanned(
                &input,
                "wrapper only works on newtype structs of the form: struct Foo(Bar);",
            )
            .to_compile_error()
            .into();
        }
    };

    let new_method = args.new_args.map(|params| {
        let param_names = params.iter().map(|pt| &pt.pat);
        quote! {
            impl #struct_name {
                pub fn new(#params) -> Self {
                    #struct_name(<#inner_type>::new(#(#param_names),*))
                }
            }
        }
    });

    let expanded = quote! {
        #input

        #new_method

        impl std::ops::Deref for #struct_name {
            type Target = #inner_type;
            fn deref(&self) -> &Self::Target { &self.0 }
        }

        impl std::ops::DerefMut for #struct_name {
            fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
        }

        impl From<#inner_type> for #struct_name {
            fn from(val: #inner_type) -> Self { #struct_name(val) }
        }

        impl From<#struct_name> for #inner_type {
            fn from(val: #struct_name) -> Self { val.0 }
        }
    };

    expanded.into()
}
