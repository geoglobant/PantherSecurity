#!/usr/bin/env bash
set -euo pipefail

export POLICY_DB_PATH="${POLICY_DB_PATH:-data/policy.db}"

cargo run --manifest-path backend/policy-service/Cargo.toml
