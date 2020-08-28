use pasta_script::error::*;
use pasta_script::ss::*;
#[test]
fn test_talk() -> PastaResult<()> {
    let actors: [(_, _, usize); 2] = [("むらさき", "\\0", 150), ("えも", "\\1", 150)];
    let mut x = SakuraScriptBuilder::new(&actors, "通常");
    x.change_actor("むらさき")?;
    x.emote("通常")?;
    x.talk("今日は、ええ天気やね。")?;

    x.change_actor("えも")?;
    x.emote("通常")?;
    x.talk("明日も晴れるかな‥‥？")?;

    x.change_actor("むらさき")?;
    x.emote("笑顔")?;
    x.talk("晴れるとええねえ。")?;

    let exp = r#"\1\s[通常]\0\s[通常]\n[150]今日は、\_w[400]ええ天気やね。\_w[800]\1\s[通常]\n[150]明日も晴れるかな‥\_w[200]‥\_w[200]？\_w[600]\0\s[笑顔]\n[150]晴れるとええねえ。\_w[800]\e"#;
    assert_eq!(exp, x.build()?.as_str());
    Ok(())
}
