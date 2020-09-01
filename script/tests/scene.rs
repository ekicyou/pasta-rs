use pasta_script::error::*;
use pasta_script::ss::scene::*;
use rhai::{Engine, EvalAltResult, ImmutableString};

#[test]
fn play_scene() -> PastaResult<()> {
    let script = r#"
    let scene = |p, env|{
        p.change_actor("むらさき")?;
        p.emote("通常")?;
        p.br_t("今日は、")?;
        p.br_t("ええ天気やね。")?;

        p.change_actor("えも")?;
        p.emote("通常")?;
        p.talk("明日も")?;
        p.br()?;
        p.talk("晴れるかな‥‥？")?;

        p.change_actor("むらさき")?;
        p.emote("笑顔")?;
        p.talk("晴れるとええねえ。")?;

        if(!p.cut()) return;

        p.change_actor("えも")?;
        p.emote("冷笑")?;
        p.talk("雨にならないかなー♪")?;

        p.change_actor("むらさき")?;
        p.change_new_line(120)?;
        p.emote("怒り")?;
        p.talk("もうっ！")?;

        if(!p.cut()) return;
    };
    scene;
    "#;
    let endine = Engine::new();
    Ok(())
}
