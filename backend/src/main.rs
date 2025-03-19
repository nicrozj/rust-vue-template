use anyhow::Context;
use axum::{routing::get, Router};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let router = Router::new().route("/test", get(|| async { "Hello world! нуныун" }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .context("error bind address")?;

    axum::serve(listener, router)
        .await
        .context("error serve app")?;

    Ok(())
}
