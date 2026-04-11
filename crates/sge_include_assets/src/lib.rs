use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro2::TokenStream as Ts2;
use quote::quote;
use rand::Rng;
use rand::distr::Alphabetic;
use std::fs;
use std::path::Path;
use syn::Ident;
use syn::parse::Parse;
use syn::parse_macro_input;

struct Params {
    path: String,
    name: Ident,
}

impl Parse for Params {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let path = input.parse::<syn::LitStr>()?.value();
        input.parse::<syn::Token![,]>()?;
        let name = input.parse()?;
        Ok(Self { path, name })
    }
}

#[proc_macro]
pub fn sge_include_assets(input: TokenStream) -> TokenStream {
    let params = parse_macro_input!(input as Params);
    let static_name = params.name;

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let root_path = Path::new(&manifest_dir).join(&params.path);

    let tree = get_all_files(root_path.to_string_lossy().into_owned());

    let mut decl = Ts2::new();
    let root_struct_name = random_struct_name().to_string();
    let (root_type, root_value) = tree.into_tokens(&mut decl, &root_struct_name);

    quote! {
        #decl
        pub static #static_name: #root_type = #root_value;
    }
    .into()
}

enum Tree {
    Leaf { name: String, data: Data },
    Branch { name: String, children: Vec<Tree> },
}

enum Data {
    Text(String),
    Blob(Vec<u8>),
    Json(serde_json::Value),
    Toml(toml::Table),
    Ron(ron::Value),
}

fn get_all_files(path: String) -> Tree {
    build_tree(Path::new(&path))
}

fn build_tree(path: &Path) -> Tree {
    let name = path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();

    if path.is_file() {
        let data = parse_file(
            path,
            fs::read(path).unwrap_or_else(|_| panic!("Failed to read file: {:?}", path)),
        );
        Tree::Leaf { name, data }
    } else if path.is_dir() {
        let mut entries: Vec<_> = fs::read_dir(path)
            .expect("Failed to read directory")
            .filter_map(Result::ok)
            .collect();
        entries.sort_by_key(|e| e.file_name());
        let children = entries.iter().map(|e| build_tree(&e.path())).collect();
        Tree::Branch { name, children }
    } else {
        Tree::Branch {
            name,
            children: vec![],
        }
    }
}

fn parse_file(path: &Path, data: Vec<u8>) -> Data {
    match path.extension().and_then(|s| s.to_str()) {
        Some("json") => json(data, path),
        Some("ron") => ron(string(data), path),
        Some("toml") => toml(data, path),
        _ => match String::from_utf8(data.clone()) {
            Ok(s) => Data::Text(s),
            Err(_) => Data::Blob(data),
        },
    }
}

fn string(data: Vec<u8>) -> String {
    String::from_utf8(data).expect("Failed to parse file as UTF-8 string")
}

fn json(data: Vec<u8>, path: &Path) -> Data {
    Data::Json(
        serde_json::from_slice(&data)
            .unwrap_or_else(|e| panic!("Failed to parse json in {:?}: {}", path, e)),
    )
}

fn ron(data: String, path: &Path) -> Data {
    Data::Ron(
        ron::from_str(&data).unwrap_or_else(|e| panic!("Failed to parse ron in {:?}: {}", path, e)),
    )
}

fn toml(data: Vec<u8>, path: &Path) -> Data {
    Data::Toml(
        toml::from_slice(&data)
            .unwrap_or_else(|e| panic!("Failed to parse toml in {:?}: {}", path, e)),
    )
}

impl Tree {
    fn name(&self) -> &str {
        match self {
            Self::Leaf { name, .. } => name,
            Self::Branch { name, .. } => name,
        }
    }

    fn into_tokens(self, decl: &mut Ts2, struct_name: &str) -> (Ts2, Ts2) {
        match self {
            Self::Leaf { data, .. } => data.into_type_and_tokens(decl),
            Self::Branch { children, .. } => {
                let ident = syn::Ident::new(struct_name, Span::call_site());

                let (field_names, field_types, field_values): (Vec<_>, Vec<Ts2>, Vec<Ts2>) =
                    children
                        .into_iter()
                        .map(|child| {
                            let field_name = sanitize_ident(child.name());
                            let field_ident = syn::Ident::new(&field_name, Span::call_site());
                            let child_struct_name = random_struct_name().to_string();
                            let (ty, val) = child.into_tokens(decl, &child_struct_name);
                            (field_ident, ty, val)
                        })
                        .fold(
                            (vec![], vec![], vec![]),
                            |(mut names, mut types, mut vals), (n, t, v)| {
                                names.push(n);
                                types.push(t);
                                vals.push(v);
                                (names, types, vals)
                            },
                        );

                decl.extend(quote! {
                    #[derive(Debug)]
                    pub struct #ident {
                        #(pub #field_names: #field_types),*
                    }
                });

                (
                    quote! { #ident },
                    quote! { #ident { #(#field_names: #field_values),* } },
                )
            }
        }
    }
}

impl Data {
    fn into_type_and_tokens(self, decl: &mut Ts2) -> (Ts2, Ts2) {
        match self {
            Self::Blob(d) => {
                let len = d.len();
                (quote! { [u8; #len] }, quote! { [#(#d),*] })
            }
            Self::Text(s) => (quote! { &'static str }, quote! { #s }),
            Self::Json(json) => json.into_tokens(decl),
            Self::Toml(table) => toml::Value::Table(table).into_tokens(decl),
            Self::Ron(val) => val.into_tokens(decl),
        }
    }
}

fn random_struct_name() -> syn::Ident {
    let suffix: String = rand::rng()
        .sample_iter(&Alphabetic)
        .filter(|c| c.is_ascii_alphabetic())
        .take(8)
        .map(|c| c as char)
        .collect();
    syn::Ident::new(&format!("Asset_{}", suffix), Span::call_site())
}

trait ValueExt: Sized {
    fn into_tokens(self, decl: &mut Ts2) -> (Ts2, Ts2);
}

fn map_to_tokens<V: ValueExt>(
    entries: impl Iterator<Item = (String, V)>,
    decl: &mut Ts2,
) -> (Ts2, Ts2) {
    let struct_ident = random_struct_name();

    let (field_idents, types, values): (Vec<_>, Vec<Ts2>, Vec<Ts2>) = entries
        .map(|(name, val)| {
            let ident = syn::Ident::new(&sanitize_ident(&name), Span::call_site());
            let (ty, val) = val.into_tokens(decl);
            (ident, ty, val)
        })
        .fold(
            (vec![], vec![], vec![]),
            |(mut idents, mut types, mut vals), (i, t, v)| {
                idents.push(i);
                types.push(t);
                vals.push(v);
                (idents, types, vals)
            },
        );

    decl.extend(quote! {
        #[derive(Debug)]
        pub struct #struct_ident {
            #(pub #field_idents: #types),*
        }
    });

    (
        quote! { #struct_ident },
        quote! { #struct_ident { #(#field_idents: #values),* } },
    )
}

fn array_to_tokens<V: ValueExt>(arr: Vec<V>, decl: &mut Ts2) -> (Ts2, Ts2) {
    if arr.is_empty() {
        return (quote! { [(); 0] }, quote! { [] });
    }
    let len = arr.len();
    let (types, values): (Vec<Ts2>, Vec<Ts2>) =
        arr.into_iter().map(|v| v.into_tokens(decl)).unzip();
    let elem_type = &types[0];
    (quote! { [#elem_type; #len] }, quote! { [#(#values),*] })
}

fn sanitize_ident(s: &str) -> String {
    let base = Path::new(s)
        .file_stem()
        .map(|stem| stem.to_string_lossy().into_owned())
        .unwrap_or_else(|| s.to_owned());

    let sanitized: String = base
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();

    if sanitized.starts_with(|c: char| c.is_ascii_digit()) {
        format!("_{}", sanitized)
    } else {
        sanitized
    }
}

fn ron_key_to_ident(key: &ron::Value) -> String {
    match key {
        ron::Value::String(s) => sanitize_ident(s),
        ron::Value::Number(i) => format!("field_{}", i.into_f64() as usize),
        other => sanitize_ident(&format!("{:?}", other)),
    }
}

impl ValueExt for serde_json::Value {
    fn into_tokens(self, decl: &mut Ts2) -> (Ts2, Ts2) {
        match self {
            Self::Null => (quote! { ()           }, quote! { ()  }),
            Self::Bool(b) => (quote! { bool         }, quote! { #b  }),
            Self::String(s) => (quote! { &'static str }, quote! { #s  }),
            Self::Number(n) => {
                if let Some(i) = n.as_i64() {
                    (quote! { i64 }, quote! { #i })
                } else if let Some(u) = n.as_u64() {
                    (quote! { u64 }, quote! { #u })
                } else if let Some(f) = n.as_f64() {
                    (quote! { f64 }, quote! { #f })
                } else {
                    panic!("Unrepresentable JSON number: {}", n)
                }
            }
            Self::Array(arr) => array_to_tokens(arr, decl),
            Self::Object(map) => map_to_tokens(map.into_iter(), decl),
        }
    }
}

impl ValueExt for toml::Value {
    fn into_tokens(self, decl: &mut Ts2) -> (Ts2, Ts2) {
        match self {
            Self::Boolean(b) => (quote! { bool         }, quote! { #b }),
            Self::Integer(i) => (quote! { i64          }, quote! { #i }),
            Self::Float(f) => (quote! { f64          }, quote! { #f }),
            Self::String(s) => (quote! { &'static str }, quote! { #s }),
            Self::Datetime(dt) => {
                let s = dt.to_string();
                (quote! { &'static str }, quote! { #s })
            }
            Self::Array(arr) => array_to_tokens(arr, decl),
            Self::Table(tbl) => map_to_tokens(tbl.into_iter(), decl),
        }
    }
}

impl ValueExt for ron::Value {
    fn into_tokens(self, decl: &mut Ts2) -> (Ts2, Ts2) {
        match self {
            Self::Unit => (quote! { ()           }, quote! { ()   }),
            Self::Bool(b) => (quote! { bool         }, quote! { #b   }),
            Self::Char(c) => (quote! { char         }, quote! { #c   }),
            Self::String(s) => (quote! { &'static str }, quote! { #s   }),
            Self::Bytes(b) => {
                let len = b.len();
                (quote! { [u8; #len] }, quote! { [#(#b),*] })
            }
            Self::Option(None) => (quote! { Option<()> }, quote! { None }),
            Self::Option(Some(inner)) => {
                let (ty, val) = (*inner).into_tokens(decl);
                (quote! { Option<#ty> }, quote! { Some(#val) })
            }
            Self::Number(n) => ron_number_to_tokens(n),
            Self::Seq(seq) => array_to_tokens(seq, decl),
            Self::Map(map) => {
                let struct_ident = random_struct_name();
                let (field_idents, types, values): (Vec<_>, Vec<Ts2>, Vec<Ts2>) = map
                    .into_iter()
                    .map(|(key, val)| {
                        let ident = syn::Ident::new(&ron_key_to_ident(&key), Span::call_site());
                        let (ty, val) = val.into_tokens(decl);
                        (ident, ty, val)
                    })
                    .fold(
                        (vec![], vec![], vec![]),
                        |(mut idents, mut types, mut vals), (i, t, v)| {
                            idents.push(i);
                            types.push(t);
                            vals.push(v);
                            (idents, types, vals)
                        },
                    );
                decl.extend(quote! {
                    #[derive(Debug)]
                    pub struct #struct_ident {
                        #(pub #field_idents: #types),*
                    }
                });
                (
                    quote! { #struct_ident },
                    quote! { #struct_ident { #(#field_idents: #values),* } },
                )
            }
        }
    }
}

fn ron_number_to_tokens(n: ron::value::Number) -> (Ts2, Ts2) {
    match n {
        ron::value::Number::I8(n) => (quote! { i8  }, quote! { #n }),
        ron::value::Number::I16(n) => (quote! { i16 }, quote! { #n }),
        ron::value::Number::I32(n) => (quote! { i32 }, quote! { #n }),
        ron::value::Number::I64(n) => (quote! { i64 }, quote! { #n }),
        ron::value::Number::U8(n) => (quote! { u8  }, quote! { #n }),
        ron::value::Number::U16(n) => (quote! { u16 }, quote! { #n }),
        ron::value::Number::U32(n) => (quote! { u32 }, quote! { #n }),
        ron::value::Number::U64(n) => (quote! { u64 }, quote! { #n }),
        ron::value::Number::F32(n) => {
            let n = n.0;
            (quote! { f32 }, quote! { #n })
        }
        ron::value::Number::F64(n) => {
            let n = n.0;
            (quote! { f64 }, quote! { #n })
        }
        _ => unimplemented!(),
    }
}
