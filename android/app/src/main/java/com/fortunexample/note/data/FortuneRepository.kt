package com.fortunexample.note.data

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import kotlinx.serialization.json.Json
import uniffi.fortune_backend.FortuneClient

interface ProfileDataSource {
    suspend fun createProfile(request: CreateProfileRequest): Result<ProfileDto>
    suspend fun updateProfile(id: String, request: UpdateProfileRequest): Result<ProfileDto>
    suspend fun listProfiles(search: String? = null): Result<List<ProfileDto>>
    suspend fun deleteProfile(id: String): Result<Boolean>
    suspend fun calculateSaju(id: String): Result<SajuCalculationOutputDto>
    suspend fun calculateWestern(id: String, houseSystem: String?): Result<WesternCalculationOutputDto>
    suspend fun calculateVedic(id: String): Result<VedicCalculationOutputDto>
    suspend fun calculatePredictiveSuite(
        id: String,
        request: PredictiveSuiteRequestDto
    ): Result<PredictiveSuiteData>
}

interface HoraryDataSource {
    suspend fun createHorary(request: CreateHoraryRequest): Result<HoraryDto>
    suspend fun updateHorary(id: String, request: UpdateHoraryRequest): Result<HoraryDto>
    suspend fun listHorary(category: String? = null, querentId: String? = null): Result<List<HoraryDto>>
    suspend fun deleteHorary(id: String): Result<Boolean>
    suspend fun calculateChart(id: String): Result<FortuneChartOutput>
    suspend fun exportWesternMarkdown(id: String): Result<String>
    suspend fun exportPrashnaMarkdown(id: String): Result<String>
}

interface BackupDataSource {
    suspend fun exportBackupJson(): Result<String>
    suspend fun importBackupJson(jsonStr: String, replaceExisting: Boolean): Result<ImportSummaryDto>
}

private class RustProfileDataSource(
    private val client: FortuneClient,
    private val json: Json
) : ProfileDataSource {
    override suspend fun createProfile(request: CreateProfileRequest): Result<ProfileDto> =
        withContext(Dispatchers.IO) {
            runCatching {
                val reqJson = json.encodeToString(CreateProfileRequest.serializer(), request)
                json.decodeFromString(ProfileDto.serializer(), client.createProfile(reqJson))
            }
        }

    override suspend fun updateProfile(id: String, request: UpdateProfileRequest): Result<ProfileDto> =
        withContext(Dispatchers.IO) {
            runCatching {
                val reqJson = json.encodeToString(CreateProfileRequest.serializer(), request)
                json.decodeFromString(ProfileDto.serializer(), client.updateProfile(id, reqJson))
            }
        }

    override suspend fun listProfiles(search: String?): Result<List<ProfileDto>> =
        withContext(Dispatchers.IO) {
            runCatching { json.decodeFromString<List<ProfileDto>>(client.listProfiles(search)) }
        }

    override suspend fun deleteProfile(id: String): Result<Boolean> =
        withContext(Dispatchers.IO) { runCatching { client.deleteProfile(id) } }

    override suspend fun calculateSaju(id: String): Result<SajuCalculationOutputDto> =
        withContext(Dispatchers.IO) {
            runCatching {
                json.decodeFromString(SajuCalculationOutputDto.serializer(), client.calculateProfileSaju(id))
            }
        }

    override suspend fun calculateWestern(
        id: String,
        houseSystem: String?
    ): Result<WesternCalculationOutputDto> = withContext(Dispatchers.IO) {
        runCatching {
            json.decodeFromString(
                WesternCalculationOutputDto.serializer(),
                client.calculateProfileWestern(id, houseSystem)
            )
        }
    }

    override suspend fun calculateVedic(id: String): Result<VedicCalculationOutputDto> =
        withContext(Dispatchers.IO) {
            runCatching {
                json.decodeFromString(VedicCalculationOutputDto.serializer(), client.calculateProfileVedic(id))
            }
        }

    override suspend fun calculatePredictiveSuite(
        id: String,
        request: PredictiveSuiteRequestDto
    ): Result<PredictiveSuiteData> = withContext(Dispatchers.IO) {
        runCatching {
            val reqJson = json.encodeToString(PredictiveSuiteRequestDto.serializer(), request)
            val resJson = client.calculateProfilePredictiveSuite(id, reqJson)
            json.decodeFromString(PredictiveSuiteData.serializer(), resJson)
        }
    }
}

private class RustHoraryDataSource(
    private val client: FortuneClient,
    private val json: Json
) : HoraryDataSource {
    override suspend fun createHorary(request: CreateHoraryRequest): Result<HoraryDto> =
        withContext(Dispatchers.IO) {
            runCatching {
                val reqJson = json.encodeToString(CreateHoraryRequest.serializer(), request)
                json.decodeFromString(HoraryDto.serializer(), client.createHorary(reqJson))
            }
        }

    override suspend fun updateHorary(id: String, request: UpdateHoraryRequest): Result<HoraryDto> =
        withContext(Dispatchers.IO) {
            runCatching {
                val reqJson = json.encodeToString(CreateHoraryRequest.serializer(), request)
                json.decodeFromString(HoraryDto.serializer(), client.updateHorary(id, reqJson))
            }
        }

    override suspend fun listHorary(category: String?, querentId: String?): Result<List<HoraryDto>> =
        withContext(Dispatchers.IO) {
            runCatching { json.decodeFromString<List<HoraryDto>>(client.listHorary(category, querentId)) }
        }

    override suspend fun deleteHorary(id: String): Result<Boolean> =
        withContext(Dispatchers.IO) { runCatching { client.deleteHorary(id) } }

    override suspend fun calculateChart(id: String): Result<FortuneChartOutput> =
        withContext(Dispatchers.IO) {
            runCatching {
                json.decodeFromString(FortuneChartOutput.serializer(), client.calculateHoraryChart(id))
            }
        }

    override suspend fun exportWesternMarkdown(id: String): Result<String> =
        withContext(Dispatchers.IO) { runCatching { client.exportHoraryWesternMarkdown(id) } }

    override suspend fun exportPrashnaMarkdown(id: String): Result<String> =
        withContext(Dispatchers.IO) { runCatching { client.exportPrashnaVedicMarkdown(id) } }
}

private class RustBackupDataSource(
    private val client: FortuneClient,
    private val json: Json
) : BackupDataSource {
    override suspend fun exportBackupJson(): Result<String> =
        withContext(Dispatchers.IO) { runCatching { client.exportBackupJson() } }

    override suspend fun importBackupJson(
        jsonStr: String,
        replaceExisting: Boolean
    ): Result<ImportSummaryDto> = withContext(Dispatchers.IO) {
        runCatching {
            val resJson = client.importBackupJson(jsonStr, replaceExisting)
            json.decodeFromString(ImportSummaryDto.serializer(), resJson)
        }
    }
}

class FortuneRepository(
    client: FortuneClient
) {
    private val json = Json {
        ignoreUnknownKeys = true
        isLenient = true
        encodeDefaults = true
    }

    val profiles: ProfileDataSource = RustProfileDataSource(client, json)
    val horary: HoraryDataSource = RustHoraryDataSource(client, json)
    val backup: BackupDataSource = RustBackupDataSource(client, json)
}
