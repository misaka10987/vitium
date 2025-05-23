use async_trait::async_trait;
use http::{
    header::{HOST, LOCATION, UPGRADE},
    uri::Builder,
};
use pingora::{
    http::{ResponseHeader, StatusCode},
    listeners::tls::TlsSettings,
    proxy::{ProxyHttp, Session, http_proxy_service},
    services::Service,
    upstreams::peer::HttpPeer,
};
use serde::{Deserialize, Serialize};
use shutup::ShutUp;
use std::net::{Ipv6Addr, SocketAddr};
use tokio::sync::watch;

#[derive(Clone, Serialize, Deserialize)]
pub struct SSLConfig {
    pub port: u16,
    pub cert: String,
    pub key: String,
}

impl Default for SSLConfig {
    fn default() -> Self {
        Self {
            port: 443,
            cert: "./cert.pem".into(),
            key: "./key.pem".into(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub port: u16,
    pub http_to_https: bool,
    pub homepage: String,
    pub hostname: String,
    pub ssl: Option<SSLConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 80,
            ssl: None,
            http_to_https: false,
            homepage: "https://github.com/misaka10987/vitium".into(),
            hostname: "localhost".into(),
        }
    }
}

pub struct ProxyServer {
    pub cfg: Config,
    pub api_port: u16,
    pub shutdown: ShutUp,
}

impl ProxyServer {
    pub fn new(cfg: Config, api_port: u16) -> Self {
        Self {
            cfg,
            api_port,
            shutdown: ShutUp::new(),
        }
    }

    pub fn start(self) -> anyhow::Result<ShutUp> {
        let cfg = self.cfg.clone();

        let shutdown = self.shutdown.clone();

        let mut proxy = http_proxy_service(&Default::default(), self);

        proxy.add_tcp(&format!("[::]:{}", cfg.port));

        if let Some(cfg) = cfg.ssl {
            // proxy.add_tls(&format!("[::]:{}",self.cfg.port), &cfg.cert, &cfg.key)?;
            let mut set = TlsSettings::intermediate(&cfg.cert, &cfg.key)?;
            set.enable_h2();
            proxy.add_tls_with_settings(&format!("[::]:{}", cfg.port), None, set);
        }

        let (send, recv) = watch::channel(false);

        shutdown.register_hook(move || send.send(true).expect("shutdown HTTP proxy"));

        tokio::spawn(async move {
            proxy.start_service(None, recv, 1).await;
        });
        Ok(shutdown)
    }

    fn get_host(&self, session: &Session) -> String {
        session
            .req_header()
            .headers
            .get(HOST)
            .map(|h| h.to_str().unwrap_or(&self.cfg.hostname))
            .unwrap_or(&self.cfg.hostname)
            .to_string()
    }

    async fn tmp_redirect(&self, session: &mut Session, location: &str) -> pingora::Result<()> {
        let mut head = ResponseHeader::build(StatusCode::TEMPORARY_REDIRECT, None).unwrap();
        head.insert_header(LOCATION, location).unwrap();
        session.write_response_header(head.into(), true).await?;
        session.write_response_body(None, true).await?;
        session.finish_body().await?;
        session.shutdown().await;
        Ok(())
    }
}

#[async_trait]
impl ProxyHttp for ProxyServer {
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
                let new = &path.path()[4..];
                let new_uri = Builder::from(uri).path_and_query(new).build().unwrap();
                head.set_uri(new_uri);
                let addr = SocketAddr::from((Ipv6Addr::LOCALHOST, self.api_port));
                let mut peer: HttpPeer = HttpPeer::new(addr, false, "".into());
                match head.headers.get(UPGRADE) {
                    Some(_) => peer.options.set_http_version(1, 1),
                    None => peer.options.set_http_version(2, 2),
                }
                return Ok(Box::new(peer));
            }
        }

        Err(Box::new(*pingora::Error::new(
            pingora::ErrorType::HTTPStatus(StatusCode::INTERNAL_SERVER_ERROR.as_u16()),
        )))
    }
}
