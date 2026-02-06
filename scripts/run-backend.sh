#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

SERVICE="${1:-all}"

# Stop any existing instances on the default ports.
stop_port() {
  local port="$1"
  local pids
  pids="$(lsof -t -iTCP:"${port}" -sTCP:LISTEN 2>/dev/null || true)"
  if [ -n "$pids" ]; then
    echo "Stopping process(es) on port ${port}: ${pids}"
    kill ${pids} || true
  fi
}

case "$SERVICE" in
  telemetry)
    stop_port 8081
    cargo build --manifest-path backend/telemetry-ingestion/Cargo.toml
    export TELEMETRY_DB_PATH="${TELEMETRY_DB_PATH:-data/telemetry.db}"
    "$ROOT_DIR/backend/telemetry-ingestion/target/debug/telemetry-ingestion"
    ;;
  policy)
    stop_port 8082
    cargo build --manifest-path backend/policy-service/Cargo.toml
    export POLICY_DB_PATH="${POLICY_DB_PATH:-data/policy.db}"
    "$ROOT_DIR/backend/policy-service/target/debug/policy-service"
    ;;
  all)
    stop_port 8081
    stop_port 8082
    # Build sequentially to avoid cargo package cache lock contention.
    cargo build --manifest-path backend/telemetry-ingestion/Cargo.toml
    cargo build --manifest-path backend/policy-service/Cargo.toml

    export TELEMETRY_DB_PATH="${TELEMETRY_DB_PATH:-data/telemetry.db}"
    export POLICY_DB_PATH="${POLICY_DB_PATH:-data/policy.db}"

    "$ROOT_DIR/backend/telemetry-ingestion/target/debug/telemetry-ingestion" &
    TELEMETRY_PID=$!

    "$ROOT_DIR/backend/policy-service/target/debug/policy-service" &
    POLICY_PID=$!

    trap 'kill ${TELEMETRY_PID} ${POLICY_PID}' INT TERM
    wait ${TELEMETRY_PID} ${POLICY_PID}
    ;;
  *)
    echo "Usage: $0 [all|telemetry|policy]"
    exit 1
    ;;
esac
