//#[test]
fn print_matchs() {
    use pasta_script::{parse, Rule};

    fn match_chars(rule: Rule) -> String {
        let mut buf = String::new();
        let mut s = String::with_capacity(6);
        for c in '\u{0000}'..'\u{ffff}' {
            s.clear();
            s.push(c);
            match parse(rule, &s) {
                Ok(_) => buf.push(c),
                _ => {}
            }
        }
        buf
    }
    fn print(rule: Rule) {
        println!("{:?}: {}", rule, match_chars(rule));
    }

    print(Rule::Pc);
    print(Rule::Pd);
    print(Rule::Ps);
    print(Rule::Pe);
    print(Rule::Pi);
    print(Rule::Pf);
    print(Rule::Po);
}
