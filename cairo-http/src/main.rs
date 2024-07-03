use crate::logger::setup_logger;

use anyhow::{Context, Result};
use axum::routing::post;
use axum::Router;
use http::endpoints;
use log::info;
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod cairo;
mod error;
mod http;
mod logger;

#[tokio::main]
async fn main() -> Result<()> {
    setup_logger().context("Setup logger")?;

    info!("Starting Cairo Http!");

    let app = Router::new()
        .route("/prove", post(endpoints::prove_handler))
        .route("/verify", post(endpoints::verify_handler));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    let tcp_listener = TcpListener::bind(addr).await?;

    axum::serve(tcp_listener, app.into_make_service())
        .await
        .context("Starting http server")
}
