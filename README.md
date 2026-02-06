# Mobile AppSec Platform

**Tagline:** Continuous mobile security from commit to runtime.

Mobile security platform for fintechs that orchestrates signals and policies from commit to app runtime. It does not replace existing scanners; it coordinates, correlates, and enforces continuous security policies.

## Objectives
- Protect apps at runtime via a mobile SDK (iOS + Android).
- Test backend and CI/CD via a language-agnostic Agent/CLI.
- Correlate CI/CD findings with runtime telemetry via the Policy Backend.
- Apply dynamic policies (ALLOW, STEP_UP, DEGRADE, DENY).

## Components
- **Mobile SDK**
  - SDK name: **PantherSecurity**
  - Rust core with Swift and Kotlin wrappers.
  - Pinning (SPKI with rotation), secure storage, integrity signals, attestation.
  - Local policy engine and enforcement for sensitive actions.
- **Security Agent / CLI (Rust)**
  - Scans backend and CI/CD, integrates SAST/SCA/secret scanning.
  - Static checks for mobile builds.
  - Exports JSON and SARIF and uploads reports to the backend.
- **Policy + Telemetry Backend**
  - Ingests events and reports.
  - Correlates, computes risk scores, and distributes policies.

## Monorepo structure
```
core/
  rust-core/
mobile/
  ios/
  android/
agent/
  cli/
  plugins/
backend/
  policy-service/
  telemetry-ingestion/
docs/
```

## Docs
- `docs/architecture.md`
- `docs/roadmap.md`
- `docs/mvp.md`

## Status
Initial monorepo skeleton. Components are still stubbed.

## API Contracts (draft)
- `docs/api-contracts.md`

## OpenAPI
- `docs/openapi.yaml`

## Scripts
- `scripts/run-telemetry-ingestion.sh`
- `scripts/run-policy-service.sh`
- `scripts/run-backend.sh`

## Prerequisites
- Rust + rustup
- iOS targets:
  - `aarch64-apple-ios`
  - `aarch64-apple-ios-sim`
- Xcode + Command Line Tools
- Tuist (project generator for Xcode). Install: `brew install tuist`
- Android Studio (with a recent JDK/Gradle)

Quick checks:
```bash
rustc --version
rustup target list --installed
tuist --version
```

## Quickstart
### Backend (local)
```bash
scripts/run-backend.sh
```
Run a single service:
```bash
scripts/run-backend.sh telemetry
scripts/run-backend.sh policy
```

### iOS SDK + Sample
1. From the repo root, build the Rust core xcframework:
   ```bash
   ./scripts/install-ios-xcframework.sh
   ```
   This creates `mobile/ios/sampleIOS/Frameworks/PantherSecurityCore.xcframework`.

2. Generate the Xcode project for the sample:
   ```bash
   cd mobile/ios/sampleIOS
   tuist generate
   ```
3. Open `mobile/ios/sampleIOS/MobileAppSecSample.xcworkspace` in Xcode.
4. Select the `MobileAppSecSample` target and run on an iOS simulator.
5. If you see linker errors (`_ps_*`), rebuild the xcframework and re-run `tuist generate`.

### Android Sample
#### Run in Android Studio (step-by-step)
1. Open Android Studio.
2. Select **Open** and choose `mobile/android`.
3. Wait for Gradle sync to finish.
4. Select an emulator or a connected device.
5. Run the `app` configuration.

If Gradle sync fails, make sure you have a recent JDK and Android SDK installed.

## Local endpoints
Default local ports:
- Telemetry ingestion: `http://localhost:8081`
- Policy service: `http://localhost:8082`

Root status pages:
- `http://localhost:8081/`
- `http://localhost:8082/`

## How to view the backend
There is no UI yet. You can verify it via logs and HTTP requests:

Check policy service:
```bash
curl "http://localhost:8082/v1/policies/current?app_id=fintech.mobile&app_version=1.0.0&env=prod&device_platform=ios"
```

Send telemetry:
```bash
curl -X POST "http://localhost:8081/v1/telemetry/events" \
  -H "Content-Type: application/json" \
  -d '{"event_id":"evt_local","app_id":"fintech.mobile","app_version":"1.0.0","env":"local","device":{"platform":"ios","os_version":"17.0","model":"iPhone"},"signals":{"jailbreak":false,"root":false,"debugger":false,"hooking":false,"proxy_detected":false},"action":{"name":"login","context":null},"timestamp":"2026-02-06T21:00:00Z","signature":"stub"}'
```

Quick smoke test (policy + telemetry):
```bash
curl "http://localhost:8082/v1/policies/current?app_id=fintech.mobile&app_version=1.0.0&env=prod&device_platform=ios" \
  && curl -X POST "http://localhost:8081/v1/telemetry/events" \
    -H "Content-Type: application/json" \
    -d '{"event_id":"evt_smoke","app_id":"fintech.mobile","app_version":"1.0.0","env":"local","device":{"platform":"ios","os_version":"17.0","model":"iPhone"},"signals":{"jailbreak":false,"root":false,"debugger":false,"hooking":false,"proxy_detected":false},"action":{"name":"login","context":null},"timestamp":"2026-02-06T21:00:00Z","signature":"stub"}'
```

More validation examples:
```bash
# Status pages
curl -i http://localhost:8081/
curl -i http://localhost:8082/

# List policies
curl -i "http://localhost:8082/v1/policies?app_id=fintech.mobile"

# Policy versions
curl -i "http://localhost:8082/v1/policies/versions?app_id=fintech.mobile"

# Create/update policy
curl -i -X POST "http://localhost:8082/v1/policies" \
  -H "Content-Type: application/json" \
  -d '{"device_platform":"ios","policy":{"policy_id":"pol_001","app_id":"fintech.mobile","app_version":"1.0.0","env":"prod","rules":[{"action":"login","decision":"STEP_UP","conditions":{"debugger":false}}],"signature":"stub","issued_at":"2026-02-06T21:00:00Z"}}'

# Upload report
curl -i -X POST "http://localhost:8082/v1/reports/upload" \
  -H "Content-Type: application/json" \
  -d '{"report_id":"rep_001","app_id":"fintech.mobile","env":"staging","source":"ci","pipeline":{"provider":"github_actions","run_id":"123"},"artifacts":{"format":"sarif","payload":"base64..."},"timestamp":"2026-02-06T21:00:00Z"}'
```

If `API_TOKEN` is set, include:
```bash
-H "Authorization: Bearer <token>"
```

DB files are stored under `data/`:
- `data/telemetry.db`
- `data/policy.db`

## Troubleshooting
- **No space left on device**: remove build artifacts (Rust targets, SwiftPM caches) and retry.
- **Undefined symbol: _ps_***: rebuild the xcframework and regenerate the Xcode project.
- **Rust build fails on first run**: ensure network access for crate downloads.
- **Address already in use (8081/8082)**: stop existing processes (Ctrl+C) or kill them:
  ```bash
  lsof -nP -iTCP:8081 -sTCP:LISTEN
  lsof -nP -iTCP:8082 -sTCP:LISTEN
  kill <PID>
  ```

## Notes
- The Rust core needs network access on the first build to download crates.
- If you see `Undefined symbol: _ps_*`, rebuild the xcframework and reopen Xcode.
