import Foundation

public struct Policy: Equatable, Codable {
    public let policyId: String
    public let appId: String
    public let appVersion: String
    public let env: String
    public let rules: [PolicyRule]
    public let issuedAt: String

    public init(policyId: String, appId: String, appVersion: String, env: String, rules: [PolicyRule], issuedAt: String) {
        self.policyId = policyId
        self.appId = appId
        self.appVersion = appVersion
        self.env = env
        self.rules = rules
        self.issuedAt = issuedAt
    }
}

public struct PolicyRule: Equatable, Codable {
    public let action: String
    public let decision: Decision
    public let conditions: PolicyConditions

    public init(action: String, decision: Decision, conditions: PolicyConditions) {
        self.action = action
        self.decision = decision
        self.conditions = conditions
    }
}

public struct PolicyConditions: Equatable, Codable {
    public let attestation: String?
    public let debugger: Bool?
    public let hooking: Bool?
    public let proxyDetected: Bool?
    public let appVersion: String?
    public let riskScoreGte: Int?

    public init(attestation: String?, debugger: Bool?, hooking: Bool?, proxyDetected: Bool?, appVersion: String?, riskScoreGte: Int?) {
        self.attestation = attestation
        self.debugger = debugger
        self.hooking = hooking
        self.proxyDetected = proxyDetected
        self.appVersion = appVersion
        self.riskScoreGte = riskScoreGte
    }
}

public enum Decision: String, Codable {
    case allow = "ALLOW"
    case stepUp = "STEP_UP"
    case degrade = "DEGRADE"
    case deny = "DENY"
}
