package com.fortunexample.note.data

import kotlinx.serialization.Serializable

@Serializable
data class ProfileDto(
    val id: String,
    val name: String,
    val gender: String,
    val birth_year: Int,
    val birth_month: Int,
    val birth_day: Int,
    val birth_hour: Int,
    val birth_minute: Int,
    val birth_second: Double = 0.0,
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
    val birth_second: Double = 0.0,
    val latitude: Double,
    val longitude: Double,
    val timezone_id: String? = null,
    val notes: String? = null
)

typealias UpdateProfileRequest = CreateProfileRequest

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

typealias UpdateHoraryRequest = CreateHoraryRequest

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

@Serializable
data class SajuCalculationOutputDto(
    val timezone_id: String,
    val utc_offset_hours: Double,
    val saju: SajuChartData
)

@Serializable
data class WesternCalculationOutputDto(
    val timezone_id: String,
    val utc_offset_hours: Double,
    val western: WesternChartData
)

@Serializable
data class VedicCalculationOutputDto(
    val timezone_id: String,
    val utc_offset_hours: Double,
    val vedic: VedicChartData
)

@Serializable
data class ImportSummaryDto(
    val imported_profiles: Int,
    val imported_horary: Int,
    val message: String
)

data class BackupPreview(
    val app: String,
    val version: Int,
    val exportedAt: String,
    val profileCount: Int,
    val horaryCount: Int,
    val rawJson: String
)

val WESTERN_ZODIAC_SIGNS = listOf(
    "Aries", "Taurus", "Gemini", "Cancer", "Leo", "Virgo",
    "Libra", "Scorpio", "Sagittarius", "Capricorn", "Aquarius", "Pisces"
)

fun getZodiacSignName(index: Int): String =
    WESTERN_ZODIAC_SIGNS.getOrElse(((index % 12) + 12) % 12) { "" }

val WESTERN_DOMICILE_RULERS = listOf(
    "Mars", "Venus", "Mercury", "Moon", "Sun", "Mercury",
    "Venus", "Mars", "Jupiter", "Saturn", "Saturn", "Jupiter"
)

fun getZodiacSignRuler(index: Int): String =
    WESTERN_DOMICILE_RULERS.getOrElse(((index % 12) + 12) % 12) { "" }

fun findHouseNumber(lon: Double, cusps: List<Double>): Int {
    if (cusps.size < 12) return 1
    val normalizedLon = (lon % 360.0 + 360.0) % 360.0
    for (i in 0 until 12) {
        val start = (cusps[i] % 360.0 + 360.0) % 360.0
        val end = (cusps[(i + 1) % 12] % 360.0 + 360.0) % 360.0
        val inHouse = if (start < end) {
            normalizedLon >= start && normalizedLon < end
        } else {
            normalizedLon >= start || normalizedLon < end
        }
        if (inHouse) return i + 1
    }
    return 1
}
