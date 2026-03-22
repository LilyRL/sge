use std::fs;

use heck::{ToShoutySnakeCase, ToSnakeCase};
use image::ImageFormat;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as Ts2};
use quote::quote;
use syn::parse::Parse;
use syn::{Expr, Ident, LitStr, Token, Visibility, parse_macro_input};

mod shape_variants;

#[proc_macro]
pub fn draw_shape_variants(input: TokenStream) -> TokenStream {
    shape_variants::expand(input.into())
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

struct Actions {
    actions: Vec<Action>,
}

impl Parse for Actions {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut actions: Vec<Action> = vec![];

        while !input.is_empty() {
            if let Some(action) = try_parse_action(input) {
                actions.push(action);
                let _ = input.parse::<Token![,]>();
            } else {
                break;
            }
        }

        Ok(Self { actions })
    }
}

impl Actions {
    fn to_tokens(&self) -> Ts2 {
        let mut ts = Ts2::new();

        for (i, action) in self.actions.iter().enumerate() {
            let action = action.to_tokens(i as u32);
            ts = quote! { #ts #action };
        }

        ts
    }
}

impl Action {
    fn to_tokens(&self, n: u32) -> Ts2 {
        let Action { name, visibility } = self;
        quote! { #visibility const #name: ::sge::prelude::Action = ::sge::prelude::Action::new(#n); }
    }
}

fn try_parse_action(input: syn::parse::ParseStream) -> Option<Action> {
    let visibility;
    if let Ok(v) = input.parse::<Visibility>() {
        visibility = v;
    } else {
        visibility = Visibility::Inherited;
    }

    let name: Ident = input.parse().ok()?;
    Some(Action { name, visibility })
}

struct Action {
    name: Ident,
    visibility: Visibility,
}

#[proc_macro]
pub fn actions(input: TokenStream) -> TokenStream {
    let actions = parse_macro_input!(input as Actions);
    actions.to_tokens().into()
}

struct Binds {
    binds: Vec<(Expr, Expr)>,
}

impl Parse for Binds {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut binds = vec![];

        while let Ok(name) = input.parse() {
            input.parse::<Token![=>]>()?;
            if let Ok(value) = input.parse() {
                input.parse::<Token![;]>()?;
                binds.push((name, value))
            } else {
                break;
            }
        }

        Ok(Self { binds })
    }
}

#[proc_macro]
pub fn bind(input: TokenStream) -> TokenStream {
    let binds = parse_macro_input!(input as Binds);

    let mut tokens = quote! {};

    for bind in binds.binds {
        let (name, value) = bind;
        tokens = quote! {
            #tokens

            ::sge::prelude::bind_button(#name, #value.into());
        };
    }

    tokens.into()
}

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

    let init_name = Ident::new(
        &format!("init_{}_storage", storage_name.to_string().to_lowercase()),
        Span::call_site(),
    );
    let get_name = Ident::new(
        &format!("get_{}_state", storage_name.to_string().to_lowercase()),
        Span::call_site(),
    );

    quote! {
        #[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Debug)]
        pub struct #ty_ref(pub(crate) usize);

        #[allow(static_mut_refs)]
        static mut #storage_const: Option<Vec<#ty>> = None;

        #[allow(static_mut_refs)]
        fn #init_name() {
            unsafe {
                #storage_const = Some(vec![]);
            }
        }

        #[allow(static_mut_refs)]
        pub fn #get_name() -> &'static mut Vec<#ty> {
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

fn ext_to_format(ext: &str) -> Option<ImageFormat> {
    match ext {
        "png" => Some(ImageFormat::Png),
        "jpg" | "jpeg" => Some(ImageFormat::Jpeg),
        "gif" => Some(ImageFormat::Gif),
        "webp" => Some(ImageFormat::WebP),
        "pnm" | "pbm" | "pgm" | "ppm" | "pam" => Some(ImageFormat::Pnm),

        "tiff" | "tif" => Some(ImageFormat::Tiff),
        "tga" => Some(ImageFormat::Tga),
        "dds" => Some(ImageFormat::Dds),
        "bmp" => Some(ImageFormat::Bmp),
        "ico" => Some(ImageFormat::Ico),
        "hdr" => Some(ImageFormat::Hdr),
        "exr" => Some(ImageFormat::OpenExr),
        "ff" | "farbfeld" => Some(ImageFormat::Farbfeld),
        "avif" => Some(ImageFormat::Avif),
        "qoi" => Some(ImageFormat::Qoi),
        _ => None,
    }
}

fn format_to_tokens(format: ImageFormat) -> Ts2 {
    match format {
        ImageFormat::Avif => quote! { ImageFormat::Avif },
        ImageFormat::Bmp => quote! { ImageFormat::Bmp },
        ImageFormat::Dds => quote! { ImageFormat::Dds },
        ImageFormat::Farbfeld => quote! { ImageFormat::Farbfeld },
        ImageFormat::Gif => quote! { ImageFormat::Gif },
        ImageFormat::Hdr => quote! { ImageFormat::Hdr },
        ImageFormat::Ico => quote! { ImageFormat::Ico },
        ImageFormat::Jpeg => quote! { ImageFormat::Jpeg },
        ImageFormat::OpenExr => quote! { ImageFormat::OpenExr },
        ImageFormat::Pnm => quote! { ImageFormat::Pnm },
        ImageFormat::Png => quote! { ImageFormat::Png },
        ImageFormat::Qoi => quote! { ImageFormat::Qoi },
        ImageFormat::Tga => quote! { ImageFormat::Tga },
        ImageFormat::Tiff => quote! { ImageFormat::Tiff },
        ImageFormat::WebP => quote! { ImageFormat::WebP },
        _ => panic!(),
    }
}

#[proc_macro]
pub fn include_texture(input: TokenStream) -> TokenStream {
    let path = parse_macro_input!(input as LitStr);
    let path_value = path.value();

    let format = match path_value.rsplit('.').next() {
        Some(ext) => match ext_to_format(ext) {
            Some(format) => format_to_tokens(format),
            None => {
                return syn::Error::new(
                    path.span(),
                    format!(
                        "Unsupported or unknown image format for file: {}",
                        path_value
                    ),
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new(
                path.span(),
                format!(
                    "Unsupported or unknown image format for file: {}",
                    path_value
                ),
            )
            .to_compile_error()
            .into();
        }
    };

    let expanded = quote! {
        load_texture(include_bytes!(#path), #format).unwrap()
    };

    TokenStream::from(expanded)
}

struct SpritesheetParams {
    var_name: Ident,
    path: String,
}

impl Parse for SpritesheetParams {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let var_name = input.parse()?;
        input.parse::<Token![,]>()?;
        let path: LitStr = input.parse()?;

        Ok(Self {
            path: path.value(),
            var_name,
        })
    }
}

struct ImageData {
    bytes: Vec<u8>,
    format: ImageFormat,
    name: String,
}

#[proc_macro]
pub fn include_spritesheet(input: TokenStream) -> TokenStream {
    let params = parse_macro_input!(input as SpritesheetParams);
    let var_name = params.var_name;

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");

    let full_path = std::path::Path::new(&manifest_dir).join(&params.path);

    let dir = match fs::read_dir(&full_path) {
        Ok(d) => d,
        Err(e) => {
            return syn::Error::new(
                Span::call_site(),
                format!("Failed to read directory '{}': {}", full_path.display(), e),
            )
            .to_compile_error()
            .into();
        }
    };

    let mut images: Vec<ImageData> = vec![];

    for entry in dir {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        if let Some(ext) = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_string())
            && let Some(format) = ext_to_format(&ext)
        {
            let name = match path.file_name() {
                Some(n) => n.to_string_lossy().to_string(),
                None => continue,
            };

            let bytes = match fs::read(&path) {
                Ok(b) => b,
                Err(e) => {
                    return syn::Error::new(
                        Span::call_site(),
                        format!("Failed to read file '{}': {}", path.display(), e),
                    )
                    .to_compile_error()
                    .into();
                }
            };

            images.push(ImageData {
                bytes,
                format,
                name,
            });
        }
    }

    let mut const_entries = vec![];
    let mut add_entries = vec![];

    for (i, image) in images.iter().enumerate() {
        let const_name = image.name.to_shouty_snake_case();
        let const_ident = Ident::new(&const_name, Span::call_site());
        let image_name = format!("{}_image", image.name.to_snake_case());
        let image_ident = Ident::new(&image_name, Span::call_site());
        let bytes = &image.bytes;
        let format = image.format;
        let format_tokens = format_to_tokens(format);

        const_entries.push(quote! {
            let #image_ident = sge::prelude::load_image(&[#(#bytes),*], #format_tokens).unwrap();
            let #const_ident: sge::prelude::SpriteKey = sge::prelude::SpriteKey(#i);
        });
        add_entries.push(quote! { spritesheet.cache_sprite_with_key(#const_ident, #image_ident); })
    }

    let len = add_entries.len();

    quote! {
        {
            use sge::prelude::*;

            #(#const_entries)*

            let #var_name = {
                let spritesheet = create_spritesheet().unwrap();

                #(#add_entries)*

                spritesheet.set_next_key(#len);

                spritesheet
            };

            #var_name
        }
    }
    .into()
}
