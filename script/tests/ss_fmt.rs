use pasta_script::ss_fmt::*;
use std::fmt::Error;

#[test]
fn test_talk() -> Result<(), Error> {
    let talk = "今日は、良い天気ですね。明日は‥‥、晴れたらいいな。";
    let exp = r#"今日は、\_w[400]良い天気ですね。\_w[800]明日は‥\_w[200]‥\_w[200]、\_w[400]晴れたらいいな。\_w[800]"#;
    assert_eq!(exp, format_talk(&talk)?);
    Ok(())
}
