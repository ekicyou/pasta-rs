use proc_macro::TokenStream;
use quote::quote;
use std::env;
use std::path::PathBuf;

#[proc_macro]
pub fn build(stream: TokenStream) -> TokenStream {
    let mut codes: Vec<String> = Vec::new();
    for t in stream {
        println!("{:?}", &t);
        let build: syn::LitStr = syn::parse(t).unwrap();
        let build = build.value();
        let mut build_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        build_path.push(build);
        let code = std::fs::read_to_string(&build_path).unwrap();
    }

    let tokens = pasta_gen::gen_pasta_code(&code).into_string();
    let tokens = quote! {
        use ::std::io::Write;
        let mut path = ::std::path::PathBuf::from(
            ::std::env::var("OUT_DIR").expect("No `OUT_DIR` env variable set"),
        );

        path.push("pasta.rs");
        let mut file = ::std::fs::File::create(&path).expect("Failed to create noodle.rs");
        file.write_all(#tokens.as_bytes()).expect("Could not write generated code to output file");
        println!("save");

        let mut cmd = ::std::process::Command::new("rustfmt");
        cmd.arg(&path);
        println!("cmd: {:?}",cmd);
        let rc = cmd.output();
        println!("output={:?}",rc);
    };
    tokens.into()
}
