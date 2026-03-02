use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use std::marker::PhantomData;
use syn::parse::ParseStream;
use syn::token::Token;
use syn::{
    braced, bracketed, parenthesized, parse::Parse, punctuated::Punctuated, LitInt, LitStr, Path,
    Token,
};

mod kw {
    use syn::custom_keyword;

    custom_keyword!(packets);
    custom_keyword!(types);
    custom_keyword!(enums);
}

struct DefineVersionsInput {
    versions: Punctuated<DefineVersionsEntry, Token![,]>,
}

struct DefineVersionsEntry {
    version: LitInt,
    branch: LitStr,
    packets: DefineVersionsDiffList<kw::packets>,
    types: DefineVersionsDiffList<kw::types>,
    enums: DefineVersionsDiffList<kw::enums>,
}

struct DefineVersionsDiffList<K: Parse> {
    phantom: PhantomData<K>,
    pub entries: Punctuated<DefineVersionsDiffEntry, Token![,]>,
    pub path: Path,
}

enum DefineVersionsDiffEntry {
    Added {
        ident: Ident,
        path: Path,
        versioned: Option<Token![*]>,
    },
    Removed {
        ident: Ident,
    },
    Modified {
        ident: Ident,
        path: Path,
        versioned: Option<Token![*]>,
    },
}

impl Parse for DefineVersionsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(DefineVersionsInput {
            versions: Punctuated::parse_terminated(input)?,
        })
    }
}

impl Parse for DefineVersionsEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let paren;
        let brace;
        parenthesized!(paren in input);

        let version = paren.parse::<LitInt>()?;
        paren.parse::<Token![,]>()?;
        let branch = paren.parse::<LitStr>()?;

        input.parse::<Token![->]>()?;
        braced!(brace in input);
        Ok(Self {
            version,
            branch,
            packets: brace.parse()?,
            types: brace.parse()?,
            enums: brace.parse()?,
        })
    }
}

impl<K> Parse for DefineVersionsDiffList<K>
where
    K: Parse,
{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<K>()?;
        input.parse::<Token![->]>()?;

        let content;
        bracketed!(content in input);
        let entries = Punctuated::parse_terminated(&content)?;

        input.parse::<Token![@]>()?;
        let path: Path = input.parse()?;

        Ok(Self {
            phantom: Default::default(),
            entries,
            path,
        })
    }
}

impl Parse for DefineVersionsDiffEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let paren;
        parenthesized!(paren in input);

        if paren.peek(Token![+]) {
            paren.parse::<Token![+]>()?;

            let ident: Ident = input.parse()?;
            input.parse::<Token![:]>()?;

            Ok(Self::Added {
                ident,
                path: input.parse()?,
                versioned: input.parse()?,
            })
        } else if paren.peek(Token![~]) {
            paren.parse::<Token![~]>()?;

            let ident: Ident = input.parse()?;
            input.parse::<Token![:]>()?;

            Ok(Self::Modified {
                ident,
                path: input.parse()?,
                versioned: input.parse()?,
            })
        } else if paren.peek(Token![-]) {
            paren.parse::<Token![-]>()?;

            let ident: Ident = input.parse()?;

            Ok(Self::Removed { ident })
        } else {
            Err(paren.error("expected one of (+), (~), or (-)"))
        }
    }
}

pub fn define_versions_internal(input: TokenStream) -> TokenStream {
    let DefineVersionsInput { versions } = syn::parse_macro_input!(input as DefineVersionsInput);

    quote! {}.into()
}
