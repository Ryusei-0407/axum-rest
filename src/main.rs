use axum::{
    async_trait,
    extract::{Extension, FromRequest, RequestParts},
    http::StatusCode,
    routing::get,
    AddExtensionLayer, Router,
};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::{net::SocketAddr, time::Duration};

async fn index() -> &'static str {
    "Hello, world!"
}

async fn using_connection_pool_extractor(
    Extension(pool): Extension<PgPool>,
) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select 'Hello World from pg'")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "example_tokio_postgres=debug")
    }
    tracing_subscriber::fmt::init();

    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("can connect to database");

    let app = Router::new()
        .route("/", get(index))
        .route("/api/v1", get(using_connection_pool_extractor))
        .layer(AddExtensionLayer::new(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on http://{addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
