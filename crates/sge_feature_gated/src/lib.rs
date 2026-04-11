use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Block, ItemFn, LitStr, Token, parse_macro_input, uote};

struct Args {
    feature: LitStr,
    name: LitStr,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let feature = input.parse::<LitStr>()?;
        let _ = input.parse::<Token![,]>();
        let name = input.parse::<LitStr>()?;

        Ok(Self { feature, name })
    }
}

#[proc_macro_attribute]
pub fn feature_gated(args: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);

    let args = parse_macro_input!(args as Args);

    let fn_name = func.sig.ident.to_string();
    let feature = args.feature;

    let disabled_func = ItemFn {
        block: Box::new(Block {
            brace_token: func.block.brace_token,
            stmts: vec![
                parse_quote! { ::log::warn!("Tried to run {}, but the {} feature isn't enabled.", #fn_name, #feature); },
            ],
        }),
        ..(func.clone())
    };

    quote! {
        #[cfg(feature = #feature)]
        #func

        #[cfg(not(feature = #feature))]
        #[allow(unused)]
        #disabled_func

    }
    .into()
}
