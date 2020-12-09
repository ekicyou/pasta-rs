use crate::block::*;
use crate::utils::*;
use squote::{quote, TokenStream};
use std::collections::{HashMap, HashSet};

pub fn gen(items: &[HasiraBlock]) -> TokenStream {
    let mut m: HashMap<String, Vec<&str>> = HashMap::new();
    for h in items {
        let id = &h.attr.id;
        for key in &h.attr.require {
            let key = require_value(key).to_owned();
            let mut entry = m.entry(key).or_insert(Vec::new());
            entry.push(id);
        }
    }

    let rand_jump = gen_func();
    quote! {
        #rand_jump
    }
}

fn gen_func() -> TokenStream {
    quote! {
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
    }
}
