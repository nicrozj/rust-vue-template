use anyhow::Context;
use axum::{routing::get, Router};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let router = Router::new().route("/", get(|| async { "Hello world!" }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .context("error bind address")?;

    axum::serve(listener, router)
        .await
        .context("error serve app")?;

    Ok(())
}
