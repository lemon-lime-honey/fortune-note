package com.fortunexample.note.ui.screens

import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.horizontalScroll
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.filled.ContentCopy
import androidx.compose.material.icons.filled.Delete
import androidx.compose.material.icons.filled.Edit
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.fortunexample.note.FortuneApp
import com.fortunexample.note.data.*
import com.fortunexample.note.ui.components.*
import com.fortunexample.note.ui.theme.*
import kotlinx.coroutines.launch
import java.util.Calendar
import java.util.Locale

private val VARGA_DEFINITIONS = listOf(
    "D1" to "Rasi",
    "D2" to "Hora",
    "D3" to "Drekkana",
    "D4" to "Chaturthamsha",
    "D6" to "Shashthamsha",
    "D7" to "Saptamsha",
    "D8" to "Ashtamamsha",
    "D9" to "Navamsha",
    "D10" to "Dashamsha",
    "D11" to "Rudramsha",
    "D12" to "Dwadashamsha",
    "D16" to "Shodashamsha",
    "D20" to "Vimshamsha",
    "D24" to "Chaturvimshamsha",
    "D27" to "Saptavimshamsha",
    "D30" to "Trimshamsha",
    "D40" to "Khavedamsha",
    "D45" to "Akshavedamsha",
    "D60" to "Shashtiamsha"
)

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ProfileDetailScreen(
    profile: ProfileDto,
    onBack: () -> Unit,
    onEdit: () -> Unit,
    onDelete: () -> Unit
) {
    val scope = rememberCoroutineScope()
    var selectedDomainTab by rememberSaveable { mutableIntStateOf(0) }
    var selectedWesternSubTab by rememberSaveable { mutableIntStateOf(0) }
    var selectedVedicSubTab by rememberSaveable { mutableIntStateOf(0) }

    var selectedVarga by rememberSaveable { mutableStateOf("D1") }
    var showVargaPickerSheet by remember { mutableStateOf(false) }
    var expandedDashaLord by remember { mutableStateOf<String?>(null) }
    var showSelectiveCopySheet by remember { mutableStateOf(false) }

    // Decoupled domain states
    var sajuOutput by remember { mutableStateOf<SajuCalculationOutputDto?>(null) }
    var isSajuLoading by remember { mutableStateOf(false) }
    var sajuError by remember { mutableStateOf<String?>(null) }

    var westernOutput by remember { mutableStateOf<WesternCalculationOutputDto?>(null) }
    var isWesternLoading by remember { mutableStateOf(false) }
    var westernError by remember { mutableStateOf<String?>(null) }
    var selectedHouse by rememberSaveable { mutableStateOf("Regiomontanus") }

    var vedicOutput by remember { mutableStateOf<VedicCalculationOutputDto?>(null) }
    var isVedicLoading by remember { mutableStateOf(false) }
    var vedicError by remember { mutableStateOf<String?>(null) }

    val todayCal = remember { Calendar.getInstance() }
    var predictiveTargetYear by remember { mutableIntStateOf(todayCal.get(Calendar.YEAR)) }
    var predictiveTargetMonth by remember { mutableIntStateOf(todayCal.get(Calendar.MONTH) + 1) }
    var predictiveTargetDay by remember { mutableIntStateOf(todayCal.get(Calendar.DAY_OF_MONTH)) }
    var predictiveTargetHour by remember { mutableStateOf<Int?>(null) }
    var predictiveTargetMinute by remember { mutableStateOf<Int?>(null) }
    var predictiveData by remember { mutableStateOf<PredictiveSuiteData?>(null) }
    var isPredictiveLoading by remember { mutableStateOf(false) }

    LaunchedEffect(profile) {
        sajuOutput = null
        westernOutput = null
        vedicOutput = null
        predictiveData = null
        isSajuLoading = true
        sajuError = null
        FortuneApp.repository.calculateProfileSaju(profile.id)
            .onSuccess {
                sajuOutput = it
                isSajuLoading = false
            }
            .onFailure {
                sajuError = it.message ?: "Failed to compute Four Pillars data."
                isSajuLoading = false
            }
    }

    LaunchedEffect(profile, selectedDomainTab, selectedHouse) {
        if (selectedDomainTab == 1 && (westernOutput == null || westernOutput?.western?.house_system != selectedHouse)) {
            isWesternLoading = true
            westernError = null
            FortuneApp.repository.calculateProfileWestern(profile.id, selectedHouse)
                .onSuccess {
                    westernOutput = it
                    isWesternLoading = false
                }
                .onFailure {
                    westernError = it.message ?: "Failed to compute Western chart."
                    isWesternLoading = false
                }
        }
    }

    LaunchedEffect(profile, selectedDomainTab) {
        if (selectedDomainTab == 2 && vedicOutput == null) {
            isVedicLoading = true
            vedicError = null
            FortuneApp.repository.calculateProfileVedic(profile.id)
                .onSuccess {
                    vedicOutput = it
                    isVedicLoading = false
                }
                .onFailure {
                    vedicError = it.message ?: "Failed to compute Vedic chart."
                    isVedicLoading = false
                }
        }
    }

    LaunchedEffect(
        profile,
        predictiveTargetYear,
        predictiveTargetMonth,
        predictiveTargetDay,
        predictiveTargetHour,
        predictiveTargetMinute,
        selectedHouse,
        selectedDomainTab
    ) {
        if (selectedDomainTab == 1) {
            isPredictiveLoading = true
            FortuneApp.repository.calculateProfilePredictiveSuite(
                id = profile.id,
                targetYear = predictiveTargetYear,
                targetMonth = predictiveTargetMonth,
                targetDay = predictiveTargetDay,
                targetHour = predictiveTargetHour,
                targetMinute = predictiveTargetMinute,
                houseSystem = selectedHouse
            ).onSuccess {
                predictiveData = it
                isPredictiveLoading = false
            }.onFailure {
                isPredictiveLoading = false
            }
        }
    }

    Scaffold(
        topBar = {
            TopAppBar(
                title = {
                    Column {
                        Text(
                            text = profile.name,
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
                                profile.birth_year,
                                profile.birth_month,
                                profile.birth_day,
                                profile.birth_hour,
                                profile.birth_minute,
                                profile.birth_second.toInt(),
                                if (profile.gender == "male") "M" else "F"
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
                    IconButton(
                        onClick = { showSelectiveCopySheet = true },
                        enabled = when (selectedDomainTab) {
                            0 -> sajuOutput != null
                            1 -> westernOutput != null
                            else -> vedicOutput != null
                        }
                    ) {
                        Icon(Icons.Default.ContentCopy, contentDescription = "Copy Markdown", tint = TextPrimary)
                    }
                    IconButton(onClick = onEdit) {
                        Icon(Icons.Default.Edit, contentDescription = "Edit", tint = TextPrimary)
                    }
                    IconButton(onClick = onDelete) {
                        Icon(Icons.Default.Delete, contentDescription = "Delete", tint = CrimsonAlert)
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
                selectedTabIndex = selectedDomainTab,
                containerColor = SurfaceLight,
                contentColor = AccentPrimary
            ) {
                Tab(
                    selected = selectedDomainTab == 0,
                    onClick = { selectedDomainTab = 0 },
                    text = { Text("FOUR PILLARS", fontSize = 12.sp, fontWeight = FontWeight.Bold, letterSpacing = 0.5.sp) }
                )
                Tab(
                    selected = selectedDomainTab == 1,
                    onClick = { selectedDomainTab = 1 },
                    text = { Text("WESTERN", fontSize = 12.sp, fontWeight = FontWeight.Bold, letterSpacing = 0.5.sp) }
                )
                Tab(
                    selected = selectedDomainTab == 2,
                    onClick = { selectedDomainTab = 2 },
                    text = { Text("VEDIC", fontSize = 12.sp, fontWeight = FontWeight.Bold, letterSpacing = 0.5.sp) }
                )
            }

            Column(
                modifier = Modifier
                    .fillMaxSize()
                    .verticalScroll(rememberScrollState())
                    .padding(14.dp)
            ) {
                when (selectedDomainTab) {
                    0 -> {
                        when {
                            isSajuLoading -> LoadingStateBox()
                            sajuError != null -> DomainErrorCard(message = sajuError!!) {
                                scope.launch {
                                    isSajuLoading = true
                                    sajuError = null
                                    FortuneApp.repository.calculateProfileSaju(profile.id)
                                        .onSuccess { sajuOutput = it; isSajuLoading = false }
                                        .onFailure { sajuError = it.message ?: "Error"; isSajuLoading = false }
                                }
                            }
                            sajuOutput != null -> SajuChartTable(
                                chart = sajuOutput!!.saju,
                                modifier = Modifier.padding(0.dp)
                            )
                        }
                    }

                    1 -> {
                        WesternDomainContent(
                            profile = profile,
                            westernOutput = westernOutput,
                            isLoading = isWesternLoading,
                            error = westernError,
                            selectedHouse = selectedHouse,
                            onSelectHouse = { selectedHouse = it },
                            selectedSubTab = selectedWesternSubTab,
                            onSelectSubTab = { selectedWesternSubTab = it },
                            predictiveData = predictiveData,
                            predictiveTargetYear = predictiveTargetYear,
                            predictiveTargetMonth = predictiveTargetMonth,
                            predictiveTargetDay = predictiveTargetDay,
                            isPredictiveLoading = isPredictiveLoading,
                            onYearChange = { predictiveTargetYear = it },
                            onDateChange = { y, m, d ->
                                predictiveTargetYear = y
                                predictiveTargetMonth = m
                                predictiveTargetDay = d
                            },
                            onTimeChange = { hr, min ->
                                predictiveTargetHour = hr
                                predictiveTargetMinute = min
                            },
                            onDateTimeChange = { y, m, d, hr, min ->
                                predictiveTargetYear = y
                                predictiveTargetMonth = m
                                predictiveTargetDay = d
                                predictiveTargetHour = hr
                                predictiveTargetMinute = min
                            },
                            onRetry = {
                                scope.launch {
                                    isWesternLoading = true
                                    westernError = null
                                    FortuneApp.repository.calculateProfileWestern(profile.id, selectedHouse)
                                        .onSuccess { westernOutput = it; isWesternLoading = false }
                                        .onFailure { westernError = it.message ?: "Error"; isWesternLoading = false }
                                }
                            }
                        )
                    }

                    2 -> {
                        VedicDomainContent(
                            vedicOutput = vedicOutput,
                            isLoading = isVedicLoading,
                            error = vedicError,
                            selectedVarga = selectedVarga,
                            onOpenVargaPicker = { showVargaPickerSheet = true },
                            selectedSubTab = selectedVedicSubTab,
                            onSelectSubTab = { selectedVedicSubTab = it },
                            expandedDashaLord = expandedDashaLord,
                            onToggleDashaLord = { lord ->
                                expandedDashaLord = if (expandedDashaLord == lord) null else lord
                            },
                            onRetry = {
                                scope.launch {
                                    isVedicLoading = true
                                    vedicError = null
                                    FortuneApp.repository.calculateProfileVedic(profile.id)
                                        .onSuccess { vedicOutput = it; isVedicLoading = false }
                                        .onFailure { vedicError = it.message ?: "Error"; isVedicLoading = false }
                                }
                            }
                        )
                    }
                }
            }
        }

        if (showVargaPickerSheet) {
            VargaPickerModalSheet(
                selectedVarga = selectedVarga,
                onSelect = { key ->
                    selectedVarga = key
                    showVargaPickerSheet = false
                },
                onDismiss = { showVargaPickerSheet = false }
            )
        }

        SelectiveCopyBottomSheet(
            isOpen = showSelectiveCopySheet,
            onDismiss = { showSelectiveCopySheet = false },
            profile = profile,
            saju = sajuOutput?.saju,
            western = westernOutput?.western,
            predictive = predictiveData,
            vedic = vedicOutput?.vedic,
            timezoneId = sajuOutput?.timezone_id ?: westernOutput?.timezone_id ?: vedicOutput?.timezone_id ?: profile.timezone_id,
            utcOffsetHours = sajuOutput?.utc_offset_hours ?: westernOutput?.utc_offset_hours ?: vedicOutput?.utc_offset_hours ?: 9.0,
            selectedDomainTab = selectedDomainTab
        )
    }
}

@Composable
private fun WesternDomainContent(
    profile: ProfileDto,
    westernOutput: WesternCalculationOutputDto?,
    isLoading: Boolean,
    error: String?,
    selectedHouse: String,
    onSelectHouse: (String) -> Unit,
    selectedSubTab: Int,
    onSelectSubTab: (Int) -> Unit,
    predictiveData: PredictiveSuiteData?,
    predictiveTargetYear: Int,
    predictiveTargetMonth: Int,
    predictiveTargetDay: Int,
    isPredictiveLoading: Boolean,
    onYearChange: (Int) -> Unit,
    onDateChange: (Int, Int, Int) -> Unit,
    onTimeChange: (Int?, Int?) -> Unit,
    onDateTimeChange: (Int, Int, Int, Int?, Int?) -> Unit,
    onRetry: () -> Unit
) {
    val traditionSystems = listOf(
        "WholeSign" to "헬레니즘 · WHOLE SIGN",
        "Regiomontanus" to "중세 · REGIOMONTANUS"
    )

    Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
        // Tradition & House system selector bar (strictly Hellenistic Whole Sign vs Medieval Regiomontanus)
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

        when {
            isLoading -> LoadingStateBox()
            error != null -> DomainErrorCard(message = error, onRetry = onRetry)
            westernOutput != null -> {
                val w = westernOutput.western
                val isWholeSign = selectedHouse.equals("WholeSign", ignoreCase = true) ||
                    w.house_system?.lowercase() in listOf("wholesign", "whole_sign", "whole", "hellenistic")

                // Chart Wheel + Angles Summary Strip
                WesternChartWheel(data = w)

                val angleItems = listOf(
                    MetricTileItem("ASC", formatLongitudeSignDeg(w.ascendant), "H1", CrimsonAlert),
                    MetricTileItem("MC", formatLongitudeSignDeg(w.midheaven), if (isWholeSign && w.hellenistic != null) "H${w.hellenistic.angles.midheaven_house}" else "H10", CelestialBlue),
                    MetricTileItem("DSC", formatLongitudeSignDeg((w.ascendant + 180.0) % 360.0), "H7"),
                    MetricTileItem("IC", formatLongitudeSignDeg((w.midheaven + 180.0) % 360.0), "H4")
                )
                CompactMetricGrid(items = angleItems, columns = 4, tileHeight = 44.dp)

                // Sub-tabs to eliminate endless scrolling
                AstroSubTabBar(
                    tabs = listOf(
                        "Placements",
                        "Aspects",
                        if (isWholeSign) "Hellenistic" else "Medieval",
                        "Predictive"
                    ),
                    selectedIndex = selectedSubTab,
                    onSelect = onSelectSubTab
                )

                when (selectedSubTab) {
                    0 -> WesternPlacementsSubSection(w)
                    1 -> WesternAspectsSubSection(w)
                    2 -> {
                        val activeTrad = if (isWholeSign) null else w.traditional
                        val activeHell = if (isWholeSign) w.hellenistic else null
                        if (activeTrad != null || activeHell != null) {
                            WesternTraditionalSection(
                                data = activeTrad,
                                hellenistic = activeHell,
                                isHorary = false
                            )
                        } else {
                            AstroPanelCard(title = if (isWholeSign) "Hellenistic Analysis" else "Medieval Analysis") {
                                Text("Analysis data unavailable.", fontSize = 12.sp, color = TextSecondary)
                            }
                        }
                    }
                    3 -> {
                        WesternPredictiveSection(
                            profile = profile,
                            predictiveData = predictiveData,
                            targetYear = predictiveTargetYear,
                            targetMonth = predictiveTargetMonth,
                            targetDay = predictiveTargetDay,
                            isLoading = isPredictiveLoading,
                            onYearChange = onYearChange,
                            onDateChange = onDateChange,
                            onTimeChange = onTimeChange,
                            onDateTimeChange = onDateTimeChange
                        )
                    }
                }
            }
        }
    }
}

@Composable
fun WesternPlacementsSubSection(w: WesternChartData) {
    Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
        // 1. Planetary & Nodal Positions Table
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

        // 2. 12 House Cusps (Compact 3x4 Tile Grid)
        if (w.house_cusps.size == 12) {
            AstroPanelCard(
                title = "12 House Cusps (${w.house_system ?: "Default"})"
            ) {
                val tiles = w.house_cusps.mapIndexed { idx, cuspLon ->
                    val hNum = idx + 1
                    val norm = ((cuspLon % 360.0) + 360.0) % 360.0
                    val sIdx = (norm / 30.0).toInt().coerceIn(0, 11)
                    val ruler = bodyCode(getZodiacSignRuler(sIdx))
                    val highlight = when (hNum) {
                        1 -> CrimsonAlert
                        10 -> CelestialBlue
                        4, 7 -> TextPrimary
                        else -> null
                    }
                    MetricTileItem(
                        tag = "H$hNum",
                        primary = formatLongitudeSignDeg(cuspLon),
                        secondary = ruler,
                        highlightColor = highlight
                    )
                }
                CompactMetricGrid(items = tiles, columns = 3)
            }
        }
    }
}

@Composable
fun WesternAspectsSubSection(w: WesternChartData) {
    val isWholeSign = w.house_system?.lowercase() in listOf("wholesign", "whole_sign", "whole", "hellenistic")
    val tradAspects = if (isWholeSign) {
        w.hellenistic?.aspects ?: emptyList()
    } else {
        w.traditional?.traditional_aspects ?: emptyList()
    }

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
        if (tradAspects.isNotEmpty()) {
            val cols = listOf(
                TableColSpec("BODY A", 0.9f, TextAlign.Start, monospace = true),
                TableColSpec("ASPECT", 1.1f, TextAlign.Center, monospace = true),
                TableColSpec("BODY B", 0.9f, TextAlign.Center, monospace = true),
                TableColSpec("ORB", 0.85f, TextAlign.End, monospace = true),
                TableColSpec("MOTION", 0.95f, TextAlign.End, monospace = true)
            )
            AstroTableHeader(cols)
            tradAspects.forEachIndexed { idx, a ->
                val aspColor = when {
                    isHardAspect(a.aspect_type) -> CrimsonAlert
                    isSoftAspect(a.aspect_type) -> CelestialBlue
                    else -> TextPrimary
                }
                val motionTag = buildString {
                    append(if (a.is_applying) "APP" else "SEP")
                    if (a.is_partile) append("·PRT")
                }
                AstroTableRow(
                    columns = cols,
                    striped = idx % 2 == 1,
                    cells = listOf(
                        TableCellData(bodyCode(a.planet1), TextPrimary, FontWeight.Bold),
                        TableCellData(aspectCode(a.aspect_type), aspColor, FontWeight.Bold),
                        TableCellData(bodyCode(a.planet2), TextPrimary, FontWeight.Bold),
                        TableCellData(formatOrb(a.orb), TextPrimary),
                        TableCellData(
                            text = motionTag,
                            color = if (a.is_applying) CelestialBlue else TextSecondary,
                            badgeColor = if (a.is_partile) CelestialBlue else null
                        )
                    )
                )
            }
        } else if (w.aspects.isNotEmpty()) {
            val cols = listOf(
                TableColSpec("BODY A", 1.0f, TextAlign.Start, monospace = true),
                TableColSpec("ASPECT", 1.2f, TextAlign.Center, monospace = true),
                TableColSpec("BODY B", 1.0f, TextAlign.Center, monospace = true),
                TableColSpec("ORB", 1.0f, TextAlign.End, monospace = true)
            )
            AstroTableHeader(cols)
            w.aspects.forEachIndexed { idx, a ->
                val aspColor = when {
                    isHardAspect(a.aspect_type) -> CrimsonAlert
                    isSoftAspect(a.aspect_type) -> CelestialBlue
                    else -> TextPrimary
                }
                AstroTableRow(
                    columns = cols,
                    striped = idx % 2 == 1,
                    cells = listOf(
                        TableCellData(bodyCode(a.planet1), TextPrimary, FontWeight.Bold),
                        TableCellData(aspectCode(a.aspect_type), aspColor, FontWeight.Bold),
                        TableCellData(bodyCode(a.planet2), TextPrimary, FontWeight.Bold),
                        TableCellData(formatOrb(a.orb), TextPrimary)
                    )
                )
            }
        } else {
            Box(modifier = Modifier.padding(12.dp)) {
                Text("No major aspects within orb.", fontSize = 12.sp, color = TextSecondary)
            }
        }
    }
}

@Composable
private fun VedicDomainContent(
    vedicOutput: VedicCalculationOutputDto?,
    isLoading: Boolean,
    error: String?,
    selectedVarga: String,
    onOpenVargaPicker: () -> Unit,
    selectedSubTab: Int,
    onSelectSubTab: (Int) -> Unit,
    expandedDashaLord: String?,
    onToggleDashaLord: (String) -> Unit,
    onRetry: () -> Unit
) {
    when {
        isLoading -> LoadingStateBox()
        error != null -> DomainErrorCard(message = error, onRetry = onRetry)
        vedicOutput != null -> {
            val v = vedicOutput.vedic
            val activeVargaChart = v.vargas[selectedVarga]
            val vargaTitle = VARGA_DEFINITIONS.find { it.first == selectedVarga }?.let { "${it.first} · ${it.second}" } ?: selectedVarga

            Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
                // Clean Varga Header + Modal Picker Button
                Surface(
                    shape = RoundedCornerShape(6.dp),
                    color = SurfaceLight,
                    border = BorderStroke(1.dp, BorderColor),
                    modifier = Modifier
                        .fillMaxWidth()
                        .clickable { onOpenVargaPicker() }
                ) {
                    Row(
                        modifier = Modifier
                            .fillMaxWidth()
                            .padding(horizontal = 12.dp, vertical = 8.dp),
                        horizontalArrangement = Arrangement.SpaceBetween,
                        verticalAlignment = Alignment.CenterVertically
                    ) {
                        Column {
                            Text(
                                text = "ACTIVE DIVISIONAL CHART",
                                fontSize = 9.sp,
                                fontWeight = FontWeight.Bold,
                                color = TextSecondary,
                                style = TabularMonoStyle
                            )
                            Text(
                                text = vargaTitle.uppercase(Locale.US),
                                fontSize = 13.sp,
                                fontWeight = FontWeight.Bold,
                                color = TextPrimary,
                                style = TabularMonoStyle
                            )
                        }
                        Surface(
                            shape = RoundedCornerShape(4.dp),
                            color = AccentPrimary
                        ) {
                            Text(
                                text = "SELECT VARGA (19)",
                                fontSize = 10.sp,
                                fontWeight = FontWeight.Bold,
                                color = BackgroundWhite,
                                style = TabularMonoStyle,
                                modifier = Modifier.padding(horizontal = 8.dp, vertical = 5.dp)
                            )
                        }
                    }
                }

                // North Indian Diamond Chart
                VedicChartDiamond(
                    data = v,
                    selectedVarga = activeVargaChart
                )

                // Sub-navigation for Vedic Sections
                AstroSubTabBar(
                    tabs = listOf("Placements", "Panchanga & Karakas", "Padas & Drishti", "Ashtakavarga", "Dasha"),
                    selectedIndex = selectedSubTab,
                    onSelect = onSelectSubTab
                )

                when (selectedSubTab) {
                    0 -> VedicPlacementsSubSection(v, selectedVarga, activeVargaChart)
                    1 -> VedicPanchangaKarakasSubSection(v)
                    2 -> VedicPadasDrishtiSubSection(v)
                    3 -> VedicAshtakavargaSubSection(v)
                    4 -> VedicDashaSubSection(v, expandedDashaLord, onToggleDashaLord)
                }
            }
        }
    }
}

@Composable
private fun VedicPlacementsSubSection(
    v: VedicChartData,
    selectedVarga: String,
    activeVargaChart: VargaChartData?
) {
    Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
        // 1. D1 Rasi Full Sidereal Table
        val ayanamshaInfo = if (v.ayanamsha_deg > 0.0) {
            " · ${v.ayanamsha_name} ${String.format(Locale.US, "%.2f°", v.ayanamsha_deg)}"
        } else ""
        AstroPanelCard(
            title = "Sidereal Graha Placements (D1 Rasi)",
            subtitle = "Lagna: ${vedicRasiCode(v.lagna_sign)} ${formatDegMin(v.lagna_sidereal % 30.0)}$ayanamshaInfo",
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

        // 2. If a non-D1 Varga is selected, show its exact placements table (sign & bhava only, no artificial degree)
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
private fun VedicPanchangaKarakasSubSection(v: VedicChartData) {
    Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
        // 1. Panchanga
        val pan = v.panchanga
        if (pan.tithi_name.isNotEmpty()) {
            AstroPanelCard(
                title = "Natal Panchanga",
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

        // 2. Jaimini Chara Karakas (7 Karakas)
        if (v.chara_karakas.isNotEmpty()) {
            AstroPanelCard(
                title = "Jaimini Chara Karakas (7-Karaka Scheme)",
                contentPadding = PaddingValues(0.dp)
            ) {
                val cols = listOf(
                    TableColSpec("KARAKA", 1.8f, TextAlign.Start),
                    TableColSpec("GRAHA", 0.9f, TextAlign.Center, monospace = true),
                    TableColSpec("EFF. DEGREE", 1.1f, TextAlign.End, monospace = true)
                )
                AstroTableHeader(cols)
                v.chara_karakas.forEachIndexed { idx, ck ->
                    val isAtmakaraka = ck.karaka.startsWith("Atmakaraka", ignoreCase = true)
                    AstroTableRow(
                        columns = cols,
                        striped = idx % 2 == 1,
                        cells = listOf(
                            TableCellData(ck.karaka, if (isAtmakaraka) CelestialBlue else TextPrimary, FontWeight.Bold),
                            TableCellData(vedicGrahaCode(ck.planet), if (isAtmakaraka) CelestialBlue else TextPrimary, FontWeight.Bold),
                            TableCellData(formatDegMin(ck.effective_degree), TextSecondary)
                        )
                    )
                }
            }
        }
    }
}

@Composable
private fun VedicPadasDrishtiSubSection(v: VedicChartData) {
    Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
        // 1. Arudha Padas (12 Tiles in 4x3 Grid)
        if (v.arudha_padas.isNotEmpty()) {
            AstroPanelCard(title = "Arudha Padas (AL ~ UL)") {
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

        // 2. Graha Drishti (Full Aspects incl. Special Bhava Aspects)
        val notableDrishti = v.drishti.filter { it.aspected_planets.isNotEmpty() || it.aspect_offset != 7 }
        AstroPanelCard(
            title = "Graha Drishti (Parashari Full & Special Aspects)",
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
                    TableColSpec("TARGET BHAVA", 1.15f, TextAlign.Center, monospace = true),
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
        // 1. Sarvashtakavarga (SAV) 4x3 Grid
        AstroPanelCard(
            title = "Sarvashtakavarga (SAV)",
            headerAction = {
                AstroStatusBadge("TOTAL ${av.total_bindus}", CelestialBlue)
            }
        ) {
            val tiles = av.sarva.mapIndexed { sIdx, pts ->
                val bhava = ((sIdx - v.lagna_sign + 12) % 12) + 1
                val highlight = when {
                    pts >= 30 -> CelestialBlue
                    pts < 25 -> CrimsonAlert
                    else -> null
                }
                MetricTileItem(
                    tag = signCode(sIdx),
                    primary = "$pts PTS",
                    secondary = "H$bhava",
                    highlightColor = highlight
                )
            }
            CompactMetricGrid(items = tiles, columns = 4)
        }

        // 2. Bhinna Ashtakavarga (BAV) Totals & Sign Breakdown
        if (av.bhinna.isNotEmpty()) {
            AstroPanelCard(
                title = "Bhinna Ashtakavarga (BAV)",
                contentPadding = PaddingValues(0.dp)
            ) {
                val cols = buildList {
                    add(TableColSpec("GRAHA", 1.1f, TextAlign.Start, monospace = true))
                    for (i in 0 until 12) {
                        add(TableColSpec("${i + 1}", 0.55f, TextAlign.Center, monospace = true))
                    }
                    add(TableColSpec("TOT", 0.85f, TextAlign.End, monospace = true))
                }
                AstroTableHeader(cols)
                av.bhinna.forEachIndexed { idx, row ->
                    val cells = buildList {
                        add(TableCellData(vedicGrahaCode(row.planet), TextPrimary, FontWeight.Bold))
                        for (i in 0 until 12) {
                            val b = row.bindus.getOrElse(i) { 0 }
                            val col = when {
                                b >= 5 -> CelestialBlue
                                b <= 2 -> CrimsonAlert
                                else -> TextSecondary
                            }
                            add(TableCellData("$b", col, if (b >= 5) FontWeight.Bold else FontWeight.Normal))
                        }
                        add(TableCellData("${row.total}", TextPrimary, FontWeight.Bold))
                    }
                    AstroTableRow(
                        columns = cols,
                        striped = idx % 2 == 1,
                        cells = cells
                    )
                }
            }
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
                Column(
                    modifier = Modifier
                        .fillMaxWidth()
                        .background(SurfaceLight)
                        .padding(vertical = 4.dp)
                ) {
                    d.sub_periods.forEach { sub ->
                        Row(
                            modifier = Modifier
                                .fillMaxWidth()
                                .height(28.dp)
                                .padding(horizontal = 20.dp),
                            horizontalArrangement = Arrangement.SpaceBetween,
                            verticalAlignment = Alignment.CenterVertically
                        ) {
                            Text(
                                text = "${vedicGrahaCode(d.lord)}-${vedicGrahaCode(sub.lord)} (${sub.lord})",
                                fontSize = 11.sp,
                                fontWeight = FontWeight.SemiBold,
                                color = TextPrimary,
                                style = TabularMonoStyle
                            )
                            Text(
                                text = "${sub.startDateFormatted} ~ ${sub.endDateFormatted}",
                                fontSize = 11.sp,
                                color = TextSecondary,
                                style = TabularMonoStyle
                            )
                        }
                    }
                }
                HorizontalDivider(color = BorderColor, thickness = 0.5.dp)
            }
        }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
private fun VargaPickerModalSheet(
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
            modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 16.dp)
                .padding(bottom = 28.dp),
            verticalArrangement = Arrangement.spacedBy(10.dp)
        ) {
            Text(
                text = "SELECT DIVISIONAL CHART (19 VARGAS)",
                fontSize = 12.sp,
                fontWeight = FontWeight.Bold,
                letterSpacing = 0.6.sp,
                color = TextPrimary,
                style = TabularMonoStyle
            )

            val rows = VARGA_DEFINITIONS.chunked(3)
            rows.forEach { rowItems ->
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.spacedBy(8.dp)
                ) {
                    for (i in 0 until 3) {
                        val item = rowItems.getOrNull(i)
                        if (item != null) {
                            val (code, name) = item
                            val isSel = code == selectedVarga
                            Column(
                                modifier = Modifier
                                    .weight(1f)
                                    .height(48.dp)
                                    .clip(RoundedCornerShape(6.dp))
                                    .background(if (isSel) AccentPrimary else SurfaceLight)
                                    .border(1.dp, if (isSel) AccentPrimary else BorderColor, RoundedCornerShape(6.dp))
                                    .clickable { onSelect(code) }
                                    .padding(horizontal = 8.dp, vertical = 5.dp),
                                verticalArrangement = Arrangement.Center
                            ) {
                                Text(
                                    text = code,
                                    fontSize = 12.sp,
                                    fontWeight = FontWeight.Bold,
                                    color = if (isSel) BackgroundWhite else CelestialBlue,
                                    style = TabularMonoStyle,
                                    maxLines = 1
                                )
                                Text(
                                    text = name,
                                    fontSize = 10.sp,
                                    color = if (isSel) BackgroundWhite.copy(alpha = 0.85f) else TextSecondary,
                                    maxLines = 1,
                                    overflow = TextOverflow.Ellipsis
                                )
                            }
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
private fun LoadingStateBox() {
    Box(
        modifier = Modifier
            .fillMaxWidth()
            .height(240.dp),
        contentAlignment = Alignment.Center
    ) {
        CircularProgressIndicator(color = AccentPrimary, strokeWidth = 2.dp)
    }
}

@Composable
private fun DomainErrorCard(
    message: String,
    onRetry: () -> Unit
) {
    Surface(
        modifier = Modifier
            .fillMaxWidth()
            .padding(vertical = 12.dp),
        color = CrimsonAlert.copy(alpha = 0.06f),
        border = BorderStroke(1.dp, CrimsonAlert.copy(alpha = 0.35f)),
        shape = RoundedCornerShape(8.dp)
    ) {
        Column(
            modifier = Modifier
                .fillMaxWidth()
                .padding(16.dp),
            horizontalAlignment = Alignment.CenterHorizontally
        ) {
            Text(
                text = message,
                color = CrimsonAlert,
                fontSize = 13.sp,
                textAlign = TextAlign.Center
            )
            Spacer(modifier = Modifier.height(10.dp))
            Button(
                onClick = onRetry,
                colors = ButtonDefaults.buttonColors(containerColor = AccentPrimary)
            ) {
                Text("RETRY", color = BackgroundWhite, fontSize = 11.sp, fontWeight = FontWeight.Bold, style = TabularMonoStyle)
            }
        }
    }
}
