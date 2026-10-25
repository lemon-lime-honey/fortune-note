package com.fortunexample.note.ui.components

import android.content.ClipData
import android.content.ClipboardManager
import android.content.Context
import android.widget.Toast
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ContentCopy
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.fortunexample.note.data.*
import com.fortunexample.note.ui.theme.*
import java.util.Locale

fun formatWesternBodyName(name: String): String = name.trim()

fun formatWesternAspectName(aspect: String): String = aspect.trim()

fun formatDignityTerm(term: String): String = term.trim()

fun formatAccidentalReason(reason: String): String = reason.trim()

@Composable
fun SectionCopyButton(
    title: String,
    textProvider: () -> String
) {
    val context = LocalContext.current
    IconButton(
        onClick = {
            val clipboard = context.getSystemService(Context.CLIPBOARD_SERVICE) as ClipboardManager
            val clip = ClipData.newPlainText(title, textProvider())
            clipboard.setPrimaryClip(clip)
            Toast.makeText(context, "Copied $title", Toast.LENGTH_SHORT).show()
        },
        modifier = Modifier.size(22.dp)
    ) {
        Icon(
            imageVector = Icons.Default.ContentCopy,
            contentDescription = "Copy $title",
            tint = TextSecondary,
            modifier = Modifier.size(13.dp)
        )
    }
}

@Composable
fun WesternTraditionalSection(
    data: WesternTraditionalData?,
    hellenistic: HellenisticData? = null,
    isHorary: Boolean = false
) {
    var selectedSubTab by remember { mutableIntStateOf(0) }
    val subTabs = buildList {
        if (data != null) {
            add("Dignities")
            add("Hours & Moon")
            add("Receptions & Cusps")
            add("Lots & Stars")
        }
        if (hellenistic != null) add("Hellenistic")
    }

    if (subTabs.isEmpty()) return

    Column(
        modifier = Modifier.fillMaxWidth(),
        verticalArrangement = Arrangement.spacedBy(10.dp)
    ) {
        AstroSubTabBar(
            tabs = subTabs,
            selectedIndex = selectedSubTab.coerceIn(0, subTabs.lastIndex),
            onSelect = { selectedSubTab = it }
        )

        val currentLabel = subTabs[selectedSubTab.coerceIn(0, subTabs.lastIndex)]
        when (currentLabel) {
            "Dignities" -> if (data != null) DignitiesMatrixSubSection(data)
            "Hours & Moon" -> if (data != null) HoursAndMoonSubSection(data, isHorary)
            "Receptions & Cusps" -> if (data != null) ReceptionsAndCuspsSubSection(data)
            "Lots & Stars" -> if (data != null) LotsAndStarsSubSection(data)
            "Hellenistic" -> if (hellenistic != null) HellenisticSubSection(hellenistic)
        }
    }
}

@Composable
private fun DignitiesMatrixSubSection(data: WesternTraditionalData) {
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
            val tags = buildList {
                if (d.is_in_domicile) add("DOM")
                if (d.is_in_exaltation) add("EXA")
                if (d.is_in_triplicity) add("TRI")
                if (d.is_in_term) add("TRM")
                if (d.is_in_face) add("FAC")
                if (d.is_in_detriment) add("DET")
                if (d.is_in_fall) add("FAL")
                if (d.is_peregrine && isEmpty()) add("PER")
            }.joinToString("·").ifEmpty { "-" }

            val essColor = when {
                d.essential_score > 0 -> CelestialBlue
                d.essential_score < 0 -> CrimsonAlert
                else -> TextSecondary
            }
            val totColor = when {
                d.total_lilly_score >= 5 -> CelestialBlue
                d.total_lilly_score <= -5 -> CrimsonAlert
                else -> TextPrimary
            }

            val isExpanded = expandedPlanet == d.planet
            AstroTableRow(
                columns = cols,
                striped = idx % 2 == 1,
                onClick = { expandedPlanet = if (isExpanded) null else d.planet },
                cells = listOf(
                    TableCellData(bodyCode(d.planet), TextPrimary, FontWeight.Bold),
                    TableCellData(formatSignAndDeg(d.sign_index, d.sign_degree), TextPrimary),
                    TableCellData("${d.house_number}", TextSecondary),
                    TableCellData(tags, essColor, FontWeight.SemiBold),
                    TableCellData(formatSignedScore(d.essential_score), essColor, FontWeight.SemiBold),
                    TableCellData(formatSignedScore(d.accidental_score), TextSecondary),
                    TableCellData(formatSignedScore(d.total_lilly_score), totColor, FontWeight.Bold)
                )
            )

            if (isExpanded) {
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
        }
    }
}

@Composable
private fun HoursAndMoonSubSection(
    data: WesternTraditionalData,
    isHorary: Boolean
) {
    Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
        // 1. Planetary Hours & Radicality
        val ph = data.planetary_hours
        AstroPanelCard(
            title = "Planetary Hours & Radicality",
            headerAction = {
                AstroStatusBadge(
                    text = if (ph.is_radical) "RADICAL" else "NON-RADICAL",
                    color = if (ph.is_radical) CelestialBlue else CrimsonAlert
                )
            }
        ) {
            val tiles = listOf(
                MetricTileItem("DAY RULER", bodyCode(ph.day_ruler), ph.day_ruler),
                MetricTileItem("HOUR RULER", bodyCode(ph.hour_ruler), ph.hour_ruler),
                MetricTileItem("HOUR NO.", "#${ph.hour_number}", if (ph.is_day_hour) "DIURNAL" else "NOCTURNAL"),
                MetricTileItem("STATUS", if (ph.is_radical) "MATCH" else "WARN", if (isHorary) "HORARY" else "NATAL")
            )
            CompactMetricGrid(items = tiles, columns = 4)

            if (ph.radicality_reason.isNotBlank()) {
                Spacer(modifier = Modifier.height(8.dp))
                Text(
                    text = ph.radicality_reason,
                    fontSize = 11.sp,
                    color = TextSecondary,
                    maxLines = 2,
                    overflow = TextOverflow.Ellipsis
                )
            }
            if (ph.considerations.isNotEmpty()) {
                Spacer(modifier = Modifier.height(6.dp))
                ph.considerations.forEach { c ->
                    Text(
                        text = "• $c",
                        fontSize = 11.sp,
                        color = CrimsonAlert,
                        fontWeight = FontWeight.Medium,
                        maxLines = 1,
                        overflow = TextOverflow.Ellipsis
                    )
                }
            }
        }

        // 2. Lunar Phase & Condition
        val mc = data.moon_condition
        AstroPanelCard(title = "Lunar Phase & Condition") {
            val tiles = listOf(
                MetricTileItem(
                    tag = "PHASE",
                    primary = mc.lunar_phase_name.uppercase(Locale.US),
                    secondary = if (mc.is_waxing) "WAX" else "WAN"
                ),
                MetricTileItem(
                    tag = "ELONGATION",
                    primary = String.format(Locale.US, "%.1f°", mc.elongation_deg),
                    secondary = "SUN-MON"
                ),
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
            CompactMetricGrid(items = tiles, columns = 2)

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

        // 3. Prenatal Syzygy (SAN)
        data.prenatal_syzygy?.let { san ->
            AstroPanelCard(title = "Prenatal Syzygy (SAN)") {
                val tiles = listOf(
                    MetricTileItem("TYPE", san.syzygy_type.uppercase(Locale.US)),
                    MetricTileItem("DEGREE", formatSignAndDeg(san.sign_index, san.sign_degree)),
                    MetricTileItem("UTC DATE", san.exact_datetime_utc.take(16))
                )
                CompactMetricGrid(items = tiles, columns = 3)
            }
        }
    }
}

@Composable
private fun ReceptionsAndCuspsSubSection(data: WesternTraditionalData) {
    Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
        // 1. Perfection & Reception Summary
        val perf = data.perfection
        if (data.mutual_receptions.isNotEmpty() ||
            perf.translation_of_light.isNotEmpty() ||
            perf.collection_of_light.isNotEmpty() ||
            perf.besiegement.isNotEmpty()
        ) {
            AstroPanelCard(
                title = "Mutual Receptions & Perfection",
                contentPadding = PaddingValues(0.dp)
            ) {
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
                perf.translation_of_light.forEach { item ->
                    AstroTableRow(
                        columns = cols,
                        cells = listOf(
                            TableCellData("TRANSLATION", CelestialBlue, FontWeight.Bold),
                            TableCellData(item, TextPrimary)
                        )
                    )
                }
                perf.collection_of_light.forEach { item ->
                    AstroTableRow(
                        columns = cols,
                        cells = listOf(
                            TableCellData("COLLECTION", CelestialBlue, FontWeight.Bold),
                            TableCellData(item, TextPrimary)
                        )
                    )
                }
                perf.besiegement.forEach { item ->
                    AstroTableRow(
                        columns = cols,
                        cells = listOf(
                            TableCellData("BESIEGEMENT", CrimsonAlert, FontWeight.Bold),
                            TableCellData(item, CrimsonAlert)
                        )
                    )
                }
            }
        }

        // 2. House Cusp Dignities & Almutens
        if (data.cusp_dignities.isNotEmpty()) {
            AstroPanelCard(
                title = "House Cusp Rulers & Almutens",
                contentPadding = PaddingValues(0.dp)
            ) {
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

        // 3. Antiscia & Contra-Antiscia
        if (data.antiscia.isNotEmpty()) {
            AstroPanelCard(
                title = "Antiscia & Contra-Antiscia",
                contentPadding = PaddingValues(0.dp)
            ) {
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
    }
}

@Composable
private fun LotsAndStarsSubSection(data: WesternTraditionalData) {
    var expandedStar by remember { mutableStateOf<String?>(null) }

    Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
        // 1. Arabic Parts / Lots
        if (data.arabic_parts.isNotEmpty()) {
            AstroPanelCard(
                title = "Arabic Parts & Lots",
                contentPadding = PaddingValues(0.dp)
            ) {
                val cols = listOf(
                    TableColSpec("LOT", 1.5f, TextAlign.Start),
                    TableColSpec("POSITION", 1.25f, TextAlign.Start, monospace = true),
                    TableColSpec("H", 0.55f, TextAlign.Center, monospace = true),
                    TableColSpec("RULER", 0.75f, TextAlign.Center, monospace = true),
                    TableColSpec("FORMULA", 1.4f, TextAlign.End, monospace = true)
                )
                AstroTableHeader(cols)
                data.arabic_parts.forEachIndexed { idx, part ->
                    AstroTableRow(
                        columns = cols,
                        striped = idx % 2 == 1,
                        cells = listOf(
                            TableCellData(part.name, TextPrimary, FontWeight.SemiBold),
                            TableCellData(formatSignAndDeg(part.sign_index, part.sign_degree), TextPrimary),
                            TableCellData("${part.house_number}", TextSecondary),
                            TableCellData(bodyCode(part.ruler), CelestialBlue, FontWeight.Bold),
                            TableCellData(part.formula.ifBlank { "-" }, TextSecondary)
                        )
                    )
                }
            }
        }

        // 2. Fixed Star Conjunctions
        AstroPanelCard(
            title = "Fixed Star Conjunctions",
            subtitle = "Tap row to view traditional nature & meaning",
            contentPadding = PaddingValues(0.dp)
        ) {
            if (data.fixed_star_conjunctions.isEmpty()) {
                Box(modifier = Modifier.padding(12.dp)) {
                    Text("No active fixed star conjunctions within orb.", fontSize = 12.sp, color = TextSecondary)
                }
            } else {
                val cols = listOf(
                    TableColSpec("STAR", 1.5f, TextAlign.Start),
                    TableColSpec("BODY", 0.75f, TextAlign.Center, monospace = true),
                    TableColSpec("ORB", 0.85f, TextAlign.End, monospace = true),
                    TableColSpec("NATURE", 0.95f, TextAlign.Center, monospace = true),
                    TableColSpec("STATUS", 0.95f, TextAlign.End, monospace = true)
                )
                AstroTableHeader(cols)
                data.fixed_star_conjunctions.forEachIndexed { idx, fsc ->
                    val key = "${fsc.star_name}_${fsc.body_name}"
                    val isExp = expandedStar == key
                    AstroTableRow(
                        columns = cols,
                        striped = idx % 2 == 1,
                        onClick = { expandedStar = if (isExp) null else key },
                        cells = listOf(
                            TableCellData(fsc.star_name, TextPrimary, FontWeight.Bold),
                            TableCellData(bodyCode(fsc.body_name), CelestialBlue, FontWeight.Bold),
                            TableCellData(formatOrb(fsc.orb), TextPrimary),
                            TableCellData(fsc.nature, TextSecondary),
                            TableCellData(
                                text = if (fsc.is_qualified) "QUAL" else "FILT",
                                color = if (fsc.is_qualified) CelestialBlue else TextSecondary,
                                badgeColor = if (fsc.is_qualified) CelestialBlue else null
                            )
                        )
                    )
                    if (isExp) {
                        Column(
                            modifier = Modifier
                                .fillMaxWidth()
                                .background(SurfaceLight)
                                .padding(horizontal = 12.dp, vertical = 8.dp),
                            verticalArrangement = Arrangement.spacedBy(2.dp)
                        ) {
                            Text(
                                text = "Mag ${fsc.star_magnitude} · Lat ${String.format(Locale.US, "%+.1f°", fsc.star_latitude)} · Max Alt ${String.format(Locale.US, "%+.1f°", fsc.max_altitude)}",
                                fontSize = 10.sp,
                                color = TextSecondary,
                                style = TabularMonoStyle
                            )
                            if (fsc.meaning.isNotBlank()) {
                                Text(text = fsc.meaning, fontSize = 11.sp, color = TextPrimary)
                            }
                        }
                        HorizontalDivider(color = BorderColor, thickness = 0.5.dp)
                    }
                }
            }
        }
    }
}

@Composable
private fun HellenisticSubSection(h: HellenisticData) {
    Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
        // 1. Angles & Bounds
        AstroPanelCard(title = "Hellenistic Angles (Whole Sign)") {
            val tiles = listOf(
                MetricTileItem("ASCENDANT", formatSignAndDeg(h.angles.ascendant_sign, h.angles.ascendant_degree), "H1"),
                MetricTileItem("MIDHEAVEN", formatSignAndDeg(h.angles.midheaven_sign, h.angles.midheaven_degree), "H${h.angles.midheaven_house}"),
                MetricTileItem("MC IN 10TH", if (h.angles.is_mc_in_10th) "YES" else "NO (H${h.angles.midheaven_house})")
            )
            CompactMetricGrid(items = tiles, columns = 3)
        }

        // 2. Planet Conditions & Egyptian Bounds
        if (h.planet_conditions.isNotEmpty()) {
            AstroPanelCard(
                title = "Planetary Conditions & Egyptian Bounds",
                contentPadding = PaddingValues(0.dp)
            ) {
                val cols = listOf(
                    TableColSpec("BODY", 0.8f, TextAlign.Start, monospace = true),
                    TableColSpec("POSITION", 1.3f, TextAlign.Start, monospace = true),
                    TableColSpec("H", 0.5f, TextAlign.Center, monospace = true),
                    TableColSpec("BOUND", 0.8f, TextAlign.Center, monospace = true),
                    TableColSpec("CONDITION", 1.6f, TextAlign.End)
                )
                AstroTableHeader(cols)
                h.planet_conditions.forEachIndexed { idx, pc ->
                    AstroTableRow(
                        columns = cols,
                        striped = idx % 2 == 1,
                        cells = listOf(
                            TableCellData(bodyCode(pc.name), if (pc.is_ruler_of_asc) CelestialBlue else TextPrimary, FontWeight.Bold),
                            TableCellData(formatSignAndDeg(pc.sign_index, pc.sign_degree), TextPrimary),
                            TableCellData("${pc.house_number}", TextSecondary),
                            TableCellData(if (pc.bound_ruler.isNotBlank()) bodyCode(pc.bound_ruler) else "-", TextSecondary),
                            TableCellData(
                                text = pc.condition_label.ifBlank { "Peregrine" },
                                color = when {
                                    pc.is_in_domicile || pc.is_in_exaltation -> CelestialBlue
                                    pc.is_in_fall -> CrimsonAlert
                                    else -> TextSecondary
                                },
                                fontWeight = FontWeight.Medium
                            )
                        )
                    )
                }
            }
        }

        // 3. Hellenistic Lots
        if (h.lots.isNotEmpty()) {
            AstroPanelCard(
                title = "Hellenistic Lots",
                contentPadding = PaddingValues(0.dp)
            ) {
                val cols = listOf(
                    TableColSpec("LOT", 1.5f, TextAlign.Start),
                    TableColSpec("POSITION", 1.25f, TextAlign.Start, monospace = true),
                    TableColSpec("H", 0.55f, TextAlign.Center, monospace = true),
                    TableColSpec("LORD", 0.75f, TextAlign.Center, monospace = true),
                    TableColSpec("FORMULA", 1.4f, TextAlign.End, monospace = true)
                )
                AstroTableHeader(cols)
                h.lots.forEachIndexed { idx, lot ->
                    AstroTableRow(
                        columns = cols,
                        striped = idx % 2 == 1,
                        cells = listOf(
                            TableCellData(lot.name, TextPrimary, FontWeight.SemiBold),
                            TableCellData(formatSignAndDeg(lot.sign_index, lot.sign_degree), TextPrimary),
                            TableCellData("${lot.house_number}", TextSecondary),
                            TableCellData(bodyCode(lot.ruler), CelestialBlue, FontWeight.Bold),
                            TableCellData(lot.formula.ifBlank { "-" }, TextSecondary)
                        )
                    )
                }
            }
        }
    }
}
