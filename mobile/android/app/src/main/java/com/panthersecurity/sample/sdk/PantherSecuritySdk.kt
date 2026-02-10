package com.panthersecurity.sample.sdk

class PantherSecuritySdk(private val config: PantherSecurityConfig) {
    suspend fun fetchPolicy(): PantherSecurityPolicy {
        return PantherSecurityPolicy(
            policyId = "policy_local",
            appId = config.appId,
            appVersion = config.appVersion,
            env = config.env,
            rules = listOf(
                PantherSecurityPolicyRule(
                    action = "login",
                    decision = "STEP_UP",
                    conditions = PantherSecurityPolicyConditions(
                        attestation = null,
                        debugger = true,
                        hooking = null,
                        proxyDetected = null,
                        appVersion = null,
                        riskScoreGte = null
                    )
                ),
                PantherSecurityPolicyRule(
                    action = "transfer",
                    decision = "DENY",
                    conditions = PantherSecurityPolicyConditions(
                        attestation = null,
                        debugger = null,
                        hooking = null,
                        proxyDetected = true,
                        appVersion = null,
                        riskScoreGte = null
                    )
                ),
                PantherSecurityPolicyRule(
                    action = "transfer",
                    decision = "STEP_UP",
                    conditions = PantherSecurityPolicyConditions(
                        attestation = null,
                        debugger = null,
                        hooking = null,
                        proxyDetected = null,
                        appVersion = null,
                        riskScoreGte = 70
                    )
                ),
                PantherSecurityPolicyRule(
                    action = "view_card",
                    decision = "DEGRADE",
                    conditions = PantherSecurityPolicyConditions(
                        attestation = null,
                        debugger = null,
                        hooking = true,
                        proxyDetected = null,
                        appVersion = null,
                        riskScoreGte = null
                    )
                ),
                PantherSecurityPolicyRule(
                    action = "add_beneficiary",
                    decision = "STEP_UP",
                    conditions = PantherSecurityPolicyConditions(
                        attestation = "fail",
                        debugger = null,
                        hooking = null,
                        proxyDetected = null,
                        appVersion = null,
                        riskScoreGte = null
                    )
                ),
                PantherSecurityPolicyRule(
                    action = "change_password",
                    decision = "DENY",
                    conditions = PantherSecurityPolicyConditions(
                        attestation = null,
                        debugger = null,
                        hooking = null,
                        proxyDetected = null,
                        appVersion = "1.0.0",
                        riskScoreGte = null
                    )
                )
            ),
            signature = "stub",
            issuedAt = "2026-02-06T00:00:00Z"
        )
    }

    suspend fun sendTelemetry(event: PantherSecurityTelemetry) {
        _ = event
    }
}

data class PantherSecurityConfig(
    val baseUrl: String,
    val appId: String,
    val appVersion: String,
    val env: String,
    val apiToken: String?
)

data class PantherSecurityPolicy(
    val policyId: String,
    val appId: String,
    val appVersion: String,
    val env: String,
    val rules: List<PantherSecurityPolicyRule>,
    val signature: String,
    val issuedAt: String
)

data class PantherSecurityPolicyRule(
    val action: String,
    val decision: String,
    val conditions: PantherSecurityPolicyConditions?
)

data class PantherSecurityPolicyConditions(
    val attestation: String?,
    val debugger: Boolean?,
    val hooking: Boolean?,
    val proxyDetected: Boolean?,
    val appVersion: String?,
    val riskScoreGte: Int?
)

data class PantherSecurityTelemetry(
    val eventId: String,
    val appId: String,
    val appVersion: String,
    val env: String,
    val action: PantherSecurityActionContext,
    val signals: PantherSecurityIntegritySignals,
    val timestamp: String
)

data class PantherSecurityIntegritySignals(
    val jailbreak: Boolean,
    val root: Boolean,
    val debugger: Boolean,
    val hooking: Boolean,
    val proxyDetected: Boolean
)

data class PantherSecurityActionContext(
    val name: String,
    val context: String?
)
