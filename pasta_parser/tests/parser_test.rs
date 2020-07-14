#[test]
fn char_test1() {
    use pasta_parser::{parse, Rule};

    {
        let m = parse(Rule::AT, "@").unwrap();
        println!("{:?}", m);
    }
    {
        let m = parse(Rule::AT, "＠").unwrap().next().unwrap();
        println!("{:?}", m);
        assert_eq!("＠", m.as_str());
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
        let m = parse(Rule::esc_char, "＠＠").unwrap().next().unwrap();
        assert_eq!("＠＠", m.as_str());
    }
}
