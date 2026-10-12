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
    val is_retrograde: Boolean,
    val speed: Double = 0.0,
    val latitude: Double = 0.0
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
    val orb: Double,
    val is_applying: Boolean = false
)

@Serializable
data class PlanetDignityData(
    val planet: String,
    val longitude: Double,
    val sign_index: Int,
    val sign_degree: Double = 0.0,
    val house_number: Int,
    val is_in_domicile: Boolean,
    val is_in_exaltation: Boolean,
    val is_in_triplicity: Boolean,
    val is_in_term: Boolean,
    val is_in_face: Boolean,
    val is_in_detriment: Boolean,
    val is_in_fall: Boolean,
    val is_peregrine: Boolean,
    val essential_score: Int,
    val accidental_score: Int,
    val total_lilly_score: Int,
    val accidental_factors: List<String> = emptyList(),
    val accidental_reasons: List<String> = emptyList()
)

@Serializable
data class MoonConditionData(
    val is_void_of_course: Boolean,
    val is_via_combusta: Boolean,
    val is_in_tincture_exception: Boolean,
    val lunar_phase_name: String,
    val is_waxing: Boolean,
    val elongation_deg: Double,
    val next_applying_aspect: String? = null
)

@Serializable
data class PlanetaryHoursData(
    val day_ruler: String,
    val hour_ruler: String,
    val hour_number: Int,
    val is_day_hour: Boolean,
    val is_radical: Boolean,
    val radicality_reason: String,
    val considerations: List<String> = emptyList()
)

@Serializable
data class TraditionalAspectData(
    val planet1: String,
    val planet2: String,
    val aspect_type: String,
    val exact_angle: Double = 0.0,
    val actual_angle: Double = 0.0,
    val orb: Double,
    val max_allowed_orb: Double,
    val is_applying: Boolean,
    val is_partile: Boolean,
    val degrees_to_perfection: Double = 0.0
)

@Serializable
data class MutualReceptionData(
    val planet1: String,
    val planet2: String,
    val reception_type: String,
    val description: String
)

@Serializable
data class PerfectionReportData(
    val translation_of_light: List<String> = emptyList(),
    val collection_of_light: List<String> = emptyList(),
    val besiegement: List<String> = emptyList()
)

@Serializable
data class AntisciaPointData(
    val body_name: String,
    val true_longitude: Double,
    val antiscion_longitude: Double,
    val antiscion_sign_index: Int,
    val antiscion_degree: Double,
    val contra_antiscion_longitude: Double
)

@Serializable
data class HouseCuspDignityData(
    val house_number: Int,
    val cusp_degree: Double,
    val sign_index: Int,
    val sign_degree: Double,
    val domicile: String,
    val exaltation: String? = null,
    val triplicity: String,
    val term: String,
    val face: String,
    val almuten: String,
    val almuten_score: Int
)

@Serializable
data class ArabicPartData(
    val name: String,
    val longitude: Double,
    val sign_index: Int,
    val sign_degree: Double,
    val house_number: Int,
    val ruler: String,
    val formula: String = ""
)

@Serializable
data class FixedStarConjunctionData(
    val star_name: String,
    val korean_name: String = "",
    val body_name: String,
    val star_longitude: Double,
    val body_longitude: Double,
    val orb: Double,
    val is_visible: Boolean = true,
    val max_altitude: Double = 0.0,
    val nature: String,
    val meaning: String
)

@Serializable
data class FixedStarInfoData(
    val name: String,
    val korean_name: String = "",
    val longitude: Double,
    val sign_index: Int,
    val sign_degree: Double,
    val declination: Double = 0.0,
    val is_visible: Boolean = true,
    val max_altitude: Double = 0.0,
    val nature: String,
    val meaning: String
)

@Serializable
data class PrenatalSyzygyData(
    val syzygy_type: String,
    val exact_datetime_utc: String,
    val exact_jd_ut1: Double,
    val longitude: Double,
    val sign_index: Int,
    val sign_degree: Double,
    val sun_longitude: Double,
    val moon_longitude: Double
)

@Serializable
data class WesternTraditionalData(
    val planet_dignities: List<PlanetDignityData> = emptyList(),
    val moon_condition: MoonConditionData,
    val planetary_hours: PlanetaryHoursData,
    val traditional_aspects: List<TraditionalAspectData> = emptyList(),
    val antiscia: List<AntisciaPointData> = emptyList(),
    val mutual_receptions: List<MutualReceptionData> = emptyList(),
    val perfection: PerfectionReportData = PerfectionReportData(),
    val cusp_dignities: List<HouseCuspDignityData> = emptyList(),
    val arabic_parts: List<ArabicPartData> = emptyList(),
    val fixed_stars: List<FixedStarInfoData> = emptyList(),
    val fixed_star_conjunctions: List<FixedStarConjunctionData> = emptyList(),
    val prenatal_syzygy: PrenatalSyzygyData? = null
) {
    val perfection_report: PerfectionReportData get() = perfection
    val house_almutens: List<HouseCuspDignityData> get() = cusp_dignities
}

@Serializable
data class WesternChartData(
    val ascendant: Double,
    val midheaven: Double,
    val house_cusps: List<Double>,
    val planets: List<WesternPlanetData>,
    val nodes: List<WesternNodeData>,
    val aspects: List<WesternAspectData>,
    val house_system: String? = null,
    val traditional: WesternTraditionalData? = null,
    val hellenistic: HellenisticData? = null
)

@Serializable
data class HellenisticPlanetConditionData(
    val name: String,
    val longitude: Double,
    val sign_index: Int,
    val sign_degree: Double,
    val house_number: Int,
    val is_in_domicile: Boolean,
    val is_in_exaltation: Boolean,
    val is_in_fall: Boolean,
    val is_ruler_of_asc: Boolean,
    val condition_label: String
)

@Serializable
data class HellenisticAnglePointsData(
    val ascendant: Double,
    val ascendant_sign: Int,
    val ascendant_degree: Double,
    val midheaven: Double,
    val midheaven_sign: Int,
    val midheaven_degree: Double,
    val midheaven_house: Int,
    val is_mc_in_10th: Boolean
)

@Serializable
data class HellenisticData(
    val planet_conditions: List<HellenisticPlanetConditionData> = emptyList(),
    val lots: List<ArabicPartData> = emptyList(),
    val angles: HellenisticAnglePointsData,
    val aspects: List<TraditionalAspectData> = emptyList(),
    val fixed_star_conjunctions: List<FixedStarConjunctionData> = emptyList(),
    val prenatal_syzygy: PrenatalSyzygyData? = null
)

@Serializable
data class AnnualProfectionData(
    val target_age: Int,
    val target_year: Int,
    val profected_house: Int,
    val profected_sign_index: Int,
    val profected_sign_name: String,
    val lord_of_the_year: String,
    val active_natal_planets: List<String> = emptyList()
)

@Serializable
data class SolarArcBodyData(
    val name: String,
    val natal_longitude: Double,
    val directed_longitude: Double,
    val sign_index: Int,
    val sign_degree: Double
)

@Serializable
data class PredictiveAspectData(
    val moving_body: String,
    val natal_body: String,
    val aspect_type: String,
    val orb: Double
)

@Serializable
data class SolarArcResultData(
    val arc_degrees: Double,
    val directed_planets: List<SolarArcBodyData> = emptyList(),
    val contacts: List<PredictiveAspectData> = emptyList()
)

@Serializable
data class ProgressedBodyData(
    val name: String,
    val progressed_longitude: Double,
    val sign_index: Int,
    val sign_degree: Double,
    val is_retrograde: Boolean
)

@Serializable
data class SecondaryProgressionData(
    val progressed_date_utc: String,
    val elapsed_tropical_years: Double,
    val planets: List<ProgressedBodyData> = emptyList(),
    val progressed_lunar_phase: String,
    val contacts: List<PredictiveAspectData> = emptyList()
)

@Serializable
data class SolarReturnData(
    val return_year: Int,
    val return_date_utc: String,
    val exact_jd_ut1: Double,
    val sun_longitude: Double,
    val ascendant: Double,
    val midheaven: Double,
    val house_cusps: List<Double> = emptyList(),
    val planets: List<ProgressedBodyData> = emptyList()
)

@Serializable
data class TransitContactData(
    val transiting_planet: String,
    val transiting_longitude: Double,
    val natal_planet: String,
    val natal_longitude: Double,
    val aspect_type: String,
    val orb: Double
)

@Serializable
data class PrimaryDirectionContactData(
    val promittor: String,
    val significator: String,
    val aspect_type: String,
    val direction_arc_deg: Double,
    val realization_age: Double
)

@Serializable
data class PredictiveSuiteData(
    val target_year: Int,
    val target_month: Int,
    val target_day: Int,
    val target_hour: Int = 0,
    val target_minute: Int = 0,
    val age: Int,
    val profection: AnnualProfectionData,
    val solar_arc: SolarArcResultData,
    val secondary_progression: SecondaryProgressionData? = null,
    val solar_return: SolarReturnData? = null,
    val transits: List<TransitContactData> = emptyList(),
    val primary_directions: List<PrimaryDirectionContactData> = emptyList()
)

@Serializable
data class VedicPlanetData(
    val name: String,
    val sidereal_longitude: Double,
    val sign_index: Int,
    val sign_degree: Double,
    val nakshatra_name: String,
    val nakshatra_pada: Int,
    val is_retrograde: Boolean,
    val latitude: Double = 0.0
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

fun getZodiacSignName(index: Int): String {
    return WESTERN_ZODIAC_SIGNS.getOrElse(((index % 12) + 12) % 12) { "" }
}

val WESTERN_DOMICILE_RULERS = listOf(
    "Mars",     // Aries
    "Venus",    // Taurus
    "Mercury",  // Gemini
    "Moon",     // Cancer
    "Sun",      // Leo
    "Mercury",  // Virgo
    "Venus",    // Libra
    "Mars",     // Scorpio
    "Jupiter",  // Sagittarius
    "Saturn",   // Capricorn
    "Saturn",   // Aquarius
    "Jupiter"   // Pisces
)

fun getZodiacSignRuler(index: Int): String {
    return WESTERN_DOMICILE_RULERS.getOrElse(((index % 12) + 12) % 12) { "" }
}

fun findHouseNumber(lon: Double, cusps: List<Double>): Int {
    if (cusps.size < 12) return 1
    val normalizedLon = (lon % 360.0 + 360.0) % 360.0
    for (i in 0 until 12) {
        val start = (cusps[i] % 360.0 + 360.0) % 360.0
        val end = (cusps[(i + 1) % 12] % 360.0 + 360.0) % 360.0
        if (start < end) {
            if (normalizedLon >= start && normalizedLon < end) return i + 1
        } else {
            if (normalizedLon >= start || normalizedLon < end) return i + 1
        }
    }
    return 1
}

