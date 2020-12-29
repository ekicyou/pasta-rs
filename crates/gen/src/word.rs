use csv;
use squote::{quote, TokenStream};

fn gen_csv_rec(rec: &csv::StringRecord) -> TokenStream {
    let mut ts = TokenStream::new();
    for a in rec {
        ts.combine(&quote! { #a, });
    }
    quote! {
        &[#ts][..]
    }
}

fn gen_push(data: &str) -> TokenStream {
    let mut rdr = csv::ReaderBuilder::new()
        .comment(Some(b'#'))
        .has_headers(true)
        .trim(csv::Trim::All)
        .terminator(csv::Terminator::CRLF)
        .from_reader(data.as_bytes());
    let header = rdr.headers().unwrap();
    let header = gen_csv_rec(header);
    let header = quote! {
        let header = #header;
    };
    let mut push = TokenStream::new();
    for rec in rdr.records() {
        let rec = rec.unwrap();
        let rec = gen_csv_rec(&rec);
        push.combine(&quote! {
            dic.push(header, #rec);
        });
    }
    quote! {
        #header
        #push
    }
}

pub fn gen_word_dic(codes: &[String]) -> TokenStream {
    let mut push_csv = TokenStream::new();
    for code in codes {
        push_csv.combine(&gen_push(code));
    }
    quote! {
        pub fn create_word_dic() -> ::pasta_core::WordDic{
            let mut dic = ::pasta_core::WordDic::new();
            #push_csv
            dic
        }
    }
}
