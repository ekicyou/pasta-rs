use crate::attr::Attr;
use std::vec::Vec;

/// 柱情報
#[derive(Debug, Clone)]
pub struct Hasira {
    /// 柱レベル
    pub level: i32,

    /// 柱の名前
    pub name: String,

    /// 柱の属性
    pub attrs: Vec<Attr>,
}
