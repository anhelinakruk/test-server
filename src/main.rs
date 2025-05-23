use std::net::SocketAddr;

use axum::{Router, routing::get};
use axum_server::tls_rustls::RustlsConfig;

mod api;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .nest("/api", api::router());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    let config = RustlsConfig::from_der(
        vec![include_bytes!("../certs/server_cert.der").to_vec()],
        include_bytes!("../certs/server_key.der").to_vec(),
    )
    .await
    .unwrap();

    println!("Listening on {}", addr);
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> String {
    "Hello, World!".to_string()
}
