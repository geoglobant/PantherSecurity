import Foundation
import AppDomain
import AppData

@MainActor
public final class AppViewModel: ObservableObject {
    @Published public private(set) var policy: Policy?
    @Published public private(set) var status: String = "idle"

    private let fetchPolicy: FetchPolicyUseCase
    private let sendTelemetry: SendTelemetryUseCase

    public init(
        fetchPolicy: FetchPolicyUseCase = FetchPolicyUseCase(repository: PolicyRepositoryImpl()),
        sendTelemetry: SendTelemetryUseCase = SendTelemetryUseCase(repository: TelemetryRepositoryImpl())
    ) {
        self.fetchPolicy = fetchPolicy
        self.sendTelemetry = sendTelemetry
    }

    public func loadPolicy() async {
        status = "loading"
        do {
            policy = try await fetchPolicy.execute()
            status = "loaded"
        } catch {
            status = "error: \(error)"
        }
    }

    public func emitLoginEvent() async {
        let event = TelemetryEvent(
            eventId: UUID().uuidString,
            appId: policy?.appId ?? "fintech.mobile",
            appVersion: policy?.appVersion ?? "1.0.0",
            env: policy?.env ?? "local",
            action: ActionContext(name: "login", context: nil),
            signals: IntegritySignals(jailbreak: false, root: false, debugger: false, hooking: false, proxyDetected: false),
            timestamp: ISO8601DateFormatter().string(from: Date())
        )

        do {
            try await sendTelemetry.execute(event: event)
            status = "telemetry sent"
        } catch {
            status = "telemetry error: \(error)"
        }
    }
}
