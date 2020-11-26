use pasta_script::ast::*;
use squote::{quote, Literal, TokenStream};
use std::iter::{FromIterator, Iterator};

pub trait GenCode {
    fn gen(&self) -> TokenStream;
}

impl GenCode for Script {
    fn gen(&self) -> TokenStream {
        TokenStream::new()
    }
}
