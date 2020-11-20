/// 属性情報
#[derive(Debug, Clone)]
pub enum Attr {
    /// ！キーワード：必須
    Require(String),

    /// ？キーワード：いづれかに一致
    Either(String),

    /// ーキーワード：忘れる
    Forget(String),

    /// ＋キーワード：覚える
    Memory(String),

    /// ＠キーワード：コマンド実行
    Action(String),

    /// ＞キーワード：トーク内ジャンプ
    ShortJump(String),

    /// ＞＞キーワード：広域ジャンプ
    LongJump(String),
}
