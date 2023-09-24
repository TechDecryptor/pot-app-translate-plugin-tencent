use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use reqwest::header::{HeaderMap,HeaderName,HeaderValue, self};
use uuid::Uuid;
use chrono::prelude::*;

fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::ACCEPT,
        HeaderValue::from_static("application/json, text/javascript, */*; q=0.01"),
    );
    headers.insert(
        header::ACCEPT_ENCODING,
        HeaderValue::from_static("gzip, deflate, br"),
    );
    headers.insert(
        header::ACCEPT_LANGUAGE,
        HeaderValue::from_static("zh-CN,zh;q=0.9"),
    );
    headers.insert(
        header::CONNECTION,
        HeaderValue::from_static("keep-alive"),
    );
    headers.insert(
        header::HOST,
        HeaderValue::from_static("fanyi.qq.com"),
    );
    headers.insert(
        header::ORIGIN,
        HeaderValue::from_static(
            "https://fanyi.qq.com",
        ),
    );
    headers.insert(
        header::REFERER,
        HeaderValue::from_static("https://fanyi.qq.com/"),
    );
    headers.insert(
        HeaderName::from_static("sec-fetch-dest"),
        HeaderValue::from_static("empty"),
    );
    headers.insert(
        HeaderName::from_static("sec-fetch-mode"),
        HeaderValue::from_static("cors"),
    );
    headers.insert(
        HeaderName::from_static("sec-fetch-site"),
        HeaderValue::from_static("same-origin"),
    );
    headers.insert(
        header::USER_AGENT,
        HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36"),
    );
    headers.insert(
        HeaderName::from_static("x-requested-with"),
        HeaderValue::from_static("XMLHttpRequest"),
    );
    headers.insert(
        HeaderName::from_static("dnt"),
        HeaderValue::from_static("1"),
    );
    headers.insert(
        HeaderName::from_static("sec-ch-ua"),
        HeaderValue::from_static("\"Chromium\";v=\"116\", \"Not)A;Brand\";v=\"24\", \"Google Chrome\";v=\"116\""),
    );
    headers.insert(
        HeaderName::from_static("sec-ch-ua-mobile"),
        HeaderValue::from_static("?0"),
    );
    headers.insert(
        HeaderName::from_static("sec-ch-ua-platform"),
        HeaderValue::from_static("\"Windows\""),
    );
    headers.insert(
        HeaderName::from_static("sec-gpc"),
        HeaderValue::from_static("1"),
    );
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded; charset=UTF-8"),
    );
    headers.insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("no-cache"),
    );

    headers
}

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
        .header("Cookie", format!("fy_guid={guid}"))
        .headers(construct_headers())
        .send()?
        .json()?;
    fn parse_auth(res: Value) -> Option<(String,String)> {
        let qtv = res.as_object()?.get("qtv")?.as_str()?.to_string();
        let qtk = res.as_object()?.get("qtv")?.as_str()?.to_string();

        Some((qtv,qtk))
    }
    let (qtv,qtk)=match parse_auth(auth_res){
        Some(v)=>v,
        None => return Err("Auth Parse Error".into()),
    };
    let dt=Utc::now();
    let time=dt.timestamp_millis();
    println!("{time}");
    let translate_id=format!("translate_uuid{time}");
    let mut body = HashMap::new();
    body.insert("source", from);
    body.insert("target", to);
    body.insert("sourceText", text);
    body.insert("qtk", &qtk);
    body.insert("qtv", &qtv);
    body.insert("ticket","");
    body.insert("randstr","");
    body.insert("sessionUuid",&translate_id);
    let res = client
        .post("https://fanyi.qq.com/api/translate")
        .headers(construct_headers())
        .header("Cookie", format!("fy_guid={guid}; openCount=1; qtv={qtv}; qtk={qtk}"))
        .form(&body)
        .send()?
        .json()?;
    fn parse_result(res: Value) -> Option<String> {
        let mut target=String::new();
        let result = res.as_object()?.get("translate")?.as_object()?.get("records")?.as_array()?;
        for line in result{
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
        let result = translate("Hello", "auto", "zh", "", needs).unwrap();
        println!("{result}");
    }
}
