use std::path::Path;
use std::sync::{Arc, Mutex};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    http::{header::AUTHORIZATION, HeaderMap},
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use rusqlite::{params, Connection};
use rust_core::adapters::serialization::{
    validate_policy, validate_report_upload, DecisionDto, PolicyConditionsDto, PolicyDto,
    PolicyRuleDto, PolicyUpsertDto, PolicyUpsertResponse, ReportUploadDto,
};
use serde::Deserialize;
use tracing::info;

#[derive(Clone)]
struct AppState {
    db: Arc<Mutex<Connection>>,
    api_token: Option<String>,
}

#[derive(Deserialize)]
struct PolicyQuery {
    app_id: String,
    app_version: String,
    env: String,
    device_platform: String,
}

#[derive(Deserialize)]
struct PolicyListQuery {
    app_id: Option<String>,
    app_version: Option<String>,
    env: Option<String>,
    device_platform: Option<String>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db_path = std::env::var("POLICY_DB_PATH").unwrap_or_else(|_| "data/policy.db".to_string());
    let api_token = std::env::var("API_TOKEN").ok();
    let conn = init_db(&db_path).expect("failed to init policy db");
    let state = AppState {
        db: Arc::new(Mutex::new(conn)),
        api_token,
    };
    seed_default_policy(&state);

    let app = Router::new()
        .route("/v1/policies/current", get(get_policy))
        .route("/v1/policies", get(list_policies).post(upsert_policy))
        .route("/v1/policies/versions", get(list_policy_versions))
        .route("/v1/reports/upload", post(upload_report))
        .with_state(state);

    let addr = "0.0.0.0:8082";
    info!("policy-service listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_policy(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<PolicyQuery>,
) -> Result<Json<PolicyDto>, (StatusCode, String)> {
    require_auth(&headers, &state.api_token)?;
    let mut conn = state.db.lock().unwrap();

    if let Ok(Some(policy)) = fetch_policy(
        &mut conn,
        &query.app_id,
        &query.app_version,
        &query.env,
        &query.device_platform,
    ) {
        return Ok(Json(policy));
    }

    let policy = default_policy(&query.app_id, &query.app_version, &query.env);
    Ok(Json(policy))
}

async fn upsert_policy(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<PolicyUpsertDto>,
) -> Result<Json<PolicyUpsertResponse>, (StatusCode, String)> {
    require_auth(&headers, &state.api_token)?;
    validate_policy(&payload.policy)
        .map_err(|err| (StatusCode::BAD_REQUEST, err.message))?;

    let mut conn = state.db.lock().unwrap();
    let stored_at = store_policy(&mut conn, &payload.policy, &payload.device_platform)
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok(Json(PolicyUpsertResponse {
        status: "ok".to_string(),
        stored_at,
    }))
}

async fn list_policies(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<PolicyListQuery>,
) -> Result<Json<Vec<PolicyRecord>>, (StatusCode, String)> {
    require_auth(&headers, &state.api_token)?;

    let mut conn = state.db.lock().unwrap();
    let records = fetch_all_policies(&mut conn)
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    let filtered = records
        .into_iter()
        .filter(|record| match &query.app_id {
            Some(app_id) => &record.policy.app_id == app_id,
            None => true,
        })
        .filter(|record| match &query.app_version {
            Some(app_version) => &record.policy.app_version == app_version,
            None => true,
        })
        .filter(|record| match &query.env {
            Some(env) => &record.policy.env == env,
            None => true,
        })
        .filter(|record| match &query.device_platform {
            Some(platform) => &record.device_platform == platform,
            None => true,
        })
        .collect::<Vec<_>>();

    Ok(Json(filtered))
}

async fn list_policy_versions(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<PolicyListQuery>,
) -> Result<Json<Vec<PolicyVersionRecord>>, (StatusCode, String)> {
    require_auth(&headers, &state.api_token)?;

    let mut conn = state.db.lock().unwrap();
    let records = fetch_policy_versions(&mut conn)
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    let filtered = records
        .into_iter()
        .filter(|record| match &query.app_id {
            Some(app_id) => &record.policy.app_id == app_id,
            None => true,
        })
        .filter(|record| match &query.app_version {
            Some(app_version) => &record.policy.app_version == app_version,
            None => true,
        })
        .filter(|record| match &query.env {
            Some(env) => &record.policy.env == env,
            None => true,
        })
        .filter(|record| match &query.device_platform {
            Some(platform) => &record.device_platform == platform,
            None => true,
        })
        .collect::<Vec<_>>();

    Ok(Json(filtered))
}

async fn upload_report(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<ReportUploadDto>,
) -> Result<Json<StatusAccepted>, (StatusCode, String)> {
    require_auth(&headers, &state.api_token)?;
    validate_report_upload(&payload)
        .map_err(|err| (StatusCode::BAD_REQUEST, err.message))?;

    let payload_json = serde_json::to_string(&payload)
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    let received_at = Utc::now().to_rfc3339();

    let conn = state.db.lock().unwrap();
    conn.execute(
        "INSERT OR IGNORE INTO reports (report_id, payload, received_at) VALUES (?1, ?2, ?3)",
        params![payload.report_id, payload_json, received_at],
    )
    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok(Json(StatusAccepted {
        status: "accepted".to_string(),
    }))
}

fn seed_default_policy(state: &AppState) {
    let policy = default_policy("fintech.mobile", "1.0.0", "prod");
    let mut conn = state.db.lock().unwrap();
    let _ = store_policy(&mut conn, &policy, "ios");
}

fn default_policy(app_id: &str, app_version: &str, env: &str) -> PolicyDto {
    PolicyDto {
        policy_id: "policy_default".to_string(),
        app_id: app_id.to_string(),
        app_version: app_version.to_string(),
        env: env.to_string(),
        rules: vec![PolicyRuleDto {
            action: "login".to_string(),
            decision: DecisionDto::StepUp,
            conditions: Some(PolicyConditionsDto {
                attestation: None,
                debugger: Some(false),
                hooking: Some(false),
                proxy_detected: Some(false),
                app_version: None,
                risk_score_gte: None,
            }),
        }],
        signature: "stub".to_string(),
        issued_at: Utc::now().to_rfc3339(),
    }
}

fn init_db(path: &str) -> Result<Connection, rusqlite::Error> {
    if let Some(parent) = Path::new(path).parent() {
        std::fs::create_dir_all(parent).ok();
    }

    let conn = Connection::open(path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS policies (
            app_id TEXT NOT NULL,
            app_version TEXT NOT NULL,
            env TEXT NOT NULL,
            device_platform TEXT NOT NULL,
            payload TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            PRIMARY KEY (app_id, app_version, env, device_platform)
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS reports (
            report_id TEXT PRIMARY KEY,
            payload TEXT NOT NULL,
            received_at TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS policy_versions (
            policy_id TEXT NOT NULL,
            issued_at TEXT NOT NULL,
            app_id TEXT NOT NULL,
            app_version TEXT NOT NULL,
            env TEXT NOT NULL,
            device_platform TEXT NOT NULL,
            payload TEXT NOT NULL,
            stored_at TEXT NOT NULL,
            PRIMARY KEY (policy_id, issued_at, device_platform)
        )",
        [],
    )?;

    Ok(conn)
}

#[derive(serde::Serialize)]
struct StatusAccepted {
    status: String,
}

#[derive(serde::Serialize)]
struct PolicyRecord {
    device_platform: String,
    policy: PolicyDto,
}

#[derive(serde::Serialize)]
struct PolicyVersionRecord {
    device_platform: String,
    policy: PolicyDto,
    stored_at: String,
}

fn store_policy(
    conn: &mut Connection,
    policy: &PolicyDto,
    device_platform: &str,
) -> Result<String, rusqlite::Error> {
    let payload = serde_json::to_string(policy).unwrap_or_else(|_| "{}".to_string());
    let updated_at = Utc::now().to_rfc3339();

    conn.execute(
        "INSERT OR REPLACE INTO policies (app_id, app_version, env, device_platform, payload, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            policy.app_id,
            policy.app_version,
            policy.env,
            device_platform,
            payload,
            updated_at
        ],
    )?;

    store_policy_version(conn, policy, device_platform, &updated_at)?;

    Ok(updated_at)
}

fn fetch_policy(
    conn: &mut Connection,
    app_id: &str,
    app_version: &str,
    env: &str,
    device_platform: &str,
) -> Result<Option<PolicyDto>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT payload FROM policies
         WHERE app_id = ?1 AND app_version = ?2 AND env = ?3 AND device_platform = ?4",
    )?;

    let mut rows = stmt.query(params![app_id, app_version, env, device_platform])?;
    if let Some(row) = rows.next()? {
        let payload: String = row.get(0)?;
        let policy = serde_json::from_str::<PolicyDto>(&payload).unwrap_or_else(|_| {
            default_policy(app_id, app_version, env)
        });
        return Ok(Some(policy));
    }

    Ok(None)
}

fn fetch_all_policies(conn: &mut Connection) -> Result<Vec<PolicyRecord>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT device_platform, payload FROM policies")?;
    let mut rows = stmt.query([])?;
    let mut records = Vec::new();

    while let Some(row) = rows.next()? {
        let device_platform: String = row.get(0)?;
        let payload: String = row.get(1)?;
        if let Ok(policy) = serde_json::from_str::<PolicyDto>(&payload) {
            records.push(PolicyRecord {
                device_platform,
                policy,
            });
        }
    }

    Ok(records)
}

fn store_policy_version(
    conn: &mut Connection,
    policy: &PolicyDto,
    device_platform: &str,
    stored_at: &str,
) -> Result<(), rusqlite::Error> {
    let payload = serde_json::to_string(policy).unwrap_or_else(|_| "{}".to_string());

    conn.execute(
        "INSERT OR REPLACE INTO policy_versions (policy_id, issued_at, app_id, app_version, env, device_platform, payload, stored_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            policy.policy_id,
            policy.issued_at,
            policy.app_id,
            policy.app_version,
            policy.env,
            device_platform,
            payload,
            stored_at
        ],
    )?;

    Ok(())
}

fn fetch_policy_versions(conn: &mut Connection) -> Result<Vec<PolicyVersionRecord>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT device_platform, payload, stored_at FROM policy_versions ORDER BY stored_at DESC",
    )?;
    let mut rows = stmt.query([])?;
    let mut records = Vec::new();

    while let Some(row) = rows.next()? {
        let device_platform: String = row.get(0)?;
        let payload: String = row.get(1)?;
        let stored_at: String = row.get(2)?;
        if let Ok(policy) = serde_json::from_str::<PolicyDto>(&payload) {
            records.push(PolicyVersionRecord {
                device_platform,
                policy,
                stored_at,
            });
        }
    }

    Ok(records)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_policy_creates_version_and_current() {
        let mut conn = init_db(":memory:").expect("db init");
        let policy = default_policy("app.test", "1.0.0", "prod");

        store_policy(&mut conn, &policy, "ios").expect("store policy");

        let current = fetch_policy(&mut conn, "app.test", "1.0.0", "prod", "ios")
            .expect("fetch policy")
            .expect("policy exists");
        assert_eq!(current.app_id, "app.test");

        let versions = fetch_policy_versions(&mut conn).expect("fetch versions");
        assert_eq!(versions.len(), 1);
        assert_eq!(versions[0].device_platform, "ios");
    }
}
