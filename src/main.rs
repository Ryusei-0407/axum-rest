use anyhow::Result;
use axum::{
    extract::{Extension, FromRequest, RequestParts},
    http::StatusCode,
    routing::{get, post},
    AddExtensionLayer, Json, Router,
};
use chrono::NaiveDateTime;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::{FromRow, Row};
use std::net::SocketAddr;

async fn check_health() -> &'static str {
    "Hello, world!"
}

struct DatabaseConnection(sqlx::pool::PoolConnection<sqlx::Postgres>);

#[axum::async_trait]
impl<B: Send> FromRequest<B> for DatabaseConnection {
    type Rejection = (StatusCode, String);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(pool) = Extension::<PgPool>::from_request(req)
            .await
            .map_err(internal_error)?;

        let conn = pool.acquire().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

#[derive(Debug, Serialize, FromRow)]
struct User {
    id: i32,
    name: String,
    age: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

async fn get_users(
    DatabaseConnection(conn): DatabaseConnection,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let mut conn = conn;
    let sql = "SELECT * FROM user_table".to_string();
    let rows = sqlx::query(&sql);
    let users: Vec<User> = rows
        .map(|row: PgRow| User {
            id: row.get("id"),
            name: row.get("name"),
            age: row.get("age"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .fetch_all(&mut conn)
        .await
        .unwrap();

    Ok(Json(users))
}

#[derive(Debug, Deserialize)]
struct InputUser {
    name: String,
    age: i32,
}

async fn create_user(DatabaseConnection(conn): DatabaseConnection, Json(user): Json<InputUser>) {
    let mut conn = conn;
    let (name, age) = (user.name, user.age);
    let sql = "INSERT INTO user_table (name, age, created_at, updated_at) VALUES ($1, $2, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP) RETURNING id, name, age, created_at, updated_at".to_string();
    sqlx::query(&sql)
        .bind(name)
        .bind(age)
        .fetch_one(&mut conn)
        .await
        .unwrap();
}

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "demo-axum=debug")
    }
    tracing_subscriber::fmt::init();

    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("can connect to database");

    let app = Router::new()
        .route("/", get(check_health))
        .route("/api/v1/users", get(get_users))
        .route("/api/v1/create", post(create_user))
        .layer(AddExtensionLayer::new(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
