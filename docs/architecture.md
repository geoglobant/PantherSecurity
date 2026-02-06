# High-Level Architecture

## Overview
The platform combines three pillars:
1. **Mobile SDK** (iOS + Android) for runtime protection.
2. **Security Agent / CLI** for CI/CD and backend checks.
3. **Policy + Telemetry Backend** for correlation, risk scoring, and policy distribution.

The initial focus is fintechs, prioritizing sensitive actions (login, refresh token, transfers, Pix, beneficiaries, card data, and credential changes).

## Components

### 1) Mobile SDK
**Rust core** with wrappers:
- Swift (iOS)
- Kotlin (Android)

Responsibilities:
- TLS pinning (SPKI with rotation)
- Secure storage (Keychain / Keystore)
- Integrity signals (jailbreak/root, debugger/hooking, proxy/MITM)
- Attestation (App Attest and Play Integrity)
- Local policy engine (in Rust)
- Action enforcement (ALLOW, STEP_UP, DEGRADE, DENY)
- Signed telemetry

### 2) Security Agent / CLI (Rust)
Runs in CI/CD or servers.

Capabilities:
- Backend scanning (TLS/HSTS/headers, CORS, rate limiting, authZ invariants, refresh token replay)
- OpenAPI/GraphQL checks
- SAST and SCA/SBOM orchestration
- Secret scanning
- Static mobile build checks (entitlements, ATS, manifest, cleartext, debuggable)

Outputs:
- JSON
- SARIF
- Upload to Policy Backend

### 3) Policy + Telemetry Backend
Main services:
- **Telemetry Ingestion**: receives SDK events and Agent reports.
- **Policy Service**: computes risk score, correlates signals, distributes policies.

Policies are segmented by:
- app
- version
- environment

Examples:
- Block transfers on a vulnerable version
- Require STEP_UP for login
- Read-only mode for incident response

## Main flows

### 1) Runtime
1. SDK collects local signals and executes attestation.
2. SDK sends signed telemetry to ingestion.
3. Policy Service correlates and computes risk score.
4. Policies return to the app and are evaluated locally.
5. Policy engine decides ALLOW/STEP_UP/DEGRADE/DENY.

### 2) CI/CD and Backend
1. Agent/CLI runs tests and scans in the pipeline.
2. Reports generated (JSON/SARIF).
3. Upload to Policy Backend.
4. Correlation with runtime for dynamic policy reinforcement.

## Interfaces and contracts
- **Rust Core <-> Mobile**: C ABI for Swift/Kotlin wrappers.
- **SDK <-> Backend**: telemetry and policy distribution API.
- **Agent <-> Backend**: report upload API.
- **Plugin system**: check extensions in Agent/CLI.

## Principles
- Clean Architecture
- Monorepo
- Rust core with hexagonal architecture (ports/adapters)
- Clear separation between collection, correlation, and enforcement
- Signed and versioned policies
