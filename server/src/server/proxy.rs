use async_trait::async_trait;
use http::uri::Builder;
use pingora::http::{ResponseHeader, StatusCode};
use pingora::proxy::http_proxy_service;
use pingora::proxy::{ProxyHttp, Session};
use pingora::services::Service;
use pingora::upstreams::peer::HttpPeer;
use serde::{Deserialize, Serialize};
use tokio::sync::watch;
use tokio::task::JoinHandle;
use tracing::trace;

use crate::wait_shutdown;

#[derive(Clone, Serialize, Deserialize)]
pub struct SSLConfig {
    pub cert: String,
    pub key: String,
}

impl Default for SSLConfig {
    fn default() -> Self {
        Self {
            cert: "./cert.pem".into(),
            key: "./key.pem".into(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub ssl: Option<SSLConfig>,
    pub http_to_https: bool,
    pub homepage: String,
    pub hostname: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ssl: None,
            http_to_https: false,
            homepage: "https://github.com/misaka10987/vitium".into(),
            hostname: "localhost".into(),
        }
    }
}

pub struct ProxyModule {
    pub cfg: Config,
}

impl ProxyModule {
    pub fn new(cfg: Config) -> Self {
        Self { cfg }
    }

    pub fn start(self) -> anyhow::Result<JoinHandle<()>> {
        let ssl_cfg = self.cfg.ssl.clone();

        let mut proxy = http_proxy_service(&Default::default(), self);

        proxy.add_tcp("[::]:10080");

        if let Some(cfg) = ssl_cfg {
            proxy.add_tls("[::]:10443", &cfg.cert, &cfg.key)?;
        }

        let (send, recv) = watch::channel(false);

        tokio::spawn(async move {
            wait_shutdown().await;
            send.send(true).unwrap();
        });

        let task = tokio::spawn(async move {
            proxy.start_service(None, recv).await;
        });
        Ok(task)
    }

    fn get_host(&self, session: &Session) -> String {
        session
            .req_header()
            .headers
            .get("Host")
            .map(|h| h.to_str().unwrap_or(&self.cfg.hostname))
            .unwrap_or(&self.cfg.hostname)
            .to_string()
    }

    async fn tmp_redirect(&self, session: &mut Session, location: &str) -> pingora::Result<()> {
        let mut head = ResponseHeader::build(StatusCode::TEMPORARY_REDIRECT, None).unwrap();
        head.insert_header("Location", location).unwrap();
        session.write_response_header(Box::new(head), true).await?;
        session.shutdown().await;
        Ok(())
    }
}

#[async_trait]
impl ProxyHttp for ProxyModule {
    type CTX = ();

    fn new_ctx(&self) -> Self::CTX {
        ()
    }

    async fn request_filter(
        &self,
        session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> pingora::Result<bool> {
        // if self.cfg.http_to_https && *session.req_header().uri.scheme().unwrap() == Scheme::HTTP {
        //     let host = self.get_host(session);
        //     let uri = session.req_header().uri.path();
        //     let new_uri = format!("https://{}{}", host, uri);
        //     self.send_redirect(session, &new_uri).await?;
        //     return Ok(true);
        // }

        let uri = session.req_header().uri.clone();
        let path = uri.path();

        if path == "/" {
            self.tmp_redirect(session, &self.cfg.homepage).await?;
            return Ok(true);
        }

        if !path.starts_with("/api") {
            let new_uri = format!("http://localhost:3000{path}");
            self.tmp_redirect(session, &new_uri).await?;
            return Ok(true);
        }

        Ok(false)
    }

    async fn upstream_peer(
        &self,
        session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> pingora::Result<Box<HttpPeer>> {
        let head = session.req_header_mut();

        let uri = head.uri.clone();

        if let Some(path) = head.uri.path_and_query() {
            if path.path().starts_with("/api") {
                trace!("here");
                let new = &path.path()[4..];
                let new_uri = Builder::from(uri).path_and_query(new).build().unwrap();
                head.set_uri(new_uri);
                trace!("redirecting");
                return Ok(Box::new(HttpPeer::new(
                    "localhost:10987",
                    false,
                    "".to_string(),
                )));
            }
        }

        Err(Box::new(*pingora::Error::new(
            pingora::ErrorType::HTTPStatus(StatusCode::TEMPORARY_REDIRECT.as_u16()),
        )))
    }
}
