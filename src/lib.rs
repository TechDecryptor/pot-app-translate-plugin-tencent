use serde_json::{Value, json};
use std::collections::HashMap;
use std::error::Error;

#[no_mangle]
pub fn translate(
    text: &str,
    from: &str,
    to: &str,
    detect: &str,
    _needs: HashMap<String, String>,
) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::blocking::ClientBuilder::new().build()?;
    let mut from =String::from(from);
    if from=="auto"{
        let lang_map=json!({
            "zh_cn" : "zh",
            "zh_tw" : "zh-TW",
            "en" : "en",
            "ja" : "ja",
            "ko" : "ko",
            "fr" : "fr",
            "es" : "es",
            "ru" : "ru",
            "de" : "de",
            "it" : "it",
            "tr" : "tr",
            "pt_pt" : "pt",
            "pt_br" : "pt",
            "vi" : "vi",
            "id" : "id",
            "th" : "th",
            "ms" : "ms",
            "ar" : "ar",
            "hi" : "hi"
        });
        if lang_map.as_object().unwrap().contains_key(detect){
            from=lang_map[detect].as_str().unwrap().to_string();
        }else{
            from=String::from("en");
        }
        
    }
    let res = client
        .get("https://wxapp.translator.qq.com/api/translate")
        .header("User-Agent", "Mozilla/5.0 (iPhone; CPU iPhone OS 16_3_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148 MicroMessenger/8.0.32(0x18002035) NetType/WIFI Language/zh_TW")
        .header("Content-Type", "application/json")
        .header("Host", " wxapp.translator.qq.com")
        .header("Referer", "https://servicewechat.com/wxb1070eabc6f9107e/117/page-frame.html")
        .query(&[("source","auto"),("target","auto"),("sourceText",text),("platform","WeChat_APP"),("guid","oqdgX0SIwhvM0TmqzTHghWBvfk22"),("candidateLangs",&format!("{from}|{to}"))])
        .send()?
        .json()?;

    fn parse_result(res: Value) -> Option<String> {
        let result = res
            .as_object()?
            .get("targetText")?
            .as_str()?.to_string();
        Some(result)
    }
    if let Some(result) = parse_result(res) {
        return Ok(Value::String(result));
    } else {
        return Err("Response Parse Error".into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_request() {
        let needs = HashMap::new();
        let result = translate("Hello", "auto", "zh", "en", needs);
        println!("{result:?}");
    }
}
