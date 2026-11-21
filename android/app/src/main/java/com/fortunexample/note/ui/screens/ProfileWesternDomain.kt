package com.fortunexample.note.ui.screens

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.horizontalScroll
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.fortunexample.note.data.*
import com.fortunexample.note.ui.components.*
import com.fortunexample.note.ui.theme.*
import java.util.Locale

data class WesternDomainUiState(
    val westernOutput: WesternCalculationOutputDto?,
    val isLoading: Boolean,
    val error: String?,
    val selectedHouse: String,
    val selectedSubTab: Int,
    val predictiveData: PredictiveSuiteData?,
    val predictiveControl: PredictiveControlState
)

@Composable
internal fun WesternDomainContent(
    profile: ProfileDto,
    state: WesternDomainUiState,
    onSelectHouse: (String) -> Unit,
    onSelectSubTab: (Int) -> Unit,
    onRetry: () -> Unit
) {
    Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
        TraditionSelectorBar(selectedHouse = state.selectedHouse, onSelectHouse = onSelectHouse)
        when {
            state.isLoading -> LoadingStateBox()
            state.error != null -> DomainErrorCard(message = state.error, onRetry = onRetry)
            state.westernOutput != null -> WesternLoadedBody(
                profile = profile,
                w = state.westernOutput.western,
                state = state,
                onSelectSubTab = onSelectSubTab
            )
        }
    }
}

@Composable
private fun TraditionSelectorBar(selectedHouse: String, onSelectHouse: (String) -> Unit) {
    val traditionSystems = listOf(
        "WholeSign" to "헬레니즘 · WHOLE SIGN",
        "Regiomontanus" to "중세 · REGIOMONTANUS"
    )
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .horizontalScroll(rememberScrollState()),
        horizontalArrangement = Arrangement.spacedBy(6.dp),
        verticalAlignment = Alignment.CenterVertically
    ) {
        Text(
            text = "TRADITION",
            fontSize = 10.sp,
            fontWeight = FontWeight.Bold,
            color = TextSecondary,
            style = TabularMonoStyle,
            modifier = Modifier.padding(end = 4.dp)
        )
        traditionSystems.forEach { (hSys, label) ->
            val isSelected = selectedHouse.equals(hSys, ignoreCase = true)
            Box(
                modifier = Modifier
                    .clip(RoundedCornerShape(4.dp))
                    .background(if (isSelected) AccentPrimary else SurfaceLight)
                    .border(1.dp, if (isSelected) AccentPrimary else BorderColor, RoundedCornerShape(4.dp))
                    .clickable { onSelectHouse(hSys) }
                    .padding(horizontal = 10.dp, vertical = 6.dp)
            ) {
                Text(
                    text = label,
                    fontSize = 10.sp,
                    fontWeight = FontWeight.Bold,
                    color = if (isSelected) BackgroundWhite else TextSecondary,
                    style = TabularMonoStyle
                )
            }
        }
    }
}

@Composable
private fun WesternLoadedBody(
    profile: ProfileDto,
    w: WesternChartData,
    state: WesternDomainUiState,
    onSelectSubTab: (Int) -> Unit
) {
    val isWholeSign = isWholeSignHouseSystem(w, state.selectedHouse)
    WesternChartWheel(data = w)
    val mcHouseTag = if (isWholeSign && w.hellenistic != null) "H${w.hellenistic.angles.midheaven_house}" else "H10"
    CompactMetricGrid(
        items = listOf(
            MetricTileItem("ASC", formatLongitudeSignDeg(w.ascendant), "H1", CrimsonAlert),
            MetricTileItem("MC", formatLongitudeSignDeg(w.midheaven), mcHouseTag, CelestialBlue),
            MetricTileItem("DSC", formatLongitudeSignDeg((w.ascendant + 180.0) % 360.0), "H7"),
            MetricTileItem("IC", formatLongitudeSignDeg((w.midheaven + 180.0) % 360.0), "H4")
        ),
        columns = 4,
        tileHeight = 44.dp
    )
    AstroSubTabBar(
        tabs = listOf("Placements", "Aspects", if (isWholeSign) "Hellenistic" else "Medieval", "Predictive"),
        selectedIndex = state.selectedSubTab,
        onSelect = onSelectSubTab
    )
    when (state.selectedSubTab) {
        0 -> WesternPlacementsSubSection(w)
        1 -> WesternAspectsSubSection(w)
        2 -> WesternTraditionalTabBody(w, isWholeSign)
        3 -> WesternPredictiveSection(profile = profile, predictiveData = state.predictiveData, control = state.predictiveControl)
    }
}

@Composable
private fun WesternTraditionalTabBody(w: WesternChartData, isWholeSign: Boolean) {
    val activeTrad = if (isWholeSign) null else w.traditional
    val activeHell = if (isWholeSign) w.hellenistic else null
    if (activeTrad != null || activeHell != null) {
        WesternTraditionalSection(data = activeTrad, hellenistic = activeHell, isHorary = false)
    } else {
        AstroPanelCard(title = if (isWholeSign) "Hellenistic Analysis" else "Medieval Analysis") {
            Text("Analysis data unavailable.", fontSize = 12.sp, color = TextSecondary)
        }
    }
}

@Composable
fun WesternPlacementsSubSection(w: WesternChartData) {
    Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
        AstroPanelCard(
            title = "Planetary & Nodal Placements",
            contentPadding = PaddingValues(0.dp),
            headerAction = {
                SectionCopyButton("Western Placements") {
                    buildString {
                        appendLine("ASC: ${formatLongitudeSignDeg(w.ascendant)} | MC: ${formatLongitudeSignDeg(w.midheaven)}")
                        w.planets.forEach { p ->
                            appendLine("${bodyCode(p.name)}: ${formatSignAndDeg(p.sign_index, p.sign_degree)} (H${p.house_number}) ${if (p.is_retrograde) "Rx" else ""}")
                        }
                        w.nodes.forEach { n ->
                            appendLine("${bodyCode(n.name)}: ${formatSignAndDeg(n.sign_index, n.sign_degree)} (H${n.house_number})")
                        }
                    }.trimEnd()
                }
            }
        ) {
            val cols = listOf(
                TableColSpec("BODY", 0.85f, TextAlign.Start, monospace = true),
                TableColSpec("SIGN & DEG", 1.35f, TextAlign.Start, monospace = true),
                TableColSpec("HOUSE", 0.7f, TextAlign.Center, monospace = true),
                TableColSpec("SPEED", 1.0f, TextAlign.End, monospace = true),
                TableColSpec("STATE", 0.75f, TextAlign.End, monospace = true)
            )
            AstroTableHeader(cols)
            w.planets.forEachIndexed { idx, p ->
                AstroTableRow(
                    columns = cols,
                    striped = idx % 2 == 1,
                    cells = listOf(
                        TableCellData(bodyCode(p.name), TextPrimary, FontWeight.Bold),
                        TableCellData(formatSignAndDeg(p.sign_index, p.sign_degree), TextPrimary),
                        TableCellData("H${p.house_number}", TextSecondary),
                        TableCellData(String.format(Locale.US, "%+.2f°", p.speed), TextSecondary),
                        TableCellData(
                            text = if (p.is_retrograde) "RETRO" else "DIR",
                            color = if (p.is_retrograde) CrimsonAlert else TextSecondary,
                            badgeColor = if (p.is_retrograde) CrimsonAlert else null
                        )
                    )
                )
            }
            w.nodes.forEachIndexed { idx, n ->
                AstroTableRow(
                    columns = cols,
                    striped = (w.planets.size + idx) % 2 == 1,
                    cells = listOf(
                        TableCellData(bodyCode(n.name), CelestialBlue, FontWeight.Bold),
                        TableCellData(formatSignAndDeg(n.sign_index, n.sign_degree), TextPrimary),
                        TableCellData("H${n.house_number}", TextSecondary),
                        TableCellData("-", TextSecondary),
                        TableCellData("NODE", TextSecondary)
                    )
                )
            }
        }

        if (w.house_cusps.size == 12) {
            AstroPanelCard(title = "12 House Cusps (${w.house_system ?: "Default"})") {
                val tiles = w.house_cusps.mapIndexed { idx, cuspLon ->
                    val hNum = idx + 1
                    val sIdx = (((cuspLon % 360.0) + 360.0) % 360.0 / 30.0).toInt().coerceIn(0, 11)
                    val highlight = when (hNum) { 1 -> CrimsonAlert; 10 -> CelestialBlue; 4, 7 -> TextPrimary; else -> null }
                    MetricTileItem("H$hNum", formatLongitudeSignDeg(cuspLon), bodyCode(getZodiacSignRuler(sIdx)), highlight)
                }
                CompactMetricGrid(items = tiles, columns = 3)
            }
        }
    }
}

@Composable
fun WesternAspectsSubSection(w: WesternChartData) {
    val isWholeSign = isWholeSignHouseSystem(w)
    val tradAspects = if (isWholeSign) w.hellenistic?.aspects.orEmpty() else w.traditional?.traditional_aspects.orEmpty()

    AstroPanelCard(
        title = "Planetary Aspects",
        contentPadding = PaddingValues(0.dp),
        headerAction = {
            SectionCopyButton("Western Aspects") {
                w.aspects.joinToString("\n") { a ->
                    "${bodyCode(a.planet1)} ${aspectShortCode(a.aspect_type)} ${bodyCode(a.planet2)} (Orb ${formatOrb(a.orb)})"
                }
            }
        }
    ) {
        when {
            tradAspects.isNotEmpty() -> TraditionalAspectsTable(tradAspects)
            w.aspects.isNotEmpty() -> StandardAspectsTable(w.aspects)
            else -> Box(modifier = Modifier.padding(12.dp)) {
                Text("No major aspects within orb.", fontSize = 12.sp, color = TextSecondary)
            }
        }
    }
}

@Composable
private fun TraditionalAspectsTable(aspects: List<TraditionalAspectData>) {
    val cols = listOf(
        TableColSpec("BODY A", 0.9f, TextAlign.Start, monospace = true),
        TableColSpec("ASPECT", 1.1f, TextAlign.Center, monospace = true),
        TableColSpec("BODY B", 0.9f, TextAlign.Center, monospace = true),
        TableColSpec("ORB", 0.85f, TextAlign.End, monospace = true),
        TableColSpec("MOTION", 0.95f, TextAlign.End, monospace = true)
    )
    AstroTableHeader(cols)
    aspects.forEachIndexed { idx, a ->
        val motionTag = (if (a.is_applying) "APP" else "SEP") + if (a.is_partile) "·PRT" else ""
        AstroTableRow(
            columns = cols,
            striped = idx % 2 == 1,
            cells = listOf(
                TableCellData(bodyCode(a.planet1), TextPrimary, FontWeight.Bold),
                TableCellData(aspectCode(a.aspect_type), aspectSemanticColor(a.aspect_type), FontWeight.Bold),
                TableCellData(bodyCode(a.planet2), TextPrimary, FontWeight.Bold),
                TableCellData(formatOrb(a.orb), TextPrimary),
                TableCellData(motionTag, if (a.is_applying) CelestialBlue else TextSecondary, badgeColor = if (a.is_partile) CelestialBlue else null)
            )
        )
    }
}

@Composable
private fun StandardAspectsTable(aspects: List<WesternAspectData>) {
    val cols = listOf(
        TableColSpec("BODY A", 1.0f, TextAlign.Start, monospace = true),
        TableColSpec("ASPECT", 1.2f, TextAlign.Center, monospace = true),
        TableColSpec("BODY B", 1.0f, TextAlign.Center, monospace = true),
        TableColSpec("ORB", 1.0f, TextAlign.End, monospace = true)
    )
    AstroTableHeader(cols)
    aspects.forEachIndexed { idx, a ->
        AstroTableRow(
            columns = cols,
            striped = idx % 2 == 1,
            cells = listOf(
                TableCellData(bodyCode(a.planet1), TextPrimary, FontWeight.Bold),
                TableCellData(aspectCode(a.aspect_type), aspectSemanticColor(a.aspect_type), FontWeight.Bold),
                TableCellData(bodyCode(a.planet2), TextPrimary, FontWeight.Bold),
                TableCellData(formatOrb(a.orb), TextPrimary)
            )
        )
    }
}
