use std::path::Path;
use std::sync::{Arc, Mutex};

use axum::{
    extract::State,
    http::StatusCode,
    http::{header::AUTHORIZATION, HeaderMap},
    routing::post,
    Json, Router,
};
use chrono::Utc;
use rusqlite::{params, Connection};
use rust_core::adapters::serialization::{
    validate_telemetry_event, TelemetryEventDto,
};
use tracing::info;

#[derive(Clone)]
struct AppState {
    db: Arc<Mutex<Connection>>,
    api_token: Option<String>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let db_path = std::env::var("TELEMETRY_DB_PATH").unwrap_or_else(|_| "data/telemetry.db".to_string());
    let api_token = std::env::var("API_TOKEN").ok();
    let conn = init_db(&db_path).expect("failed to init telemetry db");
    let state = AppState {
        db: Arc::new(Mutex::new(conn)),
        api_token,
    };

    let app = Router::new()
        .route("/v1/telemetry/events", post(ingest_event))
        .with_state(state);

    let addr = "0.0.0.0:8081";
    info!("telemetry-ingestion listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ingest_event(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<TelemetryEventDto>,
) -> Result<Json<StatusOk>, (StatusCode, String)> {
    require_auth(&headers, &state.api_token)?;
    validate_telemetry_event(&payload)
        .map_err(|err| (StatusCode::BAD_REQUEST, err.message))?;

    let payload_json = serde_json::to_string(&payload)
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    let received_at = Utc::now().to_rfc3339();

    let conn = state
        .db
        .lock()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "storage lock".to_string()))?;

    conn.execute(
        "INSERT OR IGNORE INTO events (event_id, payload, received_at) VALUES (?1, ?2, ?3)",
        params![payload.event_id, payload_json, received_at],
    )
    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok(Json(StatusOk { status: "ok".to_string() }))
}

#[derive(serde::Serialize)]
struct StatusOk {
    status: String,
}

fn init_db(path: &str) -> Result<Connection, rusqlite::Error> {
    if let Some(parent) = Path::new(path).parent() {
        std::fs::create_dir_all(parent).ok();
    }

    let conn = Connection::open(path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS events (
            event_id TEXT PRIMARY KEY,
            payload TEXT NOT NULL,
            received_at TEXT NOT NULL
        )",
        [],
    )?;

    Ok(conn)
}

fn require_auth(
    headers: &HeaderMap,
    token: &Option<String>,
) -> Result<(), (StatusCode, String)> {
    let expected = match token {
        Some(value) => value,
        None => return Ok(()),
    };

    let auth = headers
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");

    let expected_header = format!("Bearer {}", expected);
    if auth != expected_header {
        return Err((StatusCode::UNAUTHORIZED, "unauthorized".to_string()));
    }

    Ok(())
}
