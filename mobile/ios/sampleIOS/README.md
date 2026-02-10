# iOS Sample (Fintech Demo)

This sample simulates a real fintech flow and shows how PantherSecurity reacts to sensitive actions.

## Demo flow
1. Sign in (login security check).
2. Home screen with balance and quick actions.
3. Actions: transfer, add beneficiary, view card, change password.
4. The SDK evaluates each action and returns ALLOW / STEP_UP / DEGRADE / DENY.

## Demo Mode (Security Lab)
Open **Settings → Security Lab** to simulate:
- jailbreak/root/debugger/hooking/proxy
- attestation status
- risk score

This makes the app show real outcomes (block, step-up, degrade) during the flow.

## Security options explained (7 total)
The demo validates **7 security inputs** that affect policy decisions:
- **Jailbreak**: iOS device integrity is compromised; policies may force `STEP_UP` or `DENY`.
- **Root**: Android-style root signal; included for cross-platform parity. Treated as device compromise.
- **Debugger**: Detects debugging or instrumentation. Often triggers `STEP_UP` or `DENY` on sensitive actions.
- **Hooking**: Simulates runtime method hooking (e.g., Frida). Commonly degrades or blocks actions.
- **Proxy/MITM**: Indicates traffic interception. Typically blocks transfers or sensitive actions.
- **Attestation status**: Simulates attestation result (`pass`, `fail`, `unknown`, or unset). A `fail` usually forces `STEP_UP`/`DENY`.
- **Risk score**: Server-calculated risk (0–100). Higher values can downgrade or block actions.

## Security example docs
Open the example guides in:
- `docs/security-examples/mitm-proxy-pinning.md`
- `docs/security-examples/debugger-hooking.md`
- `docs/security-examples/device-integrity-jailbreak-root.md`
- `docs/security-examples/attestation.md`
- `docs/security-examples/bola-idor.md`
- `docs/security-examples/risk-score.md`

## Secure storage (API keys & user data)
This sample now saves sensitive values using **Keychain**:
- API token (simulated) saved after login
- User email saved on login

Where it lives in code:
- `mobile/ios/sampleIOS/Sources/SampleApp/SecureStorage.swift`
- `DemoState.login(...)` and `DemoState.logout()` in `SampleApp.swift`

## How to open in Xcode

### Option A (recommended): generate the Xcode Project via Tuist
1. Generate the Xcode project:
   ```bash
   cd mobile/ios/sampleIOS
   tuist generate
   ```
2. Open `mobile/ios/sampleIOS/MobileAppSecSample.xcodeproj`.

### Option B: open Package.swift directly
1. Open `mobile/ios/sampleIOS/Package.swift` in Xcode.

## FFI: link the Rust core
1. Build the Rust core for iOS device and simulator:
   ```bash
   scripts/install-ios-xcframework.sh
   ```
2. The xcframework will be copied to `mobile/ios/sampleIOS/Frameworks/PantherSecurityCore.xcframework`.
3. In Xcode, add the xcframework in **Frameworks, Libraries, and Embedded Content**.

## Tips
- After running `scripts/install-ios-xcframework.sh`, run `tuist generate` again.
- If you see `Undefined symbol: _ps_*`, rebuild the xcframework after updating the Rust core.
