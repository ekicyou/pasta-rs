//  パスタスクリプト　AST変換試験データ
//  （EMOの辞書）
//
let h = #{
    env: #{},
    s: Fn("noop"),
    a: Fn("add_action"),    // @
    r: Fn("add_require"),   // !
    e: Fn("add_either"),    // ?
    f: Fn("add_forget"),    // -
    m: Fn("add_memory"),    // +
    h: Fn("register_hasira"),
};
{   // @@@@
    let h = h;
    {   // @@@
        let h = h;
        {   // @@
            let h = h;
            h require "通常会話"
            {
// @
let h = h;
h seq   "ぎんな"
    emo     "興奮"
    talk    "アヒルやアヒル！"
    talk    "11年ぶり、大阪にアヒルが"
    talk    "帰ってくるねんで！"
  seq   "エモ"
    emo
    ;


fn _(u){
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
hasira(h);
}

            }
        }
    }
}

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