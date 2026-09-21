package com.fortunexample.note.data

import kotlinx.serialization.Serializable

@Serializable
data class ProfileDto(
    val id: String,
    val name: String,
    val gender: String, // "male" or "female"
    val birth_year: Int,
    val birth_month: Int,
    val birth_day: Int,
    val birth_hour: Int,
    val birth_minute: Int,
    val latitude: Double,
    val longitude: Double,
    val timezone_id: String,
    val notes: String = "",
    val created_at: String = "",
    val updated_at: String = ""
)

@Serializable
data class CreateProfileRequest(
    val name: String,
    val gender: String,
    val birth_year: Int,
    val birth_month: Int,
    val birth_day: Int,
    val birth_hour: Int,
    val birth_minute: Int,
    val latitude: Double,
    val longitude: Double,
    val timezone_id: String? = null,
    val notes: String? = null
)

@Serializable
data class UpdateProfileRequest(
    val name: String,
    val gender: String,
    val birth_year: Int,
    val birth_month: Int,
    val birth_day: Int,
    val birth_hour: Int,
    val birth_minute: Int,
    val latitude: Double,
    val longitude: Double,
    val timezone_id: String? = null,
    val notes: String? = null
)

@Serializable
data class HoraryDto(
    val id: String,
    val title: String,
    val category: String,
    val question_text: String,
    val query_year: Int,
    val query_month: Int,
    val query_day: Int,
    val query_hour: Int,
    val query_minute: Int,
    val query_second: Double,
    val latitude: Double,
    val longitude: Double,
    val timezone_id: String,
    val querent_profile_id: String? = null,
    val result_notes: String = "",
    val created_at: String = "",
    val updated_at: String = ""
)

@Serializable
data class CreateHoraryRequest(
    val title: String,
    val category: String,
    val question_text: String,
    val query_year: Int,
    val query_month: Int,
    val query_day: Int,
    val query_hour: Int,
    val query_minute: Int,
    val query_second: Double,
    val latitude: Double,
    val longitude: Double,
    val timezone_id: String? = null,
    val querent_profile_id: String? = null,
    val result_notes: String? = null
)

@Serializable
data class UpdateHoraryRequest(
    val title: String,
    val category: String,
    val question_text: String,
    val query_year: Int,
    val query_month: Int,
    val query_day: Int,
    val query_hour: Int,
    val query_minute: Int,
    val query_second: Double,
    val latitude: Double,
    val longitude: Double,
    val timezone_id: String? = null,
    val querent_profile_id: String? = null,
    val result_notes: String? = null
)

@Serializable
data class PillarData(
    val stem_index: Int,
    val branch_index: Int
) {
    companion object {
        val STEMS = listOf("갑", "을", "병", "정", "무", "기", "경", "신", "임", "계")
        val STEMS_HANJA = listOf("甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸")
        val BRANCHES = listOf("자", "축", "인", "묘", "진", "사", "오", "미", "신", "유", "술", "해")
        val BRANCHES_HANJA = listOf("子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥")
    }

    val korean: String get() = "${STEMS[stem_index % 10]}${BRANCHES[branch_index % 12]}"
    val hanja: String get() = "${STEMS_HANJA[stem_index % 10]}${BRANCHES_HANJA[branch_index % 12]}"
}

@Serializable
data class DaeUnPeriodData(
    val order: Int,
    val start_age: Int,
    val start_jd: Double,
    val pillar: PillarData
)

@Serializable
data class SajuChartData(
    val year_pillar: PillarData,
    val month_pillar: PillarData,
    val day_pillar: PillarData,
    val hour_pillar: PillarData,
    val daeun_number: Int,
    val is_forward_daeun: Boolean,
    val daeun_periods: List<DaeUnPeriodData>
)

@Serializable
data class WesternPlanetData(
    val name: String,
    val longitude: Double,
    val sign_index: Int,
    val sign_degree: Double,
    val house_number: Int,
    val is_retrograde: Boolean
)

@Serializable
data class WesternNodeData(
    val name: String,
    val longitude: Double,
    val sign_index: Int,
    val sign_degree: Double,
    val house_number: Int,
    val is_retrograde: Boolean
)

@Serializable
data class WesternAspectData(
    val planet1: String,
    val planet2: String,
    val aspect_type: String,
    val exact_angle: Double,
    val actual_angle: Double,
    val orb: Double
)

@Serializable
data class WesternChartData(
    val ascendant: Double,
    val midheaven: Double,
    val house_cusps: List<Double>,
    val planets: List<WesternPlanetData>,
    val nodes: List<WesternNodeData>,
    val aspects: List<WesternAspectData>
)

@Serializable
data class VedicPlanetData(
    val name: String,
    val sidereal_longitude: Double,
    val sign_index: Int,
    val sign_degree: Double,
    val nakshatra_name: String,
    val nakshatra_pada: Int,
    val is_retrograde: Boolean
)

@Serializable
data class VargaPlacementData(
    val planet_name: String,
    val sign_index: Int,
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
    return String.format(java.util.Locale.US, "%04d.%02d.%02d", year, month, day)
}

@Serializable
data class VedicChartData(
    val lagna_sidereal: Double,
    val lagna_sign: Int,
    val wholesign_bhavas: List<Double>,
    val planets: List<VedicPlanetData>,
    val vargas: Map<String, VargaChartData>,
    val vimshottari_dashas: List<DashaPeriodData>
)

@Serializable
data class FortuneChartOutput(
    val timezone_id: String,
    val utc_offset_hours: Double,
    val utc_year: Int,
    val utc_month: Int,
    val utc_day: Int,
    val utc_hour: Int,
    val utc_minute: Int,
    val utc_second: Double,
    val western: WesternChartData,
    val vedic: VedicChartData,
    val saju: SajuChartData
)
