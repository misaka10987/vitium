use axum::{routing::get, Router};
use tokio::signal;

/// A handler always returns `Hello, world!\n`.
async fn hello() -> &'static str {
    "Hello, World!\n"
}

/// The http server.
pub struct HttpServer {
    pub port: u16,
    pub chat_cap: usize,
    pub module: Vec<String>,
    pub server: (),
    app: Router,
}

impl HttpServer {
    pub async fn run(&mut self) {
        self.app = Router::new().route("/hello", get(hello));
        // listening globally on port 3000
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", 59999))
            .await
            .expect("failed to bind TCP listener");
        // run our app with hyper
        axum::serve(listener, self.app.clone())
            .with_graceful_shutdown(sig_shut())
            .await
            .unwrap()
    }
}

async fn sig_shut() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}