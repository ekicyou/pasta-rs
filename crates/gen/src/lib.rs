mod block;
mod h_checks;
mod rand_jump;
mod script;
mod utils;
mod word;

pub use crate::script::gen_code as gen_pasta_code;
pub use crate::script::gen_codes as gen_pasta_codes;
pub use crate::word::gen_word_dic;

use squote::{quote, TokenStream};
pub fn gen(pasta_codes: &[String], csv_codes: &[String]) -> TokenStream {
    let pasta = gen_pasta_codes(pasta_codes);
    let word = gen_word_dic(csv_codes);
    quote! {
        #pasta
        #word
    }
}
