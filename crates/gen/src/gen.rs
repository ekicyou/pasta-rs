use crate::block::*;
use crate::format_ident::*;
use pasta_script::ast::*;
use squote::{quote, Ident, TokenStream};

pub fn gen_code(code: &str) -> TokenStream {
    use pasta_script::*;
    let node = parse_one(Rule::script, code).unwrap();
    if let AST::Script(ref script) = PastaParser::script(node).unwrap() {
        gen_script(script)
    } else {
        TokenStream::new()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum MatchCode {
    Start,
    Hasira,
    Chain,
    Anchor,
}

pub fn gen_script(script: &Script) -> TokenStream {
    // 仕訳
    let mut root = scan(script);

    // ID設定
    fix_id("H", &mut root.hasira);
    for h in &mut root.hasira {
        let prefix = format!("{}A", &h.attr.id);
        fix_id(&prefix, &mut h.items);
    }

    // 変換
    let doc_comment = if let Some(doc_comment) = root.doc_comment {
        let text = doc_comment.comment.trim();
        quote! {
            #[doc=#text]
        }
    } else {
        TokenStream::new()
    };

    let mut item_jump = TokenStream::new();
    let mut match_items: Vec<(Ident, MatchCode, TokenStream)> = Vec::new();

    // START
    {
        let name = "START";
        let id = format_ident(name);
        item_jump.combine(&quote! {
            #[doc=#name] #id,
        });
        match_items.push((id, MatchCode::Start, TokenStream::new()));
    }

    // items
    for hasira in &root.hasira {
        let name = &hasira.attr.name;
        let id = format_ident(&hasira.attr.id);
        item_jump.combine(&quote! {
            #[doc=#name] #id,
        });
        let block_type = if name.len() == 0 {
            MatchCode::Chain
        } else {
            MatchCode::Hasira
        };
        match_items.push((id, block_type, TokenStream::new()));

        for anchor in &hasira.items {
            let name = format!("{}：{}", &name, &anchor.attr.name);
            let id = format_ident(&anchor.attr.id);
            item_jump.combine(&quote! {
                #[doc=#name] #id,
            });
            let mut code = TokenStream::new();
            for serif in &anchor.items {
                for act in &serif.items {
                    match act {
                        SerifItem::Actor(a) => {
                            let name = &a.name;
                            code.combine(&quote! {
                                s.actor(#name);
                            });
                        }
                        SerifItem::Serif(a) => {
                            let serif = &a.serif;
                            code.combine(&quote! {
                                s.serif(#serif);
                            });
                        }
                        _ => (),
                    }
                }
            }
            match_items.push((id, MatchCode::Anchor, code));
        }
    }

    // コードの最後に次の要素へのジャンプを記述
    // １．現在の要素が開始　　　⇒　　　　　　　　　　　　　　　次の要素
    // ２．次：無印柱　　　　　　⇒トーク発動、　　　　　　　　　次の要素（チェイントーク）
    // ３．次：通常の柱　　　　　⇒トーク発動、ランダムジャンプ
    // ４．現在の要素がアンカー　⇒トーク発動、ランダムジャンプ
    // ５．その他（現在が柱）　　⇒　　　　　　　　　　　　　　　次の要素
    let len = match_items.len();
    for y in 0..len {
        let x = if y != 0 { y - 1 } else { len - 1 };
        let now_id = match_items[x].0.clone();
        let next_id = match_items[y].0.clone();
        let now_type = match_items[x].1;
        let next_type = match_items[y].1;
        let (start, rand_jump, next_jump) = if now_type == MatchCode::Start {
            (false, false, true)
        } else if next_type == MatchCode::Chain {
            (true, false, true)
        } else if next_type == MatchCode::Hasira {
            (true, true, false)
        } else if now_type == MatchCode::Anchor {
            (true, true, false)
        } else {
            (false, false, true)
        };
        let start = if start {
            quote! {
                s.start().await;
            }
        } else {
            TokenStream::new()
        };
        let rand_jump = if rand_jump {
            quote! {
                jump = rand_jump();
            }
        } else {
            TokenStream::new()
        };
        let next_jump = if next_jump {
            quote! {
                jump = JT::#next_id;
            }
        } else {
            TokenStream::new()
        };

        match_items[x].2.combine(&quote! {
            #start
            #rand_jump
            #next_jump
            ;
        });
    }

    // コード合成
    let prefix_code = quote! {
        use pasta_core::Scriptor;
    };

    let enum_jump = quote! {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub enum JT{
            #item_jump
        }
    };
    let mut code_run = TokenStream::new();
    for (id, _, code) in &match_items {
        code_run.combine(&quote! {
            JT::#id => {
                #code
            }
        });
    }

    let fn_run = quote! {
        pub async fn walk<T: Scriptor>(s: &Scriptor, jump: JT){
            let mut jump = jump;
            loop{
                match jump{
                    #code_run
                }
            }
        }
    };

    quote! {
        #prefix_code
        #enum_jump
        #doc_comment
        #fn_run
    }
}

#[test]
fn gen_script_test() {
    use pasta_script::*;
    let code = r##"
パスタスクリプトテスト構文

最初の柱まではドキュメントコメントとします。

＠＠＠  ！一般トーク        # レベル３の柱

＠＠    ！午前中

＠お天気はどうですか　－天気－明日

パスタ
    おはようございます。
    明日の天気を当ててみてましょう。
        ＞１

        ：１
        ＠笑顔
　　サンダルは晴れと出ました！＠＋サンダル＠＋晴れ
    お出かけ出来たら楽しいですよ。


        ：１
        ＠曇り顔
　　サンダルは雨と出ました。＠＋サンダル＠＋雨
    引きこもりでも、雨はじっとりなのです。
        ＞＞明後日の方向


＠
パスタ
    トーク区切り。

＠同名柱
＠同名柱
"##;
    let node = parse_one(Rule::script, code).unwrap();
    if let AST::Script(ref script) = PastaParser::script(node).unwrap() {
        let ts = gen_script(script);
        println!("###src###\nts={}", ts);
    }
}
