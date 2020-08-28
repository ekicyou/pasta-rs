use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Clone, Default, Debug)]
pub struct SakuraScriptTalkFormatter<'a> {
    text: Cow<'a, str>,
}

lazy_static! {
    static ref WAIT_TABLE: HashMap<char, isize> = {
        fn ii(t: &mut HashMap<char, isize>, wait: isize, chars: &str) {
            for c in chars.chars() {
                t.insert(c, wait);
            }
        }
        let mut t = HashMap::new();
        ii(&mut t, 800, r#"。．"#);
        ii(&mut t, 600, r#"？！"#);
        ii(&mut t, 400, r#"、，）］｝」』"#);
        ii(&mut t, -200, r#"・‥…"#);

        ii(&mut t, 800, r#"｡"#);
        ii(&mut t, 400, r#"､｣"#);
        ii(&mut t, -200, r#"･"#);

        ii(&mut t, -1, "\r\n");
        t
    };
}
