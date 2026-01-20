package com.george.findmovies

interface Platform {
    val name: String
}

expect fun getPlatform(): Platform