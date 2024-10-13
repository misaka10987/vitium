use reqwest::Client;
use vitium_api::net::{Req, SignUp};

pub fn signup(user: &str, pass: &str, server: &str) -> anyhow::Result<()> {
    let req = SignUp {
        user: user.into(),
        pass: pass.into(),
    };
    let path = req.path();
    let url = format!("http://{server}{path}");
    let client = Client::new();
    let run = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    run.block_on(client.post(url).json(&req).send())?;
    Ok(())
}
