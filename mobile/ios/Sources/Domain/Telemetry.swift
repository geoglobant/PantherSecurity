import Foundation

public struct IntegritySignals: Equatable, Codable {
    public let jailbreak: Bool
    public let root: Bool
    public let debugger: Bool
    public let hooking: Bool
    public let proxyDetected: Bool

    public init(jailbreak: Bool, root: Bool, debugger: Bool, hooking: Bool, proxyDetected: Bool) {
        self.jailbreak = jailbreak
        self.root = root
        self.debugger = debugger
        self.hooking = hooking
        self.proxyDetected = proxyDetected
    }
}

public struct ActionContext: Equatable, Codable {
    public let name: String
    public let context: String?

    public init(name: String, context: String?) {
        self.name = name
        self.context = context
    }
}

public struct TelemetryEvent: Equatable, Codable {
    public let eventId: String
    public let appId: String
    public let appVersion: String
    public let env: String
    public let action: ActionContext
    public let signals: IntegritySignals
    public let timestamp: String

    public init(eventId: String, appId: String, appVersion: String, env: String, action: ActionContext, signals: IntegritySignals, timestamp: String) {
        self.eventId = eventId
        self.appId = appId
        self.appVersion = appVersion
        self.env = env
        self.action = action
        self.signals = signals
        self.timestamp = timestamp
    }
}
