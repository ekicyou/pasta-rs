use nom::IResult;


// chars
named!(identifier<&str,&str>, re_find_static!(
    concat!(
        r"^",
        r"[\p{Lu}\p{Ll}\p{Lt}\p{Lm}\p{Lo}\p{Nl}]",
        r"[\p{Lu}\p{Ll}\p{Lt}\p{Lm}\p{Lo}\p{Nl}\p{Mn}\p{Mc}\p{Nd}\p{Pc}\p{Cf}]*",
    )
));
named!(sp  <&str,&str>, re_find_static!(r"^[\t\v\f \xA0\x{FEFF}\p{Zs}]+"));
named!(eol <&str,&str>, re_find_static!(r"^((\r?\n)|[\r\x{2028}\x{2029}]|\z)"));

#[test]
fn chars_test() {
    {
        let text = "あのよろし1番";
        assert_eq!(identifier(text), IResult::Done("",text));
    }
    {
        let text = " 　\t";
        assert_eq!(sp(text), IResult::Done("",text));
    }
    {
        let text = "\r\n";
        assert_eq!(eol(text), IResult::Done("",text));
    }
    {
        let text = "\r";
        assert_eq!(eol(text), IResult::Done("",text));
    }
    {
        let text = "\n";
        assert_eq!(eol(text), IResult::Done("",text));
    }
    {
        let text = "\u{2028}";
        assert_eq!(eol(text), IResult::Done("",text));
    }
    {
        let text = "\u{2029}";
        assert_eq!(eol(text), IResult::Done("",text));
    }
    {
        let text = "";
        assert_eq!(eol(text), IResult::Done("",text));
    }
}


// mark
named!(at   <&str,&str>, re_find_static!(r"^[@＠]"));
named!(sharp<&str,&str>, re_find_static!(r"^[#＃]"));
named!(dash <&str,&str>, re_find_static!(r"^[-‐–—―−－ー]"));
named!(coron<&str,&str>, re_find_static!(r"^[：:]"));

#[test]
fn mark_test() {
    {
        let text = "@";
        assert_eq!(at(text), IResult::Done("",text));
    }
    {
        let text = "＠";
        assert_eq!(at(text), IResult::Done("",text));
    }
    {
        let text = "#";
        assert_eq!(sharp(text), IResult::Done("",text));
    }
    {
        let text = "＃";
        assert_eq!(sharp(text), IResult::Done("",text));
    }
    {
        let text = "-";
        assert_eq!(dash(text), IResult::Done("",text));
    }
    {
        let text = ":";
        assert_eq!(coron(text), IResult::Done("",text));
    }
}
