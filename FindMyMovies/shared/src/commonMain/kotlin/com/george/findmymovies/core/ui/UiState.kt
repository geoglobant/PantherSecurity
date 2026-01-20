package com.george.findmymovies.core.ui

/**
 * Estado genérico de UI para telas "state-driven".
 * Use T como um UI model (ex.: MovieSearchData) ou domínio (ex.: List<Movie>).
 */
sealed interface UiState<out T> {
    data object Idle : UiState<Nothing>
    data object Loading : UiState<Nothing>
    data class Content<T>(val data: T) : UiState<T>
    data object Empty : UiState<Nothing>
    data class Error(val message: String) : UiState<Nothing>
}
