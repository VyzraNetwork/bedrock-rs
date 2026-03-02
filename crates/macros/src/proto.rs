use const_random::const_random;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::parse::ParseStream;
use syn::token::{Brace, Bracket};
use syn::{
    braced, bracketed, parse::Parse, parse_macro_input, punctuated::Punctuated, Expr, Ident,
    LitInt, Token, Type,
};

const RANDOM_ID: u32 = const_random!(u32);

macro_rules! get_macro_ident {
    ($id:expr) => {
        format!("__export_{}_{}", $id, RANDOM_ID)
    };
}

struct IdentList {
    list: Punctuated<Ident, Token![,]>,
}

impl Parse for IdentList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(IdentList {
            list: input.parse_terminated(Ident::parse, Token![,])?,
        })
    }
}

pub fn define_packets_internal(input: TokenStream) -> TokenStream {
    let IdentList { list: packets } = syn::parse_macro_input!(input as IdentList);

    let packet_strings = packets.iter().map(|value| {
        let s = value.to_string();
        quote!(#s)
    });

    let ident = Ident::new(get_macro_ident!("packets").as_str(), Span::call_site());

    quote! {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! #ident {
            () => { [
                #(#packet_strings),*
            ] };
        }
    }
    .into()
}

struct U32List {
    list: Punctuated<LitInt, Token![,]>,
}

impl Parse for U32List {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(U32List {
            list: input.parse_terminated(
                |input| {
                    let lit = LitInt::parse(input)?;
                    lit.base10_parse::<u32>()?;
                    Ok(lit)
                },
                Token![,],
            )?,
        })
    }
}

pub fn define_versions_internal(input: TokenStream) -> TokenStream {
    let U32List { list } = syn::parse_macro_input!(input as U32List);

    let ident = Ident::new(get_macro_ident!("versions").as_str(), Span::call_site());

    quote! {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! #ident {
            ($(::)?$($callback:ident)::*!($($args:tt)*)) => {
                $($callback)::*!(
                    [#list],
                    $($args)*
                )
            };
        }
    }
    .into()
}

struct ImplVersionInput {
    version: u32,
    _colon: Token![:],
    _brace: Brace,
    packets: Punctuated<ImplVersionPacketField, Token![,]>,
}

struct ImplVersionPacketField {
    name: Ident,
    _colon: Token![:],
    packet: Type,
}

impl Parse for ImplVersionInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let packets;
        Ok(Self {
            version: input.parse::<LitInt>()?.base10_parse()?,
            _colon: input.parse()?,
            _brace: braced!(packets in input),
            packets: packets.parse_terminated(ImplVersionPacketField::parse, Token![,])?,
        })
    }
}

impl Parse for ImplVersionPacketField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            _colon: input.parse()?,
            packet: input.parse()?,
        })
    }
}

pub fn impl_version_internal(input: TokenStream) -> TokenStream {
    let ImplVersionInput {
        version, packets, ..
    } = parse_macro_input!(input as ImplVersionInput);

    let all_packets = Ident::new(get_macro_ident!("packets").as_str(), Span::call_site());
    let versions = Ident::new(get_macro_ident!("versions").as_str(), Span::call_site());

    let version_ident = Ident::new(format!("V{}", version).as_str(), Span::call_site());

    let macro_ident = Ident::new(
        get_macro_ident!(format!("{}", version)).as_str(),
        Span::call_site(),
    );
    let macro_packets = packets
        .iter()
        .map(|p| {
            let name = p.name.clone();
            let packet = p.packet.clone();

            quote! { #name: #packet }
        })
        .collect::<Vec<_>>();

    quote! {
        #[derive(Clone, Debug)]
        struct #version_ident;

        impl #version_ident {
            const previous = #versions!(bedrockrs_macros::fetch_versions!(#version, bedrockrs_macros::impl_version_inner));
        }

        #[doc(hidden)]
        #[macro_export]
        macro_rules! #macro_ident {
            (
                $(::)?$($callback:ident)::*,
                $( $args:tt )*
            ) => {
                $($callback)::*!(
                    $( $args )*,
                    [#(#macro_packets),*]
                )
            };
        }
    }
        .into()
}

struct ImplVersionInnerInput {
    version: u32,
}

struct ImplVersionInner {}

pub fn impl_version_inner_internal(input: TokenStream) -> TokenStream {
    let input2: proc_macro2::TokenStream = input.clone().into();

    dbg!(input.clone().to_string());

    quote! {
        (#input2)
    }
    .into()
}

struct FetchVersionsInput {
    _brackets: Bracket,
    versions: Punctuated<u32, Token![,]>,
    _comma: Token![,],
    version: u32,
    _comma2: Token![,],
    callback: Expr,
}

impl Parse for FetchVersionsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let versions;
        Ok(Self {
            _brackets: bracketed!(versions in input),
            versions: versions
                .parse_terminated(|i| i.parse::<LitInt>()?.base10_parse::<u32>(), Token![,])?,
            _comma: input.parse()?,
            version: input.parse::<LitInt>()?.base10_parse()?,
            _comma2: input.parse()?,
            callback: input.parse()?,
        })
    }
}

pub fn fetch_versions_internal(input: TokenStream) -> TokenStream {
    let FetchVersionsInput {
        version,
        versions,
        callback,
        ..
    } = parse_macro_input!(input as FetchVersionsInput);

    let older = versions
        .iter()
        .filter(|v| **v < version)
        .collect::<Vec<_>>();

    let mut older_macro_idents = older
        .iter()
        .map(|v| {
            Ident::new(
                get_macro_ident!(format!("{}", v)).as_str(),
                Span::call_site(),
            )
        })
        .collect::<Vec<_>>();

    if let Some((first, other)) = older_macro_idents.split_first() {
        let chain = other.into_iter().fold(quote! { #callback }, |acc, ident| {
            quote! { #ident, #acc }
        });

        quote! {
            #first!(#chain, #version)
        }
        .into()
    } else {
        quote! {
            #callback!(#version)
        }
        .into()
    }
}
