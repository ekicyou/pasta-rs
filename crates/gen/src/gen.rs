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
パスタスクリプトテスト構文

最初の柱まではドキュメントコメントとします。

＠＠＠  ！起動トーク        # レベル３の柱

＠＠    ！午前中

＠お天気はどうですか？

パスタ
    おはようございます。
    明日の天気を当ててみてましょう。
        ＞１

        ：１
        ＠笑顔
　　サンダルは晴れと出ました！
    お出かけ出来たら楽しいですよ。

        ：１
        ＠曇り顔
　　サンダルは雨と出ました。
    引きこもりでも、雨はじっとりなのです。
        ＞＞明後日の方向


＠
パスタ
    トーク区切り。

＠同名柱
＠同名柱
"##;
    let node = parse_one(Rule::script, text).unwrap();
    if let AST::Script(ref script) = PastaParser::script(node).unwrap() {
        let ts = gen_script(script);
        println!("###src###\nts={}", ts);
    }
}
