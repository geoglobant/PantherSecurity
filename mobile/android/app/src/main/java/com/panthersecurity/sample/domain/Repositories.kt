package com.panthersecurity.sample.domain

interface PolicyRepository {
    suspend fun fetchPolicy(): Policy
}

interface TelemetryRepository {
    suspend fun send(event: TelemetryEvent)
}
