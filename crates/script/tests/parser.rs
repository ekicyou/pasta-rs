use pasta_script::ast::*;
use pasta_script::Rule;
use pasta_script::{parse_one, PastaParser};

#[test]
fn escape() {
    let rule = Rule::escape;
    let text = "＠＠";
    let node = parse_one(rule, text).unwrap();
    let ast = PastaParser::escape(node).unwrap();
    assert_eq!(ast, '＠');
}

#[test]
fn error() {
    let rule = Rule::error;
    let text = "＠エラーです";
    let node = parse_one(rule, text).unwrap();
    let ast = PastaParser::error(node).unwrap();
    match ast {
        AST::Error(Error {
            start,
            end,
            error_token,
            error_str,
        }) => {
            assert_eq!(start, 0);
            assert_eq!(end, 6 * 3);
            assert_eq!(error_token, '＠');
            assert_eq!(error_str, "＠エラーです");
        }
        _ => assert!(false, "{:?}", ast),
    }
}

#[test]
fn comment() {
    let rule = Rule::comment;
    let text = "#コメントです。";
    let node = parse_one(rule, text).unwrap();
    let ast = PastaParser::comment(node).unwrap();
    assert_eq!(
        ast,
        AST::Comment(Comment {
            comment: text.to_owned()
        })
    );
}

#[test]
fn err_or_comment() {
    let rule = Rule::err_or_comment;
    {
        let text = "#コメントです。";
        let node = parse_one(rule, text).unwrap();
        let ast = PastaParser::err_or_comment(node).unwrap();
        assert_eq!(
            ast,
            AST::Comment(Comment {
                comment: text.to_owned()
            })
        );
    }
    {
        let text = "エラーなんです。";
        let node = parse_one(rule, text).unwrap();
        let ast = PastaParser::err_or_comment(node).unwrap();
        if let AST::Error(Error {
            start,
            end,
            error_token,
            error_str,
        }) = ast
        {
            assert_eq!(start, 0);
            assert_eq!(end, 24);
            assert_eq!(error_token, 'エ');
            assert_eq!(error_str, "エラーなんです。");
        } else {
            assert!(false);
        }
    }
}

#[test]
fn expr() {
    let rule = Rule::expr;
    {
        let text = "属性名";
        let node = parse_one(rule, text).unwrap();
        let ast = PastaParser::expr(node).unwrap();
        assert_eq!(
            ast,
            AST::Expr(Expr {
                expr: text.to_owned()
            })
        );
    }
}

#[allow(non_snake_case)]
fn EXPR<S: Into<String>>(text: S) -> Box<AST> {
    let text = text.into();
    Box::new(AST::Expr(Expr { expr: text }))
}

#[allow(non_snake_case)]
fn EXPR_OR_NUM<S: Into<String>>(text: S) -> Box<AST> {
    let text = text.into();
    Box::new(AST::ExprOrNum(ExprOrNum { expr: text }))
}

#[test]
fn action() {
    let rule = Rule::action;
    {
        let text = "＠式";
        let node = parse_one(rule, text).unwrap();
        let ast = PastaParser::action(node).unwrap();
        assert_eq!(ast, AST::Action(Action { expr: EXPR("式") }));
    }
}

#[test]
fn require() {
    let rule = Rule::require;
    {
        let text = "!式";
        let node = parse_one(rule, text).unwrap();
        let ast = PastaParser::require(node).unwrap();
        assert_eq!(ast, AST::Require(Require { expr: EXPR("式") }));
    }
}

#[test]
fn either() {
    let rule = Rule::either;
    {
        let text = "?式";
        let node = parse_one(rule, text).unwrap();
        let ast = PastaParser::either(node).unwrap();
        assert_eq!(ast, AST::Either(Either { expr: EXPR("式") }));
    }
}

#[test]
fn forget() {
    let rule = Rule::forget;
    {
        let text = "-式";
        let node = parse_one(rule, text).unwrap();
        let ast = PastaParser::forget(node).unwrap();
        assert_eq!(ast, AST::Forget(Forget { expr: EXPR("式") }));
    }
}

#[test]
fn memory() {
    let rule = Rule::memory;
    {
        let text = "＋式";
        let node = parse_one(rule, text).unwrap();
        let ast = PastaParser::memory(node).unwrap();
        assert_eq!(ast, AST::Memory(Memory { expr: EXPR("式") }));
    }
}

#[test]
fn h_attrs() {
    let rule = Rule::h_attrs;
    {
        let text = "！必須？どれか＋追加＠関数";
        let node = parse_one(rule, text).unwrap();
        let ast = PastaParser::h_attrs(node);
        let ast = ast.unwrap();

        let mut vv = Vec::new();
        vv.push(AST::Require(Require {
            expr: EXPR("必須")
        }));
        vv.push(AST::Either(Either {
            expr: EXPR("どれか"),
        }));
        vv.push(AST::Memory(Memory {
            expr: EXPR("追加")
        }));
        vv.push(AST::Action(Action {
            expr: EXPR("関数")
        }));
        let right = AST::Attrs(Attrs { items: vv });
        assert_eq!(ast, right);
    }
}

#[test]
fn hasira_level() {
    let rule = Rule::hasira_level;
    {
        let text = "@@@";
        let node = parse_one(rule, text).unwrap();
        let ast = PastaParser::hasira_level(node).unwrap();
        assert_eq!(ast, 3);
    }
}

#[test]
fn hasira_title() {
    let rule = Rule::hasira_title;
    {
        let text = "タイトル文字列　";
        let node = parse_one(rule, text).unwrap();
        let ast = PastaParser::hasira_title(node).unwrap();
        assert_eq!(ast, "タイトル文字列");
    }
}

#[test]
fn hasira_header() {
    let rule = Rule::hasira_header;
    {
        let text = "@@柱　";
        let node = parse_one(rule, text).unwrap();
        let ast = PastaParser::hasira_header(node).unwrap();
        assert_eq!(ast, (2, "柱"));
    }
}

#[test]
fn actor_header() {
    let rule = Rule::actor_header;
    {
        let text = "アクター名　";
        let node = parse_one(rule, text).unwrap();
        let ast = PastaParser::actor_header(node).unwrap();
        assert_eq!(ast, "アクター名");
    }
}

#[test]
fn hasira() {
    let rule = Rule::hasira;
    {
        let text = "@@@柱　＠属性１！属性２　";
        let node = parse_one(rule, text).unwrap();
        let ast = PastaParser::hasira(node).unwrap();
        match ast {
            AST::Hasira(Hasira { level, name, attrs }) => {
                assert_eq!(level, 3);
                assert_eq!(name, "柱");
                assert!(attrs.is_some());
            }
            _ => assert!(false),
        }
    }
    {
        let text = "アクター　";
        let node = parse_one(rule, text).unwrap();
        let ast = PastaParser::hasira(node).unwrap();
        match ast {
            AST::Hasira(Hasira { level, name, attrs }) => {
                assert_eq!(level, 0);
                assert_eq!(name, "アクター");
                assert!(attrs.is_none());
            }
            _ => assert!(false),
        }
    }
}

#[test]
fn serif() {
    let rule = Rule::serif;
    {
        let text = "セリフ＠＠セリフ";
        let node = parse_one(rule, text).unwrap();
        let ast = PastaParser::serif(node).unwrap();
        assert_eq!(
            ast,
            AST::Serif(Serif {
                serif: "セリフ＠セリフ".to_owned()
            })
        );
    }
}

#[test]
fn t_attr() {
    let rule = Rule::t_attr;
    {
        let text = "＠アクション";
        let node = parse_one(rule, text).unwrap();
        let ast = PastaParser::t_attr(node).unwrap();
        assert_eq!(
            ast,
            AST::Action(Action {
                expr: EXPR("アクション")
            })
        );
    }
    {
        let text = "＠!アクション";
        let node = parse_one(rule, text).unwrap();
        let ast = PastaParser::t_attr(node).unwrap();
        assert_eq!(
            ast,
            AST::Require(Require {
                expr: EXPR("アクション")
            })
        );
    }
}

#[test]
fn togaki() {
    let rule = Rule::togaki;
    fn action<S: Into<String>>(s: S) -> AST {
        let keyword = s.into();
        let expr = EXPR(keyword);
        AST::Action(Action { expr })
    }
    fn serif<S: Into<String>>(s: S) -> AST {
        AST::Serif(Serif { serif: s.into() })
    }
    fn jump<S: Into<String>>(s: S) -> AST {
        let keyword = s.into();
        let expr = EXPR_OR_NUM(keyword);
        AST::ShortJump(ShortJump { expr })
    }
    {
        let text = "　セリフ＠＠だね。＠アクション　＠アクション２";
        let node = parse_one(rule, text).unwrap();
        let ast = PastaParser::togaki(node).unwrap();

        let vv = vec![
            serif("セリフ＠だね。"),
            action("アクション"),
            action("アクション２"),
        ];
        assert_eq!(ast, AST::Togaki(Togaki { items: vv }));
    }
    {
        let text = "　＞１";
        let node = parse_one(rule, text).unwrap();
        let ast = PastaParser::togaki(node).unwrap();

        let vv = vec![jump("１")];
        assert_eq!(ast, AST::Togaki(Togaki { items: vv }));
    }
}

#[test]
fn line() {
    let rule = Rule::line;

    fn y<T>(src: Option<T>) -> Option<Box<T>> {
        src.map(|x| Box::new(x))
    }
    {
        let text = "＠柱　　不思議だね";
        let node = parse_one(rule, text).unwrap();
        let ast = PastaParser::line(node).unwrap();

        let a = Some(AST::Hasira(Hasira {
            level: 1,
            name: "柱".to_owned(),
            attrs: None,
        }));
        let e = Some(AST::Error(Error {
            start: 12,
            end: 27,
            error_token: '不',
            error_str: "不思議だね".to_owned(),
        }));
        let c = None;
        let code = y(a);
        let err = y(e);
        let comment = y(c);
        let left = AST::Line(Line { code, err, comment });

        assert_eq!(ast, left);
    }
    {
        let text = "　トークです。＃コメント";
        let node = parse_one(rule, text).unwrap();
        let ast = PastaParser::line(node).unwrap();

        let t_items = vec![AST::Serif(Serif {
            serif: "トークです。".to_owned(),
        })];
        let a = Some(AST::Togaki(Togaki { items: t_items }));
        let e = None;
        let c = Some(AST::Comment(Comment {
            comment: "＃コメント".to_owned(),
        }));
        let code = y(a);
        let err = y(e);
        let comment = y(c);
        let left = AST::Line(Line { code, err, comment });

        assert_eq!(ast, left);
    }
}

#[test]
fn script() {
    let rule = Rule::script;
    {
        let text = include_str!("parse62.pasta");
        let node = parse_one(rule, text).unwrap();
        println!("{:?}\n", node);
        let ast = PastaParser::script(node).unwrap();
        println!("{:?}", ast);
        if let AST::Script(Script { items: lines }) = ast {
            assert_eq!(lines.len(), 24);
            match &lines[0] {
                AST::DocComment(..) => {}
                x => {
                    assert!(false, "doc_comment {:?}", x);
                }
            }
            match &lines[1] {
                AST::Line(Line { code: Some(a), .. }) => match &**a {
                    AST::Hasira(Hasira { level, .. }) => {
                        assert_eq!(*level, 3);
                    }
                    x => {
                        assert!(false, "hasira {:?}", x);
                    }
                },
                x => {
                    assert!(false, "hasira {:?}", x);
                }
            }
            match &lines[5] {
                AST::Line(Line { code: Some(a), .. }) => match &**a {
                    AST::Hasira(Hasira { level, name, .. }) => {
                        assert_eq!(*level, 1);
                        assert_eq!(*name, "お天気はどうですか？");
                    }
                    x => {
                        assert!(false, "hasira {:?}", x);
                    }
                },
                x => {
                    assert!(false, "hasira {:?}", x);
                }
            }
            match &lines[7] {
                AST::Line(Line { code: Some(a), .. }) => match &**a {
                    AST::Hasira(Hasira { level, name, .. }) => {
                        assert_eq!(*level, 0);
                        assert_eq!(*name, "パスタ");
                    }
                    x => {
                        assert!(false, "hasira {:?}", x);
                    }
                },
                x => {
                    assert!(false, "hasira {:?}", x);
                }
            }
        } else {
            assert!(false);
        }
    }
}

#[test]
fn script_iter() {
    let rule = Rule::script;
    {
        let text = include_str!("parse62.pasta");
        let node = parse_one(rule, text).unwrap();
        match PastaParser::script(node).unwrap() {
            AST::Script(ref script) => {
                let items: Vec<_> = script.into_iter().collect();
                assert_eq!(items.len(), 18);
                for item in &items {
                    println!("{:?}", item);
                }
                if let AST::DocComment(_) = items[0] {
                } else {
                    panic!("not doc_comment")
                }
            }
            _ => panic!("not script"),
        };
    }
}
