#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("shiori.pest");

#[derive(Parser)]
#[grammar = "shiori.pest"]
pub struct ShioriParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;

    #[test]
    fn id_1() {
        let items = ShioriParser::parse(Rule::id, "a1")
            .unwrap_or_else(|e| panic!("{}", e))
            .collect::<Vec<_>>();
        assert_eq!(items.len(), 1);
        {
            let pair = &items[0];
            assert_eq!(pair.as_rule(), Rule::id);
            let span = pair.clone().into_span();
            assert_eq!(span.as_str(), "a1");
            assert_eq!(span.start(), 0);
            assert_eq!(span.end(), 2);
        }
    }
    #[test]
    fn id_2() {
        let items = ShioriParser::parse(Rule::id, "感じの良いID")
            .unwrap_or_else(|e| panic!("{}", e))
            .collect::<Vec<_>>();
        assert_eq!(items.len(), 1);
        {
            let pair = &items[0];
            assert_eq!(pair.as_rule(), Rule::id);
            let span = pair.clone().into_span();
            assert_eq!(span.as_str(), "感じの良いID");
            assert_eq!(span.start(), 0);
            assert_eq!(span.end(), 17);
        }
    }

    #[test]
    fn remain_1() {
        let items = ShioriParser::parse(Rule::remain, "ABC\r\n")
            .unwrap_or_else(|e| panic!("{}", e))
            .collect::<Vec<_>>();
        assert_eq!(items.len(), 1);
        {
            let pair = &items[0];
            assert_eq!(pair.as_rule(), Rule::remain);
            let span = pair.clone().into_span();
            assert_eq!(span.as_str(), "ABC");
            assert_eq!(span.start(), 0);
            assert_eq!(span.end(), 3);
        }
    }

    #[test]
    fn remain_2() {
        let items = ShioriParser::parse(Rule::remain, "ABC\rABCD\r\n")
            .unwrap_or_else(|e| panic!("{}", e))
            .collect::<Vec<_>>();
        assert_eq!(items.len(), 1);
        {
            let pair = &items[0];
            assert_eq!(pair.as_rule(), Rule::remain);
            let span = pair.clone().into_span();
            assert_eq!(span.as_str(), "ABC\rABCD");
            assert_eq!(span.start(), 0);
            assert_eq!(span.end(), 8);
        }
    }

    #[test]
    fn method_1() {
        let items = ShioriParser::parse(Rule::method, "GET")
            .unwrap_or_else(|e| panic!("{}", e))
            .collect::<Vec<_>>();
        assert_eq!(items.len(), 1);
        {
            let pair = &items[0];
            assert_eq!(pair.as_rule(), Rule::method);
            let span = pair.clone().into_span();
            assert_eq!(span.as_str(), "GET");
            assert_eq!(span.start(), 0);
            assert_eq!(span.end(), 3);
            let items = pair.clone().into_inner().collect::<Vec<_>>();
            assert_eq!(items.len(), 1);
            let pair = &items[0];
            assert_eq!(pair.as_rule(), Rule::get);
        }
    }

    #[test]
    fn method_2() {
        let items = ShioriParser::parse(Rule::method, "NOTIFY")
            .unwrap_or_else(|e| panic!("{}", e))
            .collect::<Vec<_>>();
        assert_eq!(items.len(), 1);
        {
            let pair = &items[0];
            assert_eq!(pair.as_rule(), Rule::method);
            let span = pair.clone().into_span();
            assert_eq!(span.as_str(), "NOTIFY");
            assert_eq!(span.start(), 0);
            assert_eq!(span.end(), 6);
            let items = pair.clone().into_inner().collect::<Vec<_>>();
            assert_eq!(items.len(), 1);
            let pair = &items[0];
            assert_eq!(pair.as_rule(), Rule::notify);
        }
    }

    #[test]
    fn header_1() {
        let items = ShioriParser::parse(Rule::header, "GET OPEN SHIORI/2.6\r\n")
            .unwrap_or_else(|e| panic!("{}", e))
            .collect::<Vec<_>>();
        assert_eq!(items.len(), 1);
        {
            let pair = &items[0];
            assert_eq!(pair.as_rule(), Rule::header);
            let span = pair.clone().into_span();
            assert_eq!(span.as_str(), "GET OPEN SHIORI/2.6\r\n");
            assert_eq!(span.start(), 0);
            assert_eq!(span.end(), 21);
            let items = pair.clone().into_inner().collect::<Vec<_>>();
            assert_eq!(items.len(), 2);
            let pair = &items[0];
            assert_eq!(pair.as_rule(), Rule::method);
            {
                let items = pair.clone().into_inner().collect::<Vec<_>>();
                assert_eq!(items.len(), 1);
                let pair = &items[0];
                assert_eq!(pair.as_rule(), Rule::get);
            }
            let pair = &items[1];
            assert_eq!(pair.as_rule(), Rule::header2);
            {
                let items = pair.clone().into_inner().collect::<Vec<_>>();
                assert_eq!(items.len(), 2);
                let pair = &items[0];
                assert_eq!(pair.as_rule(), Rule::id);
                assert_eq!(pair.as_str(), "OPEN");
                let pair = &items[1];
                assert_eq!(pair.as_rule(), Rule::ver);
                assert_eq!(pair.as_str(), "6");
            }
        }
    }

    #[test]
    fn header_2() {
        let items = ShioriParser::parse(Rule::header, "NOTIFY SHIORI/3.0\r\n")
            .unwrap_or_else(|e| panic!("{}", e))
            .collect::<Vec<_>>();
        assert_eq!(items.len(), 1);
        {
            let pair = &items[0];
            assert_eq!(pair.as_rule(), Rule::header);
            let span = pair.clone().into_span();
            assert_eq!(span.as_str(), "NOTIFY SHIORI/3.0\r\n");
            assert_eq!(span.start(), 0);
            assert_eq!(span.end(), 19);
            let items = pair.clone().into_inner().collect::<Vec<_>>();
            assert_eq!(items.len(), 2);
            let pair = &items[0];
            assert_eq!(pair.as_rule(), Rule::method);
            {
                let items = pair.clone().into_inner().collect::<Vec<_>>();
                assert_eq!(items.len(), 1);
                let pair = &items[0];
                assert_eq!(pair.as_rule(), Rule::notify);
            }
            let pair = &items[1];
            assert_eq!(pair.as_rule(), Rule::header3);
        }
    }


}
