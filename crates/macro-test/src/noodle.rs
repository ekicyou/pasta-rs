use pasta_core::Scriptor;
fn rand_jump<S: Scriptor>(s: &mut S, default_target: JT) -> JT {
    JT::START
}
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
                s.serif("サンダルは晴れと出ました！");
                s.serif("お出かけ出来たら楽しいですよ。");
                s.start().await;
                jump = rand_jump(s, JT::H1A3);
            }
            JT::H1A3 => {
                s.serif("サンダルは雨と出ました。");
                s.serif("引きこもりでも、雨はじっとりなのです。");
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
        s.commit_tags();
    }
}
mod h_checks {
    #![allow(non_snake_case)]
    use super::JT;
    pub fn H1(tags: &[String]) -> Option<JT> {
        let mut r0 = false;
        let mut r1 = false;
        for tag in tags {
            if tag == "起動トーク" {
                r0 = true;
            }
            if tag == "午前中" {
                r1 = true;
            }
            if r0 && r1 {
                return Some(JT::H1);
            }
        }
        None
    }
    pub fn H2(tags: &[String]) -> Option<JT> {
        let mut r0 = false;
        let mut r1 = false;
        for tag in tags {
            if tag == "起動トーク" {
                r0 = true;
            }
            if tag == "午前中" {
                r1 = true;
            }
            if r0 && r1 {
                return Some(JT::H2);
            }
        }
        None
    }
    pub fn H3(tags: &[String]) -> Option<JT> {
        let mut r0 = false;
        let mut r1 = false;
        for tag in tags {
            if tag == "起動トーク" {
                r0 = true;
            }
            if tag == "午前中" {
                r1 = true;
            }
            if r0 && r1 {
                return Some(JT::H3);
            }
        }
        None
    }
    pub fn H4(tags: &[String]) -> Option<JT> {
        let mut r0 = false;
        let mut r1 = false;
        for tag in tags {
            if tag == "起動トーク" {
                r0 = true;
            }
            if tag == "午前中" {
                r1 = true;
            }
            if r0 && r1 {
                return Some(JT::H4);
            }
        }
        None
    }
}
