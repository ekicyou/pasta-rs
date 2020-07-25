use pasta_script::{parse_node, PastaParser, Rule, AST};

#[test]
fn escape() {
    let text = "＠＠";
    let node = parse_node(Rule::escape, text).unwrap().single().unwrap();
    let ast = PastaParser::escape(node).unwrap();
    assert_eq!(ast, AST::escape('＠'));
}

#[test]
fn error() {
    let text = "＠エラーです";
    let node = parse_node(Rule::error, text).unwrap().single().unwrap();
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
    let text = "#コメントです。";
    let node = parse_node(Rule::comment, text).unwrap().single().unwrap();
    let ast = PastaParser::comment(node).unwrap();
    assert_eq!(ast, AST::comment(text.to_owned()));
}
