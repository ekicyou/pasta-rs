mod actor;
mod attr;
mod hasira;
mod togaki;

mod tests {
    use squote::{format_ident, quote, Literal, TokenStream};
    use std::iter::FromIterator;

    #[test]
    fn main() {
        let tokens = quote! {
            println!("Hello, world!");
        };
        println!("{}", tokens.into_string());
    }
}
