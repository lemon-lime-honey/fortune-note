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

    suspend fun updateProfile(id: String, request: UpdateProfileRequest): Result<ProfileDto> = withContext(Dispatchers.IO) {
        runCatching {
            val reqJson = json.encodeToString(UpdateProfileRequest.serializer(), request)
            val resJson = client.updateProfile(id, reqJson)
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

    suspend fun calculateProfileChartWithHouse(id: String, houseSystem: String?): Result<FortuneChartOutput> = withContext(Dispatchers.IO) {
        runCatching {
            val resJson = client.calculateProfileChartWithHouse(id, houseSystem)
            json.decodeFromString(FortuneChartOutput.serializer(), resJson)
        }
    }

    suspend fun calculateProfileSaju(id: String): Result<SajuCalculationOutputDto> = withContext(Dispatchers.IO) {
        runCatching {
            val resJson = client.calculateProfileSaju(id)
            json.decodeFromString(SajuCalculationOutputDto.serializer(), resJson)
        }
    }

    suspend fun calculateProfileWestern(id: String, houseSystem: String?): Result<WesternCalculationOutputDto> = withContext(Dispatchers.IO) {
        runCatching {
            val resJson = client.calculateProfileWestern(id, houseSystem)
            json.decodeFromString(WesternCalculationOutputDto.serializer(), resJson)
        }
    }

    suspend fun calculateProfileVedic(id: String): Result<VedicCalculationOutputDto> = withContext(Dispatchers.IO) {
        runCatching {
            val resJson = client.calculateProfileVedic(id)
            json.decodeFromString(VedicCalculationOutputDto.serializer(), resJson)
        }
    }

    suspend fun calculateHoraryChartWithHouse(id: String, houseSystem: String?): Result<FortuneChartOutput> = withContext(Dispatchers.IO) {
        runCatching {
            val resJson = client.calculateHoraryChartWithHouse(id, houseSystem)
            json.decodeFromString(FortuneChartOutput.serializer(), resJson)
        }
    }

    suspend fun calculateProfilePredictiveSuite(
        id: String,
        targetYear: Int,
        targetMonth: Int,
        targetDay: Int
    ): Result<PredictiveSuiteData> = withContext(Dispatchers.IO) {
        runCatching {
            val resJson = client.calculateProfilePredictiveSuite(
                id,
                targetYear,
                targetMonth.toUInt(),
                targetDay.toUInt()
            )
            json.decodeFromString(PredictiveSuiteData.serializer(), resJson)
        }
    }

    suspend fun exportProfileSajuMarkdown(id: String): Result<String> = withContext(Dispatchers.IO) {
        runCatching {
            client.exportProfileSajuMarkdown(id)
        }
    }

    suspend fun exportProfileWesternMarkdown(id: String): Result<String> = withContext(Dispatchers.IO) {
        runCatching {
            client.exportProfileWesternMarkdown(id)
        }
    }

    suspend fun exportProfileVedicMarkdown(id: String): Result<String> = withContext(Dispatchers.IO) {
        runCatching {
            client.exportProfileVedicMarkdown(id)
        }
    }

    suspend fun createHorary(request: CreateHoraryRequest): Result<HoraryDto> = withContext(Dispatchers.IO) {
        runCatching {
            val reqJson = json.encodeToString(CreateHoraryRequest.serializer(), request)
            val resJson = client.createHorary(reqJson)
            json.decodeFromString(HoraryDto.serializer(), resJson)
        }
    }

    suspend fun updateHorary(id: String, request: UpdateHoraryRequest): Result<HoraryDto> = withContext(Dispatchers.IO) {
        runCatching {
            val reqJson = json.encodeToString(UpdateHoraryRequest.serializer(), request)
            val resJson = client.updateHorary(id, reqJson)
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


    suspend fun exportHoraryWesternMarkdown(id: String): Result<String> = withContext(Dispatchers.IO) {
        runCatching {
            client.exportHoraryWesternMarkdown(id)
        }
    }

    suspend fun exportPrashnaVedicMarkdown(id: String): Result<String> = withContext(Dispatchers.IO) {
        runCatching {
            client.exportPrashnaVedicMarkdown(id)
        }
    }

    suspend fun exportBackupJson(): Result<String> = withContext(Dispatchers.IO) {
        runCatching {
            client.exportBackupJson()
        }
    }

    suspend fun importBackupJson(jsonStr: String, replaceExisting: Boolean): Result<ImportSummaryDto> = withContext(Dispatchers.IO) {
        runCatching {
            val resJson = client.importBackupJson(jsonStr, replaceExisting)
            json.decodeFromString(ImportSummaryDto.serializer(), resJson)
        }
    }
}
