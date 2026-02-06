import Foundation
import AppDomain
import PantherSecurity

public final class TelemetryRepositoryImpl: TelemetryRepository {
    private let sdk: PantherSecuritySDK

    public init(sdk: PantherSecuritySDK = .shared) {
        self.sdk = sdk
    }

    public func send(event: TelemetryEvent) async throws {
        let request = PantherSecurityTelemetryRequest(
            eventId: event.eventId,
            appId: event.appId,
            appVersion: event.appVersion,
            env: event.env,
            device: PantherSecurityDeviceInfo(platform: "ios", osVersion: "iOS", model: "iPhone"),
            signals: PantherSecurityIntegritySignals(
                jailbreak: event.signals.jailbreak,
                root: event.signals.root,
                debugger: event.signals.debugger,
                hooking: event.signals.hooking,
                proxyDetected: event.signals.proxyDetected
            ),
            action: PantherSecurityActionContext(name: event.action.name, context: event.action.context),
            timestamp: event.timestamp,
            signature: "stub-signature"
        )

        try await sdk.sendTelemetry(request)
    }
}
