package com.fortunexample.note.ui.components

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.fortunexample.note.data.PlanetDignityData
import com.fortunexample.note.data.WesternTraditionalData
import com.fortunexample.note.ui.theme.*
import java.util.Locale

private fun essentialDignityTags(d: PlanetDignityData): String = buildList {
    if (d.is_in_domicile) add("DOM")
    if (d.is_in_exaltation) add("EXA")
    if (d.is_in_triplicity) add("TRI")
    if (d.is_in_term) add("TRM")
    if (d.is_in_face) add("FAC")
    if (d.is_in_detriment) add("DET")
    if (d.is_in_fall) add("FAL")
    if (d.is_peregrine && isEmpty()) add("PER")
}.joinToString("·").ifEmpty { "-" }

@Composable
internal fun DignitiesMatrixSubSection(data: WesternTraditionalData) {
    var expandedPlanet by remember { mutableStateOf<String?>(null) }

    AstroPanelCard(
        title = "Essential & Accidental Dignities",
        subtitle = "Tap any row to inspect Accidental Lilly score breakdown",
        contentPadding = PaddingValues(0.dp),
        headerAction = {
            SectionCopyButton("Dignities") {
                buildString {
                    appendLine("[Essential & Accidental Dignities]")
                    data.planet_dignities.forEach { d ->
                        appendLine(
                            "${bodyCode(d.planet)} (${formatSignAndDeg(d.sign_index, d.sign_degree)}, H${d.house_number}): " +
                                "Ess ${formatSignedScore(d.essential_score)}, Acc ${formatSignedScore(d.accidental_score)}, Total ${formatSignedScore(d.total_lilly_score)}"
                        )
                    }
                }.trimEnd()
            }
        }
    ) {
        val cols = listOf(
            TableColSpec("BODY", 0.95f, TextAlign.Start, monospace = true),
            TableColSpec("POS", 1.35f, TextAlign.Start, monospace = true),
            TableColSpec("H", 0.55f, TextAlign.Center, monospace = true),
            TableColSpec("ESSENTIAL", 1.55f, TextAlign.Start, monospace = true),
            TableColSpec("ESS", 0.65f, TextAlign.End, monospace = true),
            TableColSpec("ACC", 0.65f, TextAlign.End, monospace = true),
            TableColSpec("TOT", 0.75f, TextAlign.End, monospace = true)
        )
        AstroTableHeader(cols)
        data.planet_dignities.forEachIndexed { idx, d ->
            val essColor = when { d.essential_score > 0 -> CelestialBlue; d.essential_score < 0 -> CrimsonAlert; else -> TextSecondary }
            val totColor = when { d.total_lilly_score >= 5 -> CelestialBlue; d.total_lilly_score <= -5 -> CrimsonAlert; else -> TextPrimary }
            val isExpanded = expandedPlanet == d.planet
            AstroTableRow(
                columns = cols,
                striped = idx % 2 == 1,
                onClick = { expandedPlanet = if (isExpanded) null else d.planet },
                cells = listOf(
                    TableCellData(bodyCode(d.planet), TextPrimary, FontWeight.Bold),
                    TableCellData(formatSignAndDeg(d.sign_index, d.sign_degree), TextPrimary),
                    TableCellData("${d.house_number}", TextSecondary),
                    TableCellData(essentialDignityTags(d), essColor, FontWeight.SemiBold),
                    TableCellData(formatSignedScore(d.essential_score), essColor, FontWeight.SemiBold),
                    TableCellData(formatSignedScore(d.accidental_score), TextSecondary),
                    TableCellData(formatSignedScore(d.total_lilly_score), totColor, FontWeight.Bold)
                )
            )
            if (isExpanded) {
                AccidentalBreakdownDrawer(d)
            }
        }
    }
}

@Composable
private fun AccidentalBreakdownDrawer(d: PlanetDignityData) {
    val reasons = d.accidental_reasons.ifEmpty { d.accidental_factors }
    Column(
        modifier = Modifier
            .fillMaxWidth()
            .background(SurfaceLight)
            .padding(horizontal = 12.dp, vertical = 8.dp),
        verticalArrangement = Arrangement.spacedBy(3.dp)
    ) {
        Text(
            text = "${d.planet.uppercase(Locale.US)} ACCIDENTAL BREAKDOWN",
            fontSize = 10.sp,
            fontWeight = FontWeight.Bold,
            color = TextSecondary,
            style = TabularMonoStyle
        )
        if (reasons.isEmpty()) {
            Text("No accidental modifiers recorded.", fontSize = 11.sp, color = TextSecondary)
        } else {
            reasons.forEach { r ->
                Text(
                    text = "• $r",
                    fontSize = 11.sp,
                    color = TextPrimary,
                    style = TabularSansStyle,
                    maxLines = 1,
                    overflow = TextOverflow.Ellipsis
                )
            }
        }
    }
    HorizontalDivider(color = BorderColor, thickness = 0.5.dp)
}

@Composable
internal fun HoursAndMoonSubSection(
    data: WesternTraditionalData,
    isHorary: Boolean
) {
    Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
        PlanetaryHoursCard(data, isHorary)
        LunarConditionCard(data, isHorary)
        data.prenatal_syzygy?.let { PrenatalSyzygyCard(it) }
    }
}

@Composable
private fun PlanetaryHoursCard(data: WesternTraditionalData, isHorary: Boolean) {
    val ph = data.planetary_hours
    AstroPanelCard(
        title = if (isHorary) "Planetary Hours & Radicality" else "Planetary Hours",
        headerAction = if (isHorary) {
            {
                AstroStatusBadge(
                    text = if (ph.is_radical) "RADICAL" else "NON-RADICAL",
                    color = if (ph.is_radical) CelestialBlue else CrimsonAlert
                )
            }
        } else null
    ) {
        val baseItems = listOf(
            MetricTileItem("DAY RULER", bodyCode(ph.day_ruler), ph.day_ruler),
            MetricTileItem("HOUR RULER", bodyCode(ph.hour_ruler), ph.hour_ruler),
            MetricTileItem("HOUR NO.", "#${ph.hour_number}", if (ph.is_day_hour) "DIURNAL" else "NOCTURNAL")
        )
        val items = if (isHorary) {
            baseItems + MetricTileItem("STATUS", if (ph.is_radical) "MATCH" else "WARN", "HORARY")
        } else {
            baseItems
        }
        CompactMetricGrid(items = items, columns = items.size)
        if (isHorary && ph.radicality_reason.isNotBlank()) {
            Spacer(modifier = Modifier.height(8.dp))
            Text(ph.radicality_reason, fontSize = 11.sp, color = TextSecondary, maxLines = 2, overflow = TextOverflow.Ellipsis)
        }
        if (isHorary && ph.considerations.isNotEmpty()) {
            Spacer(modifier = Modifier.height(6.dp))
            ph.considerations.forEach { c ->
                Text("• $c", fontSize = 11.sp, color = CrimsonAlert, fontWeight = FontWeight.Medium, maxLines = 1, overflow = TextOverflow.Ellipsis)
            }
        }
    }
}

@Composable
private fun LunarConditionCard(data: WesternTraditionalData, isHorary: Boolean) {
    val mc = data.moon_condition
    val baseItems = listOf(
        MetricTileItem("PHASE", mc.lunar_phase_name.uppercase(Locale.US), if (mc.is_waxing) "WAX" else "WAN"),
        MetricTileItem("ELONGATION", String.format(Locale.US, "%.1f°", mc.elongation_deg), "SUN-MON")
    )
    val items = if (isHorary) {
        baseItems + listOf(
            MetricTileItem(
                tag = "VOID OF COURSE",
                primary = if (mc.is_void_of_course) "YES (VOC)" else "NO (ACTIVE)",
                secondary = if (mc.is_in_tincture_exception) "EXEMPT" else "",
                highlightColor = if (mc.is_void_of_course) CrimsonAlert else CelestialBlue
            ),
            MetricTileItem(
                tag = "VIA COMBUSTA",
                primary = if (mc.is_via_combusta) "YES" else "CLEAR",
                secondary = "15°LIB-15°SCO",
                highlightColor = if (mc.is_via_combusta) CrimsonAlert else null
            )
        )
    } else {
        baseItems
    }

    AstroPanelCard(title = if (isHorary) "Lunar Phase & Condition" else "Lunar Phase") {
        CompactMetricGrid(items = items, columns = 2)
        if (isHorary) {
            Spacer(modifier = Modifier.height(8.dp))
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically
            ) {
                Text("NEXT APPLYING ASPECT", fontSize = 10.sp, fontWeight = FontWeight.Bold, color = TextSecondary, style = TabularMonoStyle)
                Text(
                    text = mc.next_applying_aspect ?: "None before sign exit",
                    fontSize = 11.sp,
                    fontWeight = FontWeight.SemiBold,
                    color = TextPrimary,
                    style = TabularMonoStyle,
                    maxLines = 1
                )
            }
        }
    }
}

@Composable
internal fun ReceptionsAndCuspsSubSection(data: WesternTraditionalData) {
    Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
        ReceptionsAndPerfectionCard(data)
        HouseCuspAlmutensCard(data)
        AntisciaTableCard(data)
    }
}

@Composable
private fun ReceptionsAndPerfectionCard(data: WesternTraditionalData) {
    val perf = data.perfection
    val hasRows = data.mutual_receptions.isNotEmpty() ||
        perf.translation_of_light.isNotEmpty() ||
        perf.collection_of_light.isNotEmpty() ||
        perf.besiegement.isNotEmpty()
    if (!hasRows) return

    AstroPanelCard(title = "Mutual Receptions & Perfection", contentPadding = PaddingValues(0.dp)) {
        val cols = listOf(
            TableColSpec("TYPE", 1.1f, TextAlign.Start, monospace = true),
            TableColSpec("BODIES / DETAIL", 2.9f, TextAlign.Start)
        )
        AstroTableHeader(cols)
        data.mutual_receptions.forEachIndexed { idx, mr ->
            AstroTableRow(
                columns = cols,
                striped = idx % 2 == 1,
                cells = listOf(
                    TableCellData(mr.reception_type.uppercase(Locale.US), CelestialBlue, FontWeight.Bold),
                    TableCellData("${bodyCode(mr.planet1)} ↔ ${bodyCode(mr.planet2)} · ${mr.description}", TextPrimary)
                )
            )
        }
        perf.translation_of_light.forEach {
            AstroTableRow(cols, listOf(TableCellData("TRANSLATION", CelestialBlue, FontWeight.Bold), TableCellData(it, TextPrimary)))
        }
        perf.collection_of_light.forEach {
            AstroTableRow(cols, listOf(TableCellData("COLLECTION", CelestialBlue, FontWeight.Bold), TableCellData(it, TextPrimary)))
        }
        perf.besiegement.forEach {
            AstroTableRow(cols, listOf(TableCellData("BESIEGEMENT", CrimsonAlert, FontWeight.Bold), TableCellData(it, CrimsonAlert)))
        }
    }
}

@Composable
private fun HouseCuspAlmutensCard(data: WesternTraditionalData) {
    if (data.cusp_dignities.isEmpty()) return
    AstroPanelCard(title = "House Cusp Rulers & Almutens", contentPadding = PaddingValues(0.dp)) {
        val cols = listOf(
            TableColSpec("H", 0.55f, TextAlign.Start, monospace = true),
            TableColSpec("CUSP", 1.35f, TextAlign.Start, monospace = true),
            TableColSpec("DOM", 0.7f, TextAlign.Center, monospace = true),
            TableColSpec("EXA", 0.7f, TextAlign.Center, monospace = true),
            TableColSpec("TRI", 0.7f, TextAlign.Center, monospace = true),
            TableColSpec("TRM", 0.7f, TextAlign.Center, monospace = true),
            TableColSpec("ALM", 0.9f, TextAlign.End, monospace = true)
        )
        AstroTableHeader(cols)
        data.cusp_dignities.forEachIndexed { idx, c ->
            AstroTableRow(
                columns = cols,
                striped = idx % 2 == 1,
                cells = listOf(
                    TableCellData("H${c.house_number}", TextPrimary, FontWeight.Bold),
                    TableCellData(formatSignAndDeg(c.sign_index, c.sign_degree), TextPrimary),
                    TableCellData(bodyCode(c.domicile), TextSecondary),
                    TableCellData(c.exaltation?.let { bodyCode(it) } ?: "-", TextSecondary),
                    TableCellData(bodyCode(c.triplicity), TextSecondary),
                    TableCellData(bodyCode(c.term), TextSecondary),
                    TableCellData("${bodyCode(c.almuten)}(${c.almuten_score})", CelestialBlue, FontWeight.Bold)
                )
            )
        }
    }
}

@Composable
private fun AntisciaTableCard(data: WesternTraditionalData) {
    if (data.antiscia.isEmpty()) return
    AstroPanelCard(title = "Antiscia & Contra-Antiscia", contentPadding = PaddingValues(0.dp)) {
        val cols = listOf(
            TableColSpec("BODY", 0.9f, TextAlign.Start, monospace = true),
            TableColSpec("NATAL", 1.3f, TextAlign.Start, monospace = true),
            TableColSpec("ANTISCION", 1.3f, TextAlign.Start, monospace = true),
            TableColSpec("CONTRA", 1.3f, TextAlign.End, monospace = true)
        )
        AstroTableHeader(cols)
        data.antiscia.forEachIndexed { idx, a ->
            AstroTableRow(
                columns = cols,
                striped = idx % 2 == 1,
                cells = listOf(
                    TableCellData(bodyCode(a.body_name), TextPrimary, FontWeight.Bold),
                    TableCellData(formatLongitudeSignDeg(a.true_longitude), TextSecondary),
                    TableCellData(formatSignAndDeg(a.antiscion_sign_index, a.antiscion_degree), CelestialBlue, FontWeight.SemiBold),
                    TableCellData(formatLongitudeSignDeg(a.contra_antiscion_longitude), TextSecondary)
                )
            )
        }
    }
}

@Composable
internal fun LotsAndStarsSubSection(data: WesternTraditionalData) {
    Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
        ArabicLotsTableCard(
            title = "Arabic Parts & Lots",
            rulerColumnHeader = "RULER",
            lots = data.arabic_parts
        )
        FixedStarConjunctionsCard(data.fixed_star_conjunctions)
    }
}
