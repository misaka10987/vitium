use anyhow::anyhow;
use reqwest::header::COOKIE;
use vitium_api::net::Req;

use crate::{auth::TOKEN, CLIENT, SERVER_ADDR};

pub async fn send<T: Req>(req: T) -> anyhow::Result<<T as Req>::Response> {
    let host = SERVER_ADDR.read().await;
    let url = format!("http://{host}{}", req.path());
    let builder = match T::METHOD {
        "GET" => CLIENT.get(url),
        "POST" => CLIENT.post(url),
        "HEAD" => CLIENT.head(url),
        "PUT" => CLIENT.put(url),
        "DELETE" => CLIENT.delete(url),
        "PATCH" => CLIENT.patch(url),
        _ => return Err(anyhow!("invalid method")),
    };
    let res = builder
        .json(&req)
        .header(COOKIE, format!("token={}", TOKEN.read().await))
        .send()
        .await?;
    if !res.status().is_success() {
        return Err(anyhow!("HTTP {}", res.status()));
    }
    let body = res.json().await?;
    Ok(body)
}
