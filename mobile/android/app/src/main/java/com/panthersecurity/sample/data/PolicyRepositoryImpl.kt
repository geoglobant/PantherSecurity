package com.panthersecurity.sample.data

import com.panthersecurity.sample.domain.Decision
import com.panthersecurity.sample.domain.Policy
import com.panthersecurity.sample.domain.PolicyConditions
import com.panthersecurity.sample.domain.PolicyRepository
import com.panthersecurity.sample.domain.PolicyRule
import com.panthersecurity.sample.sdk.PantherSecuritySdk

class PolicyRepositoryImpl(private val sdk: PantherSecuritySdk) : PolicyRepository {
    override suspend fun fetchPolicy(): Policy {
        val response = sdk.fetchPolicy()
        return Policy(
            policyId = response.policyId,
            appId = response.appId,
            appVersion = response.appVersion,
            env = response.env,
            rules = response.rules.map { rule ->
                PolicyRule(
                    action = rule.action,
                    decision = runCatching { Decision.valueOf(rule.decision) }.getOrDefault(Decision.DENY),
                    conditions = PolicyConditions(
                        attestation = rule.conditions?.attestation,
                        debugger = rule.conditions?.debugger,
                        hooking = rule.conditions?.hooking,
                        proxyDetected = rule.conditions?.proxyDetected,
                        appVersion = rule.conditions?.appVersion,
                        riskScoreGte = rule.conditions?.riskScoreGte
                    )
                )
            },
            issuedAt = response.issuedAt
        )
    }
}
