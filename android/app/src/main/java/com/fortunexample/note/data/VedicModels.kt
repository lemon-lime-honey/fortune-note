package com.fortunexample.note.data

import kotlinx.serialization.Serializable
import java.util.Locale

@Serializable
data class VedicPlanetData(
    val name: String,
    val sidereal_longitude: Double,
    val sign_index: Int,
    val sign_degree: Double,
    val nakshatra_name: String,
    val nakshatra_pada: Int,
    val is_retrograde: Boolean,
    val latitude: Double = 0.0,
    val house: Int = 1,
    val dignity: String = "",
    val combustion: String = "None"
)

@Serializable
data class VedicKarakaData(
    val karaka: String,
    val planet: String,
    val effective_degree: Double
)

@Serializable
data class VedicDrishtiData(
    val from_planet: String,
    val from_sign: Int,
    val from_house: Int,
    val aspect_offset: Int,
    val target_sign: Int,
    val target_house: Int,
    val aspected_planets: List<String> = emptyList()
)

@Serializable
data class VedicArudhaPadaData(
    val bhava: Int,
    val code: String,
    val name: String,
    val sign_index: Int,
    val sign_name: String,
    val house_from_lagna: Int
)

@Serializable
data class BhinnaAshtakavargaRowData(
    val planet: String,
    val bindus: List<Int> = emptyList(),
    val total: Int = 0
)

@Serializable
data class VedicAshtakavargaData(
    val bhinna: List<BhinnaAshtakavargaRowData> = emptyList(),
    val sarva: List<Int> = emptyList(),
    val total_bindus: Int = 0
)

@Serializable
data class VedicPanchangaData(
    val tithi_name: String = "",
    val tithi_number: Int = 1,
    val paksha: String = "",
    val vara_name: String = "",
    val vara_lord: String = "",
    val nakshatra_name: String = "",
    val nakshatra_pada: Int = 1,
    val nakshatra_lord: String = "",
    val yoga_name: String = "",
    val karana_name: String = ""
)

@Serializable
data class VargaPlacementData(
    val planet_name: String,
    val sign_index: Int,
    val sign_degree: Double = 0.0,
    val house: Int
)

@Serializable
data class VargaChartData(
    val division: String,
    val lagna_sign: Int,
    val placements: List<VargaPlacementData>
)

@Serializable
data class DashaPeriodData(
    val lord: String,
    val start_jd: Double,
    val end_jd: Double,
    val sub_periods: List<DashaPeriodData> = emptyList()
) {
    val startDateFormatted: String get() = jdToCalendarString(start_jd)
    val endDateFormatted: String get() = jdToCalendarString(end_jd)
}

fun jdToCalendarString(jd: Double): String {
    val z = (jd + 0.5).toLong()
    val a = if (z < 2299161) z else {
        val alpha = ((z - 1867216.25) / 36524.25).toLong()
        z + 1 + alpha - (alpha / 4)
    }
    val b = a + 1524
    val c = ((b - 122.1) / 365.25).toLong()
    val d = (365.25 * c).toLong()
    val e = ((b - d) / 30.6001).toLong()
    val day = (b - d - (30.6001 * e).toLong()).toInt()
    val month = (if (e < 14) e - 1 else e - 13).toInt()
    val year = (if (month > 2) c - 4716 else c - 4715).toInt()
    return String.format(Locale.US, "%04d.%02d.%02d", year, month, day)
}

@Serializable
data class VedicChartData(
    val lagna_sidereal: Double,
    val lagna_sign: Int,
    val wholesign_bhavas: List<Double>,
    val planets: List<VedicPlanetData>,
    val vargas: Map<String, VargaChartData>,
    val vimshottari_dashas: List<DashaPeriodData>,
    val chara_karakas: List<VedicKarakaData> = emptyList(),
    val drishti: List<VedicDrishtiData> = emptyList(),
    val arudha_padas: List<VedicArudhaPadaData> = emptyList(),
    val ashtakavarga: VedicAshtakavargaData = VedicAshtakavargaData(),
    val panchanga: VedicPanchangaData = VedicPanchangaData(),
    val ayanamsha_name: String = "True Lahiri (Chitra Paksha)",
    val ayanamsha_deg: Double = 0.0
)

val VEDIC_RASI_NAMES = listOf(
    "Mesha", "Vrishabha", "Mithuna", "Karka",
    "Simha", "Kanya", "Tula", "Vrishchika",
    "Dhanu", "Makara", "Kumbha", "Meena"
)

fun vedicSignFullLabel(index: Int): String {
    val idx = ((index % 12) + 12) % 12
    return "${VEDIC_RASI_NAMES[idx]} (${WESTERN_ZODIAC_SIGNS[idx]})"
}

val VARGA_DEFINITIONS = listOf(
    "D1" to "Rasi",
    "D2" to "Hora",
    "D3" to "Drekkana",
    "D4" to "Chaturthamsha",
    "D6" to "Shashthamsha",
    "D7" to "Saptamsha",
    "D8" to "Ashtamamsha",
    "D9" to "Navamsha",
    "D10" to "Dashamsha",
    "D11" to "Rudramsha",
    "D12" to "Dwadashamsha",
    "D16" to "Shodashamsha",
    "D20" to "Vimshamsha",
    "D24" to "Chaturvimshamsha",
    "D27" to "Saptavimshamsha",
    "D30" to "Trimshamsha",
    "D40" to "Khavedamsha",
    "D45" to "Akshavedamsha",
    "D60" to "Shashtiamsha"
)
