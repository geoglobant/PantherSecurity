package com.panthersecurity.sample.presentation

import com.panthersecurity.sample.domain.ActionContext
import com.panthersecurity.sample.domain.FetchPolicyUseCase
import com.panthersecurity.sample.domain.IntegritySignals
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

    suspend fun loadPolicy() {
        status = try {
            fetchPolicy.execute()
            "policy loaded"
        } catch (ex: Exception) {
            "error: ${ex.message}"
        }
    }

    suspend fun sendLoginTelemetry() {
        val event = TelemetryEvent(
            eventId = UUID.randomUUID().toString(),
            appId = "fintech.mobile",
            appVersion = "1.0.0",
            env = "local",
            action = ActionContext("login", null),
            signals = IntegritySignals(
                jailbreak = false,
                root = false,
                debugger = false,
                hooking = false,
                proxyDetected = false
            ),
            timestamp = Instant.now().toString()
        )

        status = try {
            sendTelemetry.execute(event)
            "telemetry sent"
        } catch (ex: Exception) {
            "telemetry error: ${ex.message}"
        }
    }
}
