package com.fortunexample.note.ui.screens

import android.content.ClipData
import android.content.ClipboardManager
import android.content.Context
import android.widget.Toast
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.filled.ContentCopy
import androidx.compose.material.icons.filled.Edit
import androidx.compose.material.icons.filled.MoreVert
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.fortunexample.note.data.FortuneChartOutput
import com.fortunexample.note.data.HoraryDto
import com.fortunexample.note.ui.components.*
import com.fortunexample.note.ui.theme.*
import java.util.Locale

data class HoraryDetailUiState(
    val chart: FortuneChartOutput? = null,
    val westernMarkdown: String? = null,
    val prashnaMarkdown: String? = null,
    val isLoading: Boolean = true,
    val errorMessage: String? = null
)

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun HoraryDetailScreen(
    horary: HoraryDto,
    state: HoraryDetailUiState,
    onRetry: () -> Unit,
    onBack: () -> Unit,
    onEdit: () -> Unit,
    onDelete: () -> Unit
) {
    val context = LocalContext.current
    var selectedTab by rememberSaveable { mutableIntStateOf(0) }
    var selectedWesternSubTab by rememberSaveable { mutableIntStateOf(0) }
    var selectedPrashnaSubTab by rememberSaveable { mutableIntStateOf(0) }
    var showMenu by remember { mutableStateOf(false) }

    fun copyToClipboard(label: String, text: String?, message: String) {
        if (!text.isNullOrEmpty()) {
            val clipboard = context.getSystemService(Context.CLIPBOARD_SERVICE) as ClipboardManager
            clipboard.setPrimaryClip(ClipData.newPlainText(label, text))
            Toast.makeText(context, message, Toast.LENGTH_SHORT).show()
        }
    }

    Scaffold(
        topBar = {
            HoraryDetailTopBar(
                horary = horary,
                showMenu = showMenu,
                onToggleMenu = { showMenu = it },
                onBack = onBack,
                onEdit = onEdit,
                onCopyActive = {
                    if (selectedTab == 0) copyToClipboard("Western Horary Data", state.westernMarkdown, "Copied Western Horary markdown")
                    else copyToClipboard("Vedic Prashna Data", state.prashnaMarkdown, "Copied Vedic Prashna markdown")
                },
                onCopyWestern = { copyToClipboard("Western Horary Data", state.westernMarkdown, "Copied Western Horary markdown") },
                onCopyPrashna = { copyToClipboard("Vedic Prashna Data", state.prashnaMarkdown, "Copied Vedic Prashna markdown") },
                onDelete = onDelete
            )
        },
        containerColor = BackgroundWhite
    ) { padding ->
        Column(modifier = Modifier.fillMaxSize().padding(padding)) {
            TabRow(selectedTabIndex = selectedTab, containerColor = SurfaceLight, contentColor = AccentPrimary) {
                Tab(
                    selected = selectedTab == 0,
                    onClick = { selectedTab = 0 },
                    text = { Text("HORARY (WESTERN)", fontSize = 12.sp, fontWeight = FontWeight.Bold, letterSpacing = 0.5.sp) }
                )
                Tab(
                    selected = selectedTab == 1,
                    onClick = { selectedTab = 1 },
                    text = { Text("PRASHNA (VEDIC)", fontSize = 12.sp, fontWeight = FontWeight.Bold, letterSpacing = 0.5.sp) }
                )
            }

            when {
                state.isLoading -> LoadingStateBox()
                state.errorMessage != null -> Box(modifier = Modifier.padding(14.dp)) {
                    DomainErrorCard(message = state.errorMessage, onRetry = onRetry)
                }
                state.chart != null -> HoraryChartBody(
                    horary = horary,
                    chart = state.chart,
                    selectedTab = selectedTab,
                    selectedWesternSubTab = selectedWesternSubTab,
                    onSelectWesternSubTab = { selectedWesternSubTab = it },
                    selectedPrashnaSubTab = selectedPrashnaSubTab,
                    onSelectPrashnaSubTab = { selectedPrashnaSubTab = it }
                )
            }
        }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
private fun HoraryDetailTopBar(
    horary: HoraryDto,
    showMenu: Boolean,
    onToggleMenu: (Boolean) -> Unit,
    onBack: () -> Unit,
    onEdit: () -> Unit,
    onCopyActive: () -> Unit,
    onCopyWestern: () -> Unit,
    onCopyPrashna: () -> Unit,
    onDelete: () -> Unit
) {
    TopAppBar(
        title = {
            Column {
                Text(horary.title, fontWeight = FontWeight.Bold, fontSize = 17.sp, color = TextPrimary, maxLines = 1, overflow = TextOverflow.Ellipsis)
                Text(
                    text = String.format(
                        Locale.US,
                        "%04d-%02d-%02d %02d:%02d:%02d · %s",
                        horary.query_year, horary.query_month, horary.query_day,
                        horary.query_hour, horary.query_minute, horary.query_second.toInt(),
                        horary.category.uppercase(Locale.US)
                    ),
                    fontSize = 11.sp,
                    color = TextSecondary,
                    style = TabularMonoStyle
                )
            }
        },
        navigationIcon = {
            IconButton(onClick = onBack) {
                Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = "Back", tint = TextPrimary)
            }
        },
        actions = {
            IconButton(onClick = onEdit) {
                Icon(Icons.Default.Edit, contentDescription = "Edit", tint = TextPrimary)
            }
            IconButton(onClick = onCopyActive) {
                Icon(Icons.Default.ContentCopy, contentDescription = "Copy Markdown", tint = TextPrimary)
            }
            IconButton(onClick = { onToggleMenu(true) }) {
                Icon(Icons.Default.MoreVert, contentDescription = "More Options", tint = TextPrimary)
            }
            DropdownMenu(expanded = showMenu, onDismissRequest = { onToggleMenu(false) }, modifier = Modifier.background(BackgroundWhite)) {
                DropdownMenuItem(text = { Text("Copy Western Horary Markdown", color = TextPrimary, fontSize = 13.sp) }, onClick = { onToggleMenu(false); onCopyWestern() })
                DropdownMenuItem(text = { Text("Copy Vedic Prashna Markdown", color = TextPrimary, fontSize = 13.sp) }, onClick = { onToggleMenu(false); onCopyPrashna() })
                HorizontalDivider(color = BorderColor)
                DropdownMenuItem(text = { Text("Delete Record", color = CrimsonAlert, fontSize = 13.sp, fontWeight = FontWeight.SemiBold) }, onClick = { onToggleMenu(false); onDelete() })
            }
        },
        colors = TopAppBarDefaults.topAppBarColors(containerColor = BackgroundWhite, titleContentColor = TextPrimary, navigationIconContentColor = TextPrimary)
    )
}

@Composable
private fun HoraryChartBody(
    horary: HoraryDto,
    chart: FortuneChartOutput,
    selectedTab: Int,
    selectedWesternSubTab: Int,
    onSelectWesternSubTab: (Int) -> Unit,
    selectedPrashnaSubTab: Int,
    onSelectPrashnaSubTab: (Int) -> Unit
) {
    Column(
        modifier = Modifier
            .fillMaxSize()
            .verticalScroll(rememberScrollState())
            .padding(14.dp),
        verticalArrangement = Arrangement.spacedBy(10.dp)
    ) {
        AstroPanelCard(title = "Query Statement", headerAction = { AstroStatusBadge(horary.category, CelestialBlue) }) {
            Text(horary.question_text, color = TextPrimary, fontSize = 14.sp, fontWeight = FontWeight.SemiBold)
            if (horary.result_notes.isNotBlank()) {
                Spacer(modifier = Modifier.height(6.dp))
                Text(horary.result_notes, color = TextSecondary, fontSize = 12.sp)
            }
        }

        if (selectedTab == 0) {
            HoraryWesternTab(chart.western, selectedWesternSubTab, onSelectWesternSubTab)
        } else {
            HoraryPrashnaTab(chart.vedic, selectedPrashnaSubTab, onSelectPrashnaSubTab)
        }
    }
}

@Composable
private fun HoraryWesternTab(
    w: com.fortunexample.note.data.WesternChartData,
    selectedSubTab: Int,
    onSelectSubTab: (Int) -> Unit
) {
    WesternChartWheel(data = w)
    CompactMetricGrid(
        items = listOf(
            MetricTileItem("ASC", formatLongitudeSignDeg(w.ascendant), "H1", CrimsonAlert),
            MetricTileItem("MC", formatLongitudeSignDeg(w.midheaven), "H10", CelestialBlue),
            MetricTileItem("DSC", formatLongitudeSignDeg((w.ascendant + 180.0) % 360.0), "H7"),
            MetricTileItem("IC", formatLongitudeSignDeg((w.midheaven + 180.0) % 360.0), "H4")
        ),
        columns = 4,
        tileHeight = 44.dp
    )
    AstroSubTabBar(tabs = listOf("Placements", "Aspects", "Traditional"), selectedIndex = selectedSubTab, onSelect = onSelectSubTab)
    when (selectedSubTab) {
        0 -> WesternPlacementsSubSection(w)
        1 -> WesternAspectsSubSection(w)
        2 -> if (w.traditional != null || w.hellenistic != null) {
            WesternTraditionalSection(data = w.traditional, hellenistic = w.hellenistic, isHorary = true)
        }
    }
}

@Composable
private fun HoraryPrashnaTab(
    v: com.fortunexample.note.data.VedicChartData,
    selectedSubTab: Int,
    onSelectSubTab: (Int) -> Unit
) {
    VedicChartDiamond(data = v)
    AstroSubTabBar(tabs = listOf("Placements", "Panchanga & Padas", "Drishti"), selectedIndex = selectedSubTab, onSelect = onSelectSubTab)
    when (selectedSubTab) {
        0 -> VedicPlacementsTableCard(
            v = v,
            title = "Prashna Sidereal Placements",
            subtitle = "Prashna Lagna: ${vedicRasiCode(v.lagna_sign)} ${formatDegMin(v.lagna_sidereal % 30.0)}"
        )
        1 -> Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
            VedicPanchangaCard(v.panchanga, title = "Prashna Panchanga")
            VedicArudhaPadasCard(v.arudha_padas, title = "Prashna Arudha Padas")
        }
        2 -> VedicDrishtiCard(v.drishti, title = "Prashna Graha Drishti", targetColumnHeader = "TARGET")
    }
}
