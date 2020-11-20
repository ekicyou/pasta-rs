pub struct ScenarioMaker {}

impl ScenarioMaker {
    /// actor change
    pub fn a(&mut self, text: &str) {}
    /// talk
    pub fn t(&mut self, text: &str) {}
    /// emote
    pub fn e(&mut self, text: &str) {}
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

pub async fn walk(act: ScenarioMaker, jt: JT) {
    let mut act = act;
    let mut jt = jt;
    loop {
        match jt {
            JT::START => {
                act.mode("さくらスクリプト");
                act.a("役者１");
                act.e("通常");
                act.t("一言目。");
                act.t("二言目の発言。");
                act.start().await;
                jt = JT::_1;
            }
            JT::_1 => {
                jt = JT::_2;
            }
            JT::_2 => {
                jt = JT::_3;
            }
            JT::_3 => {
                jt = JT::START;
            }
        }
    }
}
