# Android Sample (Fintech Demo)

This sample simulates a real fintech app flow and shows how PantherSecurity reacts to sensitive actions.

## Demo flow
1. Sign in (login security check).
2. Home screen with balance and quick actions.
3. Actions: transfer, add beneficiary, view card, change password.
4. The SDK evaluates each action and returns ALLOW / STEP_UP / DEGRADE / DENY.

## Demo Mode (Security Lab)
Use **Demo Mode (Security Lab)** to simulate:
- jailbreak/root/debugger/hooking/proxy
- attestation status
- risk score

This makes the app show real outcomes (block, step-up, degrade) during the flow.

## How to open
1. Open `mobile/android` in Android Studio.
2. Wait for Gradle sync.
3. Run the `app` configuration.

## Note
Android SDK is still a stub and must be connected to Rust core/FFI later.
