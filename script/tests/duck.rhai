//  パスタスクリプト　AST変換試験データ
//  （EMOの辞書）
//


/// env 引数は下記のパラメータを保持するオブジェクトマップ
/// env #{save, state, actors, scene, actor}
///     save: シリアライズ対象 
///    state: アプリ起動からの状態
///   actors: アクターテーブル
///    scene: 現在のシーンの間保持
///    actor: 現在選択されているアクター

/// actor 切り替え後の追加処理を実行します。
/// 切り替えそのものはランタイムが行います。
/// env -> value -> [env, script]
/// value: actor名
fn after_change_actor(env, value){
    [env, value];
}

/// 表情を変更します。
/// env -> value -> [env, script]
/// value: 表情指定
fn emote(env, value){
    let script = "\\s[" + value + "]";
    [env, script];
}

/// 改行を行います。引数が１以上の整数である場合、アクターの改行設定を変更して改行します。
/// env -> value -> [env, script]
/// value: 100%で文字１列分を改行
fn newline(env, value){
    
    let script = "\\n[" + env.actor.newline + "]";
    [env, script];
}


/// 会話文字列の処理を行います。ランタイムの会話処理関数へリダイレクトします。
/// env -> value -> [env, script]
/// value: 変換対象の文字列
fn talk(env, value){
    let script = pasta_talk(env.actor, value);
    [env, script];
}

/// 単語辞書の参照を行います。ランタイムの会話処理関数へリダイレクト後、talkを呼び出します。
/// env -> value -> [env, script]
/// value: 変換対象の文字列
fn word(env, value){
    let word = pasta_word(env, value);
    talk(env, word)
}

let play = screenplay();


let むらさき = actor("むらさき")
むらさき.fn_actor   = Fn("after_change_actor");
むらさき.fn_emote   = Fn("emote");
むらさき.fn_newline = Fn("newline");
むらさき.fn_talk    = Fn("talk");
むらさき.fn_word    = Fn("word");


let えも     = play.create_actor("えも")


let h = hasira();

{   // @@@@
    let h = h;
    {   // @@@
        let h = h;
        {   // @@
            let h = h;
            h.condition = cond(has("通常会話"));
            {   // @
                let h = h;
                h.title = "ダックが帰ってくる！";
                h.screenplay = |env| {
                    let a = play(env)
                    let むらさき = actor(a,"むらさき")
                    let えも     = actor(a,"えも")
                    むらさき                      // キャラクタを指定
                        E "興奮笑顔"              // E, Tはカスタムオペレータ
                        T "アヒルやアヒル！"
                        T "11年ぶり、大阪にアヒルが"
                        T "帰ってくるねんで！"
                        ;
                    えも
                        E "冷笑"
                        T "11年ぶりって、"
                        T "君、生まれてたの？"
                        ;
                    action(a);   // アクション（1回のトーク）rustに制御が返る。
                    }
                };
            }
        }
    }
}
// scriptの途中キャンセル方法は？
// 非同期に待機する方法は？スレッド作って待機でOK？
//   ⇒スクリプト側は非同期を意識しない。
//   ⇒アクションコマンドで制御がrustに戻ったのち、rust側で非同期処理する。
//   ⇒スクリプトはあくまで台本であり、どう読むかはrust側の解釈