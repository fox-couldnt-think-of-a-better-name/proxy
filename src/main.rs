mod proxy;

use anyhow::Result as AnyhowResult;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> AnyhowResult<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .pretty()
        .init();
    // let Result::Ok(app_state) = AppState::init().await else {
    //     bail!("Failed to create AppState");
    // };

    let app = Router::new()
        .route(
            "/attachments/:user_id/:image_hash",
            get(crate::proxy::verify_user_email),
        )
        .layer(TraceLayer::new_for_http());
    // .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Starting on: {addr:?}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;
    println!("Started");
    Ok(())
}
