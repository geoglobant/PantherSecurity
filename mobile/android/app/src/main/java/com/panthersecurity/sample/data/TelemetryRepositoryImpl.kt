package com.panthersecurity.sample.data

import com.panthersecurity.sample.domain.TelemetryEvent
import com.panthersecurity.sample.domain.TelemetryRepository
import com.panthersecurity.sample.sdk.PantherSecurityActionContext
import com.panthersecurity.sample.sdk.PantherSecurityIntegritySignals
import com.panthersecurity.sample.sdk.PantherSecuritySdk
import com.panthersecurity.sample.sdk.PantherSecurityTelemetry

class TelemetryRepositoryImpl(private val sdk: PantherSecuritySdk) : TelemetryRepository {
    override suspend fun send(event: TelemetryEvent) {
        val request = PantherSecurityTelemetry(
            eventId = event.eventId,
            appId = event.appId,
            appVersion = event.appVersion,
            env = event.env,
            action = PantherSecurityActionContext(event.action.name, event.action.context),
            signals = PantherSecurityIntegritySignals(
                jailbreak = event.signals.jailbreak,
                root = event.signals.root,
                debugger = event.signals.debugger,
                hooking = event.signals.hooking,
                proxyDetected = event.signals.proxyDetected
            ),
            timestamp = event.timestamp
        )
        sdk.sendTelemetry(request)
    }
}
