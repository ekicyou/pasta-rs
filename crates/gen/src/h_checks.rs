use crate::block::*;
use crate::utils::*;
use squote::{quote, TokenStream};

pub fn h_checks(hasira: &[HasiraBlock]) -> TokenStream {
    let mut funcs = TokenStream::new();
    for h in hasira {
        let fn_name = format_ident(&h.attr.id);
        let some = quote! { Some(JT::#fn_name) };
        let none = quote! { None };
        let ret_ok = quote! {{ return #some; }};
        let ret_ng = quote! {{ return #none; }};
        let mut is_require = false;
        let mut is_either = false;

        if h.attr.require.len() == 0 && h.attr.either.len() == 0 {
            funcs.combine(&quote! {
            pub fn #fn_name(tags: &[String]) -> Option<JT> {
                Some(JT::#fn_name)
            }});
            continue;
        };
        let mut expr_require = TokenStream::new();
        let mut expr_either = TokenStream::new();
        for value in (&h.attr.require).iter() {
            is_require = true;
            let value = require_value(value);
            expr_require.combine(&quote! { if !tags.contains(#value) #ret_ng });
        }
        for value in (&h.attr.either).iter() {
            is_either = true;
            let value = either_value(value);
            expr_either.combine(&quote! { if tags.contains(#value) #ret_ok });
        }
        let result = if is_require && (!is_either) {
            some
        } else {
            none
        };
        funcs.combine(&quote! {
        pub fn #fn_name(tags: &HashSet<String>) -> Option<JT> {
            #expr_require
            #expr_either
            #result
        }});
    }
    quote! {
        mod h_checks {
            #![allow(non_snake_case)]
            use super::JT;
            use std::collections::HashSet;
            #funcs
        }
    }
}
