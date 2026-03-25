use std::{convert::Infallible, net::SocketAddr, time::Duration};
use std::error::Error;

use axum::{
    extract::State,
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{
        sse::{Event, Sse},
        IntoResponse, Response,
    },
    routing::{get},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use tokio_stream::{wrappers::IntervalStream, StreamExt};

#[derive(Clone)]
struct AppState {
    db: SqlitePool,
}

#[derive(Debug, Serialize)]
struct HealthResp {
    ok: bool,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
struct User {
    id: i64,
    name: String,
}

#[derive(Debug, Deserialize)]
struct CreateUserReq {
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://data.db".to_string());

    let db = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id   INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        );
        "#,
    )
    .execute(&db)
    .await?;

    let state = AppState { db };

    let app = Router::new()
        .route("/health", get(health))
        .route("/users", get(list_users).post(create_user))
        .route("/sse", get(sse_users_count))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{addr}");
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;

    Ok(())
}

// 下面其他函数保持不变...
async fn health() -> impl IntoResponse {
    Json(HealthResp { ok: true })
}

async fn list_users(State(st): State<AppState>) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let users = sqlx::query_as::<_, User>("SELECT id, name FROM users ORDER BY id DESC")
        .fetch_all(&st.db)
        .await
        .map_err(internal_error)?;

    Ok(Json(users))
}

async fn create_user(
    State(st): State<AppState>,
    Json(req): Json<CreateUserReq>,
) -> Result<StatusCode, (StatusCode, String)> {
    if req.name.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "name is empty".to_string()));
    }

    sqlx::query("INSERT INTO users (name) VALUES (?)")
        .bind(req.name)
        .execute(&st.db)
        .await
        .map_err(internal_error)?;

    Ok(StatusCode::CREATED)
}

async fn sse_users_count(State(st): State<AppState>) -> Response {
    let interval = tokio::time::interval(Duration::from_secs(1));
    let stream = IntervalStream::new(interval).then(move |_| {
        let st = st.clone();
        async move {
            let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
                .fetch_one(&st.db)
                .await
                .unwrap_or((0,));

            let data = serde_json::json!({ "users_count": count.0 });

            Ok::<Event, Infallible>(Event::default().event("users_count").data(data.to_string()))
        }
    });

    let mut headers = HeaderMap::new();
    headers.insert("Cache-Control", HeaderValue::from_static("no-cache"));
    headers.insert("Connection", HeaderValue::from_static("keep-alive"));

    let sse = Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("ping"),
    );

    (headers, sse).into_response()
}

fn internal_error<E: std::fmt::Display>(e: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}