package com.panthersecurity.sample.presentation

import com.panthersecurity.sample.domain.ActionContext
import com.panthersecurity.sample.domain.Decision
import com.panthersecurity.sample.domain.FetchPolicyUseCase
import com.panthersecurity.sample.domain.IntegritySignals
import com.panthersecurity.sample.domain.Policy
import com.panthersecurity.sample.domain.PolicyRule
import com.panthersecurity.sample.domain.SendTelemetryUseCase
import com.panthersecurity.sample.domain.TelemetryEvent
import java.time.Instant
import java.util.UUID

class MainViewModel(
    private val fetchPolicy: FetchPolicyUseCase,
    private val sendTelemetry: SendTelemetryUseCase
) {
    var status: String = "idle"
        private set

    var policy: Policy? = null
        private set

    fun evaluateAction(
        action: String,
        signals: IntegritySignals,
        attestation: String?,
        riskScore: Int
    ): DecisionResult {
        val policy = policy ?: return DecisionResult(
            decision = Decision.DENY,
            validations = "policy not loaded"
        )

        val decision = evaluatePolicy(policy, action, signals, attestation, riskScore)
        val matched = policy.rules.firstOrNull { it.action == action }
        val summary = buildSummary(action, signals, attestation, riskScore, matched)

        return DecisionResult(decision, summary)
    }

    suspend fun loadPolicy() {
        status = try {
            policy = fetchPolicy.execute()
            "policy loaded"
        } catch (ex: Exception) {
            "error: ${ex.message}"
        }
    }

    suspend fun sendActionTelemetry(action: String, signals: IntegritySignals) {
        val event = TelemetryEvent(
            eventId = UUID.randomUUID().toString(),
            appId = policy?.appId ?: "fintech.mobile",
            appVersion = policy?.appVersion ?: "1.0.0",
            env = policy?.env ?: "local",
            action = ActionContext(action, null),
            signals = signals,
            timestamp = Instant.now().toString()
        )

        status = try {
            sendTelemetry.execute(event)
            "telemetry sent"
        } catch (ex: Exception) {
            "telemetry error: ${ex.message}"
        }
    }

    private fun evaluatePolicy(
        policy: Policy,
        action: String,
        signals: IntegritySignals,
        attestation: String?,
        riskScore: Int
    ): Decision {
        for (rule in policy.rules) {
            if (rule.action != action) continue
            if (!matches(rule, signals, attestation, riskScore, policy.appVersion)) continue
            return rule.decision
        }
        return Decision.ALLOW
    }

    private fun matches(
        rule: PolicyRule,
        signals: IntegritySignals,
        attestation: String?,
        riskScore: Int,
        appVersion: String
    ): Boolean {
        val conditions = rule.conditions

        if (conditions.attestation != null && conditions.attestation != attestation) {
            return false
        }
        if (conditions.debugger != null && conditions.debugger != signals.debugger) {
            return false
        }
        if (conditions.hooking != null && conditions.hooking != signals.hooking) {
            return false
        }
        if (conditions.proxyDetected != null && conditions.proxyDetected != signals.proxyDetected) {
            return false
        }
        if (conditions.appVersion != null && conditions.appVersion != appVersion) {
            return false
        }
        if (conditions.riskScoreGte != null && riskScore < conditions.riskScoreGte) {
            return false
        }

        return true
    }

    private fun buildSummary(
        action: String,
        signals: IntegritySignals,
        attestation: String?,
        riskScore: Int,
        matchedRule: PolicyRule?
    ): String {
        val ruleLabel = matchedRule?.decision?.name ?: "no matching rule"
        return "action=$action, jailbreak=${signals.jailbreak}, root=${signals.root}, debugger=${signals.debugger}, hooking=${signals.hooking}, proxy=${signals.proxyDetected}, attestation=${attestation ?: "none"}, riskScore=$riskScore, rule=$ruleLabel"
    }
}

data class DecisionResult(
    val decision: Decision,
    val validations: String
)
