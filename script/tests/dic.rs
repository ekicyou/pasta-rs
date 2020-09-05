use pasta_script::dic::*;
use rhai::{Engine, EvalAltResult};

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
            r#"Hasira { title: "明日の天気", condition: [Condition { expr: Has("一般会話"), finally: None }] }"#
        );
    }
    Ok(())
}
