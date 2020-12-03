#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Scriptor {
    pub tags: Vec<String>,
}

impl Scriptor {
    /// actor change
    pub fn actor(&mut self, text: &str) {}
    /// talk
    pub fn serif(&mut self, text: &str) {}
    /// action
    pub fn action(&mut self, text: &str) {}
    /// action mode change
    pub fn mode(&mut self, text: &str) {}
    /// start action
    pub async fn start(&mut self) {}
}

#[derive(Debug)]
pub enum JT {
    START,
    _1,
    _2,
    _3,
}

pub async fn walk(s: Scriptor, jt: JT) {
    let mut s = s;
    let mut jt = jt;
    loop {
        match jt {
            JT::START => {
                jt = JT::_1;
            }
            JT::_1 => {
                s.mode("さくらスクリプト");
                s.actor("役者１");
                s.action("通常");
                s.serif("一言目。");
                s.serif("二言目の発言。");
                s.start().await;
                jt = rand_jump(&mut s);
            }
            JT::_2 => {
                s.mode("さくらスクリプト");
                s.actor("役者１");
                s.action("通常");
                s.serif("３");
                s.start().await;
                jt = rand_jump(&mut s);
            }
            JT::_3 => {
                s.mode("さくらスクリプト");
                s.actor("役者１");
                s.action("通常");
                s.serif("４");
                s.start().await;
                jt = rand_jump(&mut s);
            }
        }
    }
}

fn rand_jump(s: &mut Scriptor) -> JT {
    JT::START
}

// あるタグsetに対してマッチする要素を取得する関数
// ["会話","晴れ"] =>[_1,_2]

fn select(tags: &[String]) -> JT {
    JT::START
}
