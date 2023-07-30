use worker::*;


#[event(fetch)]
async fn main(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    const BASE_URL: &str = "https://opgg-desktop-data.akamaized.net";
    let url: Url = req.url()?;
    let file_url: String = format!("{}/{}", BASE_URL, url.path());

    let response = match reqwest::get(file_url).await {
        Ok(v) => v,
        Err(e) => {
            let status = e.status().unwrap().as_u16();
            return Response::error(e.to_string(), status)
        },
    };

    let mut headers = Headers::new();
    for (name, value) in response.headers().iter() {
        let _ = headers.set(name.as_str(), value.to_str().unwrap());
    }

    let content = match response.text().await {
        Ok(v) => v,
        Err(e) => return Response::error(e.to_string(), 500)
    };

    // if file_name.ends_with(".js") || file_name.ends_with(".css") || file_name.ends_with(".html")
    // {
    //     content = content
    //         .replace("https://opgg-desktop-data.akamaized.net", "https://opgg-patcher.midknight-dev.workers.dev")
    //         .replace(r#"location.href="https://app.labs.sydney"#, r#"location.href2="https://app.labs.sydney"#)
    //         .replace("https://app.labs.sydney", "https://opgg-patcher.midknight-dev.workers.dev");

    //     if file_name.ends_with(".js")
    //     {
    //         let re = regex::Regex::new(r"<body>.*</body>").unwrap();

    //         content = re.replace_all(&content, "").to_string();
    //         content = content.replace("https://www.mobwithad.com", "https://google.com");
    //         content.push_str("\ndocument.head.insertAdjacentHTML(\"beforeend\", '<style>#ads-container,#ads-container2,#ads-container3,#sids-ads,main > div[style]:last-child{display: none !important}</style>')");
    //     }
    // }

    Ok(Response::ok(content)?.with_headers(headers))
}
