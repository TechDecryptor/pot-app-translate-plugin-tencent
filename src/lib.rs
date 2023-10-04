use chrono::prelude::*;
use dirs::config_dir;
use reqwest::header::{self, HeaderMap, HeaderName, HeaderValue};
use serde_json::{json, Value};
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
    let config_dir_path = config_dir().unwrap();
    let cookie_file_path = config_dir_path
        .join("com.pot-app.desktop")
        .join("plugins")
        .join("translate")
        .join("[plugin].com.TechDecryptor.tencent")
        .join("cookie.json");
    let cookie_file = match cookie_file_path.exists() {
        true => std::fs::File::open(&cookie_file_path)?,
        false => std::fs::File::create(&cookie_file_path)?
    };
    let metedata = cookie_file.metadata()?;
    let modified = metedata.modified()?;
    let modified = modified.elapsed()?.as_secs();

    let file_content = std::fs::read_to_string(&cookie_file_path)?;
    let mut guid = String::new();
    let mut qtv = String::new();
    let mut qtk = String::new();
    if file_content.is_empty() {
        guid = Uuid::new_v4().to_string();
    } else {
        let cookie: Value = serde_json::from_str(&file_content)?;
        guid = cookie
            .as_object()
            .unwrap()
            .get("guid")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        qtv = cookie
            .as_object()
            .unwrap()
            .get("qtv")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        qtk = cookie
            .as_object()
            .unwrap()
            .get("qtk")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
    }
    if modified > 30 || qtv.is_empty() {
        let mut auth_req = client
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
        .header("Cache-Control", "no-cache");
        if !qtv.is_empty() && !qtk.is_empty() {
            let mut params = HashMap::new();
            params.insert("qtv", &qtv);
            params.insert("qtk", &qtk);
            auth_req = auth_req
                .header("Cookie", format!("fy_guid={guid}; qtv={qtv}; qtk={qtk}"))
                .form(&params);
        } else {
            auth_req = auth_req.header("Cookie", format!("fy_guid={guid}"));
        }
        let auth_res = auth_req.send()?.json()?;
        fn parse_auth(res: Value) -> Option<(String, String)> {
            let qtv = res.as_object()?.get("qtv")?.as_str()?.to_string();
            let qtk = res.as_object()?.get("qtk")?.as_str()?.to_string();
            Some((qtv, qtk))
        }
        (qtv, qtk) = match parse_auth(auth_res) {
            Some(v) => v,
            None => return Err("Auth Parse Error".into()),
        };
        std::fs::write(
            cookie_file_path,
            json!({"guid": guid, "qtv": qtv, "qtk": qtk}).to_string(),
        )?;
    }

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
        .form(&body)
        .send()?
        .json()?;

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
