use once_cell::sync::Lazy;
use pasta_core::Scriptor;
use rand::Rng;
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum JT {
    #[doc = "START"]
    START,
    #[doc = "お天気はどうですか"]
    H1,
    #[doc = "お天気はどうですか："]
    H1A1,
    #[doc = "お天気はどうですか：１"]
    H1A2,
    #[doc = "お天気はどうですか：１"]
    H1A3,
    #[doc = ""]
    H2,
    #[doc = "："]
    H2A1,
    #[doc = "同名柱"]
    H3,
    #[doc = "同名柱："]
    H3A1,
    #[doc = "同名柱"]
    H4,
    #[doc = "同名柱："]
    H4A1,
}
#[doc = "パスタスクリプトテスト構文

最初の柱まではドキュメントコメントとします。"]
pub async fn walk<S: Scriptor>(s: &mut S, jump: JT) {
    let mut jump = jump;
    loop {
        match jump {
            JT::START => {
                jump = JT::H1;
            }
            JT::H1 => {
                jump = JT::H1A1;
            }
            JT::H1A1 => {
                s.actor("パスタ");
                s.serif("おはようございます。");
                s.serif("明日の天気を当ててみてましょう。");
                s.start().await;
                jump = rand_jump(s, JT::H1A2);
            }
            JT::H1A2 => {
                s.serif(
                    "サンダルは晴れと出ました！",
                );
                s.serif("お出かけ出来たら楽しいですよ。");
                s.start().await;
                jump = rand_jump(s, JT::H1A3);
            }
            JT::H1A3 => {
                s.serif("サンダルは雨と出ました。");
                s.serif(
                    "引きこもりでも、雨はじっとりなのです。",
                );
                s.start().await;
                jump = JT::H2;
            }
            JT::H2 => {
                jump = JT::H2A1;
            }
            JT::H2A1 => {
                s.actor("パスタ");
                s.serif("トーク区切り。");
                s.start().await;
                jump = rand_jump(s, JT::H3);
            }
            JT::H3 => {
                jump = JT::H3A1;
            }
            JT::H3A1 => {
                s.start().await;
                jump = rand_jump(s, JT::H4);
            }
            JT::H4 => {
                jump = JT::H4A1;
            }
            JT::H4A1 => {
                s.start().await;
                jump = rand_jump(s, JT::START);
            }
        }
    }
}

static FN_MAP: Lazy<HashMap<String, Vec<fn(&[String]) -> Option<JT>>>> = Lazy::new(|| {
    let mut m: HashMap<String, Vec<fn(&[String]) -> Option<JT>>> = HashMap::new();
    m.entry("KEY1".to_owned())
        .or_insert(vec![h_checks::H1, h_checks::H2]);
    m.entry("KEY2".to_owned())
        .or_insert(vec![h_checks::H1, h_checks::H3]);
    m.entry("KEY3".to_owned()).or_insert(vec![h_checks::H3]);
    m.entry("KEY4".to_owned()).or_insert(vec![h_checks::H1]);
    m.entry("KEY5".to_owned()).or_insert(vec![h_checks::H1]);
    m
});
static USED_HASIRA: Lazy<Mutex<HashSet<JT>>> = Lazy::new(|| Mutex::new(HashSet::new()));

fn rand_jump<S: Scriptor>(s: &mut S, default_target: JT) -> JT {
    let tags = s.tags();
    let mut r = s.thread_rng();
    // 要素数が一番少ないキーを探す
    let mut target: Option<_> = None;
    let mut target_len = usize::MAX;
    for tag in tags {
        match FN_MAP.get(tag) {
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
    let used_hasira_lock = &mut USED_HASIRA.lock().unwrap();
    let used_hasira = &mut **used_hasira_lock;
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
    let sel = r.gen_range(0, nouse.len());
    let sel = nouse[sel];

    // 選択要素を使用済みに登録
    used_hasira.insert(sel);

    // 選択
    sel
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
