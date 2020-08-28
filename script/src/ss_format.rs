use std::borrow::Cow;
use std::collections::HashMap;
use std::convert::AsRef;
use std::fmt::{Display, Error, Formatter};

#[derive(Clone, Default, Debug)]
pub struct SakuraScriptTalk<'a> {
    text: &'a str,
}

impl<'a> SakuraScriptTalk<'a> {
    pub fn new(text: &'a str) -> SakuraScriptTalk<'a> {
        SakuraScriptTalk { text: text }
    }
    pub fn write(f: &mut Formatter, text: &'a str) -> Result<(), Error> {
        let a = Self::new(text);
        write!(f, "{}", a);
        Ok(())
    }

    fn wait(f: &mut Formatter, ms: isize) -> Result<(), Error> {
        if ms > 0 {
            write!(f, r#"\_w[{}]"#, ms);
        }
        Ok(())
    }
}

impl<'a> Display for SakuraScriptTalk<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut remain = 0;
        for c in self.text.chars() {
            let mut pre = 0;
            let mut suf = 0;

            match WAIT_TABLE.get(&c) {
                None => {
                    pre = remain;
                    remain = 0;
                }
                Some(wait) if *wait == 0 => {
                    continue;
                }
                Some(wait) if *wait > 0 => {
                    if remain < *wait {
                        remain = *wait;
                    }
                }
                Some(wait) => {
                    pre = remain;
                    remain = 0;
                    suf = 0 - wait;
                }
            }

            Self::wait(f, pre);
            write!(f, "{}", c);
            Self::wait(f, suf);
        }
        Self::wait(f, remain);
        Ok(())
    }
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

        ii(&mut t, 0, "\r\n");
        t
    };
}
