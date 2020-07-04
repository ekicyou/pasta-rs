use nom::IResult;
use super::enums::Token;

// chars
named!(sp  <&str,&str>, tag_s!(" "));
named!(crlf<&str,&str>, tag_s!("\r\n"));
named!(identifier<&str,&str>, re_find_static!(
    concat!(
        r"^",
        r"[\p{Lu}\p{Ll}\p{Lt}\p{Lm}\p{Lo}\p{Nl}]",
        r"[\p{Lu}\p{Ll}\p{Lt}\p{Lm}\p{Lo}\p{Nl}\p{Mn}\p{Mc}\p{Nd}\p{Pc}\p{Cf}]*",
    )
));
named!(key<&str,&str>, re_find_static!(
    concat!(
        r"^",
        r"[\p{Lu}\p{Ll}\p{Lt}\p{Lm}\p{Lo}\p{Nl}]",
        r"[\p{Lu}\p{Ll}\p{Lt}\p{Lm}\p{Lo}\p{Nl}\p{Mn}\p{Mc}\p{Nd}\p{Pc}\p{Cf}]*",

        r"([-.]",
        r"[\p{Lu}\p{Ll}\p{Lt}\p{Lm}\p{Lo}\p{Nl}]",
        r"[\p{Lu}\p{Ll}\p{Lt}\p{Lm}\p{Lo}\p{Nl}\p{Mn}\p{Mc}\p{Nd}\p{Pc}\p{Cf}]*",
        r")*",
    )
));
named!(shiori2_re<&str,&str>, re_find_static!(r"^SHIORI/2\.[0-9]"));

// token
named!(shiori2<&str,Token>, map!( shiori2_re          , |a| Token::SHIORI2(a)));
named!(shiori3<&str,Token>, map!( tag_s!("SHIORI/3.0"), |_| Token::SHIORI3));
named!(get    <&str,Token>, map!( tag_s!("GET"       ), |_| Token::GET    ));
named!(notify <&str,Token>, map!( tag_s!("NOTIFY"    ), |_| Token::NOTIFY ));
named!(method <&str,Token>, alt!(get | notify));

// header3
named!(pub header3<&str,(Token,Token)>,
    do_parse!(
        method: method                  >>
                sp                      >>
                shiori3                 >>
                crlf                    >>
        (Token::SHIORI3, method)
    )
);

// header2
named!(pub header2<&str,(Token,Token,&str)>,
    do_parse!(
        method: method                  >>
                sp                      >>
        id:     identifier              >>
                sp                      >>
        shiori: shiori2                 >>
                crlf                    >>
        (shiori, method, id)
    )
);

// keyValue
named!(pub kv     <&str,Token>,
    do_parse!(
        key:    key                     >>
                tag_s!(": ")            >>
        value:  take_until_s!("\r\n")   >>
                crlf                    >>
        (Token::KV(key, value))
    )
);


#[test]
fn chars_test() {
    {
        let text = "GET 123";
        assert_eq!(identifier(text), IResult::Done(" 123","GET"));
    }
    {
        let text = "三時の.あなたは、いい人　ね";
        assert_eq!(identifier(text), IResult::Done(".あなたは、いい人　ね","三時の"));
        assert_eq!(key(text), IResult::Done("、いい人　ね","三時の.あなたは"));
    }
}

#[test]
fn token_test() {
    {
        let text = "GET";
        assert_eq!(get(text), IResult::Done("",Token::GET));
    }
    {
        let text = "GET123";
        assert_eq!(get(text), IResult::Done("123",Token::GET));
    }
    {
        let text = "XXX";
        assert!(get(text).is_err());
    }
    {
        let text = "SHIORI/3.0";
        assert!(shiori3(text).is_done());
    }
    {
        let text = "SHIORI/2.5";
        assert_eq!(shiori2(text), IResult::Done("",Token::SHIORI2(&text)));
    }
    {
        let text = "NOTIFY";
        assert!(notify(text).is_done());
    }
    {
        let text = "GET";
        assert_eq!(method(text), IResult::Done("",Token::GET));
    }
    {
        let text = "NOTIFY";
        assert_eq!(method(text), IResult::Done("",Token::NOTIFY));
    }
    {
        let text = "Charset: UTF-8\r\n";
        assert_eq!(kv(text), IResult::Done("",Token::KV("Charset", "UTF-8")));
    }
    {
        let text = "Reference0: いい日旅立ち\r\n";
        assert_eq!(kv(text), IResult::Done("",Token::KV("Reference0", "いい日旅立ち")));
    }
    {
        let text = "ダメ文字: ー ソ 十 表\r\n";
        assert_eq!(kv(text), IResult::Done("",Token::KV("ダメ文字", "ー ソ 十 表")));
    }
    {
        let text = "GET SHIORI/3.0\r\n";
        assert_eq!(header3(text), IResult::Done("",(Token::SHIORI3,Token::GET)));
    }
    {
        let text = "NOTIFY SHIORI/3.0\r\n";
        assert_eq!(header3(text), IResult::Done("",(Token::SHIORI3,Token::NOTIFY)));
    }
    {
        let text = "GET Version SHIORI/2.6\r\n";
        assert_eq!(header2(text), IResult::Done("",(
            Token::SHIORI2("SHIORI/2.6"),
            Token::GET,
            "Version",
        )));
    }
}
