use pasta_script::dic::*;
use rhai::{Engine, EvalAltResult, FnPtr};

#[test]
fn test_cond_expr() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();
    register_rhai(&mut engine)?;
    let cond = engine.eval::<ConditionExpr>(r#"has("一般会話")"#)?;
    assert_eq!(format!("{:?}", cond), r#"Has("一般会話")"#);
    Ok(())
}

#[test]
fn test_cond() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();
    register_rhai(&mut engine)?;

    {
        let cond = engine.eval::<Condition>(r#"cond(has("一般会話"))"#)?;
        assert_eq!(
            format!("{:?}", cond),
            r#"Condition { expr: Has("一般会話"), finally: None }"#
        );
    }
    {
        let cond = engine.eval::<Condition>(
            r#"
        fn foo(x) { 41 + x }
        let func = Fn("foo"); 
        cond(has("一般会話"),func)
        "#,
        )?;
        assert_eq!(
            format!("{:?}", cond),
            r#"Condition { expr: Has("一般会話"), finally: Some(FnPtr("foo", [])) }"#
        );
    }
    Ok(())
}

#[test]
fn test_hasira() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();
    register_rhai(&mut engine)?;
    {
        let hasira = engine.eval::<Hasira>(
            r#"
        let cond = cond(has("一般会話"));
        let h = hasira();
        h.title = "明日の天気";
        h.condition = cond;
        h;
        "#,
        )?;
        assert_eq!(
            format!("{:?}", hasira),
            r#"Hasira { title: "明日の天気", condition: [Condition { expr: Has("一般会話"), finally: None }], cb: PlayBuilderCallbackItem { fn_emote: FnPtr("", []), fn_talk: FnPtr("", []), fn_word: FnPtr("", []) } }"#
        );
    }
    Ok(())
}

#[test]
fn test_screenplay() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();
    register_rhai(&mut engine)?;
    {
        let script = r#"
        let play = screen_play();
        {
            let h = hasira();
            h.title = "明日の天気";
            play.push(h, |p| {
                p A "むらさき"
                        E "興奮笑顔"
                        T "アヒルやアヒル！"
                        ;
                p.action();
            });
        }
        play;
        "#;
        let play = engine.eval::<ScreenPlay>(script)?;
        assert_eq!(play.count(), 1);
    }
    Ok(())
}

#[test]
fn test_fnptr() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();
    register_rhai(&mut engine)?;
    {
        let script = r#"
        fn get_text(text){
            return text + "456"
        }
        let ptr = Fn("get_text");
        ptr;
        "#;
        let ast = engine.compile(script)?;
        let ptr = engine.eval_ast::<FnPtr>(&ast)?;
        assert_eq!(format!("{:?}", ptr), r#"FnPtr("get_text", [])"#);
        let text = ptr.call_string(&engine, &ast, "123")?;
        assert_eq!(text, r#"123456"#);
    }
    Ok(())
}
