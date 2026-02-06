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
- Tuist
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

### iOS SDK + Sample
1. Build the Rust core xcframework:
   ```bash
   scripts/install-ios-xcframework.sh
   ```
2. Generate the Xcode project for the sample:
   ```bash
   cd mobile/ios/sampleIOS
   tuist generate
   ```
3. Open `mobile/ios/sampleIOS/MobileAppSecSample.xcworkspace` in Xcode.

### How the iOS build flow works
1. **Generate the Rust framework**  
   Run from repo root:
   ```bash
   ./scripts/install-ios-xcframework.sh
   ```
   This script builds the Rust core for device + simulator and copies
   `PantherSecurityCore.xcframework` into `mobile/ios/sampleIOS/Frameworks`.

2. **Generate the Xcode project with Tuist**  
   Tuist reads `mobile/ios/sampleIOS/Project.swift` and wires the local Swift package plus the xcframework:
   ```bash
   cd mobile/ios/sampleIOS
   tuist generate
   ```

3. **Install in Xcode**  
   Open `mobile/ios/sampleIOS/MobileAppSecSample.xcworkspace`.
   In **Target > General > Frameworks, Libraries, and Embedded Content**, confirm
   `PantherSecurityCore.xcframework` is listed (Embed & Sign).

4. **If you get linker errors (_ps_*)**  
   Re-run the script and regenerate the project:
   ```bash
   ./scripts/install-ios-xcframework.sh
   cd mobile/ios/sampleIOS
   tuist generate
   ```

### Android Sample
Open `mobile/android` in Android Studio and run the app.

## Local endpoints
Default local ports:
- Telemetry ingestion: `http://localhost:8081`
- Policy service: `http://localhost:8082`

## Troubleshooting
- **No space left on device**: remove build artifacts (Rust targets, SwiftPM caches) and retry.
- **Undefined symbol: _ps_***: rebuild the xcframework and regenerate the Xcode project.
- **Rust build fails on first run**: ensure network access for crate downloads.

## Notes
- The Rust core needs network access on the first build to download crates.
- If you see `Undefined symbol: _ps_*`, rebuild the xcframework and reopen Xcode.
