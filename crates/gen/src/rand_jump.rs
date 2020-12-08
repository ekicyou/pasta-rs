use squote::{quote, TokenStream};

pub fn gen() -> TokenStream {
    quote! {
        fn rand_jump<S: Scriptor>(s: &mut S, default_target: JT) -> JT {
            JT::START
        }
    }
}
