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
