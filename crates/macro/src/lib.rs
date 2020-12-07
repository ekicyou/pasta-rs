use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use std::env;
use std::path::PathBuf;
use syn::parse2;

#[proc_macro]
pub fn build(stream: TokenStream) -> TokenStream {
    let build: syn::LitStr = syn::parse(stream).unwrap();
    let build = build.value();
    let mut build_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    build_path.push(build);
    let code = std::fs::read_to_string(&build_path).unwrap();

    let tokens = pasta_gen::gen_code(&code).into_string();
    let tokens = quote! {
        use ::std::io::Write;
        let mut path = ::std::path::PathBuf::from(
            ::std::env::var("OUT_DIR").expect("No `OUT_DIR` env variable set"),
        );

        path.push("noodle.rs");
        let mut file = ::std::fs::File::create(&path).expect("Failed to create noodle.rs");
        file.write_all(#tokens.as_bytes()).expect("Could not write generated code to output file");

        let mut cmd = ::std::process::Command::new("rustfmt");
        cmd .arg("--edition 2018")
            .arg(&path);
        let _ = cmd.output();
    };
    tokens.into()
}

mod sample {
    use squote::{quote, TokenStream};
    pub fn script() -> String {
        let ts = quote! {
            pub struct ScenarioMaker {}
        };
        ts.into_string()
    }
}

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
