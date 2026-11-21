package com.fortunexample.note.ui.screens

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.filled.ContentCopy
import androidx.compose.material.icons.filled.Delete
import androidx.compose.material.icons.filled.Edit
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.fortunexample.note.data.*
import com.fortunexample.note.ui.components.*
import com.fortunexample.note.ui.theme.*
import kotlinx.coroutines.launch
import java.util.Calendar
import java.util.Locale

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ProfileDetailScreen(
    profile: ProfileDto,
    profileRepository: ProfileDataSource,
    onBack: () -> Unit,
    onEdit: () -> Unit,
    onDelete: () -> Unit
) {
    val scope = rememberCoroutineScope()
    var selectedDomainTab by rememberSaveable { mutableIntStateOf(0) }
    var selectedWesternSubTab by rememberSaveable { mutableIntStateOf(0) }
    var selectedVedicSubTab by rememberSaveable { mutableIntStateOf(0) }
    var selectedVarga by rememberSaveable { mutableStateOf("D1") }
    var selectedHouse by rememberSaveable { mutableStateOf(DEFAULT_HOUSE_SYSTEM) }
    var showVargaPickerSheet by remember { mutableStateOf(false) }
    var expandedDashaLord by remember { mutableStateOf<String?>(null) }
    var showSelectiveCopySheet by remember { mutableStateOf(false) }

    var sajuOutput by remember { mutableStateOf<SajuCalculationOutputDto?>(null) }
    var isSajuLoading by remember { mutableStateOf(false) }
    var sajuError by remember { mutableStateOf<String?>(null) }

    var westernOutput by remember { mutableStateOf<WesternCalculationOutputDto?>(null) }
    var isWesternLoading by remember { mutableStateOf(false) }
    var westernError by remember { mutableStateOf<String?>(null) }

    var vedicOutput by remember { mutableStateOf<VedicCalculationOutputDto?>(null) }
    var isVedicLoading by remember { mutableStateOf(false) }
    var vedicError by remember { mutableStateOf<String?>(null) }

    val todayCal = remember { Calendar.getInstance() }
    var predRequest by remember {
        mutableStateOf(
            PredictiveSuiteRequestDto(
                target_year = todayCal.get(Calendar.YEAR),
                target_month = todayCal.get(Calendar.MONTH) + 1,
                target_day = todayCal.get(Calendar.DAY_OF_MONTH),
                house_system = selectedHouse
            )
        )
    }
    var predictiveData by remember { mutableStateOf<PredictiveSuiteData?>(null) }
    var lastLoadedPredKey by remember { mutableStateOf<Pair<String, PredictiveSuiteRequestDto>?>(null) }
    var isPredictiveLoading by remember { mutableStateOf(false) }

    fun loadSaju() {
        scope.launch {
            isSajuLoading = true
            sajuError = null
            profileRepository.calculateSaju(profile.id)
                .onSuccess { sajuOutput = it; isSajuLoading = false }
                .onFailure { sajuError = it.message ?: "Failed to compute Four Pillars data."; isSajuLoading = false }
        }
    }

    fun loadWestern() {
        scope.launch {
            isWesternLoading = true
            westernError = null
            profileRepository.calculateWestern(profile.id, selectedHouse)
                .onSuccess { westernOutput = it; isWesternLoading = false }
                .onFailure { westernError = it.message ?: "Failed to compute Western chart."; isWesternLoading = false }
        }
    }

    fun loadVedic() {
        scope.launch {
            isVedicLoading = true
            vedicError = null
            profileRepository.calculateVedic(profile.id)
                .onSuccess { vedicOutput = it; isVedicLoading = false }
                .onFailure { vedicError = it.message ?: "Failed to compute Vedic chart."; isVedicLoading = false }
        }
    }

    LaunchedEffect(profile) {
        sajuOutput = null; westernOutput = null; vedicOutput = null
        predictiveData = null; lastLoadedPredKey = null
        loadSaju()
    }

    LaunchedEffect(profile, selectedDomainTab, selectedHouse) {
        if (selectedDomainTab == 1 && (westernOutput == null || westernOutput?.western?.house_system != selectedHouse)) {
            loadWestern()
        }
    }

    LaunchedEffect(profile, selectedDomainTab) {
        if (selectedDomainTab == 2 && vedicOutput == null) {
            loadVedic()
        }
    }

    val effectivePredReq = predRequest.copy(house_system = selectedHouse)
    val shouldLoadPredictive = selectedDomainTab == 1 && (selectedWesternSubTab == 3 || showSelectiveCopySheet)
    LaunchedEffect(profile.id, effectivePredReq, shouldLoadPredictive) {
        val cacheKey = profile.id to effectivePredReq
        if (shouldLoadPredictive && lastLoadedPredKey != cacheKey) {
            isPredictiveLoading = true
            profileRepository.calculatePredictiveSuite(profile.id, effectivePredReq)
                .onSuccess {
                    predictiveData = it
                    lastLoadedPredKey = cacheKey
                    isPredictiveLoading = false
                }
                .onFailure { isPredictiveLoading = false }
        }
    }

    val isCopyEnabled = when (selectedDomainTab) {
        0 -> sajuOutput != null
        1 -> westernOutput != null
        else -> vedicOutput != null
    }

    Scaffold(
        topBar = {
            ProfileDetailTopBar(
                profile = profile,
                isCopyEnabled = isCopyEnabled,
                onBack = onBack,
                onOpenCopy = { showSelectiveCopySheet = true },
                onEdit = onEdit,
                onDelete = onDelete
            )
        },
        containerColor = BackgroundWhite
    ) { padding ->
        Column(modifier = Modifier.fillMaxSize().padding(padding)) {
            ProfileDomainTabs(selectedDomainTab) { selectedDomainTab = it }
            Column(
                modifier = Modifier
                    .fillMaxSize()
                    .verticalScroll(rememberScrollState())
                    .padding(14.dp)
            ) {
                when (selectedDomainTab) {
                    0 -> SajuDomainBody(sajuOutput, isSajuLoading, sajuError, ::loadSaju)
                    1 -> WesternDomainContent(
                        profile = profile,
                        state = WesternDomainUiState(
                            westernOutput = westernOutput,
                            isLoading = isWesternLoading,
                            error = westernError,
                            selectedHouse = selectedHouse,
                            selectedSubTab = selectedWesternSubTab,
                            predictiveData = predictiveData,
                            predictiveControl = PredictiveControlState(
                                targetYear = effectivePredReq.target_year,
                                targetMonth = effectivePredReq.target_month,
                                targetDay = effectivePredReq.target_day,
                                isLoading = isPredictiveLoading,
                                onYearChange = { yr -> predRequest = predRequest.copy(target_year = yr) },
                                onDateTimeChange = { y, m, d, hr, min ->
                                    predRequest = predRequest.copy(
                                        target_year = y, target_month = m, target_day = d,
                                        target_hour = hr, target_minute = min
                                    )
                                }
                            )
                        ),
                        onSelectHouse = { selectedHouse = it },
                        onSelectSubTab = { selectedWesternSubTab = it },
                        onRetry = ::loadWestern
                    )
                    2 -> VedicDomainContent(
                        state = VedicDomainUiState(
                            vedicOutput = vedicOutput,
                            isLoading = isVedicLoading,
                            error = vedicError,
                            selectedVarga = selectedVarga,
                            selectedSubTab = selectedVedicSubTab,
                            expandedDashaLord = expandedDashaLord
                        ),
                        onOpenVargaPicker = { showVargaPickerSheet = true },
                        onSelectSubTab = { selectedVedicSubTab = it },
                        onToggleDashaLord = { lord ->
                            expandedDashaLord = if (expandedDashaLord == lord) null else lord
                        },
                        onRetry = ::loadVedic
                    )
                }
            }
        }

        if (showVargaPickerSheet) {
            VargaPickerModalSheet(
                selectedVarga = selectedVarga,
                onSelect = { selectedVarga = it; showVargaPickerSheet = false },
                onDismiss = { showVargaPickerSheet = false }
            )
        }

        SelectiveCopyBottomSheet(
            isOpen = showSelectiveCopySheet,
            onDismiss = { showSelectiveCopySheet = false },
            payload = SelectiveCopyPayload(
                profile = profile,
                saju = sajuOutput?.saju,
                western = westernOutput?.western,
                predictive = predictiveData,
                vedic = vedicOutput?.vedic,
                timezoneId = sajuOutput?.timezone_id ?: westernOutput?.timezone_id ?: vedicOutput?.timezone_id ?: profile.timezone_id,
                utcOffsetHours = sajuOutput?.utc_offset_hours ?: westernOutput?.utc_offset_hours ?: vedicOutput?.utc_offset_hours
            ),
            selectedDomainTab = selectedDomainTab
        )
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
private fun ProfileDetailTopBar(
    profile: ProfileDto,
    isCopyEnabled: Boolean,
    onBack: () -> Unit,
    onOpenCopy: () -> Unit,
    onEdit: () -> Unit,
    onDelete: () -> Unit
) {
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
                        profile.birth_year, profile.birth_month, profile.birth_day,
                        profile.birth_hour, profile.birth_minute, profile.birth_second.toInt(),
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
            IconButton(onClick = onOpenCopy, enabled = isCopyEnabled) {
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
}

@Composable
private fun ProfileDomainTabs(selectedTab: Int, onSelect: (Int) -> Unit) {
    val labels = listOf("FOUR PILLARS", "WESTERN", "VEDIC")
    TabRow(selectedTabIndex = selectedTab, containerColor = SurfaceLight, contentColor = AccentPrimary) {
        labels.forEachIndexed { idx, label ->
            Tab(
                selected = selectedTab == idx,
                onClick = { onSelect(idx) },
                text = { Text(label, fontSize = 12.sp, fontWeight = FontWeight.Bold, letterSpacing = 0.5.sp) }
            )
        }
    }
}

@Composable
private fun SajuDomainBody(
    sajuOutput: SajuCalculationOutputDto?,
    isLoading: Boolean,
    error: String?,
    onRetry: () -> Unit
) {
    when {
        isLoading -> LoadingStateBox()
        error != null -> DomainErrorCard(message = error, onRetry = onRetry)
        sajuOutput != null -> SajuChartTable(chart = sajuOutput.saju, modifier = Modifier.padding(0.dp))
    }
}
