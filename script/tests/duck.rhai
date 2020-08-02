//  パスタスクリプト　AST変換試験データ
//  （EMOの辞書）
//
let h9 = #{
    env: #{},
    s: Fn("noop"),
    a: Fn("add_action"),    // @
    r: Fn("add_require"),   // !
    e: Fn("add_either"),    // ?
    f: Fn("add_forget"),    // -
    m: Fn("add_memory"),    // +
    h: Fn("register_hasira"),
};

let h4 = h9;

let h3 = h4;

let h2 = h3;
h2 require "通常会話"

let h1 = h2;
h1 title    "ダックが返ってくる！"
h1 scene    "s1";
fn s1(u){
    u.ぎんな 
        emo  "興奮"
        talk "アヒルやアヒル！"
        talk "11年ぶり、大阪にアヒルが"
        talk "帰ってくるねんで！"
        ;
    u.エモ
        emo  "あきれ"
        talk "11年ぶりって、"
        talk "君、生まれてたの？"
        ;
    u
}