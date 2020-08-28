use pasta_script::error::*;
use pasta_script::ss::*;
#[test]
fn test_talk() -> PastaResult<()> {
    let actors: [(_, _, usize); 2] = [("", "", 150), ("", "", 150)];
    let mut x = SakuraScriptBuilder::new(&actors, "通常");

    Ok(())
}
