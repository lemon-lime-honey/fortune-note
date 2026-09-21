package com.fortunexample.note

import android.app.Application
import com.fortunexample.note.data.FortuneRepository
import uniffi.fortune_backend.FortuneClient
import java.io.File

class FortuneApp : Application() {
    companion object {
        lateinit var instance: FortuneApp
            private set
        lateinit var repository: FortuneRepository
            private set
    }

    override fun onCreate() {
        super.onCreate()
        instance = this
        reloadRepository()
    }

    fun getDbFile(): File = File(filesDir, "fortune_note.db")

    fun reloadRepository() {
        val dbFile = getDbFile()
        val client = FortuneClient(dbFile.absolutePath)
        repository = FortuneRepository(client)
    }
}
