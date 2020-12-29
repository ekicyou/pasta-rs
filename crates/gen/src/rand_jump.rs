use crate::block::*;
use crate::utils::*;
use squote::{quote, TokenStream};
use std::collections::HashMap;

pub fn gen(items: &[HasiraBlock]) -> TokenStream {
    let mut m: HashMap<String, Vec<&str>> = HashMap::new();
    for h in items {
        let id = &h.attr.id;
        for key in &h.attr.require {
            let key = require_value(key).to_owned();
            let entry = m.entry(key).or_insert(Vec::new());
            entry.push(id);
        }
        for key in &h.attr.either {
            let key = either_value(key).to_owned();
            let entry = m.entry(key).or_insert(Vec::new());
            entry.push(id);
        }
    }
    let mut map_entry = TokenStream::new();
    for (key, entry) in m {
        let mut funcs = TokenStream::new();
        for id in entry {
            let id = format_ident(id);
            funcs.combine(&quote! { h_checks::#id, });
        }
        map_entry.combine(&quote! {
            m   .entry(#key.to_owned())
                .or_insert(vec![#funcs]);
        });
    }
    let fn_map = quote! {
        static FN_MAP: Lazy<HashMap<String, Vec<fn(&HashSet<String>) -> Option<JT>>>> = Lazy::new(|| {
            let mut m: HashMap<String, Vec<fn(&HashSet<String>) -> Option<JT>>> = HashMap::new();
            #map_entry
            m
        });
    };

    let rand_jump = gen_func();
    quote! {
        #fn_map
        #rand_jump
    }
}

fn gen_func() -> TokenStream {
    quote! {
        static USED_HASIRA: Lazy<Mutex<HashSet<JT>>> = Lazy::new(|| Mutex::new(HashSet::new()));
        pub fn rand_jump<S: Scriptor>(s: &mut S, default_target: JT) -> JT {
            let mut target: Option<_> = None;
            let mut target_len = usize::MAX;
            // 最も候補が少ない有効エントリを検索
            for tag in s.tags() {
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
            // 最小有効エントリから候補の絞り込み
            let targets: Vec<_> = match target {
                Some(a) => a.into_iter().filter_map(|f| f(s.tags())).collect(),
                _ => {
                    return default_target;
                }
            };
            if targets.len() == 0 {
                return default_target;
            }
            let used_hasira_lock = &mut USED_HASIRA.lock().unwrap();
            let used_hasira = &mut **used_hasira_lock;
            let nouse: Vec<_> = (&targets)
                .iter()
                .filter(|a| !used_hasira.contains(*a))
                .map(|a| a.clone())
                .collect();
            let nouse = if nouse.len() == 0 {
                for target in &targets {
                    used_hasira.remove(target);
                }
                targets
            } else {
                nouse
            };
            let sel = s.rand_range_usize(0..nouse.len());
            let sel = nouse[sel];
            used_hasira.insert(sel);
            sel
        }
    }
}
