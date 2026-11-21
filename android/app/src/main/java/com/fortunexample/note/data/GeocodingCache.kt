package com.fortunexample.note.data

import android.content.Context
import android.location.Address
import android.location.Geocoder
import android.os.Build
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.async
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.delay
import kotlinx.coroutines.suspendCancellableCoroutine
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import kotlinx.coroutines.withContext
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonArray
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.contentOrNull
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive
import java.io.BufferedReader
import java.io.File
import java.io.InputStreamReader
import java.net.HttpURLConnection
import java.net.URL
import java.net.URLEncoder
import java.util.Locale
import kotlin.coroutines.resume
import kotlin.math.abs

@Serializable
data class CachedLocation(
    val query: String,
    val displayName: String,
    val latitude: Double,
    val longitude: Double,
    val timestamp: Long = System.currentTimeMillis()
)

object GeocodingCache {
    private const val FILE_NAME = "geocoding_cache.json"
    private const val SEED_ASSET_NAME = "locations_seed.csv"
    private const val NOMINATIM_BASE_URL = "https://nominatim.openstreetmap.org/search"
    private const val NOMINATIM_USER_AGENT = "FortuneNoteAndroid/0.1.0 (offline-astrology-ephemeris)"
    private const val NOMINATIM_MIN_INTERVAL_MS = 1100L
    private const val DEDUP_EPSILON_DEG = 0.002
    private const val MAX_LOCAL_RESULTS = 80

    private val json = Json { ignoreUnknownKeys = true; prettyPrint = true }
    private val lock = Any()
    private val nominatimMutex = Mutex()
    private var lastNominatimRequestMs: Long = 0L

    private var bundledSeedsCache: List<CachedLocation>? = null
    private var userHistoryCache: MutableMap<String, CachedLocation>? = null

    private val FALLBACK_DEFAULT_SEEDS = listOf(
        CachedLocation("서울", "서울특별시", 37.5665, 126.9780, 0L),
        CachedLocation("부산", "부산광역시", 35.1796, 129.0756, 0L),
        CachedLocation("대구", "대구광역시", 35.8714, 128.6014, 0L),
        CachedLocation("인천", "인천광역시", 37.4563, 126.7052, 0L),
        CachedLocation("광주", "광주광역시", 35.1595, 126.8526, 0L),
        CachedLocation("대전", "대전광역시", 36.3504, 127.3845, 0L),
        CachedLocation("울산", "울산광역시", 35.5384, 129.3114, 0L),
        CachedLocation("세종", "세종특별자치시", 36.4800, 127.2890, 0L),
        CachedLocation("수원", "경기도 수원시", 37.2636, 127.0286, 0L),
        CachedLocation("성남", "경기도 성남시", 37.4200, 127.1268, 0L),
        CachedLocation("고양", "경기도 고양시", 37.6584, 126.8320, 0L),
        CachedLocation("용인", "경기도 용인시", 37.2411, 127.1776, 0L),
        CachedLocation("부천", "경기도 부천시", 37.5034, 126.7660, 0L),
        CachedLocation("안산", "경기도 안산시", 37.3219, 126.8309, 0L),
        CachedLocation("안양", "경기도 안양시", 37.3943, 126.9568, 0L),
        CachedLocation("남양주", "경기도 남양주시", 37.6360, 127.2165, 0L),
        CachedLocation("화성", "경기도 화성시", 37.1995, 126.8315, 0L),
        CachedLocation("평택", "경기도 평택시", 36.9921, 127.1129, 0L),
        CachedLocation("의정부", "경기도 의정부시", 37.7381, 127.0337, 0L),
        CachedLocation("파주", "경기도 파주시", 37.7600, 126.7799, 0L),
        CachedLocation("시흥", "경기도 시흥시", 37.3802, 126.8029, 0L),
        CachedLocation("김포", "경기도 김포시", 37.6152, 126.7157, 0L),
        CachedLocation("광명", "경기도 광명시", 37.4786, 126.8647, 0L),
        CachedLocation("원주", "강원특별자치도 원주시", 37.3422, 127.9202, 0L),
        CachedLocation("춘천", "강원특별자치도 춘천시", 37.8813, 127.7298, 0L),
        CachedLocation("강릉", "강원특별자치도 강릉시", 37.7519, 128.8761, 0L),
        CachedLocation("청주", "충청북도 청주시", 36.6424, 127.4890, 0L),
        CachedLocation("충주", "충청북도 충주시", 36.9910, 127.9260, 0L),
        CachedLocation("천안", "충청남도 천안시", 36.8151, 127.1139, 0L),
        CachedLocation("아산", "충청남도 아산시", 36.7898, 127.0019, 0L),
        CachedLocation("전주", "전북특별자치도 전주시", 35.8242, 127.1480, 0L),
        CachedLocation("익산", "전북특별자치도 익산시", 35.9483, 126.9576, 0L),
        CachedLocation("군산", "전북특별자치도 군산시", 35.9677, 126.7366, 0L),
        CachedLocation("여수", "전라남도 여수시", 34.7604, 127.6622, 0L),
        CachedLocation("순천", "전라남도 순천시", 34.9506, 127.4872, 0L),
        CachedLocation("목포", "전라남도 목포시", 34.8118, 126.3922, 0L),
        CachedLocation("포항", "경상북도 포항시", 36.0190, 129.3435, 0L),
        CachedLocation("구미", "경상북도 구미시", 36.1195, 128.3446, 0L),
        CachedLocation("경주", "경상북도 경주시", 35.8562, 129.2247, 0L),
        CachedLocation("창원", "경상남도 창원시", 35.2280, 128.6811, 0L),
        CachedLocation("김해", "경상남도 김해시", 35.2285, 128.8894, 0L),
        CachedLocation("진주", "경상남도 진주시", 35.1802, 128.1076, 0L),
        CachedLocation("양산", "경상남도 양산시", 35.3350, 129.0373, 0L),
        CachedLocation("제주", "제주특별자치도 제주시", 33.4996, 126.5312, 0L),
        CachedLocation("서귀포", "제주특별자치도 서귀포시", 33.2541, 126.5601, 0L)
    )

    private val DECIMAL_COORD_REGEX = Regex(
        """^\s*\(?\s*(?:위도|lat(?:itude)?)?\s*([NSns])?\s*([+-]?\d{1,2}(?:\.\d+)?)\s*°?\s*([NSns])?\s*[,/\s]+\s*(?:경도|lon(?:gitude)?|lng)?\s*([EWew])?\s*([+-]?\d{1,3}(?:\.\d+)?)\s*°?\s*([EWew])?\s*\)?\s*$"""
    )

    private val DMS_COORD_REGEX = Regex(
        """^\s*([NSns])?\s*(\d{1,2})\s*[°d]\s*(\d{1,2})\s*['′m]\s*(?:(\d{1,2}(?:\.\d+)?)\s*["″s]?)?\s*([NSns])?\s*[,/\s]+\s*([EWew])?\s*(\d{1,3})\s*[°d]\s*(\d{1,2})\s*['′m]\s*(?:(\d{1,2}(?:\.\d+)?)\s*["″s]?)?\s*([EWew])?\s*$"""
    )

    private fun getFile(context: Context): File = File(context.filesDir, FILE_NAME)

    private fun cacheKey(displayName: String, lat: Double, lon: Double): String {
        return "${displayName.trim().lowercase(Locale.ROOT)}|${String.format(Locale.US, "%.4f", lat)}|${String.format(Locale.US, "%.4f", lon)}"
    }

    private fun isValidCoordinate(lat: Double, lon: Double): Boolean {
        return lat.isFinite() && lon.isFinite() && lat in -90.0..90.0 && lon in -180.0..180.0
    }

    fun parseCoordinateQuery(raw: String): CachedLocation? {
        val trimmed = raw.trim()
        if (trimmed.isEmpty()) return null

        DMS_COORD_REGEX.matchEntire(trimmed)?.let { m ->
            val latHem = (m.groupValues[1].ifEmpty { m.groupValues[5] }).uppercase(Locale.US)
            val latDeg = m.groupValues[2].toDoubleOrNull() ?: return null
            val latMin = m.groupValues[3].toDoubleOrNull() ?: return null
            val latSec = m.groupValues[4].toDoubleOrNull() ?: 0.0

            val lonHem = (m.groupValues[6].ifEmpty { m.groupValues[10] }).uppercase(Locale.US)
            val lonDeg = m.groupValues[7].toDoubleOrNull() ?: return null
            val lonMin = m.groupValues[8].toDoubleOrNull() ?: return null
            val lonSec = m.groupValues[9].toDoubleOrNull() ?: 0.0

            if (latMin >= 60.0 || latSec >= 60.0 || lonMin >= 60.0 || lonSec >= 60.0) return null

            var lat = latDeg + latMin / 60.0 + latSec / 3600.0
            var lon = lonDeg + lonMin / 60.0 + lonSec / 3600.0
            if (latHem == "S") lat = -lat
            if (lonHem == "W") lon = -lon

            if (isValidCoordinate(lat, lon)) {
                return CachedLocation(
                    query = trimmed,
                    displayName = "직접 입력 좌표 (${String.format(Locale.US, "%.4f°, %.4f°", lat, lon)})",
                    latitude = lat,
                    longitude = lon,
                    timestamp = System.currentTimeMillis()
                )
            }
        }

        DECIMAL_COORD_REGEX.matchEntire(trimmed)?.let { m ->
            val latHem = (m.groupValues[1].ifEmpty { m.groupValues[3] }).uppercase(Locale.US)
            val rawLat = m.groupValues[2].toDoubleOrNull() ?: return null
            val lonHem = (m.groupValues[4].ifEmpty { m.groupValues[6] }).uppercase(Locale.US)
            val rawLon = m.groupValues[5].toDoubleOrNull() ?: return null

            val lat = when (latHem) {
                "S" -> -abs(rawLat)
                "N" -> abs(rawLat)
                else -> rawLat
            }
            val lon = when (lonHem) {
                "W" -> -abs(rawLon)
                "E" -> abs(rawLon)
                else -> rawLon
            }

            if (isValidCoordinate(lat, lon)) {
                return CachedLocation(
                    query = trimmed,
                    displayName = "직접 입력 좌표 (${String.format(Locale.US, "%.4f°, %.4f°", lat, lon)})",
                    latitude = lat,
                    longitude = lon,
                    timestamp = System.currentTimeMillis()
                )
            }
        }

        return null
    }

    private fun ensureBundledSeedsLoaded(context: Context): List<CachedLocation> = synchronized(lock) {
        bundledSeedsCache?.let { return it }
        val loaded = mutableListOf<CachedLocation>()
        try {
            context.assets.open(SEED_ASSET_NAME).use { inputStream ->
                BufferedReader(InputStreamReader(inputStream, Charsets.UTF_8)).use { reader ->
                    var isHeader = true
                    reader.forEachLine { line ->
                        if (isHeader) {
                            isHeader = false
                            return@forEachLine
                        }
                        if (line.isBlank()) return@forEachLine
                        val parts = parseCsvLine(line)
                        if (parts.size >= 4) {
                            val lat = parts[2].toDoubleOrNull()
                            val lon = parts[3].toDoubleOrNull()
                            if (lat != null && lon != null && isValidCoordinate(lat, lon)) {
                                loaded.add(
                                    CachedLocation(
                                        query = parts[0],
                                        displayName = parts[1],
                                        latitude = lat,
                                        longitude = lon,
                                        timestamp = 0L
                                    )
                                )
                            }
                        }
                    }
                }
            }
        } catch (_: Exception) {
            // Fall back to built-in major cities if asset is unavailable
        }

        val result = if (loaded.isNotEmpty()) loaded else FALLBACK_DEFAULT_SEEDS
        bundledSeedsCache = result
        return result
    }

    private fun parseCsvLine(line: String): List<String> {
        val result = ArrayList<String>(4)
        val current = StringBuilder()
        var inQuotes = false
        var i = 0
        while (i < line.length) {
            val ch = line[i]
            when {
                ch == '"' -> {
                    if (inQuotes && i + 1 < line.length && line[i + 1] == '"') {
                        current.append('"')
                        i++
                    } else {
                        inQuotes = !inQuotes
                    }
                }
                ch == ',' && !inQuotes -> {
                    result.add(current.toString())
                    current.setLength(0)
                }
                else -> current.append(ch)
            }
            i++
        }
        result.add(current.toString())
        return result
    }

    private fun ensureUserHistoryLoaded(context: Context): MutableMap<String, CachedLocation> = synchronized(lock) {
        userHistoryCache?.let { return it }
        val file = getFile(context)
        val map = mutableMapOf<String, CachedLocation>()
        if (file.exists()) {
            try {
                val text = file.readText()
                val list = json.decodeFromString<List<CachedLocation>>(text)
                for (item in list) {
                    if (isValidCoordinate(item.latitude, item.longitude)) {
                        map[cacheKey(item.displayName, item.latitude, item.longitude)] = item
                    }
                }
            } catch (_: Exception) {
                // Corrupt cache file; reset gracefully
            }
        }
        userHistoryCache = map
        return map
    }

    private fun saveUserHistory(context: Context, list: List<CachedLocation>) {
        try {
            val file = getFile(context)
            file.writeText(
                json.encodeToString(
                    kotlinx.serialization.builtins.ListSerializer(CachedLocation.serializer()),
                    list
                )
            )
        } catch (_: Exception) {
        }
    }

    fun putAll(context: Context, items: List<CachedLocation>) {
        val validItems = items.filter { isValidCoordinate(it.latitude, it.longitude) && it.displayName.isNotBlank() }
        if (validItems.isEmpty()) return
        synchronized(lock) {
            val cache = ensureUserHistoryLoaded(context)
            for (item in validItems) {
                cache[cacheKey(item.displayName, item.latitude, item.longitude)] = item
            }
            saveUserHistory(context, cache.values.toList())
        }
    }

    fun searchLocal(context: Context, query: String): List<CachedLocation> {
        val trimmed = query.trim()
        val parsedCoord = parseCoordinateQuery(trimmed)

        val history = synchronized(lock) {
            ensureUserHistoryLoaded(context).values.toList()
        }
        val seeds = ensureBundledSeedsLoaded(context)

        if (trimmed.isEmpty()) {
            val recent = history.sortedByDescending { it.timestamp }
            return mergeLocations(recent, FALLBACK_DEFAULT_SEEDS).take(MAX_LOCAL_RESULTS)
        }

        val qLower = trimmed.lowercase(Locale.ROOT)
        val tokens = qLower.split(Regex("\\s+")).filter { it.isNotEmpty() }

        fun matches(item: CachedLocation): Boolean {
            val disp = item.displayName.lowercase(Locale.ROOT)
            val kw = item.query.lowercase(Locale.ROOT)
            return tokens.all { tok -> disp.contains(tok) || kw.contains(tok) }
        }

        fun score(item: CachedLocation, isHistory: Boolean): Int {
            val disp = item.displayName.lowercase(Locale.ROOT)
            val kw = item.query.lowercase(Locale.ROOT)
            val primaryLabel = disp.substringBefore(" (").trim()
            return when {
                primaryLabel == qLower -> 0
                disp == qLower -> 1
                primaryLabel.startsWith(qLower) -> 2
                isHistory -> 3
                disp.contains(qLower) -> 4
                kw.startsWith(qLower) -> 5
                else -> 6
            }
        }

        val matchedHistory = history.filter(::matches).map { it to score(it, true) }
        val matchedSeeds = seeds.filter(::matches).map { it to score(it, false) }

        val ranked = (matchedHistory + matchedSeeds)
            .sortedWith(
                compareBy<Pair<CachedLocation, Int>> { it.second }
                    .thenByDescending { it.first.timestamp }
                    .thenBy { it.first.displayName.length }
            )
            .map { it.first }

        val merged = mergeLocations(
            primary = parsedCoord?.let { listOf(it) } ?: emptyList(),
            secondary = ranked
        )
        return merged.take(MAX_LOCAL_RESULTS)
    }

    fun mergeLocations(
        primary: List<CachedLocation>,
        secondary: List<CachedLocation>
    ): List<CachedLocation> {
        val result = mutableListOf<CachedLocation>()
        for (candidate in primary + secondary) {
            if (!isValidCoordinate(candidate.latitude, candidate.longitude)) continue
            val isDuplicate = result.any { existing ->
                val sameCoords = abs(existing.latitude - candidate.latitude) < DEDUP_EPSILON_DEG &&
                    abs(existing.longitude - candidate.longitude) < DEDUP_EPSILON_DEG
                val existingBase = existing.displayName.substringBefore(" (").trim().lowercase(Locale.ROOT)
                val candBase = candidate.displayName.substringBefore(" (").trim().lowercase(Locale.ROOT)
                val sameName = existingBase == candBase ||
                    existing.displayName.equals(candidate.displayName, ignoreCase = true)
                (sameName && sameCoords) || (sameCoords && (existingBase.contains(candBase) || candBase.contains(existingBase)))
            }
            if (!isDuplicate) {
                result.add(candidate)
            }
        }
        return result
    }

    suspend fun searchOnline(context: Context, query: String): Result<List<CachedLocation>> = withContext(Dispatchers.IO) {
        val q = query.trim()
        if (q.isEmpty()) return@withContext Result.success(emptyList())

        parseCoordinateQuery(q)?.let { coordLoc ->
            return@withContext Result.success(listOf(coordLoc))
        }

        coroutineScope {
            val nominatimDeferred = async { queryNominatim(q) }
            val geocoderDeferred = async { queryAndroidGeocoder(context, q) }

            val nominatimRes = nominatimDeferred.await()
            val geocoderRes = geocoderDeferred.await()

            val nominatimList = nominatimRes.getOrDefault(emptyList())
            val geocoderList = geocoderRes.getOrDefault(emptyList())

            // Nominatim first for stronger POI / station coverage, merged with Android Geocoder
            val merged = mergeLocations(nominatimList, geocoderList)
            if (merged.isNotEmpty()) {
                putAll(context, merged)
                return@coroutineScope Result.success(merged)
            }

            if (nominatimRes.isFailure && geocoderRes.isFailure) {
                val err = nominatimRes.exceptionOrNull()
                    ?: geocoderRes.exceptionOrNull()
                    ?: Exception("온라인 위치 검색에 실패했습니다.")
                return@coroutineScope Result.failure(err)
            }

            Result.success(emptyList())
        }
    }

    private suspend fun queryNominatim(query: String): Result<List<CachedLocation>> {
        return try {
            nominatimMutex.withLock {
                val now = System.currentTimeMillis()
                val elapsed = now - lastNominatimRequestMs
                if (elapsed in 0 until NOMINATIM_MIN_INTERVAL_MS) {
                    delay(NOMINATIM_MIN_INTERVAL_MS - elapsed)
                }
                lastNominatimRequestMs = System.currentTimeMillis()
            }

            val encoded = URLEncoder.encode(query, "UTF-8")
            val url = URL("$NOMINATIM_BASE_URL?format=jsonv2&addressdetails=1&limit=8&accept-language=ko,en&q=$encoded")
            val conn = (url.openConnection() as HttpURLConnection).apply {
                requestMethod = "GET"
                connectTimeout = 6000
                readTimeout = 8000
                setRequestProperty("User-Agent", NOMINATIM_USER_AGENT)
                setRequestProperty("Accept", "application/json")
                setRequestProperty("Accept-Language", "ko-KR,ko;q=0.9,en;q=0.5")
            }

            try {
                val code = conn.responseCode
                if (code !in 200..299) {
                    return Result.failure(Exception("Nominatim HTTP $code"))
                }
                val body = conn.inputStream.bufferedReader(Charsets.UTF_8).use { it.readText() }
                val array: JsonArray = json.parseToJsonElement(body).jsonArray
                val now = System.currentTimeMillis()
                val parsed = array.mapNotNull { elem ->
                    val obj = elem.jsonObject
                    val lat = obj["lat"]?.jsonPrimitive?.contentOrNull?.toDoubleOrNull() ?: return@mapNotNull null
                    val lon = obj["lon"]?.jsonPrimitive?.contentOrNull?.toDoubleOrNull() ?: return@mapNotNull null
                    if (!isValidCoordinate(lat, lon)) return@mapNotNull null

                    val displayName = formatNominatimDisplayName(obj)
                    if (displayName.isBlank()) return@mapNotNull null

                    CachedLocation(
                        query = query,
                        displayName = displayName,
                        latitude = lat,
                        longitude = lon,
                        timestamp = now
                    )
                }
                Result.success(parsed)
            } finally {
                conn.disconnect()
            }
        } catch (e: Exception) {
            Result.failure(e)
        }
    }

    private fun formatNominatimDisplayName(obj: JsonObject): String {
        val name = obj["name"]?.jsonPrimitive?.contentOrNull?.trim().orEmpty()
        val address = obj["address"] as? JsonObject
        if (address != null) {
            val countryCode = address["country_code"]?.jsonPrimitive?.contentOrNull?.lowercase(Locale.ROOT).orEmpty()
            val adminParts = if (countryCode == "kr") {
                listOfNotNull(
                    address["province"]?.jsonPrimitive?.contentOrNull,
                    address["state"]?.jsonPrimitive?.contentOrNull,
                    address["city"]?.jsonPrimitive?.contentOrNull,
                    address["county"]?.jsonPrimitive?.contentOrNull,
                    address["borough"]?.jsonPrimitive?.contentOrNull,
                    address["city_district"]?.jsonPrimitive?.contentOrNull,
                    address["suburb"]?.jsonPrimitive?.contentOrNull,
                    address["town"]?.jsonPrimitive?.contentOrNull,
                    address["quarter"]?.jsonPrimitive?.contentOrNull,
                    address["village"]?.jsonPrimitive?.contentOrNull,
                    address["neighbourhood"]?.jsonPrimitive?.contentOrNull
                )
            } else {
                listOfNotNull(
                    address["country"]?.jsonPrimitive?.contentOrNull,
                    address["state"]?.jsonPrimitive?.contentOrNull,
                    address["city"]?.jsonPrimitive?.contentOrNull,
                    address["town"]?.jsonPrimitive?.contentOrNull,
                    address["county"]?.jsonPrimitive?.contentOrNull
                )
            }.map { it.trim() }.filter { it.isNotEmpty() }.distinct()

            if (name.isNotEmpty()) {
                val filteredAdmin = adminParts.filterNot { it.equals(name, ignoreCase = true) }
                return if (filteredAdmin.isNotEmpty()) {
                    "$name (${filteredAdmin.joinToString(" ")})"
                } else {
                    name
                }
            }
            if (adminParts.isNotEmpty()) {
                return adminParts.joinToString(" ")
            }
        }

        val fallback = obj["display_name"]?.jsonPrimitive?.contentOrNull?.trim().orEmpty()
        return fallback.removePrefix("대한민국 ").trim()
    }

    private suspend fun queryAndroidGeocoder(context: Context, query: String): Result<List<CachedLocation>> {
        if (!Geocoder.isPresent()) {
            return Result.failure(IllegalStateException("Geocoder service unavailable"))
        }

        val geocoder = Geocoder(context, Locale.KOREAN)
        return try {
            val addresses: List<Address> = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
                suspendCancellableCoroutine { cont ->
                    geocoder.getFromLocationName(query, 5, object : Geocoder.GeocodeListener {
                        override fun onGeocode(results: MutableList<Address>) {
                            if (cont.isActive) cont.resume(results)
                        }

                        override fun onError(errorMessage: String?) {
                            if (cont.isActive) cont.resume(emptyList())
                        }
                    })
                }
            } else {
                @Suppress("DEPRECATION")
                geocoder.getFromLocationName(query, 5) ?: emptyList()
            }

            val now = System.currentTimeMillis()
            val parsed = addresses.mapNotNull { addr ->
                if (!addr.hasLatitude() || !addr.hasLongitude()) return@mapNotNull null
                val lat = addr.latitude
                val lon = addr.longitude
                if (!isValidCoordinate(lat, lon)) return@mapNotNull null

                var line = addr.getAddressLine(0) ?: ""
                line = line.removePrefix("대한민국 ").trim()
                if (line.isEmpty()) {
                    val parts = listOfNotNull(
                        addr.adminArea,
                        addr.subAdminArea,
                        addr.locality,
                        addr.subLocality,
                        addr.thoroughfare,
                        addr.featureName
                    ).distinct()
                    line = parts.joinToString(" ")
                }
                if (line.isNotEmpty()) {
                    CachedLocation(query, line, lat, lon, now)
                } else null
            }
            Result.success(parsed)
        } catch (e: Exception) {
            Result.failure(e)
        }
    }
}
