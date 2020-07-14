use pasta_parser::{parse, Rule};

#[test]
fn char_test1() {
    {
        let m = parse(Rule::AT, "@").unwrap();
        println!("{:?}", m);
    }
    {
        let m = parse(Rule::AT, "＠").unwrap();
        println!("{:?}", m);
    }
    {
        let m = parse(Rule::id, "id1@").unwrap().next().unwrap();
        assert_eq!("id1", m.as_str());
    }
    {
        let m = parse(Rule::id, "id_2@").unwrap().next().unwrap();
        assert_eq!("id_2", m.as_str());
    }
    {
        let m = parse(Rule::id, "あいでー５@").unwrap().next().unwrap();
        assert_eq!("あいでー５", m.as_str());
    }
    {
        let m = parse(Rule::esc2, "#").unwrap().next().unwrap();
        assert_eq!("#", m.as_str());
    }
    {
        // esc_char の対象文字取得
        let m = parse(Rule::esc_char, "＠＠")
            .unwrap()
            .flatten()
            .nth(1)
            .unwrap();
        println!("{:?}", m);
        assert_eq!("＠", m.as_str());
    }
}

#[test]
fn char_test2() {
    {
        let m = parse(Rule::comment, "#123").unwrap().next().unwrap();
        assert_eq!("#123", m.as_str());
    }
}
