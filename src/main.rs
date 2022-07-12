mod view;
mod model;

use core::panicking::panic;
use std::net::SocketAddr;
use axum::Router;
use axum::routing::get;
use sea_orm::{Database, DatabaseConnection};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "better_school=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db = Database::connect(std::env::var("DATABASE_URL").unwrap()).await?;

    let app = Router::new()
        .route("/", get(view::main_view::welcome));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
