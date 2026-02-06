package com.panthersecurity.sample.domain

data class Policy(
    val policyId: String,
    val appId: String,
    val appVersion: String,
    val env: String,
    val rules: List<PolicyRule>,
    val issuedAt: String
)

data class PolicyRule(
    val action: String,
    val decision: Decision,
    val conditions: PolicyConditions
)

data class PolicyConditions(
    val attestation: String?,
    val debugger: Boolean?,
    val hooking: Boolean?,
    val proxyDetected: Boolean?,
    val appVersion: String?,
    val riskScoreGte: Int?
)

enum class Decision { ALLOW, STEP_UP, DEGRADE, DENY }
