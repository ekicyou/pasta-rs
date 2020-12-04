use once_cell::sync::Lazy;
use rand::Rng;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum JT {
    START,
    H1,
    H2,
    H3,
}

pub async fn walk() {
    let mut fn_map = {
        let mut m: HashMap<String, Vec<fn(&[String]) -> Option<JT>>> = HashMap::new();
        m.entry("KEY1".to_owned())
            .or_insert(vec![h_checks::H1, h_checks::H2]);
        m.entry("KEY2".to_owned())
            .or_insert(vec![h_checks::H1, h_checks::H3]);
        m.entry("KEY3".to_owned()).or_insert(vec![h_checks::H3]);
        m.entry("KEY4".to_owned()).or_insert(vec![h_checks::H1]);
        m.entry("KEY5".to_owned()).or_insert(vec![h_checks::H1]);
        m
    };
    let mut used_hasira: HashSet<JT> = HashSet::new();

    let rand_jump = |tags: &[String], default_target: JT| {
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
}

mod h_checks {
    use super::JT;
    pub fn H1(tags: &[String]) -> Option<JT> {
        let mut f1 = false;
        let mut f2 = false;
        let mut f3 = false;
        let mut f4 = false;
        for tag in tags {
            if tag == "KEY1" {
                f1 = true;
            }
            if tag == "KEY1" {
                f2 = true;
            }
            if tag == "KEY3" {
                f3 = true;
            }
            if tag == "KEY4" {
                f4 = true;
            }
            if f1 && f2 && (f3 || f4) {
                return Some(JT::H1);
            }
        }
        None
    }
    pub fn H2(tags: &[String]) -> Option<JT> {
        Some(JT::H2)
    }
    pub fn H3(tags: &[String]) -> Option<JT> {
        Some(JT::H3)
    }
}
