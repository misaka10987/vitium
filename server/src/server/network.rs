use std::{
    net::{Ipv6Addr, TcpListener},
    path::PathBuf,
    sync::{OnceLock, atomic::AtomicBool},
    time::Duration,
};

use axum::{Router, routing::IntoMakeService};
use axum_server::{Handle, tls_rustls::RustlsConfig};
use local_ip_address::{local_ip, local_ipv6};
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;
use shutup::ShutUp;
use tracing::error;
use url::Url;

/// Configuration for the HTTP server.
#[serde_inline_default]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Hostname of the HTTP server.
    /// If not specified, the local IP address would be used.
    pub host: Option<String>,
    /// Port to start the HTTP server.
    /// If not specified, would use the random port assigned by system.
    pub port: Option<u16>,
    pub https: Option<HttpsConfig>,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            host: None,
            port: None,
            https: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HttpsConfig {
    pub cert: PathBuf,
    pub key: PathBuf,
}

pub struct NetworkModule {
    pub config: NetworkConfig,
    started: AtomicBool,
    shutdown: ShutUp,
    port: OnceLock<u16>,
}

fn start_http(tcp: TcpListener, app: IntoMakeService<Router>, handle: Handle) {
    tokio::spawn(async move {
        let exit = axum_server::from_tcp(tcp).handle(handle).serve(app).await;
        if let Err(e) = exit {
            error!("HTTP server exited with error: {e}");
        }
    });
}

fn start_https(tcp: TcpListener, tls: RustlsConfig, app: IntoMakeService<Router>, handle: Handle) {
    tokio::spawn(async move {
        let exit = axum_server::from_tcp_rustls(tcp, tls)
            .handle(handle)
            .serve(app)
            .await;
        if let Err(e) = exit {
            error!("HTTPS server exited with error: {e}");
        }
    });
}

impl NetworkModule {
    pub fn new(config: NetworkConfig) -> Self {
        Self {
            config,
            shutdown: ShutUp::new(),
            started: AtomicBool::new(false),
            port: OnceLock::new(),
        }
    }

    pub fn started(&self) -> bool {
        self.started.load(std::sync::atomic::Ordering::SeqCst)
    }

    pub async fn start(&self, app: IntoMakeService<Router>) -> anyhow::Result<ShutUp> {
        if self.started() {
            panic!("can not start server for multiple times")
        }
        self.started
            .store(true, std::sync::atomic::Ordering::SeqCst);

        let tcp = TcpListener::bind((Ipv6Addr::UNSPECIFIED, self.config.port.unwrap_or(0)))?;

        let port = tcp.local_addr()?.port();
        self.port.set(port).unwrap();

        let handle = Handle::new();

        if let Some(https) = &self.config.https {
            let tls = RustlsConfig::from_pem_file(&https.cert, &https.key).await?;
            start_https(tcp, tls, app, handle.clone());
        } else {
            start_http(tcp, app, handle.clone());
        }

        self.shutdown
            .register_hook(move || handle.graceful_shutdown(Some(Duration::from_secs(30))));

        Ok(self.shutdown.clone())
    }

    pub fn url(&self) -> anyhow::Result<Url> {
        let host = if let Some(host) = &self.config.host {
            host.clone()
        } else {
            let ip = local_ipv6().or(local_ip())?;
            ip.to_string()
        };
        let port = self.port.get().unwrap();
        let scheme = if self.config.https.is_some() {
            "https"
        } else {
            "http"
        };
        let url = Url::parse(&format!("{scheme}://{host}:{port}/")).unwrap();
        Ok(url)
    }
}
