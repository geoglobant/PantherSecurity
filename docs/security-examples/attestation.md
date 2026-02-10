# Attestation (iOS App Attest)

## Risk summary
Attestation proves the app instance is genuine and running on a trusted device.

## Swift example (high-level flow)
```swift
import DeviceCheck

final class AppAttestService {
    private let service = DCAppAttestService.shared

    func generateKey() async throws -> String {
        try await service.generateKey()
    }

    func attest(keyId: String, challenge: Data) async throws -> Data {
        try await service.attestKey(keyId, clientDataHash: challenge)
    }

    func assertion(keyId: String, challenge: Data) async throws -> Data {
        try await service.generateAssertion(keyId, clientDataHash: challenge)
    }
}
```

## Recommended Apple frameworks
- `DeviceCheck` (App Attest)

## Client-side test ideas
- Use a test backend endpoint that returns a challenge.
- Verify that failing attestation forces STEP_UP or DENY.

## Backend recommendations
- Verify the attestation object with Apple App Attest verification.
- Bind the attestation to a user/session and rotate challenges.
- Reject sensitive actions when attestation is missing or invalid.
