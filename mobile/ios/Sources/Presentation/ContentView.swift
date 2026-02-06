import SwiftUI
import PantherSecurity

public struct ContentView: View {
    @StateObject private var viewModel = AppViewModel()

    public init() {}

    public var body: some View {
        VStack(spacing: 16) {
            Text("Mobile AppSec Platform")
                .font(.title2)
            Text("Status: \(viewModel.status)")
                .font(.caption)

            if let policy = viewModel.policy {
                Text("Policy: \(policy.policyId)")
                    .font(.headline)
                Text("Rules: \(policy.rules.count)")
                    .font(.subheadline)
            } else {
                Text("No policy loaded")
                    .font(.subheadline)
            }

            Button("Fetch Policy") {
                Task { await viewModel.loadPolicy() }
            }

            Button("Send Login Telemetry") {
                Task { await viewModel.emitLoginEvent() }
            }
        }
        .padding()
        .onAppear {
            configureSDK()
        }
    }

    private func configureSDK() {
        let pinning = PantherSecurityPinning(
            currentSpkiHashes: ["hash_current"],
            previousSpkiHashes: ["hash_previous"],
            rotatedAt: nil,
            rotationWindowDays: 7
        )

        let config = PantherSecurityConfiguration(
            baseURL: URL(string: "http://localhost:8082")!,
            appId: "fintech.mobile",
            appVersion: "1.0.0",
            env: "local",
            apiToken: nil,
            devicePlatform: "ios",
            pinning: pinning
        )
        PantherSecuritySDK.shared.configure(config)
    }
}
