package com.panthersecurity.sample.domain

class FetchPolicyUseCase(private val repository: PolicyRepository) {
    suspend fun execute(): Policy = repository.fetchPolicy()
}

class SendTelemetryUseCase(private val repository: TelemetryRepository) {
    suspend fun execute(event: TelemetryEvent) = repository.send(event)
}
