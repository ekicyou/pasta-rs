use std::collections::HashMap;
use std::convert::AsRef;
use std::convert::From;
use std::fmt::{Display, Error, Formatter, Write};

#[derive(Clone, Default, Debug)]
struct SakuraScriptTalk<'a> {
    text: &'a str,
}

impl<'a> SakuraScriptTalk<'a> {
    fn new(text: &'a str) -> SakuraScriptTalk<'a> {
        SakuraScriptTalk { text: text }
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

            write_wait(f, pre)?;
            write!(f, "{}", c)?;
            write_wait(f, suf)?;
        }
        write_wait(f, remain)?;
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

fn write_wait(f: &mut Formatter, ms: isize) -> Result<(), Error> {
    if ms > 0 {
        write!(f, r#"\_w[{}]"#, ms)?;
    }
    Ok(())
}

pub trait SSFormatter {
    fn write_talk<S: AsRef<str>>(&mut self, text: S) -> Result<(), Error>;
    fn write_wait(&mut self, ms: isize) -> Result<(), Error>;
    fn write_surface<S: Display>(&mut self, text: S) -> Result<(), Error>;
    fn write_new_line(&mut self, percent: usize) -> Result<(), Error>;
}

impl<W: Write> SSFormatter for W {
    fn write_talk<S: AsRef<str>>(&mut self, text: S) -> Result<(), Error> {
        let text = text.as_ref();
        let f = SakuraScriptTalk::new(text);
        self.write_fmt(format_args!("{}", f))?;
        Ok(())
    }

    fn write_wait(&mut self, ms: isize) -> Result<(), Error> {
        if ms > 0 {
            self.write_fmt(format_args!(r#"\_w[{}]"#, ms))?;
        }
        Ok(())
    }

    fn write_surface<S: Display>(&mut self, text: S) -> Result<(), Error> {
        self.write_fmt(format_args!(r#"\s[{}]"#, text))
    }

    fn write_new_line(&mut self, percent: usize) -> Result<(), Error> {
        if percent > 0 {
            self.write_fmt(format_args!(r#"\n[{}]"#, percent))?;
        }
        Ok(())
    }
}
