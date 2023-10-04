use chrono::prelude::*;
use reqwest::header::{self, HeaderMap, HeaderName, HeaderValue};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use uuid::Uuid;

#[no_mangle]
pub fn translate(
    text: &str,
    from: &str,
    to: &str,
    _detect: &str,
    _needs: HashMap<String, String>,
) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::blocking::ClientBuilder::new().build()?;

    let guid = Uuid::new_v4().to_string();
    let auth_res:Value = client
        .post("https://fanyi.qq.com/api/reauth12f")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36")
        .header("Accept", "application/json, text/javascript, */*; q=0.01")
        .header("Accept-Encoding", "gzip, deflate, br")
        .header("Accept-Language", "zh-CN,zh;q=0.9")
        .header("Connection", "keep-alive")
        .header("Host", "fanyi.qq.com")
        .header("Origin", "https://fanyi.qq.com")
        .header("Referer", "https://fanyi.qq.com/")
        .header("Sec-Fetch-Dest", "empty")
        .header("Sec-Fetch-Mode", "cors")
        .header("Sec-Fetch-Site", "same-origin")
        .header("X-Requested-With", "XMLHttpRequest")
        .header("dnt", "1")
        .header("sec-ch-ua", "\"Chromium\";v=\"116\", \"Not)A;Brand\";v=\"24\", \"Google Chrome\";v=\"116\"")
        .header("sec-ch-ua-mobile", "?0")
        .header("sec-ch-ua-platform", "\"Windows\"")
        .header("sec-gpc", "1")
        .header("Cookie", format!("fy_guid={guid}"))
        .header("Cache-Control", "no-cache")
        .send()?
        .json()?;
    fn parse_auth(res: Value) -> Option<(String, String)> {
        let qtv = res.as_object()?.get("qtv")?.as_str()?.to_string();
        let qtk = res.as_object()?.get("qtk")?.as_str()?.to_string();

        Some((qtv, qtk))
    }
    let (qtv, qtk) = match parse_auth(auth_res) {
        Some(v) => v,
        None => return Err("Auth Parse Error".into()),
    };
    let dt = Utc::now();
    let time = dt.timestamp_millis();

    let translate_id = format!("translate_uuid{time}");
    let mut body = HashMap::new();
    body.insert("source", from);
    body.insert("target", to);
    body.insert("sourceText", text);
    body.insert("qtk", &qtk);
    body.insert("qtv", &qtv);
    body.insert("ticket", "");
    body.insert("randstr", "");
    body.insert("sessionUuid", &translate_id);

    let res = client
        .post("https://fanyi.qq.com/api/translate")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Connection", "keep-alive")
        .header("Host", "fanyi.qq.com")
        .header("Origin", "https://fanyi.qq.com")
        .header("Referer", "https://fanyi.qq.com/")
        .header("Sec-Fetch-Dest", "empty")
        .header("Sec-Fetch-Mode", "cors")
        .header("Sec-Fetch-Site", "same-origin")
        .header("X-Requested-With", "XMLHttpRequest")
        .header("dnt", "1")
        .header("sec-ch-ua", "\"Chromium\";v=\"116\", \"Not)A;Brand\";v=\"24\", \"Google Chrome\";v=\"116\"")
        .header("sec-ch-ua-mobile", "?0")
        .header("sec-ch-ua-platform", "\"Windows\"")
        .header("sec-gpc", "1")
        .header("Cookie", format!("fy_guid={guid}; qtv={qtv}; qtk={qtk}"))
        .form(&body).send()?.json()?;

    fn parse_result(res: Value) -> Option<String> {
        let mut target = String::new();
        let result = res
            .as_object()?
            .get("translate")?
            .as_object()?
            .get("records")?
            .as_array()?;
        for line in result {
            target.push_str(line.as_object()?.get("targetText")?.as_str()?);
        }
        Some(target)
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
        let result = translate("Hello", "auto", "zh", "", needs);
        println!("{result:?}");
    }
}
