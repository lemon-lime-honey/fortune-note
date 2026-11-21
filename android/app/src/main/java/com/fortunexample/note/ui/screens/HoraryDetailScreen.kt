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
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.fortunexample.note.data.FortuneChartOutput
import com.fortunexample.note.data.HoraryDto
import com.fortunexample.note.ui.components.*
import com.fortunexample.note.ui.theme.*
import java.util.Locale

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun HoraryDetailScreen(
    horary: HoraryDto,
    chart: FortuneChartOutput?,
    westernMarkdown: String?,
    prashnaMarkdown: String?,
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
            TopAppBar(
                title = {
                    Column {
                        Text(
                            text = horary.title,
                            fontWeight = FontWeight.Bold,
                            fontSize = 17.sp,
                            color = TextPrimary,
                            maxLines = 1,
                            overflow = TextOverflow.Ellipsis
                        )
                        Text(
                            text = String.format(
                                Locale.US,
                                "%04d-%02d-%02d %02d:%02d:%02d · %s",
                                horary.query_year,
                                horary.query_month,
                                horary.query_day,
                                horary.query_hour,
                                horary.query_minute,
                                horary.query_second.toInt(),
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
                    IconButton(onClick = {
                        if (selectedTab == 0) {
                            copyToClipboard("Western Horary Data", westernMarkdown, "Copied Western Horary markdown")
                        } else {
                            copyToClipboard("Vedic Prashna Data", prashnaMarkdown, "Copied Vedic Prashna markdown")
                        }
                    }) {
                        Icon(Icons.Default.ContentCopy, contentDescription = "Copy Markdown", tint = TextPrimary)
                    }
                    IconButton(onClick = { showMenu = true }) {
                        Icon(Icons.Default.MoreVert, contentDescription = "More Options", tint = TextPrimary)
                    }
                    DropdownMenu(
                        expanded = showMenu,
                        onDismissRequest = { showMenu = false },
                        modifier = Modifier.background(BackgroundWhite)
                    ) {
                        DropdownMenuItem(
                            text = { Text("Copy Western Horary Markdown", color = TextPrimary, fontSize = 13.sp) },
                            onClick = {
                                showMenu = false
                                copyToClipboard("Western Horary Data", westernMarkdown, "Copied Western Horary markdown")
                            }
                        )
                        DropdownMenuItem(
                            text = { Text("Copy Vedic Prashna Markdown", color = TextPrimary, fontSize = 13.sp) },
                            onClick = {
                                showMenu = false
                                copyToClipboard("Vedic Prashna Data", prashnaMarkdown, "Copied Vedic Prashna markdown")
                            }
                        )
                        HorizontalDivider(color = BorderColor)
                        DropdownMenuItem(
                            text = { Text("Delete Record", color = CrimsonAlert, fontSize = 13.sp, fontWeight = FontWeight.SemiBold) },
                            onClick = {
                                showMenu = false
                                onDelete()
                            }
                        )
                    }
                },
                colors = TopAppBarDefaults.topAppBarColors(
                    containerColor = BackgroundWhite,
                    titleContentColor = TextPrimary,
                    navigationIconContentColor = TextPrimary
                )
            )
        },
        containerColor = BackgroundWhite
    ) { padding ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(padding)
        ) {
            TabRow(
                selectedTabIndex = selectedTab,
                containerColor = SurfaceLight,
                contentColor = AccentPrimary
            ) {
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

            if (chart == null) {
                Box(modifier = Modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
                    CircularProgressIndicator(color = AccentPrimary, strokeWidth = 2.dp)
                }
            } else {
                Column(
                    modifier = Modifier
                        .fillMaxSize()
                        .verticalScroll(rememberScrollState())
                        .padding(14.dp),
                    verticalArrangement = Arrangement.spacedBy(10.dp)
                ) {
                    // Query Summary Panel
                    AstroPanelCard(
                        title = "Query Statement",
                        headerAction = {
                            AstroStatusBadge(horary.category, CelestialBlue)
                        }
                    ) {
                        Text(
                            text = horary.question_text,
                            color = TextPrimary,
                            fontSize = 14.sp,
                            fontWeight = FontWeight.SemiBold
                        )
                        if (horary.result_notes.isNotBlank()) {
                            Spacer(modifier = Modifier.height(6.dp))
                            Text(
                                text = horary.result_notes,
                                color = TextSecondary,
                                fontSize = 12.sp
                            )
                        }
                    }

                    if (selectedTab == 0) {
                        val w = chart.western
                        WesternChartWheel(data = w)

                        val angleItems = listOf(
                            MetricTileItem("ASC", formatLongitudeSignDeg(w.ascendant), "H1", CrimsonAlert),
                            MetricTileItem("MC", formatLongitudeSignDeg(w.midheaven), "H10", CelestialBlue),
                            MetricTileItem("DSC", formatLongitudeSignDeg((w.ascendant + 180.0) % 360.0), "H7"),
                            MetricTileItem("IC", formatLongitudeSignDeg((w.midheaven + 180.0) % 360.0), "H4")
                        )
                        CompactMetricGrid(items = angleItems, columns = 4, tileHeight = 44.dp)

                        AstroSubTabBar(
                            tabs = listOf("Placements", "Aspects", "Traditional"),
                            selectedIndex = selectedWesternSubTab,
                            onSelect = { selectedWesternSubTab = it }
                        )

                        when (selectedWesternSubTab) {
                            0 -> WesternPlacementsSubSection(w)
                            1 -> WesternAspectsSubSection(w)
                            2 -> {
                                if (w.traditional != null || w.hellenistic != null) {
                                    WesternTraditionalSection(
                                        data = w.traditional,
                                        hellenistic = w.hellenistic,
                                        isHorary = true
                                    )
                                }
                            }
                        }
                    } else {
                        val v = chart.vedic
                        VedicChartDiamond(data = v)

                        AstroSubTabBar(
                            tabs = listOf("Placements", "Panchanga & Padas", "Drishti"),
                            selectedIndex = selectedPrashnaSubTab,
                            onSelect = { selectedPrashnaSubTab = it }
                        )

                        when (selectedPrashnaSubTab) {
                            0 -> {
                                AstroPanelCard(
                                    title = "Prashna Sidereal Placements",
                                    subtitle = "Prashna Lagna: ${vedicRasiCode(v.lagna_sign)} ${formatDegMin(v.lagna_sidereal % 30.0)}",
                                    contentPadding = PaddingValues(0.dp)
                                ) {
                                    val cols = listOf(
                                        TableColSpec("GRAHA", 0.8f, TextAlign.Start, monospace = true),
                                        TableColSpec("RASI & DEG", 1.3f, TextAlign.Start, monospace = true),
                                        TableColSpec("H", 0.5f, TextAlign.Center, monospace = true),
                                        TableColSpec("NAKSHATRA", 1.45f, TextAlign.Start, monospace = true),
                                        TableColSpec("STATUS", 1.1f, TextAlign.End, monospace = true)
                                    )
                                    AstroTableHeader(cols)
                                    v.planets.forEachIndexed { idx, p ->
                                        val dignityCode = when (p.dignity) {
                                            "Exalted" -> "EXLT"
                                            "Moolatrikona" -> "MOOL"
                                            "Own Sign", "OwnSign" -> "OWN"
                                            "Great Friend" -> "GFRN"
                                            "Friend" -> "FRND"
                                            "Enemy" -> "ENMY"
                                            "Great Enemy" -> "GENM"
                                            "Debilitated" -> "DEBL"
                                            else -> p.dignity.take(4).trim().uppercase(Locale.US)
                                        }
                                        val statusTag = buildList {
                                            if (p.dignity.isNotBlank() && p.dignity != "Neutral") {
                                                add(dignityCode)
                                            }
                                            if (p.combustion != "None" && p.combustion.isNotBlank()) {
                                                add(
                                                    when (p.combustion) {
                                                        "Cazimi" -> "CAZ"
                                                        "DeeplyCombust" -> "DCMB"
                                                        else -> "CMB"
                                                    }
                                                )
                                            }
                                            if (p.is_retrograde && p.name != "Rahu" && p.name != "Ketu") {
                                                add("RX")
                                            }
                                        }.joinToString("·").ifEmpty { "NORM" }

                                        val statusColor = when {
                                            p.dignity == "Exalted" || p.dignity == "Moolatrikona" || p.dignity == "Own Sign" || p.dignity == "OwnSign" -> CelestialBlue
                                            p.dignity == "Debilitated" || p.combustion == "Combust" || p.combustion == "DeeplyCombust" -> CrimsonAlert
                                            else -> TextSecondary
                                        }

                                        AstroTableRow(
                                            columns = cols,
                                            striped = idx % 2 == 1,
                                            cells = listOf(
                                                TableCellData(vedicGrahaCode(p.name), TextPrimary, FontWeight.Bold),
                                                TableCellData(formatSignAndDeg(p.sign_index, p.sign_degree), TextPrimary),
                                                TableCellData("${p.house}", TextSecondary),
                                                TableCellData("${p.nakshatra_name.take(8)} P${p.nakshatra_pada}", TextSecondary),
                                                TableCellData(statusTag, statusColor, FontWeight.Bold)
                                            )
                                        )
                                    }
                                }
                            }
                            1 -> {
                                Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
                                    val pan = v.panchanga
                                    if (pan.tithi_name.isNotEmpty()) {
                                        AstroPanelCard(
                                            title = "Prashna Panchanga",
                                            contentPadding = PaddingValues(0.dp)
                                        ) {
                                            val cols = listOf(
                                                TableColSpec("LIMB", 1.1f, TextAlign.Start, monospace = true),
                                                TableColSpec("VALUE", 1.9f, TextAlign.Start),
                                                TableColSpec("META / LORD", 1.2f, TextAlign.End, monospace = true)
                                            )
                                            AstroTableHeader(cols)
                                            AstroTableRow(
                                                columns = cols,
                                                striped = false,
                                                cells = listOf(
                                                    TableCellData("TITHI", TextSecondary, FontWeight.Bold),
                                                    TableCellData(pan.tithi_name, TextPrimary, FontWeight.SemiBold),
                                                    TableCellData("#${pan.tithi_number} · ${pan.paksha.take(6).uppercase(Locale.US)}", CelestialBlue)
                                                )
                                            )
                                            AstroTableRow(
                                                columns = cols,
                                                striped = true,
                                                cells = listOf(
                                                    TableCellData("VARA", TextSecondary, FontWeight.Bold),
                                                    TableCellData(pan.vara_name, TextPrimary, FontWeight.SemiBold),
                                                    TableCellData("LORD ${vedicGrahaCode(pan.vara_lord)}", TextSecondary)
                                                )
                                            )
                                            AstroTableRow(
                                                columns = cols,
                                                striped = false,
                                                cells = listOf(
                                                    TableCellData("NAKSHATRA", TextSecondary, FontWeight.Bold),
                                                    TableCellData("${pan.nakshatra_name} (Pada ${pan.nakshatra_pada})", TextPrimary, FontWeight.SemiBold),
                                                    TableCellData("LORD ${vedicGrahaCode(pan.nakshatra_lord)}", CelestialBlue)
                                                )
                                            )
                                            AstroTableRow(
                                                columns = cols,
                                                striped = true,
                                                cells = listOf(
                                                    TableCellData("YOGA", TextSecondary, FontWeight.Bold),
                                                    TableCellData(pan.yoga_name, TextPrimary, FontWeight.SemiBold),
                                                    TableCellData("-", TextSecondary)
                                                )
                                            )
                                            AstroTableRow(
                                                columns = cols,
                                                striped = false,
                                                cells = listOf(
                                                    TableCellData("KARANA", TextSecondary, FontWeight.Bold),
                                                    TableCellData(pan.karana_name, TextPrimary, FontWeight.SemiBold),
                                                    TableCellData("-", TextSecondary)
                                                )
                                            )
                                        }
                                    }

                                    if (v.arudha_padas.isNotEmpty()) {
                                        AstroPanelCard(title = "Prashna Arudha Padas") {
                                            val tiles = v.arudha_padas.map { ap ->
                                                val isPrimary = ap.code == "AL" || ap.code == "UL"
                                                MetricTileItem(
                                                    tag = ap.code,
                                                    primary = signCode(ap.sign_index),
                                                    secondary = "H${ap.house_from_lagna}",
                                                    highlightColor = if (isPrimary) CelestialBlue else null
                                                )
                                            }
                                            CompactMetricGrid(items = tiles, columns = 4)
                                        }
                                    }
                                }
                            }
                            2 -> {
                                val notableDrishti = v.drishti.filter { it.aspected_planets.isNotEmpty() || it.aspect_offset != 7 }
                                AstroPanelCard(
                                    title = "Prashna Graha Drishti",
                                    contentPadding = PaddingValues(0.dp)
                                ) {
                                    if (notableDrishti.isEmpty()) {
                                        Box(modifier = Modifier.padding(12.dp)) {
                                            Text("No active Graha Drishti.", fontSize = 12.sp, color = TextSecondary)
                                        }
                                    } else {
                                        val cols = listOf(
                                            TableColSpec("SOURCE", 0.95f, TextAlign.Start, monospace = true),
                                            TableColSpec("DRISHTI", 0.85f, TextAlign.Center, monospace = true),
                                            TableColSpec("TARGET", 1.15f, TextAlign.Center, monospace = true),
                                            TableColSpec("OCCUPANTS", 1.35f, TextAlign.End, monospace = true)
                                        )
                                        AstroTableHeader(cols)
                                        notableDrishti.forEachIndexed { idx, d ->
                                            val occText = if (d.aspected_planets.isEmpty()) {
                                                "- (EMPTY)"
                                            } else {
                                                d.aspected_planets.joinToString(" ") { vedicGrahaCode(it) }
                                            }
                                            AstroTableRow(
                                                columns = cols,
                                                striped = idx % 2 == 1,
                                                cells = listOf(
                                                    TableCellData("${vedicGrahaCode(d.from_planet)} (H${d.from_house})", TextPrimary, FontWeight.Bold),
                                                    TableCellData("${d.aspect_offset}TH", CelestialBlue, FontWeight.Bold),
                                                    TableCellData("H${d.target_house}·${signCode(d.target_sign)}", TextSecondary),
                                                    TableCellData(
                                                        text = occText,
                                                        color = if (d.aspected_planets.isEmpty()) TextSecondary else TextPrimary,
                                                        fontWeight = if (d.aspected_planets.isEmpty()) FontWeight.Normal else FontWeight.SemiBold
                                                    )
                                                )
                                            )
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
