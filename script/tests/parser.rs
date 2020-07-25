use pasta_script::{parse_one, PastaParser, Rule, AST};

#[test]
fn escape() {
    let rule = Rule::escape;
    let text = "＠＠";
    let node = parse_one(rule, text).unwrap();
    let ast = PastaParser::escape(node).unwrap();
    assert_eq!(ast, AST::escape('＠'));
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
