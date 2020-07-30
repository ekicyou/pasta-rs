use rhai::{Engine, EvalAltResult, INT};

#[test]
fn eval() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();
    let result = engine.eval::<INT>(
        r"
                fn すべての答え() { 42 }
                すべての答え()
            ",
    );
    assert_eq!(result?, 42);

    let result = engine.eval::<INT>(
        r"
                fn _1() { 1 }
                _1()
            ",
    );
    assert!(result.is_err());

    Ok(())
}
