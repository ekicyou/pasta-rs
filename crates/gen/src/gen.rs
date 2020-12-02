use crate::block;
use crate::format_ident::*;
use pasta_script::ast::*;
use squote::{quote, Ident, Literal, TokenStream};

use std::iter::{FromIterator, Iterator};
use syn;

pub fn gen_script(script: &Script) -> TokenStream {
    let mut ts = TokenStream::new();

    // 仕訳
    let root = block::scan(script);
    println!("root={:?}", &root);

    // ID設定
    block::fix_id("", &root.hasira);
    for h in &root.hasira {
        println!("hasira.id={}", &h.attr.id);
    }

    // 変換
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
＠＠＠＠柱LV4　！会話
＠＠＠柱LV3
＠＠柱LV2
＠柱LV1
＠柱LV1
＠柱
わたし  ＠動作１　＠動作２
        言葉１、言葉２＠動作３　言葉３
"##;
    let node = parse_one(Rule::script, text).unwrap();
    if let AST::Script(ref script) = PastaParser::script(node).unwrap() {
        let ts = gen_script(script);
        println!("###src###\nts={}", ts);
    }
}
