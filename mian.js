async function translate(text, from, to, options) {
    const { utils, detect } = options;
    const { tauriFetch: fetch } = utils;
    
    if (from==="auto") {
        let langMap = {
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
        };
        if (langMap.includes(detect)) {
            from=langMap[detect];
        }else{
            from = "en"
        }
    }
    const res = await fetch("https://wxapp.translator.qq.com/api/translate", {
        method: 'GET',
        headers: {
            "User-Agent": "Mozilla/5.0 (iPhone; CPU iPhone OS 16_3_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148 MicroMessenger/8.0.32(0x18002035) NetType/WIFI Language/zh_TW",
            "Content-Type": "application/json",
            "Host": " wxapp.translator.qq.com",
            "Referer": "https://servicewechat.com/wxb1070eabc6f9107e/117/page-frame.html"
        },
        query: {
            "source":"auto",
            "target":"auto",
            "sourceText":text,
            "platform":"WeChat_APP",
            "guid":"oqdgX0SIwhvM0TmqzTHghWBvfk22",
            "candidateLangs":`${from}|${to}`
        }
    });

    if (res.ok) {
        let result = res.data;
        const { targetText } = result;
        if (targetText) {
            return targetText;
        } else {
            throw JSON.stringify(result);
        }
    } else {
        throw `Http Request Error\nHttp Status: ${res.status}\n${JSON.stringify(res.data)}`;
    }
}