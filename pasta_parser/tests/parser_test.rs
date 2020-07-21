use pasta_parser::{parse, parse_nth, Rule};
use pest::iterators::{Pair, Pairs};

fn p(rule: Rule, input: &str) -> Pairs<Rule> {
    parse(rule, input).unwrap()
}
fn n(rule: Rule, n: usize, input: &str) -> Pair<Rule> {
    parse_nth(rule, n, input).unwrap()
}

#[test]
fn char_test1() {
    {
        p(Rule::AT, "@");
        p(Rule::AT, "＠");
    }
    {
        let m = n(Rule::id, 0, "id1@");
        assert_eq!("id1", m.as_str());
    }
    {
        let m = n(Rule::id, 0, "id_2@");
        assert_eq!("id_2", m.as_str());
    }
    {
        let m = n(Rule::id, 0, "あいでー５@");
        assert_eq!("あいでー５", m.as_str());
    }
    {
        let m = n(Rule::esc2, 0, "#");
        assert_eq!("#", m.as_str());
    }
    {
        // esc_char の対象文字取得
        let m = n(Rule::esc_char, 1, "＠＠");
        println!("{:?}", m);
        assert_eq!("＠", m.as_str());
    }
}

#[test]
fn comment_test() {
    {
        let m = n(Rule::comment, 0, "#123");
        assert_eq!("#123", m.as_str());
    }
    {
        let m = n(Rule::spaces_line, 0, "#123");
        assert_eq!("#123", m.as_str());
    }
    {
        let m = n(Rule::spaces_line, 0, "   #123");
        assert_eq!("   #123", m.as_str());
    }
    {
        let m = n(Rule::spaces_line, 0, "   ");
        assert_eq!("   ", m.as_str());
    }
    {
        let m = n(Rule::doc_comment, 0, "123\nABC\n・柱");
        assert_eq!("123\nABC\n", m.as_str());
    }
}

#[test]
fn parse11() {
    {
        let m = p(Rule::esc1, "@");
        assert_eq!("", m.as_str());
    }
    {
        let m = p(Rule::esc2, "@");
        assert_eq!("@", m.as_str());
    }
    {
        let m = parse(Rule::esc_char, "@#").unwrap();
        println!("{:?}", m);
        let m = m.flatten().nth(1).unwrap();
        println!("{:?}", m);
        assert_eq!("#", m.as_str());
    }
}
