use syn::{
    Expr, Ident, Result, Token, Type, bracketed,
    parse::{Parse, ParseStream},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Flag {
    Rotation,
    Outline,
    WithOutline,
}

impl Parse for Flag {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident: Ident = input.parse()?;
        match ident.to_string().as_str() {
            "rotation" => Ok(Flag::Rotation),
            "outline" => Ok(Flag::Outline),
            "with_outline" => Ok(Flag::WithOutline),
            other => Err(syn::Error::new(
                ident.span(),
                format!("unknown flag `{other}`; expected one of: rotation, outline, with_outline"),
            )),
        }
    }
}

pub struct Param {
    pub name: Ident,
    pub ty: Type,
}

impl Parse for Param {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let _: Token![:] = input.parse()?;
        let ty: Type = input.parse()?;
        Ok(Param { name, ty })
    }
}

pub struct ShapeEntry {
    pub name: Ident,
    pub flags: Vec<Flag>,
    pub params: Vec<Param>,
    pub constructor: Expr,
}

impl Parse for ShapeEntry {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;

        let flags_content;
        bracketed!(flags_content in input);
        let flags = flags_content
            .parse_terminated(Flag::parse, Token![,])?
            .into_iter()
            .collect();

        let _: Token![:] = input.parse()?;

        let mut params = Vec::new();
        loop {
            if input.peek(Token![=>]) {
                break;
            }
            params.push(input.parse::<Param>()?);
            if input.peek(Token![,]) {
                let _: Token![,] = input.parse()?;
            } else {
                break;
            }
        }

        let _: Token![=>] = input.parse()?;

        let constructor: Expr = input.parse()?;

        Ok(ShapeEntry {
            name,
            flags,
            params,
            constructor,
        })
    }
}

pub struct MacroInput {
    pub entries: Vec<ShapeEntry>,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let entries = input
            .parse_terminated(ShapeEntry::parse, Token![,])?
            .into_iter()
            .collect();
        Ok(MacroInput { entries })
    }
}
