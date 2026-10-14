package com.fortunexample.note.data

import android.content.Context
import android.location.Address
import android.location.Geocoder
import android.os.Build
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.suspendCancellableCoroutine
import kotlinx.coroutines.withContext
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import java.io.File
import java.util.Locale
import kotlin.coroutines.resume

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
    private val json = Json { ignoreUnknownKeys = true; prettyPrint = true }
    private val lock = Any()
    private var inMemoryCache: MutableMap<String, CachedLocation>? = null

    private val DEFAULT_SEEDS = listOf(
        CachedLocation("서울", "서울특별시", 37.5665, 126.9780),
        CachedLocation("부산", "부산광역시", 35.1796, 129.0756),
        CachedLocation("대구", "대구광역시", 35.8714, 128.6014),
        CachedLocation("인천", "인천광역시", 37.4563, 126.7052),
        CachedLocation("광주", "광주광역시", 35.1595, 126.8526),
        CachedLocation("대전", "대전광역시", 36.3504, 127.3845),
        CachedLocation("울산", "울산광역시", 35.5384, 129.3114),
        CachedLocation("세종", "세종특별자치시", 36.4800, 127.2890),
        CachedLocation("수원", "경기도 수원시", 37.2636, 127.0286),
        CachedLocation("성남", "경기도 성남시", 37.4200, 127.1268),
        CachedLocation("고양", "경기도 고양시", 37.6584, 126.8320),
        CachedLocation("용인", "경기도 용인시", 37.2411, 127.1776),
        CachedLocation("부천", "경기도 부천시", 37.5034, 126.7660),
        CachedLocation("안산", "경기도 안산시", 37.3219, 126.8309),
        CachedLocation("안양", "경기도 안양시", 37.3943, 126.9568),
        CachedLocation("남양주", "경기도 남양주시", 37.6360, 127.2165),
        CachedLocation("화성", "경기도 화성시", 37.1995, 126.8315),
        CachedLocation("평택", "경기도 평택시", 36.9921, 127.1129),
        CachedLocation("의정부", "경기도 의정부시", 37.7381, 127.0337),
        CachedLocation("파주", "경기도 파주시", 37.7600, 126.7799),
        CachedLocation("시흥", "경기도 시흥시", 37.3802, 126.8029),
        CachedLocation("김포", "경기도 김포시", 37.6152, 126.7157),
        CachedLocation("광명", "경기도 광명시", 37.4786, 126.8647),
        CachedLocation("원주", "강원특별자치도 원주시", 37.3422, 127.9202),
        CachedLocation("춘천", "강원특별자치도 춘천시", 37.8813, 127.7298),
        CachedLocation("강릉", "강원특별자치도 강릉시", 37.7519, 128.8761),
        CachedLocation("청주", "충청북도 청주시", 36.6424, 127.4890),
        CachedLocation("충주", "충청북도 충주시", 36.9910, 127.9260),
        CachedLocation("천안", "충청남도 천안시", 36.8151, 127.1139),
        CachedLocation("아산", "충청남도 아산시", 36.7898, 127.0019),
        CachedLocation("전주", "전북특별자치도 전주시", 35.8242, 127.1480),
        CachedLocation("익산", "전북특별자치도 익산시", 35.9483, 126.9576),
        CachedLocation("군산", "전북특별자치도 군산시", 35.9677, 126.7366),
        CachedLocation("여수", "전라남도 여수시", 34.7604, 127.6622),
        CachedLocation("순천", "전라남도 순천시", 34.9506, 127.4872),
        CachedLocation("목포", "전라남도 목포시", 34.8118, 126.3922),
        CachedLocation("포항", "경상북도 포항시", 36.0190, 129.3435),
        CachedLocation("구미", "경상북도 구미시", 36.1195, 128.3446),
        CachedLocation("경주", "경상북도 경주시", 35.8562, 129.2247),
        CachedLocation("창원", "경상남도 창원시", 35.2280, 128.6811),
        CachedLocation("김해", "경상남도 김해시", 35.2285, 128.8894),
        CachedLocation("진주", "경상남도 진주시", 35.1802, 128.1076),
        CachedLocation("양산", "경상남도 양산시", 35.3350, 129.0373),
        CachedLocation("제주", "제주특별자치도 제주시", 33.4996, 126.5312),
        CachedLocation("서귀포", "제주특별자치도 서귀포시", 33.2541, 126.5601)
    )

    private fun getFile(context: Context): File {
        return File(context.filesDir, FILE_NAME)
    }

    private fun ensureLoaded(context: Context): MutableMap<String, CachedLocation> = synchronized(lock) {
        inMemoryCache?.let { return it }
        val file = getFile(context)
        val map = if (file.exists()) {
            try {
                val text = file.readText()
                val list = json.decodeFromString<List<CachedLocation>>(text)
                list.associateBy { it.displayName }.toMutableMap()
            } catch (_: Exception) {
                mutableMapOf()
            }
        } else {
            val initial = DEFAULT_SEEDS.associateBy { it.displayName }.toMutableMap()
            saveToFile(context, initial.values.toList())
            initial
        }
        inMemoryCache = map
        return map
    }

    private fun saveToFile(context: Context, list: List<CachedLocation>) {
        try {
            val file = getFile(context)
            file.writeText(json.encodeToString(kotlinx.serialization.builtins.ListSerializer(CachedLocation.serializer()), list))
        } catch (_: Exception) {}
    }

    fun searchLocal(context: Context, query: String): List<CachedLocation> {
        val q = query.trim().lowercase()
        val all = synchronized(lock) {
            ensureLoaded(context).values.toList()
        }
        if (q.isEmpty()) {
            return all.sortedByDescending { it.timestamp }
        }
        return all.filter {
            it.displayName.lowercase().contains(q) || it.query.lowercase().contains(q)
        }.sortedByDescending { it.timestamp }
    }

    fun putAll(context: Context, items: List<CachedLocation>) {
        if (items.isEmpty()) return
        synchronized(lock) {
            val cache = ensureLoaded(context)
            for (item in items) {
                cache[item.displayName] = item
            }
            saveToFile(context, cache.values.toList())
        }
    }

    suspend fun searchOnline(context: Context, query: String): List<CachedLocation> = withContext(Dispatchers.IO) {
        val q = query.trim()
        if (q.isEmpty()) return@withContext emptyList()
        if (!Geocoder.isPresent()) return@withContext emptyList()

        val geocoder = Geocoder(context, Locale.KOREAN)
        try {
            val addresses: List<Address> = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
                suspendCancellableCoroutine { cont ->
                    geocoder.getFromLocationName(q, 5, object : Geocoder.GeocodeListener {
                        override fun onGeocode(results: MutableList<Address>) {
                            cont.resume(results)
                        }
                        override fun onError(errorMessage: String?) {
                            cont.resume(emptyList())
                        }
                    })
                }
            } else {
                @Suppress("DEPRECATION")
                geocoder.getFromLocationName(q, 5) ?: emptyList()
            }

            val parsed = addresses.mapNotNull { addr ->
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
                    CachedLocation(q, line, addr.latitude, addr.longitude, System.currentTimeMillis())
                } else null
            }
            if (parsed.isNotEmpty()) {
                putAll(context, parsed)
            }
            parsed
        } catch (_: Exception) {
            emptyList()
        }
    }
}
