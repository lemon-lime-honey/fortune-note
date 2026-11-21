package com.fortunexample.note.data

import kotlinx.serialization.Serializable
import java.util.Locale

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
    val star_latitude: Double = 0.0,
    val star_magnitude: Double = 0.0,
    val body_longitude: Double,
    val orb: Double,
    val is_visible: Boolean = true,
    val max_altitude: Double = 0.0,
    val is_qualified: Boolean = true,
    val qualification_reason: String = "",
    val nature: String,
    val meaning: String
)

@Serializable
data class FixedStarInfoData(
    val name: String,
    val korean_name: String = "",
    val longitude: Double,
    val latitude: Double = 0.0,
    val magnitude: Double = 0.0,
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
    val condition_label: String,
    val bound_ruler: String = "",
    val triplicity_day_ruler: String = "",
    val triplicity_night_ruler: String = "",
    val triplicity_participating_ruler: String = "",
    val is_in_triplicity: Boolean = false
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
data class PredictiveSuiteRequestDto(
    val target_year: Int,
    val target_month: Int,
    val target_day: Int,
    val target_hour: Int? = null,
    val target_minute: Int? = null,
    val house_system: String? = null
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
data class ReturnAspectContactData(
    val return_point: String,
    val return_longitude: Double,
    val natal_point: String,
    val natal_longitude: Double,
    val aspect_type: String,
    val orb: Double
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
    val planets: List<ProgressedBodyData> = emptyList(),
    val natal_aspects: List<ReturnAspectContactData> = emptyList()
)

@Serializable
data class PlanetaryReturnData(
    val target_year: Int,
    val return_body: String,
    val return_date_utc: String,
    val exact_jd_ut1: Double,
    val body_longitude: Double,
    val is_retrograde: Boolean,
    val speed: Double,
    val ascendant: Double,
    val midheaven: Double,
    val house_cusps: List<Double> = emptyList(),
    val return_planets: List<ProgressedBodyData> = emptyList(),
    val natal_aspects: List<ReturnAspectContactData> = emptyList(),
    val return_internal_aspects: List<PredictiveAspectData> = emptyList()
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
    val planetary_returns: List<PlanetaryReturnData> = emptyList(),
    val transits: List<TransitContactData> = emptyList(),
    val primary_directions: List<PrimaryDirectionContactData> = emptyList()
)

private val WHOLE_SIGN_NAMES = setOf("wholesign", "whole_sign", "whole", "hellenistic")

fun isWholeSignHouseSystem(western: WesternChartData?, selectedHouse: String? = null): Boolean {
    if (selectedHouse != null && selectedHouse.lowercase(Locale.US) in WHOLE_SIGN_NAMES) return true
    if (western?.house_system?.lowercase(Locale.US) in WHOLE_SIGN_NAMES) return true
    return western?.hellenistic != null && western.traditional == null
}

fun mergeSolarAndPlanetaryReturns(data: PredictiveSuiteData): List<PlanetaryReturnData> = buildList {
    val sr = data.solar_return
    val hasSun = data.planetary_returns.any { it.return_body.equals("Sun", ignoreCase = true) }
    if (sr != null && !hasSun) {
        add(
            PlanetaryReturnData(
                target_year = sr.return_year,
                return_body = "Sun",
                return_date_utc = sr.return_date_utc,
                exact_jd_ut1 = sr.exact_jd_ut1,
                body_longitude = sr.sun_longitude,
                is_retrograde = false,
                speed = 1.0,
                ascendant = sr.ascendant,
                midheaven = sr.midheaven,
                house_cusps = sr.house_cusps,
                return_planets = sr.planets,
                natal_aspects = sr.natal_aspects
            )
        )
    }
    addAll(data.planetary_returns)
}
