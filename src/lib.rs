use worker::*;
use reqwest;

#[event(fetch)]
async fn main(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    const BASE_URL: &str = "https://opgg-desktop-data.akamaized.net";
    let url: Url;
    let file_name: &str;
    let file_url: String;

    url = match req.url() {
        Ok(v) => v,
        Err(e) => return Response::error(e.to_string(), 501)
    };
    file_name = match url.path().split("/").last() {
        Some(v) => v,
        None => return Response::error("Filename not found", 404)
    };
    file_url = format!("{}/{}", BASE_URL, file_name);

    let response = match reqwest::get(file_url).await {
        Ok(v) => v,
        Err(e) => return Response::error(e.to_string(), 501),
    };

    let mut content = response.text().await.unwrap();

    if file_name.ends_with(".js") || file_name.ends_with(".css") || file_name.ends_with(".html")
    {
        content = content
            .replace("https://opgg-desktop-data.akamaized.net", "https://opgg-patcher.midknight-dev.workers.dev")
            .replace(r#"location.href="https://app.labs.sydney"#, r#"location.href2="https://app.labs.sydney"#)
            .replace("https://app.labs.sydney", "https://opgg-patcher.midknight-dev.workers.dev");

        if file_name.ends_with(".js")
        {
            let re = regex::Regex::new(r"<body>.*</body>").unwrap();

            content = re.replace_all(&content, "").to_string();
            content = content.replace("https://www.mobwithad.com", "https://google.com");
            content.push_str("\ndocument.head.insertAdjacentHTML(\"beforeend\", '<style>#ads-container,#ads-container2,#ads-container3,#sids-ads,main > div[style]:last-child{display: none !important}</style>')");
        }

        return Response::ok(content);
    }

    return Response::ok(content);
}
