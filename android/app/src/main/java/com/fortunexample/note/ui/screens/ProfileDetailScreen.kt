package com.fortunexample.note.ui.screens

import android.content.ClipData
import android.content.ClipboardManager
import android.content.Context
import android.widget.Toast
import androidx.compose.animation.AnimatedVisibility
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.background
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
import androidx.compose.material.icons.filled.KeyboardArrowDown
import androidx.compose.material.icons.filled.KeyboardArrowUp
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.fortunexample.note.FortuneApp
import com.fortunexample.note.data.PredictiveSuiteData
import com.fortunexample.note.data.ProfileDto
import com.fortunexample.note.data.SajuCalculationOutputDto
import com.fortunexample.note.data.VedicCalculationOutputDto
import com.fortunexample.note.data.WesternCalculationOutputDto
import com.fortunexample.note.ui.components.*
import com.fortunexample.note.ui.theme.*
import kotlinx.coroutines.launch

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ProfileDetailScreen(
    profile: ProfileDto,
    onBack: () -> Unit,
    onEdit: () -> Unit,
    onDelete: () -> Unit
) {
    val context = LocalContext.current
    val scope = rememberCoroutineScope()
    var selectedDomainTab by remember { mutableStateOf(0) }
    var selectedVarga by remember { mutableStateOf("D1") }
    var expandedDashaLord by remember { mutableStateOf<String?>(null) }

    var showSelectiveCopySheet by remember { mutableStateOf(false) }

    // Decoupled domain states
    var sajuOutput by remember { mutableStateOf<SajuCalculationOutputDto?>(null) }
    var isSajuLoading by remember { mutableStateOf(false) }
    var sajuError by remember { mutableStateOf<String?>(null) }

    var westernOutput by remember { mutableStateOf<WesternCalculationOutputDto?>(null) }
    var isWesternLoading by remember { mutableStateOf(false) }
    var westernError by remember { mutableStateOf<String?>(null) }
    var selectedHouse by remember { mutableStateOf("Placidus") }

    var vedicOutput by remember { mutableStateOf<VedicCalculationOutputDto?>(null) }
    var isVedicLoading by remember { mutableStateOf(false) }
    var vedicError by remember { mutableStateOf<String?>(null) }

    var predictiveTargetYear by remember { mutableIntStateOf(2026) }
    var predictiveData by remember { mutableStateOf<PredictiveSuiteData?>(null) }
    var isPredictiveLoading by remember { mutableStateOf(false) }

    // 1. Saju immediate load on screen enter
    LaunchedEffect(profile.id) {
        if (sajuOutput == null) {
            isSajuLoading = true
            sajuError = null
            FortuneApp.repository.calculateProfileSaju(profile.id)
                .onSuccess {
                    sajuOutput = it
                    isSajuLoading = false
                }
                .onFailure {
                    sajuError = it.message ?: "사주명리 데이터 계산에 실패했습니다."
                    isSajuLoading = false
                }
        }
    }

    // 2. Western lazy load on Western tab (tab index 1)
    LaunchedEffect(profile.id, selectedDomainTab, selectedHouse) {
        if (selectedDomainTab == 1 && (westernOutput == null || westernOutput?.western?.house_system != selectedHouse)) {
            isWesternLoading = true
            westernError = null
            FortuneApp.repository.calculateProfileWestern(profile.id, selectedHouse)
                .onSuccess {
                    westernOutput = it
                    isWesternLoading = false
                }
                .onFailure {
                    westernError = it.message ?: "서양 점성술 데이터 계산에 실패했습니다."
                    isWesternLoading = false
                }
        }
    }

    // 3. Vedic lazy load on Vedic tab (tab index 2)
    LaunchedEffect(profile.id, selectedDomainTab) {
        if (selectedDomainTab == 2 && vedicOutput == null) {
            isVedicLoading = true
            vedicError = null
            FortuneApp.repository.calculateProfileVedic(profile.id)
                .onSuccess {
                    vedicOutput = it
                    isVedicLoading = false
                }
                .onFailure {
                    vedicError = it.message ?: "베딕 점성술 데이터 계산에 실패했습니다."
                    isVedicLoading = false
                }
        }
    }

    // 4. Lazy-load predictive suite only when user enters Western tab (tab index 1)
    LaunchedEffect(profile.id, predictiveTargetYear, selectedDomainTab) {
        if (selectedDomainTab == 1 && predictiveData == null) {
            isPredictiveLoading = true
            FortuneApp.repository.calculateProfilePredictiveSuite(
                profile.id,
                predictiveTargetYear,
                profile.birth_month,
                profile.birth_day
            ).onSuccess { predictiveData = it }
                .onFailure { predictiveData = null }
            isPredictiveLoading = false
        }
    }

    val vargaList = listOf(
        "D1" to "Rasi (D1)",
        "D2" to "Hora (D2)",
        "D3" to "Drekkana (D3)",
        "D4" to "Chaturthamsha (D4)",
        "D6" to "Shashthamsha (D6)",
        "D7" to "Saptamsha (D7)",
        "D8" to "Ashtamamsha (D8)",
        "D9" to "Navamsha (D9)",
        "D10" to "Dashamsha (D10)",
        "D11" to "Rudramsha (D11)",
        "D12" to "Dwadashamsha (D12)"
    )

    Scaffold(
        topBar = {
            TopAppBar(
                title = {
                    Column {
                        Text(profile.name, fontWeight = FontWeight.Bold, fontSize = 18.sp, color = TextPrimary)
                        Text(
                            text = "${profile.birth_year}-${String.format("%02d", profile.birth_month)}-${String.format("%02d", profile.birth_day)} ${String.format("%02d:%02d", profile.birth_hour, profile.birth_minute)} · ${if (profile.gender == "male") "남성" else "여성"}",
                            fontSize = 12.sp,
                            color = TextSecondary
                        )
                    }
                },
                navigationIcon = {
                    IconButton(onClick = onBack) {
                        Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = "뒤로", tint = TextPrimary)
                    }
                },
                actions = {
                    IconButton(onClick = onEdit) {
                        Icon(Icons.Default.Edit, contentDescription = "수정", tint = TextPrimary)
                    }
                    IconButton(onClick = onDelete) {
                        Icon(Icons.Default.Delete, contentDescription = "삭제", tint = AstrologicalRed)
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
                Tab(selected = selectedDomainTab == 0, onClick = { selectedDomainTab = 0 }, text = { Text("사주명리", fontWeight = FontWeight.SemiBold) })
                Tab(selected = selectedDomainTab == 1, onClick = { selectedDomainTab = 1 }, text = { Text("Western", fontWeight = FontWeight.SemiBold) })
                Tab(selected = selectedDomainTab == 2, onClick = { selectedDomainTab = 2 }, text = { Text("Vedic", fontWeight = FontWeight.SemiBold) })
            }

            Column(
                modifier = Modifier
                    .fillMaxSize()
                    .verticalScroll(rememberScrollState())
                    .padding(16.dp)
            ) {
                    // 차트 상단 헤더 및 데이터 선택 복사 버튼
                    Row(
                        modifier = Modifier
                            .fillMaxWidth()
                            .padding(bottom = 12.dp),
                        horizontalArrangement = Arrangement.SpaceBetween,
                        verticalAlignment = Alignment.CenterVertically
                    ) {
                        Text(
                            text = when (selectedDomainTab) {
                                0 -> "사주명리 분석 차트"
                                1 -> "Western Tropical Chart"
                                else -> "Vedic Sidereal Chart"
                            },
                            fontWeight = FontWeight.Bold,
                            fontSize = 15.sp,
                            color = TextPrimary
                        )
                        OutlinedButton(
                            onClick = { showSelectiveCopySheet = true },
                            enabled = when (selectedDomainTab) {
                                0 -> sajuOutput != null
                                1 -> westernOutput != null
                                else -> vedicOutput != null
                            },
                            contentPadding = PaddingValues(horizontal = 10.dp, vertical = 4.dp),
                            shape = RoundedCornerShape(8.dp),
                            border = BorderStroke(1.dp, BorderColor)
                        ) {
                            Icon(
                                Icons.Default.ContentCopy,
                                contentDescription = "데이터 선택 복사",
                                modifier = Modifier.size(15.dp),
                                tint = TextPrimary
                            )
                            Spacer(modifier = Modifier.width(4.dp))
                            Text("데이터 복사", fontSize = 12.sp, color = TextPrimary, fontWeight = FontWeight.Medium)
                        }
                    }

                    when (selectedDomainTab) {
                        0 -> {
                            if (isSajuLoading) {
                                Box(modifier = Modifier.fillMaxWidth().height(260.dp), contentAlignment = Alignment.Center) {
                                    CircularProgressIndicator(color = AccentPrimary)
                                }
                            } else if (sajuError != null) {
                                DomainErrorCard(message = sajuError!!, onRetry = {
                                    scope.launch {
                                        isSajuLoading = true
                                        sajuError = null
                                        FortuneApp.repository.calculateProfileSaju(profile.id)
                                            .onSuccess { sajuOutput = it; isSajuLoading = false }
                                            .onFailure { sajuError = it.message ?: "계산 실패"; isSajuLoading = false }
                                    }
                                })
                            } else if (sajuOutput != null) {
                                SajuChartTable(chart = sajuOutput!!.saju)
                            }
                        }
                        1 -> {
                            val houseSystems = listOf("Placidus", "Regiomontanus", "WholeSign", "Equal", "Koch", "Campanus")

                            Text(
                                text = "하우스 시스템 (House System)",
                                color = TextPrimary,
                                fontWeight = FontWeight.Bold,
                                fontSize = 14.sp
                            )
                            Spacer(modifier = Modifier.height(8.dp))
                            Row(
                                modifier = Modifier
                                    .fillMaxWidth()
                                    .horizontalScroll(rememberScrollState()),
                                horizontalArrangement = Arrangement.spacedBy(6.dp)
                            ) {
                                houseSystems.forEach { hSys ->
                                    val isSelected = selectedHouse.equals(hSys, ignoreCase = true)
                                    FilterChip(
                                        selected = isSelected,
                                        onClick = {
                                            selectedHouse = hSys
                                        },
                                        label = { Text(hSys, fontSize = 12.sp, fontWeight = if (isSelected) FontWeight.Bold else FontWeight.Normal) },
                                        colors = FilterChipDefaults.filterChipColors(
                                            selectedContainerColor = AccentPrimary,
                                            selectedLabelColor = BackgroundWhite,
                                            containerColor = SurfaceLight,
                                            labelColor = TextSecondary
                                        ),
                                        border = FilterChipDefaults.filterChipBorder(
                                            enabled = true,
                                            selected = isSelected,
                                            borderColor = BorderColor,
                                            selectedBorderColor = AccentPrimary
                                        )
                                    )
                                }
                            }

                            Spacer(modifier = Modifier.height(14.dp))

                            if (isWesternLoading) {
                                Box(modifier = Modifier.fillMaxWidth().height(260.dp), contentAlignment = Alignment.Center) {
                                    CircularProgressIndicator(color = AccentPrimary)
                                }
                            } else if (westernError != null) {
                                DomainErrorCard(message = westernError!!, onRetry = {
                                    scope.launch {
                                        isWesternLoading = true
                                        westernError = null
                                        FortuneApp.repository.calculateProfileWestern(profile.id, selectedHouse)
                                            .onSuccess { westernOutput = it; isWesternLoading = false }
                                            .onFailure { westernError = it.message ?: "계산 실패"; isWesternLoading = false }
                                    }
                                })
                            } else if (westernOutput != null) {
                                val westernChart = westernOutput!!.western
                                WesternChartWheel(chart = westernChart)
                                Spacer(modifier = Modifier.height(20.dp))

                            // Western Planetary Placements 카드
                            Card(
                                shape = RoundedCornerShape(12.dp),
                                colors = CardDefaults.cardColors(containerColor = BackgroundWhite),
                                border = BorderStroke(1.dp, BorderColor),
                                modifier = Modifier.fillMaxWidth()
                            ) {
                                Column(modifier = Modifier.padding(16.dp)) {
                                    Text("Planets & Points", color = TextPrimary, fontWeight = FontWeight.Bold, fontSize = 15.sp)
                                    Spacer(modifier = Modifier.height(10.dp))
                                    westernChart.planets.forEach { p ->
                                        val rx = if (p.is_retrograde) " [Rx]" else ""
                                        Row(
                                            modifier = Modifier.fillMaxWidth().padding(vertical = 5.dp),
                                            horizontalArrangement = Arrangement.SpaceBetween,
                                            verticalAlignment = Alignment.CenterVertically
                                        ) {
                                            Text(text = "${p.name}$rx", color = TextPrimary, fontWeight = FontWeight.Medium, fontSize = 14.sp)
                                            Text(
                                                text = "${String.format("%.2f°", p.sign_degree)} ${getSignName(p.sign_index)} · House ${p.house_number}",
                                                color = TextSecondary,
                                                fontSize = 13.sp
                                            )
                                        }
                                        HorizontalDivider(color = BorderColor.copy(alpha = 0.5f))
                                    }
                                    westernChart.nodes.forEach { n ->
                                        Row(
                                            modifier = Modifier.fillMaxWidth().padding(vertical = 5.dp),
                                            horizontalArrangement = Arrangement.SpaceBetween,
                                            verticalAlignment = Alignment.CenterVertically
                                        ) {
                                            Text(text = n.name, color = TextPrimary, fontWeight = FontWeight.Medium, fontSize = 14.sp)
                                            Text(
                                                text = "${String.format("%.2f°", n.sign_degree)} ${getSignName(n.sign_index)} · House ${n.house_number}",
                                                color = TextSecondary,
                                                fontSize = 13.sp
                                            )
                                        }
                                        HorizontalDivider(color = BorderColor.copy(alpha = 0.5f))
                                    }
                                }
                            }

                            Spacer(modifier = Modifier.height(14.dp))

                            if (westernChart.aspects.isNotEmpty()) {
                                Card(
                                    shape = RoundedCornerShape(12.dp),
                                    colors = CardDefaults.cardColors(containerColor = BackgroundWhite),
                                    border = BorderStroke(1.dp, BorderColor),
                                    modifier = Modifier.fillMaxWidth()
                                ) {
                                    Column(modifier = Modifier.padding(16.dp)) {
                                        Text("Major Aspects", color = TextPrimary, fontWeight = FontWeight.Bold, fontSize = 15.sp)
                                        Spacer(modifier = Modifier.height(10.dp))
                                        westernChart.aspects.forEach { a ->
                                            Row(
                                                modifier = Modifier.fillMaxWidth().padding(vertical = 5.dp),
                                                horizontalArrangement = Arrangement.SpaceBetween,
                                                verticalAlignment = Alignment.CenterVertically
                                            ) {
                                                Text(text = "${a.planet1} · ${a.planet2}", color = TextPrimary, fontSize = 13.sp)
                                                Text(
                                                    text = "${a.aspect_type} (Orb ${String.format("%.2f°", a.orb)})",
                                                    color = TextSecondary,
                                                    fontSize = 13.sp
                                                )
                                            }
                                            HorizontalDivider(color = BorderColor.copy(alpha = 0.5f))
                                        }
                                    }
                                }
                                Spacer(modifier = Modifier.height(14.dp))
                            }

                            // 윌리엄 릴리 전통 점성술 분석 섹션
                            WesternTraditionalSection(traditional = westernChart.traditional)

                            Spacer(modifier = Modifier.height(14.dp))

                            // 6대 서양 예측 기법 보조 메뉴 섹션
                            WesternPredictiveSection(
                                profile = profile,
                                predictiveData = predictiveData,
                                targetYear = predictiveTargetYear,
                                isLoading = isPredictiveLoading,
                                onYearChange = {
                                    predictiveTargetYear = it
                                    predictiveData = null
                                }
                            )
                            }
                        }
                        2 -> {
                            // 11종 바르가 분점도 선택 칩 바
                            Text(
                                text = "Divisional Charts (Vargas)",
                                color = TextPrimary,
                                fontWeight = FontWeight.Bold,
                                fontSize = 14.sp
                            )
                            Spacer(modifier = Modifier.height(8.dp))
                            Row(
                                modifier = Modifier
                                    .fillMaxWidth()
                                    .horizontalScroll(rememberScrollState()),
                                horizontalArrangement = Arrangement.spacedBy(6.dp)
                            ) {
                                vargaList.forEach { (key, title) ->
                                    val isSelected = selectedVarga == key
                                    FilterChip(
                                        selected = isSelected,
                                        onClick = { selectedVarga = key },
                                        label = { Text(title, fontSize = 12.sp) },
                                        colors = FilterChipDefaults.filterChipColors(
                                            selectedContainerColor = AccentPrimary,
                                            selectedLabelColor = BackgroundWhite,
                                            containerColor = SurfaceLight,
                                            labelColor = TextPrimary
                                        ),
                                        border = BorderStroke(1.dp, if (isSelected) AccentPrimary else BorderColor)
                                    )
                                }
                            }

                            Spacer(modifier = Modifier.height(12.dp))

                            if (isVedicLoading) {
                                Box(modifier = Modifier.fillMaxWidth().height(260.dp), contentAlignment = Alignment.Center) {
                                    CircularProgressIndicator(color = AccentPrimary)
                                }
                            } else if (vedicError != null) {
                                DomainErrorCard(message = vedicError!!, onRetry = {
                                    scope.launch {
                                        isVedicLoading = true
                                        vedicError = null
                                        FortuneApp.repository.calculateProfileVedic(profile.id)
                                            .onSuccess { vedicOutput = it; isVedicLoading = false }
                                            .onFailure { vedicError = it.message ?: "계산 실패"; isVedicLoading = false }
                                    }
                                })
                            } else if (vedicOutput != null) {
                                val vedicChart = vedicOutput!!.vedic

                                // 선택된 바르가 다이아몬드 차트
                                VedicChartDiamond(
                                    chart = vedicChart,
                                    selectedVargaKey = selectedVarga
                                )

                                Spacer(modifier = Modifier.height(20.dp))

                            // Sidereal Planetary Placements (낙샤트라 줄바꿈 깨짐 완전 해결)
                            Card(
                                shape = RoundedCornerShape(12.dp),
                                colors = CardDefaults.cardColors(containerColor = BackgroundWhite),
                                border = BorderStroke(1.dp, BorderColor),
                                modifier = Modifier.fillMaxWidth()
                            ) {
                                Column(modifier = Modifier.padding(16.dp)) {
                                    val activeVarga = vedicChart.vargas[selectedVarga]
                                    val vargaLagnaSign = activeVarga?.lagna_sign ?: vedicChart.lagna_sign
                                    val vargaTitle = vargaList.find { it.first == selectedVarga }?.second ?: selectedVarga
                                    Text(
                                        text = "$vargaTitle Placements",
                                        color = TextPrimary,
                                        fontWeight = FontWeight.Bold,
                                        fontSize = 14.sp
                                    )
                                    Spacer(modifier = Modifier.height(2.dp))
                                    Text(
                                        text = "Lagna: ${getSignName(vargaLagnaSign)}",
                                        color = TextSecondary,
                                        fontWeight = FontWeight.Medium,
                                        fontSize = 12.sp,
                                        textAlign = TextAlign.End,
                                        modifier = Modifier.fillMaxWidth()
                                    )
                                    Spacer(modifier = Modifier.height(10.dp))

                                    vedicChart.planets.forEach { p ->
                                        val rx = if (p.is_retrograde) " [Rx]" else ""
                                        val pPlacement = activeVarga?.placements?.find { it.planet_name == p.name }
                                        val houseNum = pPlacement?.house ?: (((p.sign_index - vargaLagnaSign + 12) % 12) + 1)
                                        val signIdx = pPlacement?.sign_index ?: p.sign_index

                                        Column(
                                            modifier = Modifier
                                                .fillMaxWidth()
                                                .padding(vertical = 6.dp)
                                        ) {
                                            Row(
                                                modifier = Modifier.fillMaxWidth(),
                                                horizontalArrangement = Arrangement.SpaceBetween,
                                                verticalAlignment = Alignment.CenterVertically
                                            ) {
                                                Text(
                                                    text = "${p.name}$rx",
                                                    color = TextPrimary,
                                                    fontWeight = FontWeight.SemiBold,
                                                    fontSize = 14.sp
                                                )
                                                Text(
                                                    text = "House $houseNum · ${getSignName(signIdx)}",
                                                    color = TextSecondary,
                                                    fontSize = 13.sp,
                                                    fontWeight = FontWeight.Medium
                                                )
                                            }
                                            Spacer(modifier = Modifier.height(2.dp))
                                            val displayDegree = if (selectedVarga == "D1") {
                                                p.sign_degree
                                            } else {
                                                pPlacement?.sign_degree ?: p.sign_degree
                                            }
                                            val degreeLabel = if (selectedVarga == "D1") "in sign" else "in varga sign"
                                            val nakshatraLabel = if (selectedVarga == "D1") {
                                                "${p.nakshatra_name} (Pada ${p.nakshatra_pada})"
                                            } else {
                                                "D1: ${p.nakshatra_name} (Pada ${p.nakshatra_pada})"
                                            }
                                            Row(
                                                modifier = Modifier.fillMaxWidth(),
                                                horizontalArrangement = Arrangement.SpaceBetween,
                                                verticalAlignment = Alignment.CenterVertically
                                            ) {
                                                Text(
                                                    text = "${String.format("%.2f°", displayDegree)} $degreeLabel",
                                                    color = TextSecondary,
                                                    fontSize = 12.sp
                                                )
                                                Text(
                                                    text = nakshatraLabel,
                                                    color = CelestialBlue,
                                                    fontSize = 12.sp,
                                                    fontWeight = FontWeight.Medium
                                                )
                                            }
                                        }
                                        HorizontalDivider(color = BorderColor.copy(alpha = 0.5f))
                                    }
                                }
                            }

                            Spacer(modifier = Modifier.height(16.dp))

                            // 빔쇼타리 다샤 아코디언 UI (안타르다샤 확장, Mahadasha 접두어 제거, YYYY.MM.DD 환산)
                            Card(
                                shape = RoundedCornerShape(12.dp),
                                colors = CardDefaults.cardColors(containerColor = BackgroundWhite),
                                border = BorderStroke(1.dp, BorderColor),
                                modifier = Modifier.fillMaxWidth()
                            ) {
                                Column(modifier = Modifier.padding(16.dp)) {
                                    Text(
                                        text = "Vimshottari Dasha Sequence",
                                        color = TextPrimary,
                                        fontWeight = FontWeight.Bold,
                                        fontSize = 15.sp
                                    )
                                    Spacer(modifier = Modifier.height(10.dp))

                                    vedicChart.vimshottari_dashas.forEach { d ->
                                        val isExpanded = expandedDashaLord == d.lord
                                        Column(modifier = Modifier.fillMaxWidth()) {
                                            Row(
                                                modifier = Modifier
                                                    .fillMaxWidth()
                                                    .clickable {
                                                        expandedDashaLord = if (isExpanded) null else d.lord
                                                    }
                                                    .padding(vertical = 10.dp),
                                                horizontalArrangement = Arrangement.SpaceBetween,
                                                verticalAlignment = Alignment.CenterVertically
                                            ) {
                                                Text(
                                                    text = d.lord,
                                                    color = TextPrimary,
                                                    fontWeight = FontWeight.Bold,
                                                    fontSize = 15.sp
                                                )
                                                Row(verticalAlignment = Alignment.CenterVertically) {
                                                    Text(
                                                        text = "${d.startDateFormatted} ~ ${d.endDateFormatted}",
                                                        color = TextSecondary,
                                                        fontSize = 12.sp
                                                    )
                                                    Spacer(modifier = Modifier.width(4.dp))
                                                    Icon(
                                                        imageVector = if (isExpanded) Icons.Default.KeyboardArrowUp else Icons.Default.KeyboardArrowDown,
                                                        contentDescription = null,
                                                        tint = TextSecondary,
                                                        modifier = Modifier.size(18.dp)
                                                    )
                                                }
                                            }

                                            // 안타르다샤 아코디언 펼침 내용
                                            AnimatedVisibility(visible = isExpanded) {
                                                Column(
                                                    modifier = Modifier
                                                        .fillMaxWidth()
                                                        .background(SurfaceLight, RoundedCornerShape(8.dp))
                                                        .padding(horizontal = 12.dp, vertical = 6.dp)
                                                ) {
                                                    if (d.sub_periods.isEmpty()) {
                                                        Text(
                                                            text = "No Antardasha records available",
                                                            color = TextSecondary,
                                                            fontSize = 12.sp,
                                                            modifier = Modifier.padding(vertical = 4.dp)
                                                        )
                                                    } else {
                                                        d.sub_periods.forEach { sub ->
                                                            Row(
                                                                modifier = Modifier
                                                                    .fillMaxWidth()
                                                                    .padding(vertical = 4.dp),
                                                                horizontalArrangement = Arrangement.SpaceBetween,
                                                                verticalAlignment = Alignment.CenterVertically
                                                            ) {
                                                                Text(
                                                                    text = sub.lord,
                                                                    color = TextPrimary,
                                                                    fontWeight = FontWeight.Medium,
                                                                    fontSize = 13.sp
                                                                )
                                                                Text(
                                                                    text = "${sub.startDateFormatted} ~ ${sub.endDateFormatted}",
                                                                    color = TextSecondary,
                                                                    fontSize = 12.sp
                                                                )
                                                            }
                                                            HorizontalDivider(color = BorderColor.copy(alpha = 0.3f))
                                                        }
                                                    }
                                                }
                                            }
                                            HorizontalDivider(color = BorderColor.copy(alpha = 0.5f))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        SelectiveCopyBottomSheet(
            isOpen = showSelectiveCopySheet,
            onDismiss = { showSelectiveCopySheet = false },
            profile = profile,
            saju = sajuOutput?.saju,
            western = westernOutput?.western,
            vedic = vedicOutput?.vedic,
            timezoneId = sajuOutput?.timezone_id ?: westernOutput?.timezone_id ?: vedicOutput?.timezone_id ?: profile.timezone_id,
            utcOffsetHours = sajuOutput?.utc_offset_hours ?: westernOutput?.utc_offset_hours ?: vedicOutput?.utc_offset_hours ?: 9.0,
            selectedDomainTab = selectedDomainTab
        )
    }
}

@Composable
private fun DomainErrorCard(
    message: String,
    onRetry: () -> Unit
) {
    Card(
        modifier = Modifier
            .fillMaxWidth()
            .padding(vertical = 16.dp),
        colors = CardDefaults.cardColors(containerColor = AstrologicalRed.copy(alpha = 0.08f)),
        border = BorderStroke(1.dp, AstrologicalRed.copy(alpha = 0.3f)),
        shape = RoundedCornerShape(10.dp)
    ) {
        Column(
            modifier = Modifier
                .fillMaxWidth()
                .padding(16.dp),
            horizontalAlignment = Alignment.CenterHorizontally
        ) {
            Text(
                text = message,
                color = AstrologicalRed,
                fontSize = 14.sp,
                textAlign = TextAlign.Center
            )
            Spacer(modifier = Modifier.height(10.dp))
            Button(
                onClick = onRetry,
                colors = ButtonDefaults.buttonColors(containerColor = AccentPrimary)
            ) {
                Text("다시 시도", color = BackgroundWhite, fontSize = 13.sp)
            }
        }
    }
}

private fun getSignName(index: Int): String {
    val signs = listOf(
        "Aries", "Taurus", "Gemini", "Cancer", "Leo", "Virgo",
        "Libra", "Scorpio", "Sagittarius", "Capricorn", "Aquarius", "Pisces"
    )
    return signs.getOrElse((index % 12 + 12) % 12) { "" }
}
