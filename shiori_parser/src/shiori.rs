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
        }
    }
}
