import Foundation

public struct FetchPolicyUseCase {
    private let repository: PolicyRepository

    public init(repository: PolicyRepository) {
        self.repository = repository
    }

    public func execute() async throws -> Policy {
        try await repository.fetchPolicy()
    }
}

public struct SendTelemetryUseCase {
    private let repository: TelemetryRepository

    public init(repository: TelemetryRepository) {
        self.repository = repository
    }

    public func execute(event: TelemetryEvent) async throws {
        try await repository.send(event: event)
    }
}
