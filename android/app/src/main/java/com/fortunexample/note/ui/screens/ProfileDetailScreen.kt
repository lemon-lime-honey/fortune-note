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
import kotlinx.coroutines.currentCoroutineContext
import kotlinx.coroutines.ensureActive
import java.util.Calendar
import java.util.Locale

private data class ProfileChartQuery(
    val domainTab: Int,
    val houseSystem: String,
    val loadPredictive: Boolean,
    val predRequest: PredictiveSuiteRequestDto
)

private class ProfileChartHolder {
    var sajuOutput by mutableStateOf<SajuCalculationOutputDto?>(null)
    var isSajuLoading by mutableStateOf(false)
    var sajuError by mutableStateOf<String?>(null)
    var sajuRetryKey by mutableIntStateOf(0)

    var westernOutput by mutableStateOf<WesternCalculationOutputDto?>(null)
    var isWesternLoading by mutableStateOf(false)
    var westernError by mutableStateOf<String?>(null)
    var westernRetryKey by mutableIntStateOf(0)
    var lastWesternKey by mutableStateOf<Pair<String, Int>?>(null)

    var vedicOutput by mutableStateOf<VedicCalculationOutputDto?>(null)
    var isVedicLoading by mutableStateOf(false)
    var vedicError by mutableStateOf<String?>(null)
    var vedicRetryKey by mutableIntStateOf(0)
    var lastVedicKey by mutableStateOf<Int?>(null)

    var predictiveData by mutableStateOf<PredictiveSuiteData?>(null)
    var isPredictiveLoading by mutableStateOf(false)
    var lastPredKey by mutableStateOf<Pair<String, PredictiveSuiteRequestDto>?>(null)
}

private suspend fun <T> executeChartFetch(
    setLoading: (Boolean) -> Unit,
    setError: (String?) -> Unit,
    fallbackError: String,
    fetch: suspend () -> Result<T>,
    onLoaded: (T) -> Unit
) {
    setLoading(true)
    setError(null)
    val result = fetch()
    currentCoroutineContext().ensureActive()
    result.onSuccess(onLoaded).onFailure { setError(it.message ?: fallbackError) }
    setLoading(false)
}

@Composable
private fun rememberProfileChartHolder(
    profile: ProfileDto,
    profileRepository: ProfileDataSource,
    query: ProfileChartQuery
): ProfileChartHolder {
    val holder = remember(profile) { ProfileChartHolder() }

    LaunchedEffect(profile, holder.sajuRetryKey) {
        executeChartFetch(
            setLoading = { holder.isSajuLoading = it },
            setError = { holder.sajuError = it },
            fallbackError = "Failed to compute Four Pillars data.",
            fetch = { profileRepository.calculateSaju(profile.id) },
            onLoaded = { holder.sajuOutput = it }
        )
    }

    LaunchedEffect(profile, query.domainTab, query.houseSystem, holder.westernRetryKey) {
        val targetKey = query.houseSystem to holder.westernRetryKey
        if (query.domainTab != 1) {
            holder.isWesternLoading = false
            return@LaunchedEffect
        }
        if (holder.lastWesternKey == targetKey) return@LaunchedEffect
        executeChartFetch(
            setLoading = { holder.isWesternLoading = it },
            setError = { holder.westernError = it },
            fallbackError = "Failed to compute Western chart.",
            fetch = { profileRepository.calculateWestern(profile.id, query.houseSystem) },
            onLoaded = { holder.westernOutput = it; holder.lastWesternKey = targetKey }
        )
    }

    LaunchedEffect(profile, query.domainTab, holder.vedicRetryKey) {
        if (query.domainTab != 2) {
            holder.isVedicLoading = false
            return@LaunchedEffect
        }
        if (holder.lastVedicKey == holder.vedicRetryKey) return@LaunchedEffect
        executeChartFetch(
            setLoading = { holder.isVedicLoading = it },
            setError = { holder.vedicError = it },
            fallbackError = "Failed to compute Vedic chart.",
            fetch = { profileRepository.calculateVedic(profile.id) },
            onLoaded = { holder.vedicOutput = it; holder.lastVedicKey = holder.vedicRetryKey }
        )
    }

    LaunchedEffect(profile, query.predRequest, query.loadPredictive) {
        val cacheKey = profile.id to query.predRequest
        if (!query.loadPredictive) {
            holder.isPredictiveLoading = false
            return@LaunchedEffect
        }
        if (holder.lastPredKey == cacheKey) return@LaunchedEffect
        holder.isPredictiveLoading = true
        val result = profileRepository.calculatePredictiveSuite(profile.id, query.predRequest)
        currentCoroutineContext().ensureActive()
        result.onSuccess { holder.predictiveData = it; holder.lastPredKey = cacheKey }
        holder.isPredictiveLoading = false
    }

    return holder
}

@Composable
fun ProfileDetailScreen(
    profile: ProfileDto,
    profileRepository: ProfileDataSource,
    onBack: () -> Unit,
    onEdit: () -> Unit,
    onDelete: () -> Unit
) {
    var selectedDomainTab by rememberSaveable { mutableIntStateOf(0) }
    var selectedWesternSubTab by rememberSaveable { mutableIntStateOf(0) }
    var selectedVedicSubTab by rememberSaveable { mutableIntStateOf(0) }
    var selectedVarga by rememberSaveable { mutableStateOf("D1") }
    var selectedHouse by rememberSaveable { mutableStateOf(DEFAULT_HOUSE_SYSTEM) }
    var showVargaPickerSheet by remember { mutableStateOf(false) }
    var expandedDashaLord by remember { mutableStateOf<String?>(null) }
    var showSelectiveCopySheet by remember { mutableStateOf(false) }

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

    val effectivePredReq = predRequest.copy(house_system = selectedHouse)
    val shouldLoadPredictive = selectedDomainTab == 1 && (selectedWesternSubTab == 3 || showSelectiveCopySheet)
    val charts = rememberProfileChartHolder(
        profile = profile,
        profileRepository = profileRepository,
        query = ProfileChartQuery(selectedDomainTab, selectedHouse, shouldLoadPredictive, effectivePredReq)
    )

    val isCopyEnabled = when (selectedDomainTab) {
        0 -> charts.sajuOutput != null
        1 -> charts.westernOutput != null
        else -> charts.vedicOutput != null
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
                    0 -> SajuDomainBody(charts.sajuOutput, charts.isSajuLoading, charts.sajuError) { charts.sajuRetryKey++ }
                    1 -> WesternDomainContent(
                        profile = profile,
                        state = WesternDomainUiState(
                            westernOutput = charts.westernOutput,
                            isLoading = charts.isWesternLoading,
                            error = charts.westernError,
                            selectedHouse = selectedHouse,
                            selectedSubTab = selectedWesternSubTab,
                            predictiveData = charts.predictiveData,
                            predictiveControl = PredictiveControlState(
                                targetYear = effectivePredReq.target_year,
                                targetMonth = effectivePredReq.target_month,
                                targetDay = effectivePredReq.target_day,
                                isLoading = charts.isPredictiveLoading,
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
                        onRetry = { charts.westernRetryKey++ }
                    )
                    2 -> VedicDomainContent(
                        state = VedicDomainUiState(
                            vedicOutput = charts.vedicOutput,
                            isLoading = charts.isVedicLoading,
                            error = charts.vedicError,
                            selectedVarga = selectedVarga,
                            selectedSubTab = selectedVedicSubTab,
                            expandedDashaLord = expandedDashaLord
                        ),
                        onOpenVargaPicker = { showVargaPickerSheet = true },
                        onSelectSubTab = { selectedVedicSubTab = it },
                        onToggleDashaLord = { lord ->
                            expandedDashaLord = if (expandedDashaLord == lord) null else lord
                        },
                        onRetry = { charts.vedicRetryKey++ }
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
                saju = charts.sajuOutput?.saju,
                western = charts.westernOutput?.western,
                predictive = charts.predictiveData,
                vedic = charts.vedicOutput?.vedic,
                timezoneId = charts.sajuOutput?.timezone_id
                    ?: charts.westernOutput?.timezone_id
                    ?: charts.vedicOutput?.timezone_id
                    ?: profile.timezone_id,
                utcOffsetHours = charts.sajuOutput?.utc_offset_hours
                    ?: charts.westernOutput?.utc_offset_hours
                    ?: charts.vedicOutput?.utc_offset_hours
            ),
            selectedDomainTab = selectedDomainTab,
            isPredictiveLoading = charts.isPredictiveLoading
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
