mod word;

pub use word::WordDic;

use async_trait::*;
use std::collections::HashSet;
use std::ops::Range;

#[async_trait]
pub trait Scriptor {
    /// アクター切替
    fn actor(&mut self, t: &str);
    /// アクション（表情）の指定
    fn action(&mut self, t: &str);
    /// セリフの指定
    fn serif(&mut self, t: &str);
    /// トーク開始
    async fn start(&mut self);
    /// タグを取得
    fn tags(&self) -> &HashSet<String>;
    /// 書き込み可能でタグを取得
    fn tags_mut(&mut self) -> &mut HashSet<String>;
    /// タグ要素を覚える
    fn memory(&mut self, tag: &str);
    /// タグ要素を忘れる
    fn forget(&mut self, tag: &str);
    /// タグ要素の記憶・忘却の確定
    fn commit_tags(&mut self);

    /// u32の乱数を返す
    fn rand_u32(&self) -> u32;

    /// u32の乱数を返す
    fn rand_i32(&self) -> i32;

    /// f32の乱数を返す
    fn rand_f32(&self) -> f32;

    /// f64の乱数を返す
    fn rand_f64(&self) -> f64;

    /// usizeの範囲で乱数を返す
    fn rand_range_usize(&self, range: Range<usize>) -> usize;

    /// f64の範囲で乱数を返す
    fn rand_range_f64(&self, range: Range<f64>) -> f64;
}
