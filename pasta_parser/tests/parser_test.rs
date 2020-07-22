use pasta_parser::{parse, parse_nth, Rule};
use pest::error::Error;
use pest::iterators::{Pair, Pairs};

fn r(rule: Rule, input: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    parse(rule, input)
}
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

#[test]
fn parse21() {
    {
        let m = p(Rule::comment, "# 123");
        assert_eq!("# 123", m.as_str());
        let mut f = m.flatten();
        let m = f.nth(1).unwrap();
        assert_eq!("123", m.as_str());
    }
}

#[test]
fn parse22() {
    {
        let m = p(Rule::spaces_line, "  # 12345\r\n123");
        assert_eq!("  # 12345\r\n", m.as_str());
    }
}

#[test]
fn parse23() {
    {
        let m = p(Rule::doc_comment, "12\r\n34\r\n・柱");
        assert_eq!("12\r\n34\r\n", m.as_str());
    }
}

#[test]
fn parse24() {
    {
        let m = p(Rule::eol_check, "  #comment\r\n");
        println!("{}", m);
        let mut f = m.flatten();
        let m = f.next().unwrap();
        assert_eq!("  #comment\r\n", m.as_str());
        let m = f.next().unwrap();
        assert_eq!("#comment", m.as_str());
        let m = f.next().unwrap();
        assert_eq!("comment", m.as_str());
        assert!(f.next().is_none());
    }
    {
        let m = p(Rule::eol_check, "  エラー\r\n");
        println!("{}", m);
        let mut f = m.flatten();
        let m = f.next().unwrap();
        assert_eq!("  エラー\r\n", m.as_str());
        let m = f.next().unwrap();
        assert_eq!("エラー", m.as_str());
        let m = f.next().unwrap();
        assert_eq!("エ", m.as_str());
        let m = f.next().unwrap();
        assert_eq!("ラー", m.as_str());
        assert!(f.next().is_none());
    }
}

#[test]
fn parse31() {
    {
        let mut f = p(Rule::expr, "keyword   keyword").flatten();
        let m = f.next().unwrap();
        assert_eq!("keyword   ", m.as_str());
        let m = f.next().unwrap();
        assert_eq!("keyword", m.as_str());
        assert!(f.next().is_none());
    }
}

#[test]
fn parse32() {
    {
        let mut f = p(Rule::all_attr, "@keyword").flatten();
        f.next();
        let m = f.next().unwrap();
        assert_eq!(Rule::action, m.as_rule())
    }
    {
        let mut f = p(Rule::all_attr, "!keyword").flatten();
        f.next();
        let m = f.next().unwrap();
        assert_eq!(Rule::require, m.as_rule())
    }
    {
        let mut f = p(Rule::all_attr, "?keyword").flatten();
        f.next();
        let m = f.next().unwrap();
        assert_eq!(Rule::either, m.as_rule())
    }
    {
        let mut f = p(Rule::all_attr, "-keyword").flatten();
        f.next();
        let m = f.next().unwrap();
        assert_eq!(Rule::forget, m.as_rule())
    }
    {
        let mut f = p(Rule::all_attr, "+keyword").flatten();
        f.next();
        let m = f.next().unwrap();
        assert_eq!(Rule::memory, m.as_rule())
    }
}

#[test]
fn parse41() {
    {
        let mut f = p(Rule::hasira_start, "・・・セクション").flatten();
        let m = f.next().unwrap();
        assert_eq!("・・・", m.as_str());
        assert!(f.next().is_none());
    }
    {
        let mut f = p(Rule::hasira_start, "アクター").flatten();
        let m = f.next().unwrap();
        assert_eq!("", m.as_str());
        assert!(f.next().is_none());
    }
    {
        assert!(r(Rule::hasira_start, "　エラー").is_err());
    }
}

#[test]
fn parse42() {
    {
        let mut f = p(Rule::h_title, "柱のタイトルなの　ね").flatten();
        let m = f.next().unwrap();
        assert_eq!("柱のタイトルなの", m.as_str());
        assert!(f.next().is_none());
    }
    {
        let m = p(Rule::hasira, "・タイトル：！属性１？属性２　＃コメント1");
        println!("{}", m);
        let mut f = m;
        let m = f.next().unwrap();
        assert_eq!(Rule::hasira, m.as_rule());
        let mut f = m.into_inner();
        let m = f.next().unwrap();
        assert_eq!(Rule::hasira_start, m.as_rule());
        assert_eq!("・", m.as_str());
        let m = f.next().unwrap();
        assert_eq!(Rule::h_title, m.as_rule());
        assert_eq!("タイトル", m.as_str());
        let m = f.next().unwrap();
        assert_eq!(Rule::h_attr, m.as_rule());
        assert_eq!("！属性１", m.as_str());
        let m = f.next().unwrap();
        assert_eq!(Rule::h_attr, m.as_rule());
        assert_eq!("？属性２　", m.as_str());
        assert!(f.next().is_none());
    }
}
#[test]
fn parse52() {
    {
        let m = p(Rule::serif, "セリフっぽいのです。＠＠エスケープ大丈夫？");
        println!("{}", m);
        let mut f = m;
        let m = f.next().unwrap();
        assert_eq!(Rule::serif, m.as_rule());
        let mut f = m.into_inner();
        //
        let m = f.next().unwrap();
        assert_eq!("セリフっぽいのです。", m.as_str());
        let m = f.next().unwrap();
        assert_eq!("＠＠", m.as_str());
        let m = f.next().unwrap();
        assert_eq!("エスケープ大丈夫？", m.as_str());
        assert!(f.next().is_none());
    }
}
