import Foundation

public protocol PantherSecurityClient {
    func fetchPolicy(config: PantherSecurityConfiguration) async throws -> PantherSecurityPolicyResponse
    func sendTelemetry(_ event: PantherSecurityTelemetryRequest, config: PantherSecurityConfiguration) async throws
}

public final class PantherSecuritySDK {
    public static let shared = PantherSecuritySDK()

    public var client: PantherSecurityClient
    public private(set) var configuration: PantherSecurityConfiguration?
    private let core = PantherSecurityCore()

    public init(client: PantherSecurityClient = HTTPClient()) {
        self.client = client
    }

    public func configure(_ config: PantherSecurityConfiguration) {
        self.configuration = config
    }

    public func fetchPolicy() async throws -> PantherSecurityPolicyResponse {
        guard let config = configuration else {
            throw PantherSecurityError.notConfigured
        }
        return try await client.fetchPolicy(config: config)
    }

    public func sendTelemetry(_ event: PantherSecurityTelemetryRequest) async throws {
        guard let config = configuration else {
            throw PantherSecurityError.notConfigured
        }
        try await client.sendTelemetry(event, config: config)
    }

    public func evaluateDecision(
        policy: PantherSecurityPolicyResponse,
        action: PantherSecurityActionContext,
        signals: PantherSecurityIntegritySignals,
        attestationStatus: String? = nil,
        riskScore: UInt32 = 0
    ) -> PantherSecurityDecision {
        core.evaluate(policy: policy, action: action, signals: signals, attestationStatus: attestationStatus, riskScore: riskScore)
    }

    public func validatePinning(presentedSpkiHash: String) -> Bool {
        guard let config = configuration, let pinning = config.pinning else {
            return true
        }
        return core.validatePinning(pinning: pinning, presentedSpkiHash: presentedSpkiHash)
    }
}

public typealias PantherSecurity = PantherSecuritySDK

public enum PantherSecurityError: Error {
    case notConfigured
    case invalidURL
    case httpError(Int)
    case decodingError
}

public final class HTTPClient: PantherSecurityClient {
    private let session: URLSession

    public init(session: URLSession = .shared) {
        self.session = session
    }

    public func fetchPolicy(config: PantherSecurityConfiguration) async throws -> PantherSecurityPolicyResponse {
        var components = URLComponents(url: config.baseURL.appendingPathComponent("/v1/policies/current"), resolvingAgainstBaseURL: false)
        components?.queryItems = [
            URLQueryItem(name: "app_id", value: config.appId),
            URLQueryItem(name: "app_version", value: config.appVersion),
            URLQueryItem(name: "env", value: config.env),
            URLQueryItem(name: "device_platform", value: config.devicePlatform)
        ]
        guard let url = components?.url else {
            throw PantherSecurityError.invalidURL
        }

        var request = URLRequest(url: url)
        request.httpMethod = "GET"
        if let token = config.apiToken {
            request.setValue("Bearer \(token)", forHTTPHeaderField: "Authorization")
        }

        let (data, response) = try await session.data(for: request)
        guard let http = response as? HTTPURLResponse else {
            throw PantherSecurityError.decodingError
        }
        guard (200..<300).contains(http.statusCode) else {
            throw PantherSecurityError.httpError(http.statusCode)
        }

        let decoder = JSONDecoder()
        return try decoder.decode(PantherSecurityPolicyResponse.self, from: data)
    }

    public func sendTelemetry(_ event: PantherSecurityTelemetryRequest, config: PantherSecurityConfiguration) async throws {
        let url = config.baseURL.appendingPathComponent("/v1/telemetry/events")
        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        if let token = config.apiToken {
            request.setValue("Bearer \(token)", forHTTPHeaderField: "Authorization")
        }

        let encoder = JSONEncoder()
        request.httpBody = try encoder.encode(event)

        let (_, response) = try await session.data(for: request)
        guard let http = response as? HTTPURLResponse else {
            throw PantherSecurityError.decodingError
        }
        guard (200..<300).contains(http.statusCode) else {
            throw PantherSecurityError.httpError(http.statusCode)
        }
    }
}
