package com.george.findmymovies.core.error

import io.ktor.client.plugins.*
import io.ktor.http.*
/**
 * Erros de domínio (independentes de plataforma/UI).
 */
sealed interface DomainError {
    data object Network : DomainError
    data object Timeout : DomainError
    data object RateLimited : DomainError
    data object Unauthorized : DomainError
    data object NotFound : DomainError
    data class Remote(val message: String) : DomainError
    data object Unknown : DomainError
}

fun DomainError.toUserMessage(): String = when (this) {
    DomainError.Network -> "Network error. Please check your connection and try again."
    DomainError.Timeout -> "Request timed out. Please try again."
    DomainError.RateLimited -> "Too many requests. Please try again in a moment."
    DomainError.Unauthorized -> "Unauthorized request. Please verify your API key."
    DomainError.NotFound -> "Content not found."
    is DomainError.Remote -> this.message.ifBlank { "Remote service error." }
    DomainError.Unknown -> "Unexpected error."
}

/**
 * Mapeamento conservador de falhas comuns do Ktor para DomainError.
 */
fun Throwable.toDomainError(): DomainError = when (this) {
    is HttpRequestTimeoutException,
    is ConnectTimeoutException,
    is SocketTimeoutException -> DomainError.Timeout

    is ResponseException -> when (response.status) {
        HttpStatusCode.Unauthorized,
        HttpStatusCode.Forbidden -> DomainError.Unauthorized
        HttpStatusCode.NotFound -> DomainError.NotFound
        HttpStatusCode.TooManyRequests -> DomainError.RateLimited
        else -> DomainError.Remote("HTTP ${response.status.value}")
    }

    // Falhas de rede em geral (DNS, sem conexão, etc.) podem vir como exceções distintas por engine/plataforma.
    // Mantemos um fallback simples.
    else -> DomainError.Unknown
}
