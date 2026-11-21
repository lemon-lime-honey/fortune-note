package com.fortunexample.note.ui.components

import android.app.DatePickerDialog
import android.app.TimePickerDialog
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.horizontalScroll
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.fortunexample.note.data.PlanetaryReturnData
import com.fortunexample.note.data.PredictiveSuiteData
import com.fortunexample.note.data.findHouseNumber
import com.fortunexample.note.data.mergeSolarAndPlanetaryReturns
import com.fortunexample.note.ui.theme.*
import java.util.Calendar
import java.util.Locale

@Composable
internal fun PlanetaryReturnsTabContent(data: PredictiveSuiteData) {
    val returns = remember(data) { mergeSolarAndPlanetaryReturns(data) }
    var selectedBodyIdx by remember { mutableIntStateOf(0) }

    if (returns.isEmpty()) {
        AstroPanelCard(title = "Planetary Returns") {
            Text("No planetary returns available for ${data.target_year}.", fontSize = 12.sp, color = TextSecondary)
        }
        return
    }

    val safeIdx = selectedBodyIdx.coerceIn(0, returns.lastIndex)
    val currentReturn = returns[safeIdx]

    Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
        AstroSubTabBar(
            tabs = returns.map { "${bodyCode(it.return_body)} Return" },
            selectedIndex = safeIdx,
            onSelect = { selectedBodyIdx = it }
        )
        ReturnHeaderCard(currentReturn)
        ReturnPlacementsCard(currentReturn)
        ReturnNatalAspectsCard(currentReturn)
    }
}

@Composable
private fun ReturnHeaderCard(ret: PlanetaryReturnData) {
    AstroPanelCard(
        title = "${ret.return_body} Return (${ret.target_year})",
        subtitle = "Exact UTC: ${ret.return_date_utc}",
        headerAction = {
            SectionCopyButton("${ret.return_body} Return") {
                buildString {
                    appendLine("[${ret.return_body} Return (${ret.target_year})]")
                    appendLine("- Exact UTC: ${ret.return_date_utc}")
                    appendLine("- Return ASC: ${formatLongitudeSignDeg(ret.ascendant)} | Return MC: ${formatLongitudeSignDeg(ret.midheaven)}")
                    appendLine("- Return Placements:")
                    ret.return_planets.forEach { p ->
                        val hNum = findHouseNumber(p.progressed_longitude, ret.house_cusps)
                        val rx = if (p.is_retrograde) " [Rx]" else ""
                        appendLine("  * ${p.name}: ${formatSignAndDeg(p.sign_index, p.sign_degree)} (H$hNum)$rx")
                    }
                    if (ret.natal_aspects.isNotEmpty()) {
                        appendLine("- Return-to-Natal Aspects:")
                        ret.natal_aspects.forEach { a ->
                            appendLine("  * Return ${a.return_point} ${a.aspect_type} Natal ${a.natal_point} (Orb ${formatOrb(a.orb)})")
                        }
                    }
                }.trimEnd()
            }
        }
    ) {
        CompactMetricGrid(
            items = listOf(
                MetricTileItem("RETURN ASC", formatLongitudeSignDeg(ret.ascendant), "H1", CrimsonAlert),
                MetricTileItem("RETURN MC", formatLongitudeSignDeg(ret.midheaven), "H10", CelestialBlue),
                MetricTileItem("RETURN ${bodyCode(ret.return_body)}", formatLongitudeSignDeg(ret.body_longitude), if (ret.is_retrograde) "RX" else "DIR")
            ),
            columns = 3
        )
    }
}

@Composable
private fun ReturnPlacementsCard(ret: PlanetaryReturnData) {
    AstroPanelCard(title = "Return Chart Placements", contentPadding = PaddingValues(0.dp)) {
        val cols = listOf(
            TableColSpec("BODY", 0.9f, TextAlign.Start, monospace = true),
            TableColSpec("POSITION", 1.4f, TextAlign.Start, monospace = true),
            TableColSpec("RET H", 0.7f, TextAlign.Center, monospace = true),
            TableColSpec("STATE", 0.8f, TextAlign.End, monospace = true)
        )
        AstroTableHeader(cols)
        ret.return_planets.forEachIndexed { idx, p ->
            val hNum = findHouseNumber(p.progressed_longitude, ret.house_cusps)
            AstroTableRow(
                columns = cols,
                striped = idx % 2 == 1,
                cells = listOf(
                    TableCellData(bodyCode(p.name), TextPrimary, FontWeight.Bold),
                    TableCellData(formatSignAndDeg(p.sign_index, p.sign_degree), TextPrimary),
                    TableCellData("H$hNum", TextSecondary),
                    TableCellData(
                        text = if (p.is_retrograde) "RETRO" else "DIR",
                        color = if (p.is_retrograde) CrimsonAlert else TextSecondary,
                        badgeColor = if (p.is_retrograde) CrimsonAlert else null
                    )
                )
            )
        }
    }
}

@Composable
private fun ReturnNatalAspectsCard(ret: PlanetaryReturnData) {
    if (ret.natal_aspects.isEmpty()) return
    AstroPanelCard(title = "Return-to-Natal Aspects", contentPadding = PaddingValues(0.dp)) {
        val cols = listOf(
            TableColSpec("RET", 1.0f, TextAlign.Start, monospace = true),
            TableColSpec("ASPECT", 1.1f, TextAlign.Center, monospace = true),
            TableColSpec("NATAL", 1.0f, TextAlign.Center, monospace = true),
            TableColSpec("ORB", 0.9f, TextAlign.End, monospace = true)
        )
        AstroTableHeader(cols)
        ret.natal_aspects.forEachIndexed { idx, a ->
            AstroTableRow(
                columns = cols,
                striped = idx % 2 == 1,
                cells = listOf(
                    TableCellData("R·${bodyCode(a.return_point)}", TextPrimary, FontWeight.Bold),
                    TableCellData(aspectCode(a.aspect_type), aspectSemanticColor(a.aspect_type), FontWeight.Bold),
                    TableCellData("N·${bodyCode(a.natal_point)}", TextPrimary, FontWeight.SemiBold),
                    TableCellData(formatOrb(a.orb), TextSecondary)
                )
            )
        }
    }
}

private fun shiftTransitCalendar(
    data: PredictiveSuiteData,
    field: Int,
    delta: Int,
    onDateTimeChange: (Int, Int, Int, Int?, Int?) -> Unit
) {
    val cal = Calendar.getInstance().apply {
        set(Calendar.YEAR, data.target_year)
        set(Calendar.MONTH, (data.target_month - 1).coerceIn(0, 11))
        set(Calendar.DAY_OF_MONTH, data.target_day.coerceIn(1, 31))
        set(Calendar.HOUR_OF_DAY, data.target_hour.coerceIn(0, 23))
        set(Calendar.MINUTE, data.target_minute.coerceIn(0, 59))
        add(field, delta)
    }
    onDateTimeChange(
        cal.get(Calendar.YEAR),
        cal.get(Calendar.MONTH) + 1,
        cal.get(Calendar.DAY_OF_MONTH),
        cal.get(Calendar.HOUR_OF_DAY),
        cal.get(Calendar.MINUTE)
    )
}

@Composable
internal fun TransitsTabContent(
    data: PredictiveSuiteData,
    onDateTimeChange: (Int, Int, Int, Int?, Int?) -> Unit
) {
    val context = LocalContext.current
    val timestampStr = String.format(
        Locale.US,
        "%04d-%02d-%02d %02d:%02d LOCAL",
        data.target_year, data.target_month, data.target_day, data.target_hour, data.target_minute
    )

    Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
        AstroPanelCard(title = "Transit Ephemeris Controls", subtitle = timestampStr) {
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .horizontalScroll(rememberScrollState()),
                horizontalArrangement = Arrangement.spacedBy(6.dp)
            ) {
                TransitActionChip("NOW") {
                    val now = Calendar.getInstance()
                    onDateTimeChange(
                        now.get(Calendar.YEAR), now.get(Calendar.MONTH) + 1,
                        now.get(Calendar.DAY_OF_MONTH), now.get(Calendar.HOUR_OF_DAY), now.get(Calendar.MINUTE)
                    )
                }
                TransitActionChip("DATE...") {
                    DatePickerDialog(
                        context,
                        { _, year, monthZero, day -> onDateTimeChange(year, monthZero + 1, day, data.target_hour, data.target_minute) },
                        data.target_year, (data.target_month - 1).coerceIn(0, 11), data.target_day.coerceIn(1, 31)
                    ).show()
                }
                TransitActionChip("TIME...") {
                    TimePickerDialog(
                        context,
                        { _, hour, minute -> onDateTimeChange(data.target_year, data.target_month, data.target_day, hour, minute) },
                        data.target_hour.coerceIn(0, 23), data.target_minute.coerceIn(0, 59), true
                    ).show()
                }
                TransitActionChip("-1D") { shiftTransitCalendar(data, Calendar.DAY_OF_MONTH, -1, onDateTimeChange) }
                TransitActionChip("+1D") { shiftTransitCalendar(data, Calendar.DAY_OF_MONTH, 1, onDateTimeChange) }
                TransitActionChip("-1H") { shiftTransitCalendar(data, Calendar.HOUR_OF_DAY, -1, onDateTimeChange) }
                TransitActionChip("+1H") { shiftTransitCalendar(data, Calendar.HOUR_OF_DAY, 1, onDateTimeChange) }
            }
        }
        ActiveTransitsCard(data, timestampStr)
    }
}

@Composable
private fun ActiveTransitsCard(data: PredictiveSuiteData, timestampStr: String) {
    AstroPanelCard(
        title = "Active Transits to Natal (Orb ≤ 2.0°)",
        contentPadding = PaddingValues(0.dp),
        headerAction = {
            SectionCopyButton("Active Transits") {
                buildString {
                    appendLine("[Active Transits ($timestampStr)]")
                    if (data.transits.isEmpty()) {
                        appendLine("No active major transits within 2.0° orb.")
                    } else {
                        data.transits.forEach { t ->
                            appendLine("- Transit ${t.transiting_planet} (${formatLongitudeSignDeg(t.transiting_longitude)}) ${t.aspect_type} Natal ${t.natal_planet} (${formatLongitudeSignDeg(t.natal_longitude)}) (Orb ${formatOrb(t.orb)})")
                        }
                    }
                }.trimEnd()
            }
        }
    ) {
        if (data.transits.isEmpty()) {
            Box(modifier = Modifier.padding(12.dp)) {
                Text("No active major transits within 2.0° orb.", fontSize = 12.sp, color = TextSecondary)
            }
            return@AstroPanelCard
        }
        val cols = listOf(
            TableColSpec("TRANSIT", 1.3f, TextAlign.Start, monospace = true),
            TableColSpec("ASPECT", 1.0f, TextAlign.Center, monospace = true),
            TableColSpec("NATAL", 1.3f, TextAlign.Start, monospace = true),
            TableColSpec("ORB", 0.8f, TextAlign.End, monospace = true)
        )
        AstroTableHeader(cols)
        data.transits.forEachIndexed { idx, t ->
            AstroTableRow(
                columns = cols,
                striped = idx % 2 == 1,
                cells = listOf(
                    TableCellData("${bodyCode(t.transiting_planet)} ${formatLongitudeSignDeg(t.transiting_longitude)}", TextPrimary, FontWeight.Bold),
                    TableCellData(aspectShortCode(t.aspect_type), aspectSemanticColor(t.aspect_type), FontWeight.Bold),
                    TableCellData("${bodyCode(t.natal_planet)} ${formatLongitudeSignDeg(t.natal_longitude)}", TextSecondary),
                    TableCellData(formatOrb(t.orb), TextPrimary, FontWeight.SemiBold)
                )
            )
        }
    }
}

@Composable
private fun TransitActionChip(label: String, onClick: () -> Unit) {
    Box(
        modifier = Modifier
            .clip(RoundedCornerShape(4.dp))
            .background(SurfaceLight)
            .border(1.dp, BorderColor, RoundedCornerShape(4.dp))
            .clickable(onClick = onClick)
            .padding(horizontal = 9.dp, vertical = 5.dp),
        contentAlignment = Alignment.Center
    ) {
        Text(text = label, fontSize = 10.sp, fontWeight = FontWeight.Bold, color = TextPrimary, style = TabularMonoStyle, maxLines = 1)
    }
}

@Composable
internal fun PrimaryDirectionsTabContent(data: PredictiveSuiteData) {
    AstroPanelCard(
        title = "Primary Directions (Naibod Key)",
        contentPadding = PaddingValues(0.dp),
        headerAction = {
            SectionCopyButton("Primary Directions") {
                buildString {
                    appendLine("[Primary Directions (Naibod Key, Target Year ${data.target_year})]")
                    if (data.primary_directions.isEmpty()) {
                        appendLine("No Primary Directions hits in current window.")
                    } else {
                        data.primary_directions.forEach { pd ->
                            appendLine(String.format(Locale.US, "- %s %s %s (Arc %.2f°, Age %.1fY)", pd.promittor, pd.aspect_type, pd.significator, pd.direction_arc_deg, pd.realization_age))
                        }
                    }
                }.trimEnd()
            }
        }
    ) {
        if (data.primary_directions.isEmpty()) {
            Box(modifier = Modifier.padding(12.dp)) {
                Text("No Primary Directions hits in current window.", fontSize = 12.sp, color = TextSecondary)
            }
            return@AstroPanelCard
        }
        val cols = listOf(
            TableColSpec("PROMITTOR", 0.95f, TextAlign.Start, monospace = true),
            TableColSpec("ASPECT", 1.05f, TextAlign.Center, monospace = true),
            TableColSpec("SIGNIFICATOR", 1.0f, TextAlign.Center, monospace = true),
            TableColSpec("ARC", 0.85f, TextAlign.End, monospace = true),
            TableColSpec("AGE", 0.85f, TextAlign.End, monospace = true)
        )
        AstroTableHeader(cols)
        data.primary_directions.forEachIndexed { idx, pd ->
            AstroTableRow(
                columns = cols,
                striped = idx % 2 == 1,
                cells = listOf(
                    TableCellData(bodyCode(pd.promittor), TextPrimary, FontWeight.Bold),
                    TableCellData(aspectShortCode(pd.aspect_type), aspectSemanticColor(pd.aspect_type), FontWeight.Bold),
                    TableCellData(bodyCode(pd.significator), TextPrimary, FontWeight.SemiBold),
                    TableCellData(String.format(Locale.US, "%05.2f°", pd.direction_arc_deg), TextSecondary),
                    TableCellData(String.format(Locale.US, "%.1fY", pd.realization_age), CelestialBlue, FontWeight.Bold)
                )
            )
        }
    }
}
