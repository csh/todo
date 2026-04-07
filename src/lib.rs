extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, spanned::Spanned, ItemTrait, LitBool, Token, TraitItem, TraitItemFn};

#[proc_macro_attribute]
pub fn todo(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemTrait);
    let enable_in_release = parse_enable_in_release(attr);
    expand_todo_impl(input, enable_in_release).into()
}

fn expand_todo_impl(input: ItemTrait, enable_in_release: bool) -> proc_macro2::TokenStream {
    let trait_ident = &input.ident;
    let vis = &input.vis;
    let generics = &input.generics;

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let todo_ident = format_ident!("Todo{}", trait_ident);
    let cfg_attr = if enable_in_release {
        None
    } else {
        Some(quote! { #[cfg(debug_assertions)] })
    };

    let trait_methods = input.items.iter().map(|item| match item {
        TraitItem::Fn(method) => expand_trait_method(method),
        TraitItem::Const(item) => syn::Error::new(
            item.span(),
            format!(
                "cannot generate {} for items with associated consts",
                todo_ident
            ),
        )
        .to_compile_error(),
        TraitItem::Type(item) => syn::Error::new(
            item.span(),
            format!(
                "cannot generate {} for items with associated types",
                todo_ident
            ),
        )
        .to_compile_error(),
        _ => syn::Error::new(
            item.span(),
            format!("cannot generate {}; unsupported type", todo_ident),
        )
        .to_compile_error(),
    });

    quote! {
        #input

        #cfg_attr
        #vis struct #todo_ident #generics #where_clause;

        impl #impl_generics #trait_ident #ty_generics for #todo_ident #ty_generics #where_clause {
            #(#trait_methods)*
        }
    }
}

fn expand_trait_method(method: &TraitItemFn) -> proc_macro2::TokenStream {
    let attrs = &method.attrs;
    let sig = &method.sig;

    quote! {
        #(#attrs)*
        #sig {
            ::core::todo!()
        }
    }
}

struct EnableInRelease {
    _key: Ident,
    _eq: Token![=],
    value: LitBool,
}

impl Parse for EnableInRelease {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _key: input.parse()?,
            _eq: input.parse()?,
            value: input.parse()?,
        })
    }
}

fn parse_enable_in_release(attr: TokenStream) -> bool {
    if attr.is_empty() {
        return false;
    }

    let parsed = syn::parse::<EnableInRelease>(attr);
    match parsed {
        Ok(v) if v._key == "enable_in_release" => v.value.value,
        _ => false,
    }
}
