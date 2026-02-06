#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

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
