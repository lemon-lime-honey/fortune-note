package com.fortunexample.note.data

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import kotlinx.serialization.json.Json
import uniffi.fortune_backend.FortuneClient

class FortuneRepository(
    private val client: FortuneClient
) {
    private val json = Json {
        ignoreUnknownKeys = true
        isLenient = true
        encodeDefaults = true
    }

    suspend fun createProfile(request: CreateProfileRequest): Result<ProfileDto> = withContext(Dispatchers.IO) {
        runCatching {
            val reqJson = json.encodeToString(CreateProfileRequest.serializer(), request)
            val resJson = client.createProfile(reqJson)
            json.decodeFromString(ProfileDto.serializer(), resJson)
        }
    }

    suspend fun getProfile(id: String): Result<ProfileDto?> = withContext(Dispatchers.IO) {
        runCatching {
            val resJson = client.getProfile(id) ?: return@runCatching null
            json.decodeFromString(ProfileDto.serializer(), resJson)
        }
    }

    suspend fun listProfiles(search: String? = null): Result<List<ProfileDto>> = withContext(Dispatchers.IO) {
        runCatching {
            val resJson = client.listProfiles(search)
            json.decodeFromString<List<ProfileDto>>(resJson)
        }
    }

    suspend fun deleteProfile(id: String): Result<Boolean> = withContext(Dispatchers.IO) {
        runCatching {
            client.deleteProfile(id)
        }
    }

    suspend fun calculateProfileChart(id: String): Result<FortuneChartOutput> = withContext(Dispatchers.IO) {
        runCatching {
            val resJson = client.calculateProfileChart(id)
            json.decodeFromString(FortuneChartOutput.serializer(), resJson)
        }
    }

    suspend fun exportProfileMarkdown(id: String): Result<String> = withContext(Dispatchers.IO) {
        runCatching {
            client.exportProfileMarkdown(id)
        }
    }

    suspend fun createHorary(request: CreateHoraryRequest): Result<HoraryDto> = withContext(Dispatchers.IO) {
        runCatching {
            val reqJson = json.encodeToString(CreateHoraryRequest.serializer(), request)
            val resJson = client.createHorary(reqJson)
            json.decodeFromString(HoraryDto.serializer(), resJson)
        }
    }

    suspend fun getHorary(id: String): Result<HoraryDto?> = withContext(Dispatchers.IO) {
        runCatching {
            val resJson = client.getHorary(id) ?: return@runCatching null
            json.decodeFromString(HoraryDto.serializer(), resJson)
        }
    }

    suspend fun listHorary(category: String? = null, querentId: String? = null): Result<List<HoraryDto>> = withContext(Dispatchers.IO) {
        runCatching {
            val resJson = client.listHorary(category, querentId)
            json.decodeFromString<List<HoraryDto>>(resJson)
        }
    }

    suspend fun deleteHorary(id: String): Result<Boolean> = withContext(Dispatchers.IO) {
        runCatching {
            client.deleteHorary(id)
        }
    }

    suspend fun calculateHoraryChart(id: String): Result<FortuneChartOutput> = withContext(Dispatchers.IO) {
        runCatching {
            val resJson = client.calculateHoraryChart(id)
            json.decodeFromString(FortuneChartOutput.serializer(), resJson)
        }
    }

    suspend fun exportHoraryMarkdown(id: String): Result<String> = withContext(Dispatchers.IO) {
        runCatching {
            client.exportHoraryMarkdown(id)
        }
    }
}
