use pasta_script::di::*;
use rhai::{Engine, EvalAltResult, FnPtr};

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
