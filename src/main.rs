#[macro_use]
extern crate diesel;
extern crate dotenv;

mod view;
mod form;
mod model;
mod error;

use std::net::SocketAddr;
use axum::{Extension, Router};
use axum::routing::get;
use diesel::{Connection, SqliteConnection};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use dotenv::dotenv;
use diesel::r2d2::{Pool, ConnectionManager};

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "better_school=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = Pool::builder()
        .max_size(1)
        .build(connection)
        .expect("Could not build connection pool");

    let app = Router::new()
        .route("/", get(view::main_view::welcome))
        .layer(Extension(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
