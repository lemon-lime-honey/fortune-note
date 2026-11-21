package com.fortunexample.note.ui.components

import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.Remove
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.fortunexample.note.data.PredictiveAspectData
import com.fortunexample.note.data.PredictiveSuiteData
import com.fortunexample.note.data.ProfileDto
import com.fortunexample.note.data.computeCompletedAge
import com.fortunexample.note.ui.theme.*
import java.util.Locale

data class PredictiveControlState(
    val targetYear: Int,
    val targetMonth: Int,
    val targetDay: Int,
    val isLoading: Boolean,
    val onYearChange: (Int) -> Unit,
    val onDateTimeChange: (Int, Int, Int, Int?, Int?) -> Unit
)

@Composable
fun WesternPredictiveSection(
    profile: ProfileDto,
    predictiveData: PredictiveSuiteData?,
    control: PredictiveControlState
) {
    var selectedTechTab by remember { mutableIntStateOf(0) }
    val techTabs = listOf("Profections", "Solar Arc", "Progressions", "Returns", "Transits", "Directions")
    val resolvedAge = predictiveData?.age ?: computeCompletedAge(
        birthYear = profile.birth_year,
        birthMonth = profile.birth_month,
        birthDay = profile.birth_day,
        targetYear = control.targetYear,
        targetMonth = control.targetMonth,
        targetDay = control.targetDay
    )

    Column(
        modifier = Modifier.fillMaxWidth(),
        verticalArrangement = Arrangement.spacedBy(10.dp)
    ) {
        PredictiveYearControlBar(
            targetYear = control.targetYear,
            resolvedAge = resolvedAge,
            isLoading = control.isLoading,
            onYearChange = control.onYearChange
        )
        AstroSubTabBar(tabs = techTabs, selectedIndex = selectedTechTab, onSelect = { selectedTechTab = it })

        if (predictiveData == null) {
            AstroPanelCard(title = "Loading Predictive Data") {
                Box(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(vertical = 24.dp),
                    contentAlignment = Alignment.Center
                ) {
                    Text(
                        text = if (control.isLoading) "Computing ephemeris tables..." else "Predictive data unavailable.",
                        fontSize = 12.sp,
                        color = TextSecondary
                    )
                }
            }
        } else {
            when (selectedTechTab) {
                0 -> ProfectionTabContent(predictiveData)
                1 -> SolarArcTabContent(predictiveData)
                2 -> SecondaryProgressionTabContent(predictiveData)
                3 -> PlanetaryReturnsTabContent(predictiveData)
                4 -> TransitsTabContent(predictiveData, control.onDateTimeChange)
                5 -> PrimaryDirectionsTabContent(predictiveData)
            }
        }
    }
}

@Composable
private fun PredictiveYearControlBar(
    targetYear: Int,
    resolvedAge: Int,
    isLoading: Boolean,
    onYearChange: (Int) -> Unit
) {
    Surface(
        shape = RoundedCornerShape(8.dp),
        color = BackgroundWhite,
        border = BorderStroke(1.dp, BorderColor),
        modifier = Modifier.fillMaxWidth()
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 12.dp, vertical = 8.dp),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            Column {
                Row(
                    verticalAlignment = Alignment.CenterVertically,
                    horizontalArrangement = Arrangement.spacedBy(8.dp)
                ) {
                    Text(
                        text = "PREDICTIVE EPHEMERIS",
                        fontSize = 11.sp,
                        fontWeight = FontWeight.Bold,
                        letterSpacing = 0.5.sp,
                        color = TextPrimary
                    )
                    if (isLoading) {
                        CircularProgressIndicator(color = AccentPrimary, strokeWidth = 1.8.dp, modifier = Modifier.size(13.dp))
                    }
                }
                Text(
                    text = "Target Year $targetYear · Age $resolvedAge",
                    fontSize = 11.sp,
                    color = TextSecondary,
                    style = TabularMonoStyle
                )
            }

            Row(verticalAlignment = Alignment.CenterVertically, horizontalArrangement = Arrangement.spacedBy(4.dp)) {
                IconButton(onClick = { onYearChange(targetYear - 1) }, modifier = Modifier.size(28.dp)) {
                    Icon(Icons.Default.Remove, contentDescription = "Previous Year", tint = TextPrimary, modifier = Modifier.size(16.dp))
                }
                Surface(shape = RoundedCornerShape(4.dp), color = SurfaceLight, border = BorderStroke(1.dp, BorderColor)) {
                    Text(
                        text = "$targetYear",
                        fontSize = 13.sp,
                        fontWeight = FontWeight.Bold,
                        color = TextPrimary,
                        style = TabularMonoStyle,
                        modifier = Modifier.padding(horizontal = 10.dp, vertical = 4.dp)
                    )
                }
                IconButton(onClick = { onYearChange(targetYear + 1) }, modifier = Modifier.size(28.dp)) {
                    Icon(Icons.Default.Add, contentDescription = "Next Year", tint = TextPrimary, modifier = Modifier.size(16.dp))
                }
            }
        }
    }
}

@Composable
private fun ProfectionTabContent(data: PredictiveSuiteData) {
    val prof = data.profection
    AstroPanelCard(
        title = "Annual Profection (${data.target_year})",
        headerAction = {
            SectionCopyButton("Annual Profection") {
                val occ = if (prof.active_natal_planets.isEmpty()) "None" else prof.active_natal_planets.joinToString(", ")
                buildString {
                    appendLine("[Annual Profection ${data.target_year}]")
                    appendLine("- Target Age: ${prof.target_age}")
                    appendLine("- Profected House: House ${prof.profected_house} (${prof.profected_sign_name})")
                    appendLine("- Lord of the Year (Time Lord): ${prof.lord_of_the_year}")
                    appendLine("- Natal Occupants in House ${prof.profected_house}: $occ")
                }.trimEnd()
            }
        }
    ) {
        CompactMetricGrid(
            items = listOf(
                MetricTileItem("TARGET AGE", "${prof.target_age} YRS", "${prof.target_year}"),
                MetricTileItem("PROFECTED HOUSE", "H${prof.profected_house}", signCode(prof.profected_sign_index), CelestialBlue),
                MetricTileItem("TIME LORD", bodyCode(prof.lord_of_the_year), prof.lord_of_the_year, CelestialBlue),
                MetricTileItem(
                    tag = "NATAL OCCUPANTS",
                    primary = if (prof.active_natal_planets.isEmpty()) "NONE" else prof.active_natal_planets.joinToString(" ") { bodyCode(it) },
                    secondary = "IN H${prof.profected_house}"
                )
            ),
            columns = 2
        )
    }
}

@Composable
internal fun PredictiveContactsTable(
    movingHeader: String,
    movingPrefix: String,
    contacts: List<PredictiveAspectData>,
    emptyMessage: String
) {
    if (contacts.isEmpty()) {
        Box(modifier = Modifier.padding(12.dp)) {
            Text(emptyMessage, fontSize = 12.sp, color = TextSecondary)
        }
        return
    }
    val cols = listOf(
        TableColSpec(movingHeader, 1.0f, TextAlign.Start, monospace = true),
        TableColSpec("ASPECT", 1.1f, TextAlign.Center, monospace = true),
        TableColSpec("NATAL", 1.0f, TextAlign.Center, monospace = true),
        TableColSpec("ORB", 0.9f, TextAlign.End, monospace = true)
    )
    AstroTableHeader(cols)
    contacts.forEachIndexed { idx, c ->
        AstroTableRow(
            columns = cols,
            striped = idx % 2 == 1,
            cells = listOf(
                TableCellData("$movingPrefix${bodyCode(c.moving_body)}", TextPrimary, FontWeight.Bold),
                TableCellData(aspectCode(c.aspect_type), aspectSemanticColor(c.aspect_type), FontWeight.Bold),
                TableCellData("N·${bodyCode(c.natal_body)}", TextPrimary, FontWeight.SemiBold),
                TableCellData(formatOrb(c.orb), TextSecondary)
            )
        )
    }
}

@Composable
private fun SolarArcTabContent(data: PredictiveSuiteData) {
    val sa = data.solar_arc
    Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
        AstroPanelCard(
            title = "Solar Arc Contacts (Orb ≤ 1.0°)",
            subtitle = String.format(Locale.US, "Arc Offset: +%05.2f°", sa.arc_degrees),
            contentPadding = PaddingValues(0.dp),
            headerAction = {
                SectionCopyButton("Solar Arc") {
                    buildString {
                        appendLine(String.format(Locale.US, "[Solar Arc Directions (%d, Offset +%.2f°)]", data.target_year, sa.arc_degrees))
                        sa.directed_planets.forEach { p ->
                            appendLine("- SA ${p.name}: ${formatSignAndDeg(p.sign_index, p.sign_degree)} (Natal: ${formatLongitudeSignDeg(p.natal_longitude)})")
                        }
                        if (sa.contacts.isNotEmpty()) {
                            appendLine("Contacts (Orb <= 1.0°):")
                            sa.contacts.forEach { c ->
                                appendLine("  * SA ${stripBodyPrefix(c.moving_body)} ${c.aspect_type} Natal ${stripBodyPrefix(c.natal_body)} (Orb ${formatOrb(c.orb)})")
                            }
                        }
                    }.trimEnd()
                }
            }
        ) {
            PredictiveContactsTable("SA BODY", "SA ", sa.contacts, "No exact Solar Arc aspects within 1.0° orb.")
        }

        AstroPanelCard(title = "Solar Arc Directed Positions", contentPadding = PaddingValues(0.dp)) {
            val cols = listOf(
                TableColSpec("BODY", 0.9f, TextAlign.Start, monospace = true),
                TableColSpec("NATAL POS", 1.4f, TextAlign.Start, monospace = true),
                TableColSpec("DIRECTED POS", 1.4f, TextAlign.End, monospace = true)
            )
            AstroTableHeader(cols)
            sa.directed_planets.forEachIndexed { idx, p ->
                AstroTableRow(
                    columns = cols,
                    striped = idx % 2 == 1,
                    cells = listOf(
                        TableCellData(bodyCode(p.name), TextPrimary, FontWeight.Bold),
                        TableCellData(formatLongitudeSignDeg(p.natal_longitude), TextSecondary),
                        TableCellData(formatSignAndDeg(p.sign_index, p.sign_degree), CelestialBlue, FontWeight.SemiBold)
                    )
                )
            }
        }
    }
}

@Composable
private fun SecondaryProgressionTabContent(data: PredictiveSuiteData) {
    val sp = data.secondary_progression
    if (sp == null) {
        AstroPanelCard(title = "Secondary Progressions") {
            Text("Secondary progression data unavailable.", fontSize = 12.sp, color = TextSecondary)
        }
        return
    }

    Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
        AstroPanelCard(
            title = "Secondary Progressions Summary",
            subtitle = "Ephemeris Date: ${sp.progressed_date_utc}",
            headerAction = {
                SectionCopyButton("Secondary Progressions") {
                    buildString {
                        appendLine("[Secondary Progressions (${sp.progressed_date_utc})]")
                        appendLine("- Lunar Phase: ${sp.progressed_lunar_phase}")
                        sp.planets.forEach { p ->
                            val rx = if (p.is_retrograde) " [Rx]" else ""
                            appendLine("- Progressed ${p.name}: ${formatSignAndDeg(p.sign_index, p.sign_degree)}$rx")
                        }
                        if (sp.contacts.isNotEmpty()) {
                            appendLine("Contacts (Orb <= 1.0°):")
                            sp.contacts.forEach { c ->
                                appendLine("  * Progressed ${stripBodyPrefix(c.moving_body)} ${c.aspect_type} Natal ${stripBodyPrefix(c.natal_body)} (Orb ${formatOrb(c.orb)})")
                            }
                        }
                    }.trimEnd()
                }
            }
        ) {
            CompactMetricGrid(
                items = listOf(
                    MetricTileItem("LUNAR PHASE", sp.progressed_lunar_phase.uppercase(Locale.US)),
                    MetricTileItem("ELAPSED SPAN", String.format(Locale.US, "%.2f DAYS", sp.elapsed_tropical_years), "1D = 1YR")
                ),
                columns = 2
            )
        }

        AstroPanelCard(title = "Progressed Planet Positions", contentPadding = PaddingValues(0.dp)) {
            val cols = listOf(
                TableColSpec("BODY", 1.0f, TextAlign.Start, monospace = true),
                TableColSpec("PROGRESSED POS", 1.6f, TextAlign.Start, monospace = true),
                TableColSpec("STATE", 0.9f, TextAlign.End, monospace = true)
            )
            AstroTableHeader(cols)
            sp.planets.forEachIndexed { idx, p ->
                AstroTableRow(
                    columns = cols,
                    striped = idx % 2 == 1,
                    cells = listOf(
                        TableCellData(bodyCode(p.name), TextPrimary, FontWeight.Bold),
                        TableCellData(formatSignAndDeg(p.sign_index, p.sign_degree), TextPrimary),
                        TableCellData(
                            text = if (p.is_retrograde) "RETRO" else "DIR",
                            color = if (p.is_retrograde) CrimsonAlert else TextSecondary,
                            badgeColor = if (p.is_retrograde) CrimsonAlert else null
                        )
                    )
                )
            }
        }

        AstroPanelCard(title = "Progressed-to-Natal Contacts (Orb ≤ 1.0°)", contentPadding = PaddingValues(0.dp)) {
            PredictiveContactsTable("PROG", "P·", sp.contacts, "No active Progressed-to-Natal aspects within 1.0° orb.")
        }
    }
}
