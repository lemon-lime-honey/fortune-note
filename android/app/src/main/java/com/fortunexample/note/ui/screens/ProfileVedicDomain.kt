package com.fortunexample.note.ui.screens

import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.fortunexample.note.data.*
import com.fortunexample.note.ui.components.*
import com.fortunexample.note.ui.theme.*
import java.util.Locale

data class VedicDomainUiState(
    val vedicOutput: VedicCalculationOutputDto?,
    val isLoading: Boolean,
    val error: String?,
    val selectedVarga: String,
    val selectedSubTab: Int,
    val expandedDashaLord: String?
)

@Composable
internal fun VedicDomainContent(
    state: VedicDomainUiState,
    onOpenVargaPicker: () -> Unit,
    onSelectSubTab: (Int) -> Unit,
    onToggleDashaLord: (String) -> Unit,
    onRetry: () -> Unit
) {
    when {
        state.isLoading -> LoadingStateBox()
        state.error != null -> DomainErrorCard(message = state.error, onRetry = onRetry)
        state.vedicOutput != null -> VedicLoadedBody(
            v = state.vedicOutput.vedic,
            state = state,
            onOpenVargaPicker = onOpenVargaPicker,
            onSelectSubTab = onSelectSubTab,
            onToggleDashaLord = onToggleDashaLord
        )
    }
}

@Composable
private fun VedicLoadedBody(
    v: VedicChartData,
    state: VedicDomainUiState,
    onOpenVargaPicker: () -> Unit,
    onSelectSubTab: (Int) -> Unit,
    onToggleDashaLord: (String) -> Unit
) {
    val activeVargaChart = v.vargas[state.selectedVarga]
    val vargaTitle = VARGA_DEFINITIONS.find { it.first == state.selectedVarga }?.let { "${it.first} · ${it.second}" } ?: state.selectedVarga

    Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
        VargaHeaderButton(vargaTitle = vargaTitle, onClick = onOpenVargaPicker)
        VedicChartDiamond(data = v, selectedVarga = activeVargaChart)
        AstroSubTabBar(
            tabs = listOf("Placements", "Panchanga & Karakas", "Padas & Drishti", "Ashtakavarga", "Dasha"),
            selectedIndex = state.selectedSubTab,
            onSelect = onSelectSubTab
        )
        when (state.selectedSubTab) {
            0 -> VedicPlacementsTab(v, state.selectedVarga, activeVargaChart)
            1 -> Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
                VedicPanchangaCard(v.panchanga, title = "Natal Panchanga")
                VedicCharaKarakasCard(v.chara_karakas)
            }
            2 -> Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
                VedicArudhaPadasCard(v.arudha_padas, title = "Arudha Padas (AL ~ UL)")
                VedicDrishtiCard(v.drishti, title = "Graha Drishti (Parashari Full & Special Aspects)", targetColumnHeader = "TARGET BHAVA")
            }
            3 -> VedicAshtakavargaSubSection(v)
            4 -> VedicDashaSubSection(v, state.expandedDashaLord, onToggleDashaLord)
        }
    }
}

@Composable
private fun VargaHeaderButton(vargaTitle: String, onClick: () -> Unit) {
    Surface(
        shape = RoundedCornerShape(6.dp),
        color = SurfaceLight,
        border = BorderStroke(1.dp, BorderColor),
        modifier = Modifier.fillMaxWidth().clickable(onClick = onClick)
    ) {
        Row(
            modifier = Modifier.fillMaxWidth().padding(horizontal = 12.dp, vertical = 8.dp),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            Column {
                Text("ACTIVE DIVISIONAL CHART", fontSize = 9.sp, fontWeight = FontWeight.Bold, color = TextSecondary, style = TabularMonoStyle)
                Text(vargaTitle.uppercase(Locale.US), fontSize = 13.sp, fontWeight = FontWeight.Bold, color = TextPrimary, style = TabularMonoStyle)
            }
            Surface(shape = RoundedCornerShape(4.dp), color = AccentPrimary) {
                Text("SELECT VARGA (19)", fontSize = 10.sp, fontWeight = FontWeight.Bold, color = BackgroundWhite, style = TabularMonoStyle, modifier = Modifier.padding(horizontal = 8.dp, vertical = 5.dp))
            }
        }
    }
}

@Composable
private fun VedicPlacementsTab(
    v: VedicChartData,
    selectedVarga: String,
    activeVargaChart: VargaChartData?
) {
    val ayanamshaInfo = if (v.ayanamsha_deg > 0.0) {
        " · ${v.ayanamsha_name} ${String.format(Locale.US, "%.2f°", v.ayanamsha_deg)}"
    } else ""
    Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
        VedicPlacementsTableCard(
            v = v,
            title = "Sidereal Graha Placements (D1 Rasi)",
            subtitle = "Lagna: ${vedicRasiCode(v.lagna_sign)} ${formatDegMin(v.lagna_sidereal % 30.0)}$ayanamshaInfo"
        )
        if (selectedVarga != "D1" && activeVargaChart != null) {
            AstroPanelCard(
                title = "$selectedVarga Divisional Placements",
                subtitle = "$selectedVarga Lagna: ${vedicRasiCode(activeVargaChart.lagna_sign)}",
                contentPadding = PaddingValues(0.dp)
            ) {
                val cols = listOf(
                    TableColSpec("GRAHA", 1.1f, TextAlign.Start, monospace = true),
                    TableColSpec("DIVISIONAL RASI", 1.9f, TextAlign.Start, monospace = true),
                    TableColSpec("BHAVA", 0.9f, TextAlign.End, monospace = true)
                )
                AstroTableHeader(cols)
                activeVargaChart.placements.forEachIndexed { idx, vp ->
                    AstroTableRow(
                        columns = cols,
                        striped = idx % 2 == 1,
                        cells = listOf(
                            TableCellData(vedicGrahaCode(vp.planet_name), TextPrimary, FontWeight.Bold),
                            TableCellData(vedicRasiCode(vp.sign_index), TextPrimary),
                            TableCellData("H${vp.house}", CelestialBlue, FontWeight.Bold)
                        )
                    )
                }
            }
        }
    }
}

@Composable
private fun VedicCharaKarakasCard(karakas: List<VedicKarakaData>) {
    if (karakas.isEmpty()) return
    AstroPanelCard(title = "Jaimini Chara Karakas (7-Karaka Scheme)", contentPadding = PaddingValues(0.dp)) {
        val cols = listOf(
            TableColSpec("KARAKA", 1.8f, TextAlign.Start),
            TableColSpec("GRAHA", 0.9f, TextAlign.Center, monospace = true),
            TableColSpec("EFF. DEGREE", 1.1f, TextAlign.End, monospace = true)
        )
        AstroTableHeader(cols)
        karakas.forEachIndexed { idx, ck ->
            val isAk = ck.karaka.startsWith("Atmakaraka", ignoreCase = true)
            val col = if (isAk) CelestialBlue else TextPrimary
            AstroTableRow(
                columns = cols,
                striped = idx % 2 == 1,
                cells = listOf(
                    TableCellData(ck.karaka, col, FontWeight.Bold),
                    TableCellData(vedicGrahaCode(ck.planet), col, FontWeight.Bold),
                    TableCellData(formatDegMin(ck.effective_degree), TextSecondary)
                )
            )
        }
    }
}

@Composable
private fun VedicAshtakavargaSubSection(v: VedicChartData) {
    val av = v.ashtakavarga
    if (av.total_bindus == 0) {
        AstroPanelCard(title = "Ashtakavarga") {
            Text("Ashtakavarga data unavailable.", fontSize = 12.sp, color = TextSecondary)
        }
        return
    }
    Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
        AstroPanelCard(
            title = "Sarvashtakavarga (SAV)",
            headerAction = { AstroStatusBadge("TOTAL ${av.total_bindus}", CelestialBlue) }
        ) {
            val tiles = av.sarva.mapIndexed { sIdx, pts ->
                val bhava = ((sIdx - v.lagna_sign + 12) % 12) + 1
                val highlight = when { pts >= 30 -> CelestialBlue; pts < 25 -> CrimsonAlert; else -> null }
                MetricTileItem(signCode(sIdx), "$pts PTS", "H$bhava", highlight)
            }
            CompactMetricGrid(items = tiles, columns = 4)
        }
        if (av.bhinna.isNotEmpty()) {
            BhinnaAshtakavargaCard(av.bhinna)
        }
    }
}

@Composable
private fun BhinnaAshtakavargaCard(bhinna: List<BhinnaAshtakavargaRowData>) {
    AstroPanelCard(title = "Bhinna Ashtakavarga (BAV)", contentPadding = PaddingValues(0.dp)) {
        val cols = buildList {
            add(TableColSpec("GRAHA", 1.1f, TextAlign.Start, monospace = true))
            for (i in 1..12) add(TableColSpec("$i", 0.55f, TextAlign.Center, monospace = true))
            add(TableColSpec("TOT", 0.85f, TextAlign.End, monospace = true))
        }
        AstroTableHeader(cols)
        bhinna.forEachIndexed { idx, row ->
            val cells = buildList {
                add(TableCellData(vedicGrahaCode(row.planet), TextPrimary, FontWeight.Bold))
                for (i in 0 until 12) {
                    val b = row.bindus.getOrElse(i) { 0 }
                    val col = when { b >= 5 -> CelestialBlue; b <= 2 -> CrimsonAlert; else -> TextSecondary }
                    add(TableCellData("$b", col, if (b >= 5) FontWeight.Bold else FontWeight.Normal))
                }
                add(TableCellData("${row.total}", TextPrimary, FontWeight.Bold))
            }
            AstroTableRow(columns = cols, striped = idx % 2 == 1, cells = cells)
        }
    }
}

@Composable
private fun VedicDashaSubSection(
    v: VedicChartData,
    expandedDashaLord: String?,
    onToggleDashaLord: (String) -> Unit
) {
    AstroPanelCard(
        title = "Vimshottari Dasha Sequence",
        subtitle = "Tap any Mahadasha row to expand Antardasha sub-periods",
        contentPadding = PaddingValues(0.dp)
    ) {
        val cols = listOf(
            TableColSpec("MAHADASHA", 1.2f, TextAlign.Start, monospace = true),
            TableColSpec("START DATE", 1.3f, TextAlign.Start, monospace = true),
            TableColSpec("END DATE", 1.3f, TextAlign.End, monospace = true)
        )
        AstroTableHeader(cols)
        v.vimshottari_dashas.forEachIndexed { idx, d ->
            val isExpanded = expandedDashaLord == d.lord
            AstroTableRow(
                columns = cols,
                striped = idx % 2 == 1,
                onClick = { onToggleDashaLord(d.lord) },
                cells = listOf(
                    TableCellData("${vedicGrahaCode(d.lord)} · ${d.lord}", if (isExpanded) CelestialBlue else TextPrimary, FontWeight.Bold),
                    TableCellData(d.startDateFormatted, TextSecondary),
                    TableCellData(d.endDateFormatted, TextPrimary, FontWeight.SemiBold)
                )
            )
            if (isExpanded && d.sub_periods.isNotEmpty()) {
                AntardashaSubList(d)
            }
        }
    }
}

@Composable
private fun AntardashaSubList(d: DashaPeriodData) {
    Column(modifier = Modifier.fillMaxWidth().background(SurfaceLight).padding(vertical = 4.dp)) {
        d.sub_periods.forEach { sub ->
            Row(
                modifier = Modifier.fillMaxWidth().height(28.dp).padding(horizontal = 20.dp),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically
            ) {
                Text("${vedicGrahaCode(d.lord)}-${vedicGrahaCode(sub.lord)} (${sub.lord})", fontSize = 11.sp, fontWeight = FontWeight.SemiBold, color = TextPrimary, style = TabularMonoStyle)
                Text("${sub.startDateFormatted} ~ ${sub.endDateFormatted}", fontSize = 11.sp, color = TextSecondary, style = TabularMonoStyle)
            }
        }
    }
    HorizontalDivider(color = BorderColor, thickness = 0.5.dp)
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
internal fun VargaPickerModalSheet(
    selectedVarga: String,
    onSelect: (String) -> Unit,
    onDismiss: () -> Unit
) {
    ModalBottomSheet(
        onDismissRequest = onDismiss,
        sheetState = rememberModalBottomSheetState(skipPartiallyExpanded = true),
        containerColor = BackgroundWhite
    ) {
        Column(
            modifier = Modifier.fillMaxWidth().padding(horizontal = 16.dp).padding(bottom = 28.dp),
            verticalArrangement = Arrangement.spacedBy(10.dp)
        ) {
            Text("SELECT DIVISIONAL CHART (19 VARGAS)", fontSize = 12.sp, fontWeight = FontWeight.Bold, letterSpacing = 0.6.sp, color = TextPrimary, style = TabularMonoStyle)
            VARGA_DEFINITIONS.chunked(3).forEach { rowItems ->
                Row(modifier = Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                    for (i in 0 until 3) {
                        val item = rowItems.getOrNull(i)
                        if (item != null) {
                            VargaTileButton(item.first, item.second, item.first == selectedVarga, { onSelect(item.first) }, Modifier.weight(1f))
                        } else {
                            Spacer(modifier = Modifier.weight(1f))
                        }
                    }
                }
            }
        }
    }
}

@Composable
private fun VargaTileButton(
    code: String,
    name: String,
    isSelected: Boolean,
    onClick: () -> Unit,
    modifier: Modifier
) {
    Column(
        modifier = modifier
            .height(48.dp)
            .clip(RoundedCornerShape(6.dp))
            .background(if (isSelected) AccentPrimary else SurfaceLight)
            .border(1.dp, if (isSelected) AccentPrimary else BorderColor, RoundedCornerShape(6.dp))
            .clickable(onClick = onClick)
            .padding(horizontal = 8.dp, vertical = 5.dp),
        verticalArrangement = Arrangement.Center
    ) {
        Text(code, fontSize = 12.sp, fontWeight = FontWeight.Bold, color = if (isSelected) BackgroundWhite else CelestialBlue, style = TabularMonoStyle, maxLines = 1)
        Text(name, fontSize = 10.sp, color = if (isSelected) BackgroundWhite.copy(alpha = 0.85f) else TextSecondary, maxLines = 1, overflow = TextOverflow.Ellipsis)
    }
}
