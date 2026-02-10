import XCTest
import PantherSecurity

final class SecurityPolicyTests: XCTestCase {
    private func makePolicy(rules: [PantherSecurityPolicyRule]) -> PantherSecurityPolicyResponse {
        PantherSecurityPolicyResponse(
            policyId: "policy_test",
            appId: "fintech.mobile",
            appVersion: "1.0.0",
            env: "test",
            rules: rules,
            signature: "stub",
            issuedAt: "2026-02-10T00:00:00Z"
        )
    }

    private func signals(
        jailbreak: Bool = false,
        root: Bool = false,
        debugger: Bool = false,
        hooking: Bool = false,
        proxyDetected: Bool = false
    ) -> PantherSecurityIntegritySignals {
        PantherSecurityIntegritySignals(
            jailbreak: jailbreak,
            root: root,
            debugger: debugger,
            hooking: hooking,
            proxyDetected: proxyDetected
        )
    }

    private func decision(
        action: String,
        policy: PantherSecurityPolicyResponse,
        signals: PantherSecurityIntegritySignals,
        attestation: String? = nil,
        riskScore: UInt32 = 0
    ) -> PantherSecurityDecision {
        PantherSecuritySDK.shared.evaluateDecision(
            policy: policy,
            action: PantherSecurityActionContext(name: action, context: nil),
            signals: signals,
            attestationStatus: attestation,
            riskScore: riskScore
        )
    }

    func testDebuggerTriggersStepUpForLogin() {
        let rule = PantherSecurityPolicyRule(
            action: "login",
            decision: "STEP_UP",
            conditions: PantherSecurityPolicyConditions(
                attestation: nil,
                debugger: true,
                hooking: nil,
                proxyDetected: nil,
                appVersion: nil,
                riskScoreGte: nil
            )
        )

        let policy = makePolicy(rules: [rule])
        let decision = decision(
            action: "login",
            policy: policy,
            signals: signals(debugger: true)
        )

        XCTAssertEqual(decision, .stepUp)
    }

    func testProxyBlocksTransfer() {
        let rule = PantherSecurityPolicyRule(
            action: "transfer",
            decision: "DENY",
            conditions: PantherSecurityPolicyConditions(
                attestation: nil,
                debugger: nil,
                hooking: nil,
                proxyDetected: true,
                appVersion: nil,
                riskScoreGte: nil
            )
        )

        let policy = makePolicy(rules: [rule])
        let decision = decision(
            action: "transfer",
            policy: policy,
            signals: signals(proxyDetected: true)
        )

        XCTAssertEqual(decision, .deny)
    }

    func testAttestationFailRequiresStepUp() {
        let rule = PantherSecurityPolicyRule(
            action: "add_beneficiary",
            decision: "STEP_UP",
            conditions: PantherSecurityPolicyConditions(
                attestation: "fail",
                debugger: nil,
                hooking: nil,
                proxyDetected: nil,
                appVersion: nil,
                riskScoreGte: nil
            )
        )

        let policy = makePolicy(rules: [rule])
        let decision = decision(
            action: "add_beneficiary",
            policy: policy,
            signals: signals(),
            attestation: "fail"
        )

        XCTAssertEqual(decision, .stepUp)
    }

    func testHighRiskDegradesViewCard() {
        let rule = PantherSecurityPolicyRule(
            action: "view_card",
            decision: "DEGRADE",
            conditions: PantherSecurityPolicyConditions(
                attestation: nil,
                debugger: nil,
                hooking: nil,
                proxyDetected: nil,
                appVersion: nil,
                riskScoreGte: 70
            )
        )

        let policy = makePolicy(rules: [rule])
        let decision = decision(
            action: "view_card",
            policy: policy,
            signals: signals(),
            riskScore: 90
        )

        XCTAssertEqual(decision, .degrade)
    }

    func testNoMatchDefaultsToAllow() {
        let rule = PantherSecurityPolicyRule(
            action: "login",
            decision: "DENY",
            conditions: PantherSecurityPolicyConditions(
                attestation: nil,
                debugger: true,
                hooking: nil,
                proxyDetected: nil,
                appVersion: nil,
                riskScoreGte: nil
            )
        )

        let policy = makePolicy(rules: [rule])
        let decision = decision(
            action: "transfer",
            policy: policy,
            signals: signals()
        )

        XCTAssertEqual(decision, .allow)
    }
}
