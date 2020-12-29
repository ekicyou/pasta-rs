use async_trait::*;
use macro_test::gen_sample::*;
use pasta_core::Scriptor;
use rand;
use std::collections::HashSet;
use std::ops::Range;

struct TestScriptor {
    tags: HashSet<String>,
}

#[async_trait]
impl Scriptor for TestScriptor {
    /// トーク開始
    async fn start(&mut self) {}

    /// アクター切替
    fn actor(&mut self, _t: &str) {}

    /// アクション（表情）の指定
    fn action(&mut self, _t: &str) {}

    /// セリフの指定
    fn serif(&mut self, _t: &str) {}

    /// タグを取得
    fn tags(&self) -> &HashSet<String> {
        &self.tags
    }

    /// タグを取得
    fn tags_mut(&mut self) -> &mut HashSet<String> {
        &mut self.tags
    }

    /// タグ要素を覚える
    fn memory(&mut self, _tag: &str) {}

    /// タグ要素を忘れる
    fn forget(&mut self, _tag: &str) {}

    /// タグ要素の記憶・忘却の確定
    fn commit_tags(&mut self) {}

    /// u32の乱数を返す
    fn rand_u32(&self) -> u32 {
        0u32
    }

    /// u32の乱数を返す
    fn rand_i32(&self) -> i32 {
        0
    }

    /// f32の乱数を返す
    fn rand_f32(&self) -> f32 {
        0.0
    }

    /// f64の乱数を返す
    fn rand_f64(&self) -> f64 {
        0.0
    }

    /// usizeの範囲で乱数を返す
    fn rand_range_usize(&self, range: Range<usize>) -> usize {
        range.start
    }

    /// f64の範囲で乱数を返す
    fn rand_range_f64(&self, range: Range<f64>) -> f64 {
        range.start
    }
}

#[test]
fn rund_test() {
    let a = rand::random::<u32>();
    println!("{}", a);
}
