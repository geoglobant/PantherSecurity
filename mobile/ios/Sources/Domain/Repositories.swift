import Foundation

public protocol PolicyRepository {
    func fetchPolicy() async throws -> Policy
}

public protocol TelemetryRepository {
    func send(event: TelemetryEvent) async throws
}
