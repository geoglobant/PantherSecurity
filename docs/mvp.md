# Expected MVP (Phase 1)

This document describes the expected MVP for the first product phase.

## SDK
- [x] Initialization and configuration (core)
- [x] Policy fetch (core)
- [x] Basic risk signals (baseline)
- [x] Pinning (SPKI logic with rotation in core)
- [x] Decision engine (ALLOW / STEP_UP / DENY)
- [x] Initial iOS project with Clean Architecture and integrated SDK (HTTP)
- [x] PantherSecurity SDK with Swift wrapper via FFI (policy + pinning)

## Android
- [x] Initial Android project with equivalent Clean Architecture (stub)

## Agent CLI
Expected commands:
- [x] `agent scan perimeter` (stub)
- [x] `agent scan rate-limit` (stub)
- [x] `agent scan authz` (stub)
- [x] `agent scan mobile-build` (stub)
- [x] `agent report` (upload to backend)

## Backend
- [x] Event ingestion endpoint
- [x] Policy distribution endpoint
- [x] Simple storage (SQLite)
