use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Ident, Token, parse::Parse, parse_macro_input};

struct RefTypeParams {
    ty: Ident,
    ty_ref: Ident,
    storage_name: Ident,
}

impl Parse for RefTypeParams {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ty = input.parse()?;
        let _ = input.parse::<Token![,]>();
        let ty_ref = input.parse()?;
        let _ = input.parse::<Token![,]>();
        let storage_name = input.parse()?;

        Ok(Self {
            ty,
            ty_ref,
            storage_name,
        })
    }
}

#[proc_macro]
pub fn gen_ref_type(input: TokenStream) -> TokenStream {
    let RefTypeParams {
        ty,
        ty_ref,
        storage_name,
    } = parse_macro_input!(input as RefTypeParams);

    let storage_const = Ident::new(
        &format!("{}_STORAGE", storage_name).to_uppercase(),
        Span::call_site(),
    );

    let alloc_const = Ident::new(
        &format!("{}_ALLOC", storage_name).to_uppercase(),
        Span::call_site(),
    );

    let init_name = Ident::new(
        &format!("init_{}_storage", storage_name.to_string().to_lowercase()),
        Span::call_site(),
    );

    let get_name = Ident::new(
        &format!("get_{}_state", storage_name.to_string().to_lowercase()),
        Span::call_site(),
    );

    let get_alloc_name = Ident::new(
        &format!("get_{}_alloc", storage_name.to_string().to_lowercase()),
        Span::call_site(),
    );

    quote! {
        #[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Debug)]
        pub struct #ty_ref(pub(crate) usize);

        #[allow(static_mut_refs)]
        static mut #storage_const: Option<Vec<#ty, &'static ::ref_type::Bump>> = None;

        #[allow(static_mut_refs)]
        static mut #alloc_const: Option<::ref_type::Bump> = None;

        #[allow(static_mut_refs)]
        fn #init_name() {
            unsafe {
                #alloc_const = Some(::ref_type::Bump::new());
                #storage_const = Some(Vec::new_in(&*#get_alloc_name()));
            }
        }

        fn #get_alloc_name() -> &'static mut ::ref_type::Bump {
            unsafe { #alloc_const.as_mut().unwrap_or_else(|| panic!()) }
        }

        #[allow(static_mut_refs)]
        pub fn #get_name() -> &'static mut Vec<#ty, &'static ::ref_type::Bump> {
            unsafe { #storage_const.as_mut().unwrap_or_else(|| panic!()) }
        }

        impl #ty_ref {
            pub fn get(&self) -> &'static #ty {
                &#get_name()[self.0]
            }

            pub fn get_mut(&self) -> &'static mut #ty {
                &mut #get_name()[self.0]
            }

            pub fn new() -> Self {
                let id = #get_name().len();
                Self(id)
            }

            pub fn replace(&self, new: #ty) {
                #get_name()[self.0] = new;
            }

            pub fn _id(&self) -> usize {
                self.0
            }
        }

        impl #ty {
            pub fn create(self) -> #ty_ref {
                let storage = #get_name();
                let id = storage.len();
                storage.push(self);

                #ty_ref(id)
            }
        }

        impl std::ops::Deref for #ty_ref {
            type Target = #ty;
            fn deref(&self) -> &Self::Target {
                &#get_name()[self.0]
            }
        }

        impl std::ops::DerefMut for #ty_ref {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut #get_name()[self.0]
            }
        }
    }
    .into()
}
