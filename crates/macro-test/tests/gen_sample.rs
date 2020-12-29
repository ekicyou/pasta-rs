use async_trait::*;
use futures::channel::mpsc::{channel, Receiver, Sender};
use futures::executor::LocalPool;
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use futures::task::LocalSpawnExt;
use macro_test::gen_sample::*;
use pasta_core::Scriptor;
use std::collections::HashSet;
use std::ops::Range;

struct TestScriptor {
    tags: HashSet<String>,
    talk: String,
    tx: Sender<String>,
}

impl TestScriptor {
    pub fn new() -> (Self, Receiver<String>) {
        let (tx, rx) = channel(0);
        (
            Self {
                tags: Default::default(),
                talk: Default::default(),
                tx: tx,
            },
            rx,
        )
    }
}

#[async_trait]
impl Scriptor for TestScriptor {
    /// トーク開始
    async fn start(&mut self) {
        let t = std::mem::replace(&mut self.talk, Default::default());
        self.tx.send(t).await.unwrap();
    }

    /// アクター切替
    fn actor(&mut self, t: &str) {
        let s = format!("actor({})", t);
        self.talk.push_str(&format!("{}\n", &s));
    }

    /// アクション（表情）の指定
    fn action(&mut self, t: &str) {
        let s = format!("action({})", t);
        self.talk.push_str(&format!("{}\n", &s));
    }

    /// セリフの指定
    fn serif(&mut self, t: &str) {
        let s = format!("serif({})", t);
        self.talk.push_str(&format!("{}\n", &s));
    }

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
fn rand_jump_test() {
    let (mut s, _) = TestScriptor::new();
    s.tags_mut().insert("通常トーク".to_owned());
    s.tags_mut().insert("お昼過ぎ".to_owned());
    let jt = rand_jump(&mut s, JT::START);
    assert_eq!(JT::H5, jt);
}

#[test]
fn talk_test_1() {
    let mut pool = LocalPool::new();
    let spawner = pool.spawner();

    let (mut s, mut rx) = TestScriptor::new();
    spawner
        .spawn_local(async move {
            s.tags_mut().insert("通常トーク".to_owned());
            s.tags_mut().insert("お昼過ぎ".to_owned());
            let jump = walk_one(&mut s, JT::START).await;
            let jump = walk_one(&mut s, jump).await;
            walk_one(&mut s, jump).await;
        })
        .unwrap();

    let act = pool.run_until(async move { rx.next().await.unwrap() });
    let mut right: String = Default::default();
    right.push_str("actor(パスタ)\n");
    right.push_str("serif(こんにちは！)\n");
    right.push_str("serif(お昼過ぎになりましたね。)\n");
    assert_eq!(right, act);
}

#[test]
fn talk_test_2() {
    let mut pool = LocalPool::new();
    let spawner = pool.spawner();

    let (mut s, mut rx) = TestScriptor::new();
    spawner
        .spawn_local(async move {
            s.tags_mut().insert("通常トーク".to_owned());
            s.tags_mut().insert("お昼過ぎ".to_owned());
            walk(&mut s, JT::START).await;
        })
        .unwrap();

    let left = pool.run_until(async move { rx.next().await.unwrap() });
    let mut right: String = Default::default();
    right.push_str("actor(パスタ)\n");
    right.push_str("serif(こんにちは！)\n");
    right.push_str("serif(お昼過ぎになりましたね。)\n");
    assert_eq!(right, left);
}
