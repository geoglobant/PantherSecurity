#!/usr/bin/env bash
set -euo pipefail

scripts/run-telemetry-ingestion.sh &
TELEMETRY_PID=$!

scripts/run-policy-service.sh &
POLICY_PID=$!

trap 'kill ${TELEMETRY_PID} ${POLICY_PID}' INT TERM
wait ${TELEMETRY_PID} ${POLICY_PID}
