#!/usr/bin/env bash
set -euo pipefail

export TELEMETRY_DB_PATH="${TELEMETRY_DB_PATH:-data/telemetry.db}"

cargo run --manifest-path backend/telemetry-ingestion/Cargo.toml
