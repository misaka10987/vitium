use reqwest::Client;
use tokio::runtime;
use vitium_api::net::{EditPass, Req, SignUp};

pub fn signup(user: &str, pass: &str, server: &str) -> anyhow::Result<()> {
    let req = SignUp {
        user: user.into(),
        pass: pass.into(),
    };
    let path = req.path();
    let url = format!("http://{server}{path}");
    let client = Client::new();
    let run = runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    run.block_on(client.post(url).json(&req).send())?;
    Ok(())
}

pub fn chpass(user: &str, old: &str, new: &str, server: &str) -> anyhow::Result<()> {
    let req = EditPass(new.into());
    let path = req.path();
    let url = format!("http://{server}{path}");
    let client = Client::new();
    let run = runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    let future = client
        .post(url)
        .json(&req)
        .basic_auth(user, Some(old))
        .send();
    run.block_on(future)?;
    Ok(())
}
