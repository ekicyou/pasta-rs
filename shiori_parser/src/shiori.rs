use pest::Parser;

#[derive(Parser)]
#[grammar = "shiori.pest"]
pub struct ShioriParser;

#[test]
fn test_shiori_parser() {
    let pairs = ShioriParser::parse(Rule::id, "a1").unwrap_or_else(|e| panic!("{}", e));

    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.clone().into_span());
        println!("Text:    {}", pair.clone().into_span().as_str());
    }
}
