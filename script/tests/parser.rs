use pasta_script::{parse_node, PastaParser, Rule, AST};

#[test]
fn escape_test() {
    let text = "＠＠";
    let node = parse_node(Rule::escape, text).unwrap().single().unwrap();
    let ast = PastaParser::escape(node).unwrap();
    assert_eq!(ast, AST::escape('＠'));
}

#[test]
fn comment_test() {
    let text = "#コメントです。";
    let node = parse_node(Rule::comment, text).unwrap().single().unwrap();
    let ast = PastaParser::comment(node).unwrap();
    assert_eq!(ast, AST::comment("#コメントです。"));
}
