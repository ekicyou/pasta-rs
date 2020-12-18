mod block;
mod h_checks;
mod rand_jump;
mod script;
mod utils;
mod word;

pub use crate::script::gen_code as gen_pasta_code;
pub use crate::script::gen_codes as gen_pasta_codes;
pub use crate::word::gen_word_dic;
pub use crate::word::new_word_dic;
pub use crate::word::push_word_dic;

use squote::{quote, TokenStream};
pub fn gen(pasta_codes: &[String], csv_codes: &[String]) -> TokenStream {
    let pasta = gen_pasta_codes(pasta_codes);

    let mut dic = new_word_dic();
    for csv in csv_codes {
        push_word_dic(&mut dic, csv);
    }
    let word = gen_word_dic(dic);

    quote! {
        #pasta
        #word
    }
}
