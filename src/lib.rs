use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

#[no_mangle]
pub fn translate(
    text: &str,
    from: &str,
    to: &str,
    _needs: HashMap<String, String>,
) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::blocking::ClientBuilder::new().build()?;
    let mut body = HashMap::new();
    body.insert("source", from);
    body.insert("target", to);
    body.insert("sourceText", text);
    let res = client
        .post("https://fanyi.qq.com/api/translate")
        .header("Content-Type","application/x-www-form-urlencoded")
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
        let result = translate("Hello", "auto", "zh", needs).unwrap();
        println!("{result}");
    }
}
