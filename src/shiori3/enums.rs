#[derive(PartialEq,Eq,Debug,Copy,Clone)]
pub enum Token<'a> {
    KV(&'a str, &'a str),
    GET,
    NOTIFY,
    SHIORI3,
    SHIORI2(&'a str),
    NONE,
}
