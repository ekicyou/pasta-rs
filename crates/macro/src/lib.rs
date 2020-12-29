use proc_macro::TokenStream;
use quote::quote;
use std::env;
use std::iter::FromIterator;
use std::path::PathBuf;

#[proc_macro]
pub fn build(stream: TokenStream) -> TokenStream {
    let mut pasta_codes: Vec<String> = Vec::new();
    let mut csv_codes: Vec<String> = Vec::new();
    for t in stream {
        let t = std::iter::repeat(t).take(1);
        let t = TokenStream::from_iter(t);
        let build: Result<syn::LitStr, _> = syn::parse(t);
        if let Ok(build) = build {
            let build = build.value();
            let mut build_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
            build_path.push(build);
            let code = std::fs::read_to_string(&build_path)
                .expect(&format!("file not found: path={:?}", &build_path));
            match build_path.extension() {
                Some(ext) => {
                    if ext == "pasta" {
                        pasta_codes.push(code);
                    } else if ext == "csv" {
                        csv_codes.push(code);
                    }
                }
                _ => {}
            }
        }
    }

    let tokens = pasta_gen::gen(pasta_codes.as_slice(), csv_codes.as_slice()).into_string(); //.replace(" ; ", ";\n");

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
