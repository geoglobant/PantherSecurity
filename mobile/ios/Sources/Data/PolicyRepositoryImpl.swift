import Foundation
import AppDomain
import PantherSecurity

public final class PolicyRepositoryImpl: PolicyRepository {
    private let sdk: PantherSecuritySDK

    public init(sdk: PantherSecuritySDK = .shared) {
        self.sdk = sdk
    }

    public func fetchPolicy() async throws -> Policy {
        let response = try await sdk.fetchPolicy()
        return Policy(
            policyId: response.policyId,
            appId: response.appId,
            appVersion: response.appVersion,
            env: response.env,
            rules: response.rules.map { rule in
                PolicyRule(
                    action: rule.action,
                    decision: Decision(rawValue: rule.decision) ?? .deny,
                    conditions: PolicyConditions(
                        attestation: rule.conditions?.attestation,
                        debugger: rule.conditions?.debugger,
                        hooking: rule.conditions?.hooking,
                        proxyDetected: rule.conditions?.proxyDetected,
                        appVersion: rule.conditions?.appVersion,
                        riskScoreGte: rule.conditions?.riskScoreGte
                    )
                )
            },
            issuedAt: response.issuedAt
        )
    }
}
