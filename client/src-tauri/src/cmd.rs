use std::time::Duration;

use reqwest::ClientBuilder;
use vitium_api::net::{EditPass, Req, SignUp};

#[tokio::main]
pub async fn signup(user: &str, pass: &str, server: &str) -> anyhow::Result<()> {
    let req = SignUp {
        user: user.into(),
        pass: pass.into(),
    };
    let path = req.path();
    let url = format!("http://{server}{path}");
    let client = ClientBuilder::new()
        .timeout(Duration::from_secs(30))
        .build()?;
    client.post(url).json(&req).send().await?;
    Ok(())
}

#[tokio::main]
pub async fn chpass(user: &str, old: &str, new: &str, server: &str) -> anyhow::Result<()> {
    let req = EditPass(new.into());
    let path = req.path();
    let url = format!("http://{server}{path}");
    let client = ClientBuilder::new()
        .timeout(Duration::from_secs(30))
        .build()?;
    client
        .post(url)
        .json(&req)
        .basic_auth(user, Some(old))
        .send()
        .await?;
    Ok(())
}
