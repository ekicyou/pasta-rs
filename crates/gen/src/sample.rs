use once_cell::sync::Lazy;
use std::collections::HashMap;

static LEN_MAP: Lazy<HashMap<String, usize>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.entry("KEY1".to_owned()).or_insert(2);
    m.entry("KEY2".to_owned()).or_insert(5);
    m.entry("KEY3".to_owned()).or_insert(12);
    m
});

pub enum LT {
    H1,
    H2,
    H3,
}

static FN_MAP: Lazy<HashMap<String, Vec<dyn Fn(&[String]) -> Option<LT>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.entry("KEY1".to_owned()).or_insert(2);
    m.entry("KEY2".to_owned()).or_insert(5);
    m.entry("KEY3".to_owned()).or_insert(12);
    m
});

mod checks {
    use super::LT;
    pub fn H1(tags: &[String]) -> Option<LT> {
        Some(LT::H1)
    }
    pub fn H2(tags: &[String]) -> Option<LT> {
        Some(LT::H1)
    }
    pub fn H3(tags: &[String]) -> Option<LT> {
        Some(LT::H1)
    }
}
