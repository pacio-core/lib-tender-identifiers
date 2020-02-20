package com.pacio.rustylibrary

external fun hello(to: String): String
external fun helloDirect(to: String): String

fun loadRustyLib() {
    System.loadLibrary("rustylib")
}