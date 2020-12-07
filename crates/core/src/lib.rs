use async_trait::*;
use rand::rngs::ThreadRng;

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
    fn tags(&self) -> &[String];
    /// タグ要素を覚える
    fn memory(&mut self, tag: &str);
    /// タグ要素を忘れる
    fn forget(&mut self, tag: &str);
    /// タグ要素の記憶・忘却の確定
    fn commit_tags(&mut self);

    /// 乱数ジェネレータを返す。
    fn thread_rng(&self) -> ThreadRng;
}
