package com.george.findmymovies.core.ui

sealed interface UiState<out T> {
    data object  idle : UiState<Nothing>
    data object  loading:UiState<Nothing>
    data class  content<T>(val data :T) : UiState<T>
    data object empty:UiState<Nothing>
    data class  Error(val message:String):UiState<Nothing>
}