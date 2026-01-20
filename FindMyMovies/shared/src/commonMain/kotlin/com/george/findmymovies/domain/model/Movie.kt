package com.george.findmymovies.domain.model

data class Movie(
    val imdbId: String,
    val title: String,
    val year: String?,
    val posterUrl: String?
)

data class MovieDetails(
    val imdbId: String,
    val title: String,
    val year: String?,
    val rated: String?,
    val released: String?,
    val runtime: String?,
    val genre: String?,
    val director: String?,
    val writer: String?,
    val actors: String?,
    val plot: String?,
    val language: String?,
    val country: String?,
    val awards: String?,
    val posterUrl: String?,
    val imdbRating: String?,
    val imdbVotes: String?,
    val ratings: List<Rating>
) {
    data class Rating(
        val source: String,
        val value: String
    )
}
