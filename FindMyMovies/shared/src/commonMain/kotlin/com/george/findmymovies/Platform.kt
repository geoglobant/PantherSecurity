package com.george.findmymovies

interface Platform {
    val name: String
}

expect fun getPlatform(): Platform