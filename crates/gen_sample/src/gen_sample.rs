use pasta_core::Scriptor;
use rand::Rng;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum JT {
    START,
    H1,
    H1A1,
    H1A2,
    H1A3,
    H2,
    H3,
}

pub async fn walk<S: Scriptor>(s: S, jt: JT) {
    let fn_map = {
        let mut m: HashMap<String, Vec<fn(&[String]) -> Option<JT>>> = HashMap::new();
        m.entry("一般トーク".to_owned())
            .or_insert(vec![h_checks::H1, h_checks::H2]);
        m.entry("午前中".to_owned())
            .or_insert(vec![h_checks::H1, h_checks::H3]);
        m.entry("KEY3".to_owned()).or_insert(vec![h_checks::H3]);
        m.entry("KEY4".to_owned()).or_insert(vec![h_checks::H1]);
        m.entry("KEY5".to_owned()).or_insert(vec![h_checks::H1]);
        m
    };
    let mut used_hasira: HashSet<JT> = HashSet::new();

    // 次のトークを選択
    let mut next_talk = |tags: &[String], default_target: JT| {
        // 要素数が一番少ないキーを探す
        let mut target: Option<_> = None;
        let mut target_len = usize::MAX;
        for tag in tags {
            match fn_map.get(tag) {
                None => {
                    continue;
                }
                Some(a) => {
                    let len = a.len();
                    if len < target_len {
                        target_len = len;
                        target = Some(a);
                    }
                }
            }
        }
        // 柱を抽出
        let targets: Vec<_> = match target {
            Some(a) => a.into_iter().filter_map(|f| f(tags)).collect(),
            _ => {
                return default_target;
            }
        };
        if targets.len() == 0 {
            return default_target;
        }

        // 使用済み柱の除去
        let nouse: Vec<_> = (&targets)
            .iter()
            .filter(|a| !used_hasira.contains(*a))
            .map(|a| a.clone())
            .collect();
        let nouse = if nouse.len() == 0 {
            // 要素０⇒使用済み柱からtargetsを除外
            for target in &targets {
                used_hasira.remove(target);
            }
            targets
        } else {
            nouse
        };

        // １つを選択
        let mut r = rand::thread_rng();
        let sel = r.gen_range(0, nouse.len());
        let sel = nouse[sel];

        // 選択要素を使用済みに登録
        used_hasira.insert(sel);

        // 選択
        sel
    };

    // スクリプト
    let mut s = s;
    let mut jt = jt;
    loop {
        jt = match jt {
            JT::START => next_talk(s.tags(), JT::H1),
            JT::H1 => {
                s.forget("天気");
                s.forget("明日");
                JT::H1A1
            }
            JT::H1A1 => {
                s.actor("パスタ");
                s.serif("おはようございます。");
                s.serif("明日の天気を当ててみてましょう。");
                // s.start().await;
                local_jump::H1A1(s.tags()) // ジャンプがある場合はs.start()しない
            }
            JT::H1A2 => {
                s.action("笑顔");
                s.serif("サンダルは晴れと出ました！");
                s.memory("サンダル");
                s.memory("晴れ");
                s.serif("お出かけ出来たら楽しいですよ。");
                s.start().await;
                next_talk(s.tags(), JT::H2)
            }
            JT::H1A3 => {
                s.action("曇り顔");
                s.serif("サンダルは雨と出ました。");
                s.memory("サンダル");
                s.memory("雨");
                s.serif("引きこもりでも、雨はじっとりなのです。");
                s.start().await;
                next_talk(s.tags(), JT::H2)
            }
            JT::H2 => JT::H3,
            JT::H3 => JT::START,
        };
        s.commit_tags();
    }
}
mod local_jump {
    #![allow(non_snake_case)]
    use super::JT;
    pub fn H1A1(tags: &[String]) -> JT {
        JT::H1A1
    }
}
mod h_checks {
    #![allow(non_snake_case)]
    use super::JT;
    pub fn H1(tags: &[String]) -> Option<JT> {
        let mut r1 = false;
        let mut r2 = false;
        let mut e1 = false;
        let mut e2 = false;
        for tag in tags {
            if tag == "KEY1" {
                r1 = true;
            }
            if tag == "KEY2" {
                r2 = true;
            }
            if tag == "KEY4" {
                e1 = true;
            }
            if tag == "KEY5" {
                e2 = true;
            }
            if r1 && r2 && (e1 || e2) {
                return Some(JT::H1);
            }
        }
        None
    }
    pub fn H2(tags: &[String]) -> Option<JT> {
        let mut r1 = false;
        for tag in tags {
            if tag == "KEY1" {
                r1 = true;
            }
            if r1 {
                return Some(JT::H2);
            }
        }
        None
    }
    pub fn H3(tags: &[String]) -> Option<JT> {
        let mut r1 = false;
        let mut r2 = false;
        for tag in tags {
            if tag == "KEY2" {
                r1 = true;
            }
            if tag == "KEY3" {
                r2 = true;
            }
            if r1 && r2 {
                return Some(JT::H3);
            }
        }
        None
    }
}
