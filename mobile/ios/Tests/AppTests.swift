import XCTest
import PantherSecurity
import AppDomain
import AppData

final class AppTests: XCTestCase {
    func testPolicyMapping() async throws {
        let sdk = PantherSecuritySDK(client: MockPantherSecurityClient())
        let repo = PolicyRepositoryImpl(sdk: sdk)
        sdk.configure(
            PantherSecurityConfiguration(
                baseURL: URL(string: "http://localhost:8082")!,
                appId: "fintech.mobile",
                appVersion: "1.0.0",
                env: "local",
                apiToken: nil,
                devicePlatform: "ios",
                pinning: nil
            )
        )

        let policy = try await repo.fetchPolicy()
        XCTAssertEqual(policy.rules.count, 1)
        XCTAssertEqual(policy.rules.first?.decision, .stepUp)
    }
}

private final class MockPantherSecurityClient: PantherSecurityClient {
    func fetchPolicy(config: PantherSecurityConfiguration) async throws -> PantherSecurityPolicyResponse {
        PantherSecurityPolicyResponse(
            policyId: "policy_test",
            appId: config.appId,
            appVersion: config.appVersion,
            env: config.env,
            rules: [PantherSecurityPolicyRule(action: "login", decision: "STEP_UP", conditions: nil)],
            signature: "stub",
            issuedAt: "2026-02-06T00:00:00Z"
        )
    }

    func sendTelemetry(_ event: PantherSecurityTelemetryRequest, config: PantherSecurityConfiguration) async throws {
        _ = event
    }
}
