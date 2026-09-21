package com.fortunexample.note

import android.app.Application
import com.fortunexample.note.data.FortuneRepository
import uniffi.fortune_backend.FortuneClient
import java.io.File

class FortuneApp : Application() {
    companion object {
        lateinit var repository: FortuneRepository
            private set
    }

    override fun onCreate() {
        super.onCreate()
        val dbFile = File(filesDir, "fortune_note.db")
        val client = FortuneClient(dbFile.absolutePath)
        repository = FortuneRepository(client)
    }
}
