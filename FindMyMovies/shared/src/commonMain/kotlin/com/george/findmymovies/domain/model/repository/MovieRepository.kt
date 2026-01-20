package com.george.findmymovies.domain.repository

import com.george.findmymovies.domain.model.Movie
import com.george.findmymovies.domain.model.MovieDetails

interface MovieRepository {
    /**
     * OMDb search: usa query + paginação (page começa em 1).
     */
    suspend fun searchMovies(query: String, page: Int = 1): SearchPage

    /**
     * OMDb details: busca por imdbId (ex.: tt0133093).
     */
    suspend fun getMovieDetails(imdbId: String, plot: Plot = Plot.Full): MovieDetails

    data class SearchPage(
        val items: List<Movie>,
        val page: Int,
        val totalResults: Int
    )

    enum class Plot { Short, Full }
}
