use anyhow::Result;
use axum::{
    extract::{ConnectInfo, Extension, FromRequest, Path, RequestParts},
    http::StatusCode,
    routing::{delete, get, post},
    AddExtensionLayer, Json, Router,
};
use chrono::NaiveDateTime;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::{FromRow, Row};
use std::net::SocketAddr;
use uuid::Uuid;

async fn check_health() -> &'static str {
    "OK!"
}

async fn handler(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> String {
    format!("your IP address is: {}", addr)
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
    uuid: Uuid,
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
            uuid: row.get("uuid"),
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

async fn get_user_by_uuid(
    Path(uuid): Path<Uuid>,
    DatabaseConnection(conn): DatabaseConnection,
) -> Result<Json<User>, (StatusCode, String)> {
    let mut conn = conn;
    let sql = "SELECT * FROM user_table WHERE uuid = $1".to_string();
    let user = sqlx::query_as(&sql)
        .bind(uuid)
        .fetch_one(&mut conn)
        .await
        .unwrap();

    Ok(Json(user))
}

#[derive(Debug, Deserialize)]
struct InputUser {
    name: String,
    age: i32,
}

async fn create_user(
    DatabaseConnection(conn): DatabaseConnection,
    Json(payload): Json<InputUser>,
) -> Result<String, (StatusCode, String)> {
    let mut conn = conn;
    let sql = "INSERT INTO user_table (name, age, created_at, updated_at) VALUES ($1, $2, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP) RETURNING uuid, name, age, created_at, updated_at".to_string();
    sqlx::query(&sql)
        .bind(payload.name)
        .bind(payload.age)
        .fetch_one(&mut conn)
        .await
        .unwrap();

    Ok("Sucess".to_string())
}

#[derive(Debug, Deserialize)]
struct Name {
    name: String,
}

async fn update_name(
    Path(uuid): Path<Uuid>,
    DatabaseConnection(conn): DatabaseConnection,
    Json(payload): Json<Name>,
) -> Result<Json<User>, (StatusCode, String)> {
    let mut conn = conn;
    let sql = "UPDATE user_table SET name = $1, updated_at = CURRENT_TIMESTAMP WHERE uuid = $2 RETURNING uuid, name, age, created_at, updated_at".to_string();
    let res = sqlx::query_as(&sql)
        .bind(payload.name)
        .bind(uuid)
        .fetch_one(&mut conn)
        .await
        .unwrap();

    Ok(Json(res))
}

#[derive(Debug, Deserialize)]
struct Age {
    age: i32,
}

async fn update_age(
    Path(uuid): Path<Uuid>,
    DatabaseConnection(conn): DatabaseConnection,
    Json(payload): Json<Age>,
) -> Result<Json<User>, (StatusCode, String)> {
    let mut conn = conn;
    let sql = "UPDATE user_table SET age = $1, updated_at = CURRENT_TIMESTAMP WHERE uuid = $2 RETURNING uuid, name, age, created_at, updated_at".to_string();
    let res = sqlx::query_as(&sql)
        .bind(payload.age)
        .bind(uuid)
        .fetch_one(&mut conn)
        .await
        .unwrap();

    Ok(Json(res))
}

async fn delete_user(
    Path(uuid): Path<Uuid>,
    DatabaseConnection(conn): DatabaseConnection,
) -> Result<String, (StatusCode, String)> {
    let mut conn = conn;
    let sql = "DELETE FROM user_table WHERE uuid = $1".to_string();
    sqlx::query(&sql)
        .bind(uuid)
        .execute(&mut conn)
        .await
        .unwrap();

    Ok("Sucess".to_string())
}

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "app=debug")
    }
    tracing_subscriber::fmt::init();

    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("can connect to database");

    let update_routes = Router::new()
        .route("/name/:uuid", post(update_name))
        .route("/age/:uuid", post(update_age));

    let api_routes = Router::new()
        .route("/users", get(get_users))
        .route("/users/:uuid", get(get_user_by_uuid))
        .route("/create", post(create_user))
        .route("/delete/:uuid", delete(delete_user))
        .nest("/update", update_routes);

    let app = Router::new()
        .route("/", get(check_health))
        .route("/addr", get(handler))
        .nest("/api/v1", api_routes)
        .layer(AddExtensionLayer::new(pool))
        .into_make_service_with_connect_info::<SocketAddr, _>();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{addr}");

    axum::Server::bind(&addr).serve(app).await.unwrap();
}
