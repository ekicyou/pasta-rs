use pasta_script::{parse, parse_nth, Rule};
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
        let m = n(Rule::esc_char, 0, "#");
        assert_eq!("#", m.as_str());
    }
    {
        // escape の対象文字取得
        let m = n(Rule::escape, 1, "＠＠");
        println!("{:?}", m);
        assert_eq!("＠", m.as_str());
    }
}

#[test]
fn comment_test() {
    {
        let m = n(Rule::comment, 0, "#123");
        println!("{}", m);
        assert_eq!("#123", m.as_str());
    }
    {
        let m = n(Rule::spaces_line, 0, "#123\n");
        println!("{}", m);
        assert_eq!("#123\n", m.as_str());
    }
    {
        let m = n(Rule::spaces_line, 0, "   #123\n");
        println!("{}", m);
        assert_eq!("   #123\n", m.as_str());
    }
    {
        let m = n(Rule::spaces_line, 0, "   \n");
        println!("{}", m);
        assert_eq!("   \n", m.as_str());
    }
    {
        let m = n(Rule::doc_comment, 0, "123\nABC\n＠柱");
        println!("{}", m);
        assert_eq!("123\nABC\n", m.as_str());
    }
}

#[test]
fn parse11() {
    {
        let m = p(Rule::esc_mark, "@");
        assert_eq!("", m.as_str());
    }
    {
        let m = p(Rule::esc_char, "@");
        assert_eq!("@", m.as_str());
    }
    {
        let m = parse(Rule::escape, "@#").unwrap();
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
        let m = p(Rule::err_or_comment, "  #comment\r\n");
        println!("{}", m);
        let mut f = m.flatten();
        let m = f.next().unwrap();
        assert_eq!("#comment", m.as_str());
        let m = f.next().unwrap();
        assert_eq!("comment", m.as_str());
        assert!(f.next().is_none());
    }
    {
        let m = p(Rule::err_or_comment, "  エラー\r\n");
        println!("{}", m);
        let mut f = m.flatten();
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
        println!("{}", m);
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
        let mut f = p(Rule::hasira_level, "＠＠＠セクション").flatten();
        let m = f.next().unwrap();
        assert_eq!("＠＠＠", m.as_str());
        assert!(f.next().is_none());
    }
    {
        let mut f = p(Rule::hasira_header, "＠柱タイトル　");
        println!("{}", f);
        let m = f.next().unwrap();
        assert_eq!("＠", m.as_str());
        let m = f.next().unwrap();
        assert_eq!("柱タイトル", m.as_str());
        assert!(f.next().is_none());
    }
    {
        assert!(r(Rule::hasira_header, "　エラー").is_err());
    }
}

#[test]
fn parse42() {
    {
        let mut f = p(Rule::actor_header, "アクター");
        println!("{}", f);
        let m = f.next().unwrap();
        assert_eq!("アクター", m.as_str());
        assert!(f.next().is_none());
    }
    {
        assert!(r(Rule::actor_header, "　エラー").is_err());
    }
}

#[test]
fn parse43() {
    {
        let m = p(
            Rule::hasira,
            "＠タイトル　！属性１？属性２　＃コメント1\r\n",
        );
        let mut f = m;
        let m = f.next().unwrap();
        assert_eq!(Rule::hasira, m.as_rule());
        let mut f = m.into_inner();
        let m = f.next().unwrap();
        assert_eq!(Rule::hasira_level, m.as_rule());
        assert_eq!("＠", m.as_str());
        let m = f.next().unwrap();
        assert_eq!(Rule::hasira_title, m.as_rule());
        assert_eq!("タイトル", m.as_str());

        let m = f.next().unwrap();
        assert_eq!(Rule::h_attrs, m.as_rule());
        assert_eq!("！属性１？属性２　", m.as_str());
        assert!(f.next().is_none());
    }
    {
        let mut f = p(Rule::hasira, "＠空白区切り　　＠属性");
        let m = f.next().unwrap();
        assert_eq!(m.as_rule(), Rule::hasira);
        let mut f = m.into_inner();
        assert_eq!(f.next().unwrap().as_rule(), Rule::hasira_level);
        assert_eq!(f.next().unwrap().as_rule(), Rule::hasira_title);
        assert_eq!(f.next().unwrap().as_rule(), Rule::h_attrs);
        assert!(f.next().is_none());
    }
    {
        let mut f = p(Rule::hasira, "＠＠＠  ！起動トーク\r\n");
        let m = f.next().unwrap();
        assert_eq!(m.as_rule(), Rule::hasira);
        let mut f = m.into_inner();
        assert_eq!(f.next().unwrap().as_rule(), Rule::hasira_level);
        assert_eq!(f.next().unwrap().as_rule(), Rule::h_attrs);
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
    {
        let m = p(Rule::serif, "セリフっぽいのです。＠エスケープ大丈夫？");
        println!("{}", m);
        let mut f = m;
        let m = f.next().unwrap();
        assert_eq!(Rule::serif, m.as_rule());
        let mut f = m.into_inner();
        //
        let m = f.next().unwrap();
        assert_eq!("セリフっぽいのです。", m.as_str());
        assert!(f.next().is_none());
    }
}

#[test]
fn parse53() {
    {
        let m = p(
            Rule::togaki,
            "　　セリフっぽいのです。＠属性　＠！必須　＠＠エスケープ大丈夫？",
        );
        let mut f = m;
        let m = f.next().unwrap();
        assert_eq!(Rule::togaki, m.as_rule());
        let mut f = m.into_inner();
        //
        let m = f.next().unwrap();
        println!("{}", m);
        assert_eq!("セリフっぽいのです。", m.as_str());
        let m = f.next().unwrap();
        println!("{}", m);
        assert_eq!("＠属性　", m.as_str());
        let m = f.next().unwrap();
        println!("{}", m);
        assert_eq!("！必須　", m.as_str());
        let m = f.next().unwrap();
        println!("{}", m);
        assert_eq!("＠＠エスケープ大丈夫？", m.as_str());
        assert!(f.next().is_none());
    }
}

#[test]
fn parse61_1() {
    {
        let m = p(Rule::line, "＠タイトル　！属性１？属性２　＃コメント1\r\n");
        let mut f = m;
        let m = f.next().unwrap();
        assert_eq!(Rule::line, m.as_rule());
        let mut f = m.into_inner();
        let m = f.next().unwrap();
        assert_eq!(Rule::hasira, m.as_rule());
        let m = f.next().unwrap();
        assert_eq!(Rule::comment, m.as_rule());
        assert!(f.next().is_none());
    }
    {
        let m = p(
            Rule::line,
            "　　セリフっぽいのです。＠属性　＃コメント1\r\n",
        );
        let mut f = m;
        let m = f.next().unwrap();
        assert_eq!(Rule::line, m.as_rule());
        let mut f = m.into_inner();
        let m = f.next().unwrap();
        assert_eq!(Rule::togaki, m.as_rule());
        let m = f.next().unwrap();
        assert_eq!(Rule::comment, m.as_rule());
        assert!(f.next().is_none());
    }
    {
        let m = p(
            Rule::line,
            "　　セリフっぽいのです。＠属性　＠＊おかしい？＃コメント\r\n",
        );
        let mut f = m;
        let m = f.next().unwrap();
        assert_eq!(Rule::line, m.as_rule());
        let mut f = m.into_inner();
        let m = f.next().unwrap();
        println!("[{}]", m);
        assert_eq!(Rule::togaki, m.as_rule());
        let m = f.next().unwrap();
        println!("[{}]", m);
        assert_eq!(Rule::error, m.as_rule());
        assert!(f.next().is_none());
    }
}

#[test]
fn parse61_2() {
    let text = "＠＠＠  ！起動トーク\r\n";
    {
        let mut f = r(Rule::hasira, text).unwrap();
        let m = f.next().unwrap();
        assert_eq!(m.as_rule(), Rule::hasira);
        let mut f = m.into_inner();
        assert_eq!(f.next().unwrap().as_rule(), Rule::hasira_level);
        assert_eq!(f.next().unwrap().as_rule(), Rule::h_attrs);
        assert!(f.next().is_none());
    }
    {
        let m = r(Rule::line, text).unwrap();
        let mut f = m;
        let m = f.next().unwrap();
        assert_eq!(Rule::line, m.as_rule());
        let mut f = m.into_inner();
        let m = f.next().unwrap();
        println!("[{}]", m);
        assert_eq!(Rule::hasira, m.as_rule());
        assert!(f.next().is_none());
    }
}

#[test]
fn parse62_1() {
    {
        let text = r#"パスタスクリプトテスト構文

最初の柱まではドキュメントコメントとします。

＠タイトル　！属性１？属性２　＃コメント
"#;
        let mut f = p(Rule::script, text);
        println!("[{}]", f);
        let m = f.next().unwrap();
        println!("[{}]", m);
        println!("[{}]", m.as_str());
        assert_eq!(Rule::doc_comment, m.as_rule());
        let m = f.next().unwrap();
        println!("[{}]", m);
        println!("[{}]", m.as_str());
        assert_eq!(Rule::line, m.as_rule());
        assert!(f.next().is_none());
    }
}
#[test]
fn parse62_2() {
    {
        let text = include_str!("parse62.pasta");
        let mut f = p(Rule::script, text);
        println!("[{}]", f);
        let m = f.next().unwrap();
        println!("[{}]", m);
        println!("[{}]", m.as_str());
        assert_eq!(Rule::doc_comment, m.as_rule());
        let m = f.next().unwrap();
        println!("[{}]", m);
        assert_eq!(Rule::line, m.as_rule());
    }
}
