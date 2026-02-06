import Foundation

public struct PantherSecurityConfiguration: Equatable {
    public let baseURL: URL
    public let appId: String
    public let appVersion: String
    public let env: String
    public let apiToken: String?
    public let devicePlatform: String
    public let pinning: PantherSecurityPinning?

    public init(baseURL: URL, appId: String, appVersion: String, env: String, apiToken: String?, devicePlatform: String, pinning: PantherSecurityPinning? = nil) {
        self.baseURL = baseURL
        self.appId = appId
        self.appVersion = appVersion
        self.env = env
        self.apiToken = apiToken
        self.devicePlatform = devicePlatform
        self.pinning = pinning
    }
}

public struct PantherSecurityPinning: Equatable {
    public let currentSpkiHashes: [String]
    public let previousSpkiHashes: [String]
    public let rotatedAt: String?
    public let rotationWindowDays: Int?

    public init(currentSpkiHashes: [String], previousSpkiHashes: [String], rotatedAt: String?, rotationWindowDays: Int?) {
        self.currentSpkiHashes = currentSpkiHashes
        self.previousSpkiHashes = previousSpkiHashes
        self.rotatedAt = rotatedAt
        self.rotationWindowDays = rotationWindowDays
    }
}

public struct PantherSecurityPolicyResponse: Codable, Equatable {
    public let policyId: String
    public let appId: String
    public let appVersion: String
    public let env: String
    public let rules: [PantherSecurityPolicyRule]
    public let signature: String
    public let issuedAt: String

    enum CodingKeys: String, CodingKey {
        case policyId = "policy_id"
        case appId = "app_id"
        case appVersion = "app_version"
        case env
        case rules
        case signature
        case issuedAt = "issued_at"
    }

    public init(policyId: String, appId: String, appVersion: String, env: String, rules: [PantherSecurityPolicyRule], signature: String, issuedAt: String) {
        self.policyId = policyId
        self.appId = appId
        self.appVersion = appVersion
        self.env = env
        self.rules = rules
        self.signature = signature
        self.issuedAt = issuedAt
    }
}

public struct PantherSecurityPolicyRule: Codable, Equatable {
    public let action: String
    public let decision: String
    public let conditions: PantherSecurityPolicyConditions?

    public init(action: String, decision: String, conditions: PantherSecurityPolicyConditions?) {
        self.action = action
        self.decision = decision
        self.conditions = conditions
    }
}

public struct PantherSecurityPolicyConditions: Codable, Equatable {
    public let attestation: String?
    public let debugger: Bool?
    public let hooking: Bool?
    public let proxyDetected: Bool?
    public let appVersion: String?
    public let riskScoreGte: Int?

    enum CodingKeys: String, CodingKey {
        case attestation
        case debugger
        case hooking
        case proxyDetected = "proxy_detected"
        case appVersion = "app_version"
        case riskScoreGte = "risk_score_gte"
    }

    public init(attestation: String?, debugger: Bool?, hooking: Bool?, proxyDetected: Bool?, appVersion: String?, riskScoreGte: Int?) {
        self.attestation = attestation
        self.debugger = debugger
        self.hooking = hooking
        self.proxyDetected = proxyDetected
        self.appVersion = appVersion
        self.riskScoreGte = riskScoreGte
    }
}

public struct PantherSecurityTelemetryRequest: Codable, Equatable {
    public let eventId: String
    public let appId: String
    public let appVersion: String
    public let env: String
    public let device: PantherSecurityDeviceInfo
    public let signals: PantherSecurityIntegritySignals
    public let action: PantherSecurityActionContext
    public let timestamp: String
    public let signature: String

    enum CodingKeys: String, CodingKey {
        case eventId = "event_id"
        case appId = "app_id"
        case appVersion = "app_version"
        case env
        case device
        case signals
        case action
        case timestamp
        case signature
    }

    public init(eventId: String, appId: String, appVersion: String, env: String, device: PantherSecurityDeviceInfo, signals: PantherSecurityIntegritySignals, action: PantherSecurityActionContext, timestamp: String, signature: String) {
        self.eventId = eventId
        self.appId = appId
        self.appVersion = appVersion
        self.env = env
        self.device = device
        self.signals = signals
        self.action = action
        self.timestamp = timestamp
        self.signature = signature
    }
}

public struct PantherSecurityDeviceInfo: Codable, Equatable {
    public let platform: String
    public let osVersion: String
    public let model: String

    enum CodingKeys: String, CodingKey {
        case platform
        case osVersion = "os_version"
        case model
    }

    public init(platform: String, osVersion: String, model: String) {
        self.platform = platform
        self.osVersion = osVersion
        self.model = model
    }
}

public struct PantherSecurityIntegritySignals: Codable, Equatable {
    public let jailbreak: Bool
    public let root: Bool
    public let debugger: Bool
    public let hooking: Bool
    public let proxyDetected: Bool

    enum CodingKeys: String, CodingKey {
        case jailbreak
        case root
        case debugger
        case hooking
        case proxyDetected = "proxy_detected"
    }

    public init(jailbreak: Bool, root: Bool, debugger: Bool, hooking: Bool, proxyDetected: Bool) {
        self.jailbreak = jailbreak
        self.root = root
        self.debugger = debugger
        self.hooking = hooking
        self.proxyDetected = proxyDetected
    }
}

public struct PantherSecurityActionContext: Codable, Equatable {
    public let name: String
    public let context: String?

    public init(name: String, context: String?) {
        self.name = name
        self.context = context
    }
}

public enum PantherSecurityDecision: String {
    case allow = "ALLOW"
    case stepUp = "STEP_UP"
    case degrade = "DEGRADE"
    case deny = "DENY"
}
