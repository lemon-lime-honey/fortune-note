package com.fortunexample.note.ui.components

import androidx.compose.ui.graphics.Color
import com.fortunexample.note.data.VEDIC_RASI_NAMES
import com.fortunexample.note.data.VedicPlanetData
import com.fortunexample.note.data.WESTERN_ZODIAC_SIGNS
import com.fortunexample.note.ui.theme.CelestialBlue
import com.fortunexample.note.ui.theme.CrimsonAlert
import com.fortunexample.note.ui.theme.SajuSlate
import com.fortunexample.note.ui.theme.TextPrimary
import com.fortunexample.note.ui.theme.TextSecondary
import java.util.Locale
import kotlin.math.abs

val ZODIAC_CODES = listOf(
    "ARI", "TAU", "GEM", "CAN", "LEO", "VIR",
    "LIB", "SCO", "SAG", "CAP", "AQU", "PIS"
)

fun signCode(index: Int): String = ZODIAC_CODES.getOrElse(((index % 12) + 12) % 12) { "ARI" }

fun signFullName(index: Int): String = WESTERN_ZODIAC_SIGNS.getOrElse(((index % 12) + 12) % 12) { "Aries" }

fun vedicRasiCode(index: Int): String {
    val idx = ((index % 12) + 12) % 12
    return "${VEDIC_RASI_NAMES[idx]} (${ZODIAC_CODES[idx]})"
}

private val BODY_PREFIXES = listOf("Dir ", "Prog ", "Nat ", "Tr ", "SA ", "Return ", "Progressed ")

fun stripBodyPrefix(name: String): String {
    var s = name.trim()
    for (prefix in BODY_PREFIXES) {
        if (s.startsWith(prefix, ignoreCase = true)) {
            s = s.substring(prefix.length).trim()
            break
        }
    }
    return s
}

private val WESTERN_BODY_CODES = mapOf(
    "sun" to "SUN", "surya" to "SUN",
    "moon" to "MON", "chandra" to "MON",
    "mercury" to "MER", "budha" to "MER",
    "venus" to "VEN", "shukra" to "VEN",
    "mars" to "MAR", "mangala" to "MAR", "kuja" to "MAR",
    "jupiter" to "JUP", "guru" to "JUP", "brihaspati" to "JUP",
    "saturn" to "SAT", "shani" to "SAT",
    "uranus" to "URA", "neptune" to "NEP", "pluto" to "PLU",
    "north node" to "NN", "northnode" to "NN", "true node" to "NN", "mean node" to "NN", "rahu" to "NN",
    "south node" to "SN", "southnode" to "SN", "ketu" to "SN",
    "ascendant" to "ASC", "asc" to "ASC", "lagna" to "ASC",
    "midheaven" to "MC", "mc" to "MC",
    "descendant" to "DSC", "dsc" to "DSC",
    "imum coeli" to "IC", "immum coeli" to "IC", "ic" to "IC",
    "part of fortune" to "POF", "lot of fortune" to "POF", "fortune" to "POF",
    "part of spirit" to "POS", "lot of spirit" to "POS", "spirit" to "POS"
)

fun bodyCode(name: String): String {
    val clean = stripBodyPrefix(name)
    return WESTERN_BODY_CODES[clean.lowercase(Locale.US)]
        ?: clean.take(3).uppercase(Locale.US)
}

private val VEDIC_GRAHA_CODES = mapOf(
    "sun" to "SUR", "surya" to "SUR",
    "moon" to "CHA", "chandra" to "CHA",
    "mars" to "MAN", "mangala" to "MAN", "kuja" to "MAN",
    "mercury" to "BUD", "budha" to "BUD",
    "jupiter" to "GUR", "guru" to "GUR",
    "venus" to "SHU", "shukra" to "SHU",
    "saturn" to "SHA", "shani" to "SHA",
    "rahu" to "RAH", "north node" to "RAH", "northnode" to "RAH",
    "ketu" to "KET", "south node" to "KET", "southnode" to "KET",
    "lagna" to "LAG", "ascendant" to "LAG", "asc" to "LAG"
)

fun vedicGrahaCode(name: String): String {
    val clean = stripBodyPrefix(name)
    return VEDIC_GRAHA_CODES[clean.lowercase(Locale.US)] ?: bodyCode(clean)
}

private data class ParsedAspect(val baseLower: String, val isConverse: Boolean)

private fun parseAspectLabel(aspectType: String): ParsedAspect {
    val trimmed = aspectType.trim()
    val lower = trimmed.lowercase(Locale.US)
    val isConverse = lower.endsWith("(converse)") || lower.endsWith("(c)")
    val base = if (isConverse) {
        lower.removeSuffix("(converse)").removeSuffix("(c)").trim()
    } else {
        lower
    }
    return ParsedAspect(base, isConverse)
}

private val ASPECT_FULL_CODES = mapOf(
    "conjunction" to "CNJ 0°", "conj" to "CNJ 0°",
    "sextile" to "SXT 60°", "sext" to "SXT 60°",
    "square" to "SQR 90°", "sqr" to "SQR 90°",
    "trine" to "TRI 120°", "tri" to "TRI 120°",
    "opposition" to "OPP 180°", "opp" to "OPP 180°",
    "quincunx" to "QNX 150°", "inconjunct" to "QNX 150°",
    "semisextile" to "SSX 30°",
    "semisquare" to "SSQ 45°",
    "sesquiquadrate" to "SES 135°"
)

private val ASPECT_SHORT_CODES = mapOf(
    "conjunction" to "CNJ", "conj" to "CNJ",
    "sextile" to "SXT", "sext" to "SXT",
    "square" to "SQR", "sqr" to "SQR",
    "trine" to "TRI", "tri" to "TRI",
    "opposition" to "OPP", "opp" to "OPP"
)

fun aspectCode(aspectType: String): String {
    val parsed = parseAspectLabel(aspectType)
    val code = ASPECT_FULL_CODES[parsed.baseLower]
        ?: parsed.baseLower.take(7).uppercase(Locale.US)
    return if (parsed.isConverse) "$code (C)" else code
}

fun aspectShortCode(aspectType: String): String {
    val parsed = parseAspectLabel(aspectType)
    val code = ASPECT_SHORT_CODES[parsed.baseLower]
        ?: parsed.baseLower.take(3).uppercase(Locale.US)
    return if (parsed.isConverse) "$code (C)" else code
}

fun isHardAspect(aspectType: String): Boolean {
    val lower = aspectType.trim().lowercase(Locale.US)
    return lower.contains("square") || lower.contains("opposition")
}

fun isSoftAspect(aspectType: String): Boolean {
    val lower = aspectType.trim().lowercase(Locale.US)
    return lower.contains("trine") || lower.contains("sextile")
}

fun aspectSemanticColor(aspectType: String): Color = when {
    isHardAspect(aspectType) -> CrimsonAlert
    isSoftAspect(aspectType) -> CelestialBlue
    else -> TextPrimary
}

fun formatDegMin(degInSign: Double): String {
    val normalized = ((degInSign % 30.0) + 30.0) % 30.0
    val totalMinutes = (normalized * 60.0).toInt()
    val d = (totalMinutes / 60).coerceIn(0, 29)
    val m = (totalMinutes % 60).coerceIn(0, 59)
    return String.format(Locale.US, "%02d°%02d'", d, m)
}

fun formatSignAndDeg(signIndex: Int, degInSign: Double): String =
    "${signCode(signIndex)} ${formatDegMin(degInSign)}"

fun formatLongitudeSignDeg(longitude: Double): String {
    val norm = ((longitude % 360.0) + 360.0) % 360.0
    val signIdx = (norm / 30.0).toInt().coerceIn(0, 11)
    return "${signCode(signIdx)} ${formatDegMin(norm - signIdx * 30.0)}"
}

fun formatOrb(orb: Double): String = String.format(Locale.US, "%04.2f°", abs(orb))

fun formatSignedScore(score: Int): String = when {
    score > 0 -> String.format(Locale.US, "+%d", score)
    score < 0 -> String.format(Locale.US, "%d", score)
    else -> "0"
}

data class VedicGrahaStatusDisplay(
    val tag: String,
    val color: Color
)

private val VEDIC_DIGNITY_CODES = mapOf(
    "Exalted" to "EXLT",
    "Moolatrikona" to "MOOL",
    "Own Sign" to "OWN",
    "OwnSign" to "OWN",
    "Great Friend" to "GFRN",
    "Friend" to "FRND",
    "Enemy" to "ENMY",
    "Great Enemy" to "GENM",
    "Debilitated" to "DEBL"
)

private val POSITIVE_VEDIC_DIGNITIES = setOf("Exalted", "Moolatrikona", "Own Sign", "OwnSign")

fun formatVedicGrahaStatus(planet: VedicPlanetData): VedicGrahaStatusDisplay {
    val isUnimpairedCombust = planet.combustion.contains("Unimpaired", ignoreCase = true)
    val combustionTag = when {
        planet.combustion == "None" || planet.combustion.isBlank() -> null
        planet.combustion == "DeeplyCombust" -> "DCMB"
        isUnimpairedCombust -> "CMB·U"
        else -> "CMB"
    }
    val tag = buildList {
        if (planet.dignity.isNotBlank() && planet.dignity != "Neutral") {
            add(VEDIC_DIGNITY_CODES[planet.dignity] ?: planet.dignity.take(4).trim().uppercase(Locale.US))
        }
        if (combustionTag != null) add(combustionTag)
        if (planet.is_retrograde && planet.name != "Rahu" && planet.name != "Ketu") add("RX")
    }.joinToString("·").ifEmpty { "NORM" }

    val color = when {
        planet.dignity in POSITIVE_VEDIC_DIGNITIES -> CelestialBlue
        planet.dignity == "Debilitated" || planet.combustion == "Combust" || planet.combustion == "DeeplyCombust" -> CrimsonAlert
        isUnimpairedCombust -> SajuSlate
        else -> TextSecondary
    }
    return VedicGrahaStatusDisplay(tag, color)
}
