import SwiftUI
import PantherSecurity

@main
struct SampleApp: App {
    var body: some Scene {
        WindowGroup {
            ContentView()
        }
    }
}

struct ContentView: View {
    @State private var status: String = "idle"
    @State private var policyId: String = "-"

    var body: some View {
        VStack(spacing: 16) {
            Text("Mobile AppSec Sample")
                .font(.title2)
            Text("Status: \(status)")
                .font(.caption)
            Text("Policy: \(policyId)")
                .font(.subheadline)

            Button("Fetch Policy") {
                Task { await fetchPolicy() }
            }

            Button("Send Login Telemetry") {
                Task { await sendTelemetry() }
            }
        }
        .padding()
        .onAppear { configureSDK() }
    }

    private func configureSDK() {
        let config = PantherSecurityConfiguration(
            baseURL: URL(string: "http://localhost:8082")!,
            appId: "fintech.mobile",
            appVersion: "1.0.0",
            env: "local",
            apiToken: nil,
            devicePlatform: "ios",
            pinning: nil
        )
        PantherSecuritySDK.shared.configure(config)
    }

    private func fetchPolicy() async {
        do {
            let policy = try await PantherSecuritySDK.shared.fetchPolicy()
            policyId = policy.policyId
            status = "policy loaded"
        } catch {
            status = "error: \(error)"
        }
    }

    private func sendTelemetry() async {
        let event = PantherSecurityTelemetryRequest(
            eventId: UUID().uuidString,
            appId: "fintech.mobile",
            appVersion: "1.0.0",
            env: "local",
            device: PantherSecurityDeviceInfo(platform: "ios", osVersion: "iOS", model: "iPhone"),
            signals: PantherSecurityIntegritySignals(jailbreak: false, root: false, debugger: false, hooking: false, proxyDetected: false),
            action: PantherSecurityActionContext(name: "login", context: nil),
            timestamp: ISO8601DateFormatter().string(from: Date()),
            signature: "stub-signature"
        )

        do {
            try await PantherSecuritySDK.shared.sendTelemetry(event)
            status = "telemetry sent"
        } catch {
            status = "telemetry error: \(error)"
        }
    }
}
