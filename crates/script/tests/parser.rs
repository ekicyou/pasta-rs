use pasta_script::PastaAST as AST;
use pasta_script::PastaParserRule as Rule;
use pasta_script::{parse, parse_one, PastaParser};

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
        AST::error(start, end, token, text) => {
            assert_eq!(start, 0);
            assert_eq!(end, 6 * 3);
            assert_eq!(token, '＠');
            assert_eq!(text, "＠エラーです");
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
    assert_eq!(ast, AST::comment(text.to_owned()));
}

#[test]
fn err_or_comment() {
    let rule = Rule::err_or_comment;
    {
        let text = "#コメントです。";
        let node = parse_one(rule, text).unwrap();
        let ast = PastaParser::err_or_comment(node).unwrap();
        assert_eq!(ast, AST::comment(text.to_owned()));
    }
    {
        let text = "エラーなんです。";
        let node = parse_one(rule, text).unwrap();
        let ast = PastaParser::err_or_comment(node).unwrap();
        if let AST::error(s, e, c, t) = ast {
            assert_eq!(s, 0);
            assert_eq!(e, 24);
            assert_eq!(c, 'エ');
            assert_eq!(t, "エラーなんです。");
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
        assert_eq!(ast, AST::expr(text.to_owned()));
    }
}

#[test]
fn action() {
    let rule = Rule::action;
    {
        let text = "＠式";
        let node = parse_one(rule, text).unwrap();
        let ast = PastaParser::action(node).unwrap();
        assert_eq!(ast, AST::action(Box::new(AST::expr("式".to_owned()))));
    }
}

#[test]
fn require() {
    let rule = Rule::require;
    {
        let text = "!式";
        let node = parse_one(rule, text).unwrap();
        let ast = PastaParser::require(node).unwrap();
        assert_eq!(ast, AST::require(Box::new(AST::expr("式".to_owned()))));
    }
}

#[test]
fn either() {
    let rule = Rule::either;
    {
        let text = "?式";
        let node = parse_one(rule, text).unwrap();
        let ast = PastaParser::either(node).unwrap();
        assert_eq!(ast, AST::either(Box::new(AST::expr("式".to_owned()))));
    }
}

#[test]
fn forget() {
    let rule = Rule::forget;
    {
        let text = "-式";
        let node = parse_one(rule, text).unwrap();
        let ast = PastaParser::forget(node).unwrap();
        assert_eq!(ast, AST::forget(Box::new(AST::expr("式".to_owned()))));
    }
}

#[test]
fn memory() {
    let rule = Rule::memory;
    {
        let text = "＋式";
        let node = parse_one(rule, text).unwrap();
        let ast = PastaParser::memory(node).unwrap();
        assert_eq!(ast, AST::memory(Box::new(AST::expr("式".to_owned()))));
    }
}

#[test]
fn h_attrs() {
    let rule = Rule::h_attrs;
    fn x<S: Into<String>>(s: S) -> Box<AST> {
        let keyword = s.into();
        Box::new(AST::expr(keyword))
    }
    {
        let text = "！必須？どれか＋追加＠関数";
        let node = parse_one(rule, text).unwrap();
        let ast = PastaParser::h_attrs(node);
        let ast = ast.unwrap();

        let mut vv = Vec::new();
        vv.push(AST::require(x("必須")));
        vv.push(AST::either(x("どれか")));
        vv.push(AST::memory(x("追加")));
        vv.push(AST::action(x("関数")));
        let right = AST::attrs(vv);
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
            AST::hasira(level, title, attrs) => {
                assert_eq!(level, 3);
                assert_eq!(title, "柱");
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
            AST::hasira(level, title, attrs) => {
                assert_eq!(level, 0);
                assert_eq!(title, "アクター");
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
        assert_eq!(ast, AST::serif("セリフ＠セリフ".to_owned()));
    }
}

#[test]
fn t_attr() {
    let rule = Rule::t_attr;
    fn x<S: Into<String>>(s: S) -> Box<AST> {
        let keyword = s.into();
        Box::new(AST::expr(keyword))
    }
    {
        let text = "＠アクション";
        let node = parse_one(rule, text).unwrap();
        let ast = PastaParser::t_attr(node).unwrap();
        assert_eq!(ast, AST::action(x("アクション")));
    }
    {
        let text = "＠!アクション";
        let node = parse_one(rule, text).unwrap();
        let ast = PastaParser::t_attr(node).unwrap();
        assert_eq!(ast, AST::require(x("アクション")));
    }
}

#[test]
fn togaki() {
    let rule = Rule::togaki;
    fn action<S: Into<String>>(s: S) -> AST {
        let keyword = s.into();
        let expr = Box::new(AST::expr(keyword));
        AST::action(expr)
    }
    fn serif<S: Into<String>>(s: S) -> AST {
        AST::serif(s.into())
    }
    {
        let text = "　セリフ＠＠だね。＠アクション";
        let node = parse_one(rule, text).unwrap();
        let ast = PastaParser::togaki(node).unwrap();

        let vv = vec![serif("セリフ＠だね。"), action("アクション")];
        assert_eq!(ast, AST::togaki(vv));
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

        let a = Some(AST::hasira(1, "柱".to_owned(), None));
        let e = Some(AST::error(12, 27, '不', "不思議だね".to_owned()));
        let c = None;
        let a = y(a);
        let e = y(e);
        let c = y(c);
        let left = AST::line(a, e, c);

        assert_eq!(ast, left);
    }
    {
        let text = "　トークです。＃コメント";
        let node = parse_one(rule, text).unwrap();
        let ast = PastaParser::line(node).unwrap();

        let t_items = vec![AST::serif("トークです。".to_owned())];
        let a = Some(AST::togaki(t_items));
        let e = None;
        let c = Some(AST::comment("＃コメント".to_owned()));
        let a = y(a);
        let e = y(e);
        let c = y(c);
        let left = AST::line(a, e, c);

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
        if let AST::script(lines) = ast {
            assert_eq!(lines.len(), 18);
            match &lines[0] {
                AST::DocComment(..) => {}
                x => {
                    assert!(false, "doc_comment {:?}", x);
                }
            }
            match &lines[1] {
                AST::line(..) => {}
                x => {
                    assert!(false, "hasira {:?}", x);
                }
            }
            match &lines[2] {
                AST::line(..) => {}
                x => {
                    assert!(false, "hasira {:?}", x);
                }
            }
        } else {
            assert!(false);
        }
    }
}
