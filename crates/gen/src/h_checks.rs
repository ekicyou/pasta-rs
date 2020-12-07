use crate::block::*;
use crate::format_ident::*;
use pasta_script::ast::*;
use squote::{quote, Ident, TokenStream};

fn expr_key(expr: &Expr) -> &str {
    &expr.expr
}
fn require_value(node: &Require) -> &str {
    if let AST::Expr(expr) = &**node.expr() {
        &expr.expr
    } else {
        ""
    }
}
fn either_value(node: &Either) -> &str {
    if let AST::Expr(expr) = &**node.expr() {
        &expr.expr
    } else {
        ""
    }
}
pub fn h_checks(hasira: &[HasiraBlock]) -> TokenStream {
    let mut funcs = TokenStream::new();
    for h in hasira {
        let fn_name = format_ident(&h.attr.id);
        if h.attr.require.len() == 0 && h.attr.either.len() == 0 {
            funcs.combine(&quote! {
            pub fn $fn_name(tags: &[String]) -> Option<JT> {
                Some(JT::$fn_name)
            }});
            continue;
        };
        let mut init = TokenStream::new();
        let mut expr = TokenStream::new();
        let mut ok_require = TokenStream::new();
        let mut ok_either = TokenStream::new();
        for (index, value) in (&h.attr.require).iter().enumerate() {
            let id = format!("r{}", index);
            let id = format_ident(&id);
            let value = require_value(value);
            init.combine(&quote! { let mut            #id = false; });
            expr.combine(&quote! { if tag == #value { #id = true; }});
            if index == 0 {
                ok_require = quote! {#id};
            } else {
                ok_require.combine(&quote! { && #id})
            }
        }
        for (index, value) in (&h.attr.either).iter().enumerate() {
            let id = format!("e{}", index);
            let id = format_ident(&id);
            let value = either_value(value);
            init.combine(&quote! { let mut            #id = false; });
            expr.combine(&quote! { if tag == #value { #id = true; }});
            if index == 0 {
                ok_either = quote! {#id};
            } else {
                ok_either.combine(&quote! { || #id })
            }
        }
        let if_ok = if h.attr.require.len() > 0 && h.attr.either.len() > 0 {
            quote! { #ok_require && ( #ok_either ) }
        } else if h.attr.require.len() > 0 {
            quote! { #ok_require }
        } else {
            quote! { #ok_either }
        };
        funcs.combine(&quote! {
        pub fn $fn_name(tags: &[String]) -> Option<JT> {
            #init
            for tag in tags{
                #expr
                if #if_ok {
                    return Some(JT::$fn_name);
                }
            }
            None
        }});
    }
    quote! {
        mod h_checks {
            #![allow(non_snake_case)]
            use super::JT;
            #funcs
        }
    }
}
