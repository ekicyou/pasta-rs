use log::*;

#[test]
fn char_test1() {
    use pasta_parser::{PastaParser, Rule};
    use pest::Parser;

    {
        let m = PastaParser::parse(Rule::field, "-273.15").unwrap();
        debug!("{:?}", m);
    }
    {
        let m = PastaParser::parse(Rule::AT, "@").unwrap();
        debug!("{:?}", m);
    }
}
