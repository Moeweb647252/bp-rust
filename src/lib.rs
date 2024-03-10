use std::str::FromStr;

use worker::*;

#[event(fetch)]
async fn main(mut _req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let url = _req.url()?;
    let req_url = url.to_string().replace(
        if let Some(p) = url.port() {
            format!("{}:{}/", url.host_str().unwrap(), p)
        } else {
            format!("{}/", url.host_str().unwrap())
        }
        .as_str(),
        "",
    );
    let mut headers = _req.headers().clone();
    headers.set(
        "host",
        Url::from_str(&req_url)?
            .host()
            .unwrap()
            .to_string()
            .as_str(),
    )?;
    let mut req_init = RequestInit::new();
    req_init.with_headers(headers).with_method(_req.method());
    let req = Request::new_with_init(&req_url, &req_init)?;
    let mut resp = Fetch::Request(req).send().await?;
    Response::from_stream(resp.stream()?)
}
