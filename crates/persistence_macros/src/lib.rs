use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_attribute]
pub fn persistent(_args: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let vis = &input.vis;
    let attrs = &input.attrs;
    let generics = &input.generics;

    let struct_body = match &input.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(fields) => {
                let fs = fields.named.iter().map(|f| {
                    let fname = &f.ident;
                    let fty = &f.ty;
                    let fattrs = &f.attrs;
                    quote! { #(#fattrs)* #fname: #fty }
                });
                quote! { { #(#fs,)* } }
            }
            Fields::Unnamed(fields) => {
                let fs = fields.unnamed.iter().map(|f| {
                    let fty = &f.ty;
                    let fattrs = &f.attrs;
                    quote! { #(#fattrs)* #fty }
                });
                quote! { ( #(#fs,)* ); }
            }
            Fields::Unit => quote! { ; },
        },
        _ => {
            return syn::Error::new_spanned(name, "#[persistent] only supports structs")
                .to_compile_error()
                .into();
        }
    };

    let expanded = quote! {
        #(#attrs)*
        #[derive(::persistence::rkyv::Archive, ::persistence::rkyv::Serialize, ::persistence::rkyv::Deserialize)]
        #vis struct #name #generics #struct_body

        impl #name #generics {
            pub fn save(&self, path: impl AsRef<std::path::Path>) -> ::persistence::Result<()> {
                use ::persistence::rkyv::ser::Serializer as _;
                let bytes = ::persistence::rkyv::to_bytes::<::persistence::rkyv::rancor::Error>(self)?;
                std::fs::write(path, bytes)?;
                Ok(())
            }

            pub fn load(path: impl AsRef<std::path::Path>) -> ::persistence::Result<Self> {
                let bytes = std::fs::read(path)?;
                let archived = ::persistence::rkyv::access::<<Self as ::persistence::rkyv::Archive>::Archived, ::persistence::rkyv::rancor::Error>(&bytes[..])?;
                Ok(::persistence::rkyv::deserialize::<Self, ::persistence::rkyv::rancor::Error>(archived).unwrap())
            }
        }
    };

    TokenStream::from(expanded)
}
