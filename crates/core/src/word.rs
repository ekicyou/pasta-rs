/*
# 単語辞書

| @|作品略称|名称                  |略称    |カテゴリー|CV              |#所属   |
|-:|:------:|:---------------------|:-------|:---------|:---------------|:-------|
| 2|推し武道|推しが武道館に行ったら|        |作品      |                |マンガ  |
| 3|推し武道|えりぴよ              |        |キャラ    |ファイルーズあい|ドルオタ|
| 3|推し武道|くまさ                |        |キャラ    |前野智昭        |ドルオタ|
|17|私に天使|私に天使が舞い降りた！|        |作品      |                |マンガ  |
|18|私に天使|星野みやこ            |みゃー姉|キャラ    |上田麗奈        |星野家  |
|18|私に天使|星野ひなた            |ひなた  |キャラ    |長江里加        |星野家  |

```
# 属性⇒値
作品略称    ⇒私に天使
名称        ⇒星野みやこ
呼称        ⇒みゃー姉
カテゴリー  ⇒キャラ
CV          ⇒上田麗奈
所属        ⇒星野家

# （名称）とは何か
星野みやこ  ⇒私に天使
            ⇒キャラ
            ⇒上田麗奈
            ⇒星野家

# （呼称）とは何か


# 逆参照の作成

```
*/

use std::collections::{HashMap, HashSet};

pub struct WordDic {
    map: HashMap<String, HashSet<String>>,
}

impl WordDic {
    pub fn get<S: Into<String>>(&self, key: S) -> Option<&HashSet<String>> {
        let key = key.into();
        self.map.get(&key)
    }

    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// １要素を単語辞書に登録します。
    pub fn push_kv<K: Into<String>, V: Into<String>>(&mut self, k: K, v: V) {
        let k = k.into();
        let v = v.into();
        if k == "" {
            return;
        }
        if v == "" {
            return;
        }
        let hs = self.map.entry(k).or_insert(HashSet::new());
        hs.insert(v);
    }

    /// csvレコード１行を単語辞書に登録します。
    pub fn push<'a, T: AsRef<str>>(&mut self, header: &'a [T], record: &'a [T]) {
        // 属性⇒値
        let mut name1: Option<String> = None;
        let mut name2: Option<String> = None;
        {
            let zip = header.iter().zip(record);
            for (k, v) in zip {
                let k = k.as_ref();
                let v = v.as_ref();
                if k == "名称" {
                    name1 = Some(v.to_owned());
                } else if k == "呼称" {
                    name2 = Some(v.to_owned());
                }
                self.push_kv(k, v);
            }
        }
        // 名称⇒値
        if let Some(name) = name1 {
            for v in record {
                let v = v.as_ref();
                if v == name {
                    continue;
                }
                self.push_kv(&name, v);
                self.push_kv(v, &name);
            }
        } // 呼称⇒値
        if let Some(name) = name2 {
            for v in record {
                let v = v.as_ref();
                if v == name {
                    continue;
                }
                self.push_kv(&name, v);
                self.push_kv(v, &name);
            }
        }
    }
}

#[test]
fn create_dic() {
    use std::collections::BTreeSet;
    let header = &["作品略称", "名称", "呼称", "カテゴリー", "CV", "所属"][..];
    let rec1 = &[
        "私に天使",
        "私に天使が舞い降りた！",
        "",
        "作品",
        "",
        "マンガ",
    ][..];
    let rec2 = &[
        "私に天使",
        "星野みやこ",
        "みゃー姉",
        "キャラ",
        "上田麗奈",
        "星野家",
    ][..];
    let rec3 = &[
        "私に天使",
        "星野ひなた",
        "ひなた",
        "キャラ",
        "長江里加",
        "星野家",
    ][..];
    let mut dic = WordDic::new();
    dic.push(header, rec1);
    dic.push(header, rec2);
    dic.push(header, rec3);
    let set = dic.get("名称").unwrap();
    let set: BTreeSet<_> = set.iter().cloned().collect();
    let set: Vec<_> = set.iter().cloned().collect();
    assert_eq!(
        set,
        vec!["星野ひなた", "星野みやこ", "私に天使が舞い降りた！"]
    );

    let set = dic.get("みゃー姉").unwrap();
    let set: BTreeSet<_> = set.iter().cloned().collect();
    let set: Vec<_> = set.iter().cloned().collect();
    assert_eq!(
        set,
        vec!["キャラ", "上田麗奈", "星野みやこ", "星野家", "私に天使"]
    );

    let set = dic.get("呼称").unwrap();
    let set: BTreeSet<_> = set.iter().cloned().collect();
    let set: Vec<_> = set.iter().cloned().collect();
    assert_eq!(set, vec!["星野みやこ"]);

    let set = dic.get("私に天使").unwrap();
    let set: BTreeSet<_> = set.iter().cloned().collect();
    let set: Vec<_> = set.iter().cloned().collect();
    assert_eq!(
        set,
        vec!["星野ひなた", "星野みやこ", "私に天使が舞い降りた！"]
    );

    let set = dic.get("星野家").unwrap();
    let set: BTreeSet<_> = set.iter().cloned().collect();
    let set: Vec<_> = set.iter().cloned().collect();
    assert_eq!(set, vec!["所属", "星野ひなた", "星野みやこ"]);
}
