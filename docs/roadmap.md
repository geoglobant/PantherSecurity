# Roadmap

This roadmap focuses on delivering a functional MVP with runtime signals, basic correlation, and policy enforcement.
This document is updated on each relevant delivery.

## Phase 0 - Foundation (2 to 4 weeks)
- Monorepo structure and architecture patterns.
- Initial contracts between SDK, Agent, and Backend.
- Initial OpenAPI for telemetry, policy, and report upload.
- Rust core skeleton (C ABI, telemetry models, base policy engine).
- Agent/CLI skeleton with plugin pipeline.
- Backend skeleton (telemetry ingestion + policy service).

## Phase 1 - MVP (4 to 6 weeks)
- SDK: initialization/config, policy fetch, basic risk signals, pinning.
- SDK: decision engine (ALLOW / STEP_UP / DENY).
- Backend: event ingestion endpoint and policy distribution.
- Backend: simple storage.
- Agent CLI: `agent scan perimeter`, `agent scan rate-limit`, `agent scan authz`, `agent scan mobile-build`, `agent report`.

## Phase 2 - MVP CI/CD + Correlation (4 to 6 weeks)
- Agent/CLI with backend checks (TLS/HSTS/headers, CORS, rate limiting).
- Static mobile build checks (ATS/entitlements/manifest).
- Export JSON and SARIF.
- Basic correlation between Agent findings and runtime signals.

## Phase 3 - Hardening (6 to 8 weeks)
- Full attestation (App Attest / Play Integrity).
- More expressive policy engine (conditional rules).
- Server-side risk score with per-signal weights.
- Kill switch and read-only mode for incident response.
- Observability and audit trails.

## Out of scope (for now)
- Advanced fraud detection and ML.
- Automated remediation in the pipeline.
- Public plugin marketplace.

## Success metrics (MVP)
- 90% of sensitive actions covered by enforcement.
- Policy propagation time < 2 minutes.
- Runtime alerts correlated with CI/CD findings.

## Recent updates
- 2026-02-06: HIGHLIGHT - Project documentation and READMEs standardized to English.
- 2026-02-06: HIGHLIGHT - Script to copy xcframework into the iOS sample at `scripts/install-ios-xcframework.sh`.
- 2026-02-06: HIGHLIGHT - Script to build iOS xcframework from Rust core at `scripts/build-ios-xcframework.sh`.
- 2026-02-06: HIGHLIGHT - SDK renamed to PantherSecurity (Swift package and samples updated).
- 2026-02-06: HIGHLIGHT - Swift wrapper via FFI for Rust core (policy + pinning).
- 2026-02-06: HIGHLIGHT - Android project started with equivalent clean architecture.
- 2026-02-06: HIGHLIGHT - iOS sample with Xcode project generated (Tuist) at `mobile/ios/sampleIOS`.
- 2026-02-06: HIGHLIGHT - Initial iOS project with Clean Architecture and SDK integrated for testing.
- 2026-02-06: HIGHLIGHT - SPKI pinning logic with rotation added to core SDK.
- 2026-02-06: HIGHLIGHT - Core SDK with init/config, policy fetch, and baseline signals in `core/rust-core/src/sdk.rs`.
- 2026-02-06: HIGHLIGHT - Core HTTP adapter supports `POST /v1/policies` (admin upsert).
- 2026-02-06: HIGHLIGHT - Policy upsert now returns `stored_at` in the backend.
- 2026-02-06: HIGHLIGHT - Core HTTP adapter for telemetry and policy fetch.
- 2026-02-06: HIGHLIGHT - Backend now stores policy history and exposes `/v1/policies/versions`.
- 2026-02-06: HIGHLIGHT - Basic tests added to policy-service.
- 2026-02-06: HIGHLIGHT - Policy listing via `GET /v1/policies`.
- 2026-02-06: HIGHLIGHT - Core SDK now supports auth via `TelemetryAuth`/`TelemetryEnvelope`.
- 2026-02-06: HIGHLIGHT - Admin policy endpoint added at `/v1/policies`.
- 2026-02-06: HIGHLIGHT - Basic auth via `Authorization: Bearer` in backend services.
- 2026-02-06: HIGHLIGHT - Agent CLI now accepts `--token` for `agent report`.
- 2026-02-06: HIGHLIGHT - Agent CLI now sends `agent report` to `/v1/reports/upload`.
- 2026-02-06: HIGHLIGHT - Simple storage with SQLite for events, policies, and reports.
- 2026-02-06: HIGHLIGHT - Local scripts to run backend at `scripts/run-backend.sh`.
- 2026-02-06: Monorepo skeleton created.
- 2026-02-06: Initial documents (architecture, contracts, roadmap) published.
- 2026-02-06: OpenAPI draft available at `docs/openapi.yaml`.
- 2026-02-06: Rust core skeleton (hexagonal) created at `core/rust-core`.
- 2026-02-06: C ABI/FFI stubs added to core at `core/rust-core/src/adapters/ffi.rs`.
- 2026-02-06: Serialization models (DTOs) aligned to OpenAPI added at `core/rust-core/src/adapters/serialization.rs`.
- 2026-02-06: FFI expanded to evaluate PolicySet and batch rules in core.
- 2026-02-06: Strict serialization with validations (deny unknown fields + validators) in core.
- 2026-02-06: Agent/CLI skeleton with plugin pipeline in `agent/cli`.
- 2026-02-06: MVP document published at `docs/mvp.md`.
- 2026-02-06: Agent CLI basic commands defined (`scan` and `report`).
- 2026-02-06: Backend stubs created for event ingestion and policy distribution.
