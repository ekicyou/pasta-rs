use csv;
use pasta_core::WordDic;
use squote::{quote, TokenStream};

pub fn new_word_dic() -> WordDic {
    WordDic::new()
}

pub fn push_word_dic(dic: &mut WordDic, data: &str) {
    let mut rdr = csv::ReaderBuilder::new()
        .comment(Some(b'#'))
        .has_headers(true)
        .trim(csv::Trim::All)
        .terminator(csv::Terminator::CRLF)
        .from_reader(data.as_bytes());
    let header = rdr.headers().unwrap();
    let header: Vec<_> = header.into_iter().map(|a| a.to_owned()).collect();
    for rec in rdr.records() {
        let rec = rec.unwrap();
        let rec: Vec<_> = rec.into_iter().collect();
        dic.push(header.as_slice(), rec.as_slice());
    }
}

pub fn gen_word_dic(dic: WordDic) -> TokenStream {
    let mut push = TokenStream::new();
    let map = dic.map();
    for (k, items) in map {
        push.combine(&quote! {
            let k = #k;
        });
        for v in items {
            push.combine(&quote! {
                dic.push_kv(k, #v);
            });
        }
    }
    quote! {
        pub fn create_word_dic() -> ::pasta_core::WordDic{
            let mut dic = ::pasta_core::WordDic::new();
            #push
            dic
        }
    }
}
