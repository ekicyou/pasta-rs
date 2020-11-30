use crate::format_ident::*;
use pasta_script::ast::*;
use squote::{quote, Ident, Literal, TokenStream};
use std::iter::{FromIterator, Iterator};

pub fn gen_script(script: &Script) -> TokenStream {
    let mut ts = TokenStream::new();

    for ast in script {
        match ast {
            AST::DocComment(a) => doc_comment(&mut ts, a),
            AST::Comment(a) => comment(&mut ts, a),
            AST::Error(a) => error(&mut ts, a),
            AST::Hasira(h) => {}
            AST::Anchor(a) => {}
            AST::ShortJump(j) => {}
            AST::LongJump(j) => {}
            AST::Togaki(t) => {}
            _ => (),
        }
    }

    ts
}

fn doc_comment(ts: &mut TokenStream, item: &DocComment) {
    for line in item.comment.lines() {
        ts.append(Ident::new(format!("//! {}\n", line)));
    }
}

fn comment(ts: &mut TokenStream, item: &Comment) {
    ts.append(Ident::new(format!("// {}\n", item.comment)));
}

fn error(ts: &mut TokenStream, item: &Error) {
    ts.append(Ident::new(format!(
        "// [ERROR!]({}-{}), '{}' :{}]\n",
        item.start, item.end, item.error_token, item.error_str
    )));
}

#[test]
fn doc_comment_test() {
    let mut ts = TokenStream::new();
    let doc = DocComment {
        comment: "コメントです\n２行目です\n３行目です".to_owned(),
    };
    doc_comment(&mut ts, &doc);
    println!("{}", ts);
}
