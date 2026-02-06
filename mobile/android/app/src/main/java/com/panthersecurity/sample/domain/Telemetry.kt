package com.panthersecurity.sample.domain

data class IntegritySignals(
    val jailbreak: Boolean,
    val root: Boolean,
    val debugger: Boolean,
    val hooking: Boolean,
    val proxyDetected: Boolean
)

data class ActionContext(
    val name: String,
    val context: String?
)

data class TelemetryEvent(
    val eventId: String,
    val appId: String,
    val appVersion: String,
    val env: String,
    val action: ActionContext,
    val signals: IntegritySignals,
    val timestamp: String
)
