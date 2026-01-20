package com.george.findmymovies.data.remote

import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.request.*
import io.ktor.http.*
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

/**
 * OMDb API client (REST).
 * Base URL: https://www.omdbapi.com/
 * Parâmetros comuns:
 *  - apikey
 *  - s (search)
 *  - page
 *  - i (imdbId)
 *  - plot=short|full
 */
class OmdbApi(
    private val httpClient: HttpClient,
    private val baseUrl: String = "https://www.omdbapi.com/"
) {
    suspend fun searchMovies(apiKey: String, query: String, page: Int): SearchResponseDto {
        return httpClient.get(baseUrl) {
            url {
                parameters.append("apikey", apiKey)
                parameters.append("s", query)
                parameters.append("type", "movie")
                parameters.append("page", page.toString())
            }
        }.body()
    }

    suspend fun movieDetails(apiKey: String, imdbId: String, plot: String): MovieDetailsDto {
        return httpClient.get(baseUrl) {
            url {
                parameters.append("apikey", apiKey)
                parameters.append("i", imdbId)
                parameters.append("plot", plot) // short|full
            }
        }.body()
    }
}

/**
 * DTOs OMDb
 * Observação: OMDb frequentemente retorna "Response":"False" e "Error":"Movie not found!"
 */
@Serializable
data class SearchResponseDto(
    @SerialName("Search") val search: List<MovieItemDto> = emptyList(),
    @SerialName("totalResults") val totalResults: String? = null,
    @SerialName("Response") val response: String,
    @SerialName("Error") val error: String? = null
)

@Serializable
data class MovieItemDto(
    @SerialName("Title") val title: String? = null,
    @SerialName("Year") val year: String? = null,
    @SerialName("imdbID") val imdbId: String? = null,
    @SerialName("Type") val type: String? = null,
    @SerialName("Poster") val poster: String? = null
)

@Serializable
data class MovieDetailsDto(
    @SerialName("Title") val title: String? = null,
    @SerialName("Year") val year: String? = null,
    @SerialName("Rated") val rated: String? = null,
    @SerialName("Released") val released: String? = null,
    @SerialName("Runtime") val runtime: String? = null,
    @SerialName("Genre") val genre: String? = null,
    @SerialName("Director") val director: String? = null,
    @SerialName("Writer") val writer: String? = null,
    @SerialName("Actors") val actors: String? = null,
    @SerialName("Plot") val plot: String? = null,
    @SerialName("Language") val language: String? = null,
    @SerialName("Country") val country: String? = null,
    @SerialName("Awards") val awards: String? = null,
    @SerialName("Poster") val poster: String? = null,
    @SerialName("imdbRating") val imdbRating: String? = null,
    @SerialName("imdbVotes") val imdbVotes: String? = null,
    @SerialName("imdbID") val imdbId: String? = null,
    @SerialName("Ratings") val ratings: List<RatingDto> = emptyList(),
    @SerialName("Response") val response: String,
    @SerialName("Error") val error: String? = null
)

@Serializable
data class RatingDto(
    @SerialName("Source") val source: String? = null,
    @SerialName("Value") val value: String? = null
)
