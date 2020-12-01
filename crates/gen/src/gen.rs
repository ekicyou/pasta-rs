use crate::block;
use crate::format_ident::*;
use pasta_script::ast::*;
use squote::{quote, Ident, Literal, TokenStream};

use std::iter::{FromIterator, Iterator};
use syn;

pub fn gen_script(script: &Script) -> TokenStream {
    let mut ts = TokenStream::new();

    let root = block::scan(script);

    if let Some(doc_comment) = root.doc_comment {
        let text = doc_comment.comment.trim();
        ts.combine(&quote! {
            #[doc=#text]
        });
    }

    ts
}

#[test]
fn gen_script_test() {
    use pasta_script::*;
    let text = r##"
どきゅめんと
"##;
    let node = parse_one(Rule::script, text).unwrap();
    if let AST::Script(ref script) = PastaParser::script(node).unwrap() {
        let ts = gen_script(script);
        println!("###src###\nts={}", ts);
    }
}
