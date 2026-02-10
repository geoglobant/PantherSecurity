import SwiftUI
import PantherSecurity

@main
struct SampleApp: App {
    var body: some Scene {
        WindowGroup {
            RootView()
        }
    }
}

enum AttestationChoice: String, CaseIterable, Identifiable {
    case none
    case pass
    case fail
    case unknown

    var id: String { rawValue }

    var statusValue: String? {
        switch self {
        case .none: return nil
        case .pass: return "pass"
        case .fail: return "fail"
        case .unknown: return "unknown"
        }
    }
}

enum CardMode: String {
    case hidden
    case masked
    case full
}

enum PendingAction: String {
    case login
    case transfer
    case viewCard
    case addBeneficiary
    case changePassword

    var policyAction: String {
        switch self {
        case .viewCard: return "view_card"
        case .addBeneficiary: return "add_beneficiary"
        case .changePassword: return "change_password"
        default: return rawValue
        }
    }

    var label: String {
        switch self {
        case .viewCard: return "View Card"
        case .addBeneficiary: return "Add Beneficiary"
        case .changePassword: return "Change Password"
        default: return rawValue.capitalized
        }
    }
}

@MainActor
final class DemoState: ObservableObject {
    @Published var isLoggedIn = false
    @Published var policy: PantherSecurityPolicyResponse?
    @Published var status: String = "idle"
    @Published var lastDecision: PantherSecurityDecision?
    @Published var lastMessage: String = "-"
    @Published var lastAction: PendingAction?
    @Published var cardMode: CardMode = .hidden

    @Published var showStepUp = false
    @Published var pendingAction: PendingAction?

    @Published var jailbreak = false
    @Published var root = false
    @Published var debugger = false
    @Published var hooking = false
    @Published var proxyDetected = false
    @Published var attestationChoice: AttestationChoice = .none
    @Published var riskScore: Double = 0

    private let appId = "fintech.mobile"
    private let appVersion = "1.0.0"
    private let env = "local"
    private let storage = SecureStorage(service: "com.panther.mobileappsecsample")
    private let tokenKey = "api_token"
    private let emailKey = "user_email"
    private var pendingLoginEmail: String?

    init() {
        let storedToken = storage.read(tokenKey)
        configureSDK(apiToken: storedToken)
    }

    func login(email: String, password: String) {
        pendingLoginEmail = email
        requestAction(.login)
    }

    func fetchPolicy() async {
        status = "loading policy"
        do {
            let response = try await PantherSecuritySDK.shared.fetchPolicy()
            policy = response
            status = "policy loaded"
        } catch {
            policy = fallbackPolicy()
            status = "policy loaded (local fallback)"
        }
    }

    func requestAction(_ action: PendingAction) {
        lastAction = action
        if policy == nil {
            status = "loading policy"
            Task {
                await fetchPolicy()
                requestAction(action)
            }
            return
        }

        let outcome = evaluateDecision(for: action)
        lastDecision = outcome.decision
        lastMessage = outcome.message

        switch outcome.decision {
        case .allow:
            status = "allowed"
            applyAllowed(action)
        case .degrade:
            status = "degraded"
            applyDegrade(action)
        case .stepUp:
            status = "step-up required"
            pendingAction = action
            showStepUp = true
        case .deny:
            status = "denied"
        }

        Task { await sendTelemetry(action: action) }
    }

    func completeStepUp() {
        guard let action = pendingAction else { return }
        showStepUp = false
        pendingAction = nil
        status = "step-up completed"
        applyAllowed(action)
    }

    func cancelStepUp() {
        showStepUp = false
        pendingAction = nil
    }

    func logout() {
        storage.delete(tokenKey)
        storage.delete(emailKey)
        configureSDK(apiToken: nil)
        isLoggedIn = false
        status = "logged out"
        lastDecision = nil
        lastMessage = "-"
        lastAction = nil
        cardMode = .hidden
    }

    private func configureSDK(apiToken: String?) {
        let pinning = PantherSecurityPinning(
            currentSpkiHashes: ["hash_current"],
            previousSpkiHashes: ["hash_previous"],
            rotatedAt: nil,
            rotationWindowDays: 7
        )

        let config = PantherSecurityConfiguration(
            baseURL: URL(string: "http://localhost:8082")!,
            appId: appId,
            appVersion: appVersion,
            env: env,
            apiToken: apiToken,
            devicePlatform: "ios",
            pinning: pinning
        )
        PantherSecuritySDK.shared.configure(config)
    }

    private func evaluateDecision(for action: PendingAction) -> (decision: PantherSecurityDecision, message: String) {
        guard let policy else {
            return (.deny, "Policy not loaded")
        }

        let signals = currentSignals()
        let actionContext = PantherSecurityActionContext(name: action.policyAction, context: nil)
        let decision = PantherSecuritySDK.shared.evaluateDecision(
            policy: policy,
            action: actionContext,
            signals: signals,
            attestationStatus: attestationChoice.statusValue,
            riskScore: UInt32(riskScore)
        )

        let matchedRule = matchingRule(for: action.policyAction)
        let reason = reasonForRule(matchedRule)

        return (decision, reason)
    }

    private func matchingRule(for action: String) -> PantherSecurityPolicyRule? {
        guard let policy else { return nil }
        for rule in policy.rules where rule.action == action {
            if matches(rule.conditions) {
                return rule
            }
        }
        return nil
    }

    private func matches(_ conditions: PantherSecurityPolicyConditions?) -> Bool {
        guard let conditions else { return true }

        if let required = conditions.attestation, required != attestationChoice.statusValue {
            return false
        }
        if let required = conditions.debugger, required != debugger {
            return false
        }
        if let required = conditions.hooking, required != hooking {
            return false
        }
        if let required = conditions.proxyDetected, required != proxyDetected {
            return false
        }
        if let required = conditions.appVersion, required != appVersion {
            return false
        }
        if let required = conditions.riskScoreGte, Int(riskScore) < required {
            return false
        }

        return true
    }

    private func reasonForRule(_ rule: PantherSecurityPolicyRule?) -> String {
        guard let conditions = rule?.conditions else {
            return "No policy rule matched"
        }

        if conditions.proxyDetected == true { return "Proxy/MITM detected" }
        if conditions.debugger == true { return "Debugger detected" }
        if conditions.hooking == true { return "Hooking detected" }
        if let risk = conditions.riskScoreGte { return "High risk score (>=\(risk))" }
        if let att = conditions.attestation { return "Attestation \(att)" }
        if let version = conditions.appVersion { return "Blocked for app version \(version)" }

        return "Policy rule matched"
    }

    private func currentSignals() -> PantherSecurityIntegritySignals {
        PantherSecurityIntegritySignals(
            jailbreak: jailbreak,
            root: root,
            debugger: debugger,
            hooking: hooking,
            proxyDetected: proxyDetected
        )
    }

    private func applyAllowed(_ action: PendingAction) {
        switch action {
        case .login:
            isLoggedIn = true
            let email = pendingLoginEmail ?? "maria@pantherbank.com"
            let token = "demo-token-\(UUID().uuidString)"
            _ = storage.save(token, for: tokenKey)
            _ = storage.save(email, for: emailKey)
            configureSDK(apiToken: token)
            pendingLoginEmail = nil
        case .viewCard:
            cardMode = .full
        case .transfer:
            status = "transfer completed"
        case .addBeneficiary:
            status = "beneficiary added"
        case .changePassword:
            status = "password changed"
        }
    }

    private func fallbackPolicy() -> PantherSecurityPolicyResponse {
        PantherSecurityPolicyResponse(
            policyId: "policy_local",
            appId: appId,
            appVersion: appVersion,
            env: env,
            rules: [
                PantherSecurityPolicyRule(
                    action: "login",
                    decision: "STEP_UP",
                    conditions: PantherSecurityPolicyConditions(
                        attestation: nil,
                        debugger: true,
                        hooking: nil,
                        proxyDetected: nil,
                        appVersion: nil,
                        riskScoreGte: nil
                    )
                ),
                PantherSecurityPolicyRule(
                    action: "transfer",
                    decision: "DENY",
                    conditions: PantherSecurityPolicyConditions(
                        attestation: nil,
                        debugger: nil,
                        hooking: nil,
                        proxyDetected: true,
                        appVersion: nil,
                        riskScoreGte: nil
                    )
                ),
                PantherSecurityPolicyRule(
                    action: "transfer",
                    decision: "STEP_UP",
                    conditions: PantherSecurityPolicyConditions(
                        attestation: nil,
                        debugger: nil,
                        hooking: nil,
                        proxyDetected: nil,
                        appVersion: nil,
                        riskScoreGte: 70
                    )
                ),
                PantherSecurityPolicyRule(
                    action: "view_card",
                    decision: "DEGRADE",
                    conditions: PantherSecurityPolicyConditions(
                        attestation: nil,
                        debugger: nil,
                        hooking: true,
                        proxyDetected: nil,
                        appVersion: nil,
                        riskScoreGte: nil
                    )
                ),
                PantherSecurityPolicyRule(
                    action: "add_beneficiary",
                    decision: "STEP_UP",
                    conditions: PantherSecurityPolicyConditions(
                        attestation: "fail",
                        debugger: nil,
                        hooking: nil,
                        proxyDetected: nil,
                        appVersion: nil,
                        riskScoreGte: nil
                    )
                ),
                PantherSecurityPolicyRule(
                    action: "change_password",
                    decision: "DENY",
                    conditions: PantherSecurityPolicyConditions(
                        attestation: nil,
                        debugger: nil,
                        hooking: nil,
                        proxyDetected: nil,
                        appVersion: "1.0.0",
                        riskScoreGte: nil
                    )
                )
            ],
            signature: "stub",
            issuedAt: ISO8601DateFormatter().string(from: Date())
        )
    }

    private func applyDegrade(_ action: PendingAction) {
        switch action {
        case .viewCard:
            cardMode = .masked
        default:
            status = "action degraded"
        }
    }

    private func sendTelemetry(action: PendingAction) async {
        let event = PantherSecurityTelemetryRequest(
            eventId: UUID().uuidString,
            appId: appId,
            appVersion: appVersion,
            env: env,
            device: PantherSecurityDeviceInfo(platform: "ios", osVersion: "iOS", model: "iPhone"),
            signals: currentSignals(),
            action: PantherSecurityActionContext(name: action.policyAction, context: nil),
            timestamp: ISO8601DateFormatter().string(from: Date()),
            signature: "stub-signature"
        )

        _ = try? await PantherSecuritySDK.shared.sendTelemetry(event)
    }
}

struct RootView: View {
    @StateObject private var state = DemoState()

    var body: some View {
        ZStack {
            AppBackground()
            Group {
                if state.isLoggedIn {
                    HomeView(state: state)
                } else {
                    LoginView(state: state)
                }
            }
        }
        .task {
            await state.fetchPolicy()
        }
        .sheet(isPresented: $state.showStepUp) {
            StepUpSheet(
                actionLabel: state.pendingAction?.label ?? "Action",
                onConfirm: { state.completeStepUp() },
                onCancel: { state.cancelStepUp() }
            )
        }
    }
}

struct LoginView: View {
    @ObservedObject var state: DemoState
    @State private var email = ""
    @State private var password = ""
    @State private var showHelp = false
    @State private var infoItem: SecurityInfo?

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 18) {
                VStack(alignment: .leading, spacing: 6) {
                    Text("Panther Bank")
                        .font(.system(.title, design: .rounded).weight(.semibold))
                    Text("Financial security demo")
                        .font(.footnote)
                        .foregroundColor(AppTheme.muted)
                }

                SectionCard {
                    VStack(spacing: 12) {
                        TextField("Email", text: $email)
                            .textInputAutocapitalization(.never)
                            .autocorrectionDisabled()
                            .textFieldStyle(.roundedBorder)
                        SecureField("Password", text: $password)
                            .textFieldStyle(.roundedBorder)
                    }
                }

                Button {
                    state.login(email: email, password: password)
                } label: {
                    Text("Sign In")
                        .frame(maxWidth: .infinity)
                }
                .buttonStyle(PrimaryButtonStyle())

                InfoLink(
                    title: "Login security info",
                    info: .login,
                    selectedInfo: $infoItem
                )

                Button("Need help?") {
                    showHelp = true
                }
                .font(.footnote.weight(.semibold))
                .foregroundColor(AppTheme.accent)
                .buttonStyle(.plain)

                SectionCard {
                    VStack(alignment: .leading, spacing: 8) {
                        Text("How to use")
                            .font(.headline)
                        Text("1. Sign in to start a session")
                            .font(.footnote)
                            .foregroundColor(AppTheme.muted)
                        Text("2. Open Demo Mode and simulate risk")
                            .font(.footnote)
                            .foregroundColor(AppTheme.muted)
                        Text("3. Try Transfer or View Card")
                            .font(.footnote)
                            .foregroundColor(AppTheme.muted)
                    }
                }

                SectionCard {
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Security check")
                            .font(.caption)
                            .foregroundColor(AppTheme.muted)
                        HStack {
                            Text(state.lastDecision?.rawValue ?? "-")
                                .font(.headline)
                            Spacer()
                            Pill(text: state.lastDecision?.rawValue ?? "PENDING")
                        }
                        Text(state.lastMessage)
                            .font(.footnote)
                            .foregroundColor(AppTheme.muted)
                    }
                }

            }
            .padding(20)
        }
        .sheet(isPresented: $showHelp) {
            HelpSheet()
        }
        .sheet(item: $infoItem) { info in
            SecurityInfoSheet(info: info)
        }
    }
}

struct HomeView: View {
    @ObservedObject var state: DemoState

    @State private var showTransfer = false
    @State private var showBeneficiary = false
    @State private var showSettings = false
    @State private var showHelp = false
    @State private var infoItem: SecurityInfo?

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 18) {
                    header
                    balanceCard
                    quickActions
                    securityCard
                    cardSection
                }
                .padding(20)
            }
            .navigationTitle("Panther Bank")
            .toolbar {
                ToolbarItem(placement: .topBarLeading) {
                    Button {
                        showHelp = true
                    } label: {
                        Image(systemName: "questionmark.circle")
                    }
                }
                ToolbarItemGroup(placement: .topBarTrailing) {
                    Button {
                        showSettings = true
                    } label: {
                        Image(systemName: "gearshape")
                    }

                    Button("Logout") {
                        state.logout()
                    }
                    .font(.subheadline)
                }
            }
            .onAppear {
                Task { await state.fetchPolicy() }
            }
            .sheet(isPresented: $showTransfer) {
                TransferSheet(state: state)
            }
            .sheet(isPresented: $showBeneficiary) {
                BeneficiarySheet(state: state)
            }
            .sheet(isPresented: $showSettings) {
                SettingsSheet(state: state)
            }
            .sheet(isPresented: $showHelp) {
                HelpSheet()
            }
            .sheet(item: $infoItem) { info in
                SecurityInfoSheet(info: info)
            }
        }
    }

    private var header: some View {
        VStack(alignment: .leading, spacing: 6) {
            Text("Good evening, Maria")
                .font(.system(.title2, design: .rounded).weight(.semibold))
            Text("Personal account")
                .font(.footnote)
                .foregroundColor(AppTheme.muted)
        }
        .frame(maxWidth: .infinity, alignment: .leading)
    }

    private var balanceCard: some View {
        SectionCard {
            VStack(alignment: .leading, spacing: 8) {
                Text("Available balance")
                    .font(.caption)
                    .foregroundColor(AppTheme.muted)
                Text("$ 12,480.90")
                    .font(.system(.title2, design: .rounded).weight(.semibold))
                Text("Updated just now")
                    .font(.caption2)
                    .foregroundColor(AppTheme.muted)
            }
            .frame(maxWidth: .infinity, alignment: .leading)
        }
    }

    private var quickActions: some View {
        SectionCard {
            VStack(alignment: .leading, spacing: 12) {
                ActionBlock(
                    title: "Transfer money",
                    isPrimary: true,
                    info: .transfer,
                    selectedInfo: $infoItem
                ) {
                    showTransfer = true
                }

                ActionBlock(
                    title: "Add beneficiary",
                    isPrimary: false,
                    info: .addBeneficiary,
                    selectedInfo: $infoItem
                ) {
                    showBeneficiary = true
                }
            }
            .frame(maxWidth: .infinity)
        }
    }

    private var securityCard: some View {
        SectionCard {
            VStack(alignment: .leading, spacing: 8) {
                Text("Security check")
                    .font(.caption)
                    .foregroundColor(AppTheme.muted)
                HStack {
                    Text(state.lastDecision?.rawValue ?? "-")
                        .font(.headline)
                    Spacer()
                    Pill(text: state.lastDecision?.rawValue ?? "PENDING")
                }
                if let action = state.lastAction {
                    HStack(spacing: 6) {
                        Text("Action:")
                            .font(.caption)
                            .foregroundColor(AppTheme.muted)
                        Text(action.label)
                            .font(.subheadline.weight(.semibold))
                            .foregroundColor(AppTheme.accent)
                        Text("(\(action.policyAction))")
                            .font(.caption)
                            .foregroundColor(AppTheme.muted)
                    }
                }
                Text(state.lastMessage)
                    .font(.footnote)
                    .foregroundColor(AppTheme.muted)
            }
            .frame(maxWidth: .infinity, alignment: .leading)
        }
    }

    private var cardSection: some View {
        SectionCard {
            VStack(alignment: .leading, spacing: 12) {
                Text("Card")
                    .font(.headline)

                Text(cardText)
                    .font(.system(.title3, design: .rounded))
                    .monospaced()

                Button("View card") {
                    state.requestAction(.viewCard)
                }
                .buttonStyle(SecondaryButtonStyle())

                InfoLink(
                    title: "Card access security info",
                    info: .viewCard,
                    selectedInfo: $infoItem
                )
            }
            .frame(maxWidth: .infinity, alignment: .leading)
        }
    }

    private var cardText: String {
        switch state.cardMode {
        case .hidden: return "•••• •••• •••• ••••"
        case .masked: return "4987 •••• •••• 1200"
        case .full: return "4987 0312 9988 1200"
        }
    }
}

struct TransferSheet: View {
    @ObservedObject var state: DemoState
    @State private var amount = ""
    @Environment(\.dismiss) private var dismiss

    var body: some View {
        NavigationStack {
            VStack(spacing: 16) {
                Text("Transfer")
                    .font(.headline)
                TextField("Amount", text: $amount)
                    .keyboardType(.decimalPad)
                    .textFieldStyle(.roundedBorder)
                Button("Send") {
                    state.requestAction(.transfer)
                    dismiss()
                }
                .buttonStyle(PrimaryButtonStyle())
            }
            .padding(24)
            .navigationTitle("Transfer")
            .navigationBarTitleDisplayMode(.inline)
        }
    }
}

struct BeneficiarySheet: View {
    @ObservedObject var state: DemoState
    @State private var name = ""
    @State private var account = ""
    @Environment(\.dismiss) private var dismiss

    var body: some View {
        NavigationStack {
            VStack(spacing: 16) {
                Text("Add Beneficiary")
                    .font(.headline)
                TextField("Name", text: $name)
                    .textFieldStyle(.roundedBorder)
                TextField("Account", text: $account)
                    .keyboardType(.numberPad)
                    .textFieldStyle(.roundedBorder)
                Button("Save") {
                    state.requestAction(.addBeneficiary)
                    dismiss()
                }
                .buttonStyle(PrimaryButtonStyle())
            }
            .padding(24)
            .navigationTitle("New Beneficiary")
            .navigationBarTitleDisplayMode(.inline)
        }
    }
}

struct SettingsSheet: View {
    @ObservedObject var state: DemoState
    @State private var showHelp = false
    @State private var infoItem: SecurityInfo?

    var body: some View {
        NavigationStack {
            VStack(alignment: .leading, spacing: 16) {
                SectionCard {
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Security")
                            .font(.headline)
                        NavigationLink {
                            SecurityLabView(state: state)
                        } label: {
                            SettingsRow(
                                title: "Security Lab",
                                subtitle: "Simulate signals and attestation",
                                systemImage: "shield.lefthalf.filled"
                            )
                        }
                        .buttonStyle(.plain)
                        Divider()
                        SettingsRow(
                            title: "Change password",
                            subtitle: "Requires step-up verification",
                            systemImage: "key.fill"
                        ) {
                            state.requestAction(.changePassword)
                        }
                        InfoLink(
                            title: "Password change security info",
                            info: .changePassword,
                            selectedInfo: $infoItem
                        )
                    }
                }
                SectionCard {
                    SettingsRow(
                        title: "Help & demo guide",
                        subtitle: "Understand signals and outcomes",
                        systemImage: "questionmark.circle"
                    ) {
                        showHelp = true
                    }
                }
                SectionCard {
                    VStack(alignment: .leading, spacing: 6) {
                        Text("Policy status")
                            .font(.caption)
                            .foregroundColor(AppTheme.muted)
                        Text(state.status)
                            .font(.footnote)
                            .foregroundColor(AppTheme.muted)
                    }
                }
                Spacer()
            }
            .padding(20)
            .navigationTitle("Settings")
            .navigationBarTitleDisplayMode(.inline)
            .sheet(isPresented: $showHelp) {
                HelpSheet()
            }
            .sheet(item: $infoItem) { info in
                SecurityInfoSheet(info: info)
            }
        }
    }
}

struct HelpSheet: View {
    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(alignment: .leading, spacing: 16) {
                    SectionCard {
                        VStack(alignment: .leading, spacing: 8) {
                            Text("Quick demo flow")
                                .font(.headline)
                            Text("1. Sign in to start the session.")
                                .font(.footnote)
                                .foregroundColor(AppTheme.muted)
                            Text("2. Open Security Lab to simulate risk.")
                                .font(.footnote)
                                .foregroundColor(AppTheme.muted)
                            Text("3. Try Transfer, View Card, or Add Beneficiary.")
                                .font(.footnote)
                                .foregroundColor(AppTheme.muted)
                        }
                    }

                    SectionCard {
                        VStack(alignment: .leading, spacing: 8) {
                            Text("How the rules work")
                                .font(.headline)
                            Text("The SDK evaluates a policy rule for each action using the current signals, attestation, and risk score.")
                                .font(.footnote)
                                .foregroundColor(AppTheme.muted)
                            Text("The simulator only changes those inputs. The policy decides the outcome.")
                                .font(.footnote)
                                .foregroundColor(AppTheme.muted)
                        }
                    }

                    SectionCard {
                        VStack(alignment: .leading, spacing: 8) {
                            Text("Security inputs (7)")
                                .font(.headline)
                            HelpRow(title: "Jailbreak", detail: "Device integrity compromised.")
                            HelpRow(title: "Root", detail: "Rooted device signal.")
                            HelpRow(title: "Debugger", detail: "Debug/instrumentation detected.")
                            HelpRow(title: "Hooking", detail: "Runtime method hooks detected.")
                            HelpRow(title: "Proxy/MITM", detail: "Traffic interception detected.")
                            HelpRow(title: "Attestation", detail: "Pass/Fail/Unknown status.")
                            HelpRow(title: "Risk score", detail: "Server score 0–100.")
                        }
                    }

                    SectionCard {
                        VStack(alignment: .leading, spacing: 8) {
                            Text("Decisions")
                                .font(.headline)
                            HelpRow(title: "ALLOW", detail: "Action proceeds normally.")
                            HelpRow(title: "STEP_UP", detail: "Extra verification required.")
                            HelpRow(title: "DEGRADE", detail: "Limited functionality.")
                            HelpRow(title: "DENY", detail: "Action blocked.")
                        }
                    }

                    SectionCard {
                        VStack(alignment: .leading, spacing: 8) {
                            Text("Risk score & tests")
                                .font(.headline)
                            Text("The slider simulates a server risk score (0–100). Policies can trigger step-up or deny when the score is high.")
                                .font(.footnote)
                                .foregroundColor(AppTheme.muted)
                            Text("The actions you tap are the tests: each one runs a policy evaluation and logs the decision.")
                                .font(.footnote)
                                .foregroundColor(AppTheme.muted)
                        }
                    }

                    SectionCard {
                        VStack(alignment: .leading, spacing: 8) {
                            Text("Example policies (demo)")
                                .font(.headline)
                            HelpRow(title: "Transfer", detail: "Proxy/MITM → DENY. High risk score → STEP_UP.")
                            HelpRow(title: "Add beneficiary", detail: "Attestation fail → STEP_UP.")
                            HelpRow(title: "View card", detail: "Hooking → DEGRADE (masked).")
                            HelpRow(title: "Change password", detail: "Blocked on legacy app version.")
                        }
                    }

                    SectionCard {
                        VStack(alignment: .leading, spacing: 8) {
                            Text("How to reduce risk")
                                .font(.headline)
                            RiskHelpRow(
                                title: "BOLA/IDOR",
                                detail: "Enforce server-side ownership checks on beneficiary creation and transfers.",
                                url: URL(string: "https://owasp.org/API-Security/editions/2023/en/0xa1-broken-object-level-authorization/")!
                            )
                            RiskHelpRow(
                                title: "MITM",
                                detail: "Use TLS pinning and detect proxies to stop traffic interception.",
                                url: URL(string: "https://mas.owasp.org/MASTG/techniques/generic/MASTG-TECH-0120/")!
                            )
                            RiskHelpRow(
                                title: "Hooking",
                                detail: "Harden runtime integrity and block sensitive flows when tampering is detected.",
                                url: URL(string: "https://mas.owasp.org/MASTG/techniques/generic/MASTG-TECH-0051/")!
                            )
                            RiskHelpRow(
                                title: "Attestation",
                                detail: "Require a valid attestation for account changes.",
                                url: URL(string: "https://developer.apple.com/documentation/devicecheck/validating_apps_that_connect_to_your_server")!
                            )
                        }
                    }

                    SectionCard {
                        VStack(alignment: .leading, spacing: 8) {
                            Text("Security example docs")
                                .font(.headline)
                            Link("MITM / TLS pinning", destination: URL(string: "https://github.com/geoglobant/PantherSecurity/blob/main/docs/security-examples/mitm-proxy-pinning.md")!)
                            Link("Debugger & hooking", destination: URL(string: "https://github.com/geoglobant/PantherSecurity/blob/main/docs/security-examples/debugger-hooking.md")!)
                            Link("Jailbreak / root", destination: URL(string: "https://github.com/geoglobant/PantherSecurity/blob/main/docs/security-examples/device-integrity-jailbreak-root.md")!)
                            Link("Attestation", destination: URL(string: "https://github.com/geoglobant/PantherSecurity/blob/main/docs/security-examples/attestation.md")!)
                            Link("BOLA / IDOR", destination: URL(string: "https://github.com/geoglobant/PantherSecurity/blob/main/docs/security-examples/bola-idor.md")!)
                            Link("Risk score", destination: URL(string: "https://github.com/geoglobant/PantherSecurity/blob/main/docs/security-examples/risk-score.md")!)
                        }
                        .font(.footnote.weight(.semibold))
                        .foregroundColor(AppTheme.accent)
                    }
                }
                .padding(20)
            }
            .navigationTitle("Help")
            .navigationBarTitleDisplayMode(.inline)
        }
    }
}

struct HelpRow: View {
    let title: String
    let detail: String

    var body: some View {
        HStack(alignment: .top, spacing: 8) {
            Text(title)
                .font(.subheadline.weight(.semibold))
                .frame(width: 90, alignment: .leading)
            Text(detail)
                .font(.caption)
                .foregroundColor(AppTheme.muted)
            Spacer()
        }
    }
}

struct RiskHelpRow: View {
    let title: String
    let detail: String
    let url: URL

    var body: some View {
        VStack(alignment: .leading, spacing: 4) {
            HelpRow(title: title, detail: detail)
            Link("Learn more", destination: url)
                .font(.caption.weight(.semibold))
                .foregroundColor(AppTheme.accent)
        }
    }
}

struct ActionBlock: View {
    let title: String
    let isPrimary: Bool
    let info: SecurityInfo
    @Binding var selectedInfo: SecurityInfo?
    let action: () -> Void

    var body: some View {
        VStack(alignment: .leading, spacing: 6) {
            if isPrimary {
                Button(title) {
                    action()
                }
                .buttonStyle(PrimaryButtonStyle())
            } else {
                Button(title) {
                    action()
                }
                .buttonStyle(SecondaryButtonStyle())
            }

            InfoLink(
                title: "Security details",
                info: info,
                selectedInfo: $selectedInfo
            )
        }
    }
}

struct InfoLink: View {
    let title: String
    let info: SecurityInfo
    @Binding var selectedInfo: SecurityInfo?

    var body: some View {
        Button {
            selectedInfo = info
        } label: {
            HStack(spacing: 6) {
                Image(systemName: "info.circle")
                Text(title)
            }
            .font(.caption.weight(.semibold))
            .foregroundColor(AppTheme.accent)
        }
        .buttonStyle(.plain)
    }
}

struct SecurityInfoSheet: View {
    let info: SecurityInfo

    var body: some View {
        NavigationStack {
            VStack(alignment: .leading, spacing: 16) {
                SectionCard {
                    VStack(alignment: .leading, spacing: 8) {
                        Text(info.title)
                            .font(.headline)
                        Text(info.summary)
                            .font(.footnote)
                            .foregroundColor(AppTheme.muted)
                    }
                }

                SectionCard {
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Learn more")
                            .font(.headline)
                        Link("Open reference", destination: info.learnMoreURL)
                            .font(.footnote.weight(.semibold))
                            .foregroundColor(AppTheme.accent)
                    }
                }

                Spacer()
            }
            .padding(20)
            .navigationTitle("Security Context")
            .navigationBarTitleDisplayMode(.inline)
            .presentationDetents([.medium, .large])
        }
    }
}

enum SecurityInfo: String, Identifiable {
    case login
    case transfer
    case addBeneficiary
    case viewCard
    case changePassword

    var id: String { rawValue }

    var title: String {
        switch self {
        case .login: return "Login security validation"
        case .transfer: return "Transfer security validation"
        case .addBeneficiary: return "Beneficiary security validation"
        case .viewCard: return "Card access security validation"
        case .changePassword: return "Password change security validation"
        }
    }

    var summary: String {
        switch self {
        case .login:
            return "Validates device integrity, debugger/hooking, and attestation before allowing login. Risky sessions can require step-up or be blocked."
        case .transfer:
            return "Validates MITM/proxy signals, risk score, and attestation before sending money. High risk or interception typically denies or step-ups."
        case .addBeneficiary:
            return "Validates attestation before adding a beneficiary. In this demo, attestation FAIL triggers STEP_UP to prevent account takeover."
        case .viewCard:
            return "Validates hooking/debugger and risk score. High risk can degrade the card view (masked)."
        case .changePassword:
            return "Requires strong signals and often step-up. High risk or policy rules can deny to prevent takeover."
        }
    }

    var learnMoreURL: URL {
        switch self {
        case .login:
            return URL(string: "https://mas.owasp.org/MASVS/controls/MASVS-AUTH-1/")!
        case .transfer:
            return URL(string: "https://owasp.org/API-Security/editions/2023/en/0xa1-broken-object-level-authorization/")!
        case .addBeneficiary:
            return URL(string: "https://owasp.org/API-Security/editions/2023/en/0xa3-broken-object-property-level-authorization/")!
        case .viewCard:
            return URL(string: "https://mas.owasp.org/MASTG/techniques/generic/MASTG-TECH-0051/")!
        case .changePassword:
            return URL(string: "https://mas.owasp.org/MASVS/controls/MASVS-AUTH-3/")!
        }
    }
}

struct SecurityLabView: View {
    @ObservedObject var state: DemoState

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(alignment: .leading, spacing: 16) {
                    SectionCard {
                        VStack(alignment: .leading, spacing: 8) {
                            Text("What this screen does")
                                .font(.headline)
                            Text("This is a simulator. It doesn’t block anything by itself. It changes the signals and risk score that the SDK sends into the policy engine.")
                                .font(.footnote)
                                .foregroundColor(AppTheme.muted)
                            Text("Policies decide the result: ALLOW, STEP_UP, DEGRADE, or DENY.")
                                .font(.footnote)
                                .foregroundColor(AppTheme.muted)
                        }
                    }

                    SectionCard {
                        VStack(alignment: .leading, spacing: 8) {
                            Text("Runtime signals")
                                .font(.subheadline.weight(.semibold))
                            Toggle("Jailbreak", isOn: $state.jailbreak)
                                .controlSize(.small)
                                .toggleStyle(SwitchToggleStyle(tint: AppTheme.accent))
                            Toggle("Root", isOn: $state.root)
                                .controlSize(.small)
                                .toggleStyle(SwitchToggleStyle(tint: AppTheme.accent))
                            Toggle("Debugger", isOn: $state.debugger)
                                .controlSize(.small)
                                .toggleStyle(SwitchToggleStyle(tint: AppTheme.accent))
                            Toggle("Hooking", isOn: $state.hooking)
                                .controlSize(.small)
                                .toggleStyle(SwitchToggleStyle(tint: AppTheme.accent))
                            Toggle("Proxy/MITM", isOn: $state.proxyDetected)
                                .controlSize(.small)
                                .toggleStyle(SwitchToggleStyle(tint: AppTheme.accent))
                        }
                    }

                    SectionCard {
                        VStack(alignment: .leading, spacing: 8) {
                            Text("Attestation")
                                .font(.subheadline.weight(.semibold))
                            Picker("Status", selection: $state.attestationChoice) {
                                ForEach(AttestationChoice.allCases) { option in
                                    Text(option.rawValue.uppercased()).tag(option)
                                }
                            }
                            .pickerStyle(.segmented)
                        }
                    }

                    SectionCard {
                        VStack(alignment: .leading, spacing: 8) {
                            Text("Server Risk Score (simulated)")
                                .font(.subheadline.weight(.semibold))
                            Slider(value: $state.riskScore, in: 0...100, step: 1)
                                .tint(AppTheme.accent)
                            Text("Risk Score: \(Int(state.riskScore))")
                                .font(.footnote)
                                .foregroundColor(AppTheme.muted)
                            Text("This simulates the server risk score used by policies (0–100). Higher values may trigger step-up or degrade.")
                                .font(.caption)
                                .foregroundColor(AppTheme.muted)
                        }
                    }
                }
                .padding(20)
            }
            .navigationTitle("Security Simulator")
            .navigationBarTitleDisplayMode(.inline)
        }
    }
}

struct StepUpSheet: View {
    let actionLabel: String
    let onConfirm: () -> Void
    let onCancel: () -> Void

    var body: some View {
        NavigationStack {
            VStack(spacing: 16) {
                Text("Additional verification required")
                    .font(.headline)
                Text("Action: \(actionLabel)")
                    .font(.subheadline)
                    .foregroundColor(AppTheme.muted)

                Button("Verify with Biometrics") {
                    onConfirm()
                }
                .buttonStyle(PrimaryButtonStyle())

                Button("Cancel") {
                    onCancel()
                }
                .buttonStyle(SecondaryButtonStyle())
            }
            .padding(24)
            .navigationTitle("Step-Up")
            .navigationBarTitleDisplayMode(.inline)
        }
    }
}

struct SectionCard<Content: View>: View {
    let content: Content

    init(@ViewBuilder content: () -> Content) {
        self.content = content()
    }

    var body: some View {
        content
            .padding(12)
            .frame(maxWidth: .infinity, alignment: .leading)
            .background(AppTheme.card)
            .overlay(
                RoundedRectangle(cornerRadius: 16)
                    .stroke(AppTheme.cardBorder, lineWidth: 1)
            )
            .cornerRadius(16)
    }
}

struct Pill: View {
    let text: String

    var body: some View {
        Text(text)
            .font(.caption2)
            .padding(.horizontal, 10)
            .padding(.vertical, 4)
            .background(AppTheme.pill)
            .cornerRadius(12)
    }
}

struct AppBackground: View {
    var body: some View {
        LinearGradient(
            colors: [AppTheme.backgroundTop, AppTheme.backgroundBottom],
            startPoint: .topLeading,
            endPoint: .bottomTrailing
        )
        .ignoresSafeArea()
        .overlay(
            ZStack {
                Circle()
                    .fill(AppTheme.accent.opacity(0.15))
                    .frame(width: 260, height: 260)
                    .offset(x: 120, y: -160)
                Circle()
                    .fill(AppTheme.accent.opacity(0.08))
                    .frame(width: 320, height: 320)
                    .offset(x: -160, y: 260)
            }
        )
    }
}

enum AppTheme {
    static let backgroundTop = Color(red: 0.98, green: 0.97, blue: 0.95)
    static let backgroundBottom = Color(red: 0.93, green: 0.96, blue: 0.95)
    static let card = Color.white.opacity(0.9)
    static let cardBorder = Color.black.opacity(0.04)
    static let muted = Color.black.opacity(0.55)
    static let pill = Color.black.opacity(0.06)
    static let accent = Color(red: 0.18, green: 0.46, blue: 0.40)
}

struct PrimaryButtonStyle: ButtonStyle {
    func makeBody(configuration: Configuration) -> some View {
        configuration.label
            .font(.system(.subheadline, design: .rounded).weight(.semibold))
            .padding(.vertical, 8)
            .padding(.horizontal, 14)
            .background(AppTheme.accent)
            .foregroundColor(.white)
            .cornerRadius(10)
            .opacity(configuration.isPressed ? 0.85 : 1)
    }
}

struct SecondaryButtonStyle: ButtonStyle {
    func makeBody(configuration: Configuration) -> some View {
        configuration.label
            .font(.system(.subheadline, design: .rounded).weight(.medium))
            .padding(.vertical, 6)
            .padding(.horizontal, 12)
            .background(Color.white.opacity(0.7))
            .overlay(
                RoundedRectangle(cornerRadius: 12)
                    .stroke(AppTheme.cardBorder, lineWidth: 1)
            )
            .cornerRadius(10)
            .opacity(configuration.isPressed ? 0.85 : 1)
    }
}

struct SettingsRow: View {
    let title: String
    let subtitle: String
    let systemImage: String
    let action: (() -> Void)?

    init(title: String, subtitle: String, systemImage: String, action: (() -> Void)? = nil) {
        self.title = title
        self.subtitle = subtitle
        self.systemImage = systemImage
        self.action = action
    }

    var body: some View {
        if let action {
            Button(action: action) {
                rowContent
            }
            .buttonStyle(.plain)
        } else {
            rowContent
        }
    }

    private var rowContent: some View {
        HStack(spacing: 12) {
            Image(systemName: systemImage)
                .foregroundColor(AppTheme.accent)
            VStack(alignment: .leading, spacing: 2) {
                Text(title)
                    .font(.subheadline.weight(.semibold))
                Text(subtitle)
                    .font(.caption)
                    .foregroundColor(AppTheme.muted)
            }
            Spacer()
            Image(systemName: "chevron.right")
                .font(.caption)
                .foregroundColor(AppTheme.muted)
        }
        .padding(.vertical, 4)
    }
}
