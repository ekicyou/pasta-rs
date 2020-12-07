use crate::format_ident::*;
use squote::{quote, Ident, TokenStream};

pub fn gen() -> TokenStream {
    quote! {
        fn rand_jump<S: Scriptor>(s: &mut S, default_target: JT) -> JT {
            JT::START
        }
    }
}
