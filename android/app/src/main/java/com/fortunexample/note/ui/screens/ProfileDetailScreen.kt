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
import androidx.compose.ui.platform.LocalClipboardManager
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.AnnotatedString
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
import com.fortunexample.note.data.findHouseNumber
import com.fortunexample.note.data.getZodiacSignName
import com.fortunexample.note.data.getZodiacSignRuler
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
    val clipboardManager = LocalClipboardManager.current
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
    var selectedHouse by remember { mutableStateOf("Regiomontanus") }

    var vedicOutput by remember { mutableStateOf<VedicCalculationOutputDto?>(null) }
    var isVedicLoading by remember { mutableStateOf(false) }
    var vedicError by remember { mutableStateOf<String?>(null) }

    val todayCal = remember { java.util.Calendar.getInstance() }
    var predictiveTargetYear by remember { mutableIntStateOf(todayCal.get(java.util.Calendar.YEAR)) }
    var predictiveTargetMonth by remember { mutableIntStateOf(todayCal.get(java.util.Calendar.MONTH) + 1) }
    var predictiveTargetDay by remember { mutableIntStateOf(todayCal.get(java.util.Calendar.DAY_OF_MONTH)) }
    var predictiveTargetHour by remember { mutableStateOf<Int?>(null) }
    var predictiveTargetMinute by remember { mutableStateOf<Int?>(null) }
    var predictiveData by remember { mutableStateOf<PredictiveSuiteData?>(null) }
    var isPredictiveLoading by remember { mutableStateOf(false) }

    // 1. Saju immediate load on screen enter or profile update
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
                sajuError = it.message ?: "사주명리 데이터 계산에 실패했습니다."
                isSajuLoading = false
            }
    }

    // 2. Western lazy load on Western tab (tab index 1)
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
                    westernError = it.message ?: "서양 점성술 데이터 계산에 실패했습니다."
                    isWesternLoading = false
                }
        }
    }

    // 3. Vedic lazy load on Vedic tab (tab index 2)
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
                    vedicError = it.message ?: "베딕 점성술 데이터 계산에 실패했습니다."
                    isVedicLoading = false
                }
        }
    }

    // 4. Lazy-load predictive suite when user enters Western tab (tab index 1) or changes target date/time/house
    LaunchedEffect(profile, predictiveTargetYear, predictiveTargetMonth, predictiveTargetDay, predictiveTargetHour, predictiveTargetMinute, selectedHouse, selectedDomainTab) {
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
                            val houseSystems = listOf("Regiomontanus", "Placidus", "WholeSign", "Equal", "Koch", "Campanus")

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
                                    Row(
                                        modifier = Modifier.fillMaxWidth(),
                                        horizontalArrangement = Arrangement.SpaceBetween,
                                        verticalAlignment = Alignment.CenterVertically
                                    ) {
                                        Text("Planets & Points", color = TextPrimary, fontWeight = FontWeight.Bold, fontSize = 15.sp)
                                        IconButton(
                                            onClick = {
                                                val sb = StringBuilder()
                                                sb.appendLine("[Western Tropical Chart - Planets & Points]")
                                                sb.appendLine("Ascendant: ${String.format("%.2f°", westernChart.ascendant)} (${getZodiacSignName((westernChart.ascendant / 30.0).toInt() % 12)})")
                                                sb.appendLine("Midheaven (MC): ${String.format("%.2f°", westernChart.midheaven)} (${getZodiacSignName((westernChart.midheaven / 30.0).toInt() % 12)})")
                                                westernChart.planets.forEach { p ->
                                                    val rx = if (p.is_retrograde) " [Rx]" else ""
                                                    val speedStr = String.format("%+.2f°/d", p.speed)
                                                    sb.appendLine("• ${p.name}$rx: ${String.format("%.2f°", p.sign_degree)} ${getZodiacSignName(p.sign_index)} · House ${p.house_number} · $speedStr")
                                                }
                                                westernChart.nodes.forEach { n ->
                                                    sb.appendLine("• ${n.name}: ${String.format("%.2f°", n.sign_degree)} ${getZodiacSignName(n.sign_index)} · House ${n.house_number}")
                                                }
                                                clipboardManager.setText(AnnotatedString(sb.toString().trimEnd()))
                                                Toast.makeText(context, "Planets & Points 복사 완료", Toast.LENGTH_SHORT).show()
                                            },
                                            modifier = Modifier.size(28.dp)
                                        ) {
                                            Icon(
                                                Icons.Default.ContentCopy,
                                                contentDescription = "Planets & Points 복사",
                                                tint = TextSecondary,
                                                modifier = Modifier.size(16.dp)
                                            )
                                        }
                                    }
                                    Spacer(modifier = Modifier.height(10.dp))
                                    // 앵글 포인트 (ASC, MC) 명시적 렌더링
                                    val ascSignIdx = ((westernChart.ascendant / 30.0).toInt() % 12 + 12) % 12
                                    val ascSignDeg = westernChart.ascendant % 30.0
                                    val mcSignIdx = ((westernChart.midheaven / 30.0).toInt() % 12 + 12) % 12
                                    val mcSignDeg = westernChart.midheaven % 30.0
                                    val mcHouseNum = if (westernChart.house_cusps.isNotEmpty()) findHouseNumber(westernChart.midheaven, westernChart.house_cusps) else 10

                                    Row(
                                        modifier = Modifier.fillMaxWidth().padding(vertical = 5.dp),
                                        horizontalArrangement = Arrangement.SpaceBetween,
                                        verticalAlignment = Alignment.CenterVertically
                                    ) {
                                        Text(text = "Ascendant (ASC)", color = TextPrimary, fontWeight = FontWeight.SemiBold, fontSize = 14.sp)
                                        Text(
                                            text = "${String.format("%.2f°", ascSignDeg)} ${getZodiacSignName(ascSignIdx)} · House 1",
                                            color = AccentPrimary,
                                            fontWeight = FontWeight.Medium,
                                            fontSize = 13.sp
                                        )
                                    }
                                    HorizontalDivider(color = BorderColor.copy(alpha = 0.5f))

                                    Row(
                                        modifier = Modifier.fillMaxWidth().padding(vertical = 5.dp),
                                        horizontalArrangement = Arrangement.SpaceBetween,
                                        verticalAlignment = Alignment.CenterVertically
                                    ) {
                                        Text(text = "Midheaven (MC)", color = TextPrimary, fontWeight = FontWeight.SemiBold, fontSize = 14.sp)
                                        Text(
                                            text = "${String.format("%.2f°", mcSignDeg)} ${getZodiacSignName(mcSignIdx)} · House $mcHouseNum",
                                            color = AccentPrimary,
                                            fontWeight = FontWeight.Medium,
                                            fontSize = 13.sp
                                        )
                                    }
                                    HorizontalDivider(color = BorderColor.copy(alpha = 0.5f))

                                    westernChart.planets.forEach { p ->
                                        val rx = if (p.is_retrograde) " [Rx]" else ""
                                        val speedStr = String.format("%+.2f°/d", p.speed)
                                        Row(
                                            modifier = Modifier.fillMaxWidth().padding(vertical = 5.dp),
                                            horizontalArrangement = Arrangement.SpaceBetween,
                                            verticalAlignment = Alignment.CenterVertically
                                        ) {
                                            Row(verticalAlignment = Alignment.CenterVertically) {
                                                Text(text = "${p.name}$rx", color = TextPrimary, fontWeight = FontWeight.Medium, fontSize = 14.sp)
                                                Spacer(modifier = Modifier.width(6.dp))
                                                Text(
                                                    text = speedStr,
                                                    color = if (p.is_retrograde) AccentPrimary else TextSecondary.copy(alpha = 0.8f),
                                                    fontSize = 11.sp,
                                                    fontWeight = FontWeight.Normal
                                                )
                                            }
                                            Text(
                                                text = "${String.format("%.2f°", p.sign_degree)} ${getZodiacSignName(p.sign_index)} · House ${p.house_number}",
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
                                                text = "${String.format("%.2f°", n.sign_degree)} ${getZodiacSignName(n.sign_index)} · House ${n.house_number}",
                                                color = TextSecondary,
                                                fontSize = 13.sp
                                            )
                                        }
                                        HorizontalDivider(color = BorderColor.copy(alpha = 0.5f))
                                    }
                                }
                            }

                            Spacer(modifier = Modifier.height(14.dp))

                            // 12 하우스 커스프 도수/사인 카드
                            if (westernChart.house_cusps.isNotEmpty()) {
                                Card(
                                    shape = RoundedCornerShape(12.dp),
                                    colors = CardDefaults.cardColors(containerColor = BackgroundWhite),
                                    border = BorderStroke(1.dp, BorderColor),
                                    modifier = Modifier.fillMaxWidth()
                                ) {
                                    Column(modifier = Modifier.padding(16.dp)) {
                                        val houseSysName = westernChart.house_system ?: selectedHouse
                                        val isWholeSign = houseSysName.lowercase() in listOf("wholesign", "whole_sign", "whole")
                                        val mcHouseNum = if (westernChart.house_cusps.isNotEmpty()) findHouseNumber(westernChart.midheaven, westernChart.house_cusps) else 10
                                        val cardTitle = if (isWholeSign) "House Signs ($houseSysName)" else "House Cusps ($houseSysName)"
                                        Row(
                                            modifier = Modifier.fillMaxWidth(),
                                            horizontalArrangement = Arrangement.SpaceBetween,
                                            verticalAlignment = Alignment.CenterVertically
                                        ) {
                                            Text(
                                                text = cardTitle,
                                                color = TextPrimary,
                                                fontWeight = FontWeight.Bold,
                                                fontSize = 15.sp
                                            )
                                            IconButton(
                                                onClick = {
                                                    val sb = StringBuilder()
                                                    sb.appendLine("[Western Tropical Chart - $cardTitle]")
                                                    westernChart.house_cusps.forEachIndexed { idx, deg ->
                                                        val hNum = idx + 1
                                                        val signIdx = ((deg / 30.0).toInt() % 12 + 12) % 12
                                                        val signName = getZodiacSignName(signIdx)
                                                        val ruler = getZodiacSignRuler(signIdx)
                                                        val signDeg = deg % 30.0
                                                        if (isWholeSign) {
                                                            val mcTag = if (hNum == mcHouseNum) " [MC Location]" else ""
                                                            sb.appendLine("• House ${String.format("%02d", hNum)}$mcTag: $signName · Ruler: $ruler")
                                                        } else {
                                                            val angleTag = when (hNum) {
                                                                1 -> " (ASC)"
                                                                4 -> " (IC)"
                                                                7 -> " (DSC)"
                                                                10 -> " (MC)"
                                                                else -> ""
                                                            }
                                                            sb.appendLine("• House ${String.format("%02d", hNum)}$angleTag: ${String.format("%.2f°", signDeg)} $signName")
                                                        }
                                                    }
                                                    clipboardManager.setText(AnnotatedString(sb.toString().trimEnd()))
                                                    Toast.makeText(context, "$cardTitle 복사 완료", Toast.LENGTH_SHORT).show()
                                                },
                                                modifier = Modifier.size(28.dp)
                                            ) {
                                                Icon(
                                                    Icons.Default.ContentCopy,
                                                    contentDescription = "$cardTitle 복사",
                                                    tint = TextSecondary,
                                                    modifier = Modifier.size(16.dp)
                                                )
                                            }
                                        }
                                        Spacer(modifier = Modifier.height(10.dp))
                                        val chunks = westernChart.house_cusps.chunked(2)
                                        chunks.forEachIndexed { rowIdx, pair ->
                                            Row(
                                                modifier = Modifier
                                                    .fillMaxWidth()
                                                    .padding(vertical = 4.dp),
                                                horizontalArrangement = Arrangement.SpaceBetween
                                            ) {
                                                val h1 = rowIdx * 2 + 1
                                                val deg1 = pair[0]
                                                val sIdx1 = ((deg1 / 30.0).toInt() % 12 + 12) % 12
                                                val s1 = getZodiacSignName(sIdx1)
                                                val r1 = getZodiacSignRuler(sIdx1)
                                                val d1 = deg1 % 30.0
                                                val tag1 = if (!isWholeSign) {
                                                    when (h1) { 1 -> " (ASC)"; 7 -> " (DSC)"; else -> "" }
                                                } else if (h1 == mcHouseNum) {
                                                    " [MC]"
                                                } else {
                                                    ""
                                                }
                                                val text1 = if (isWholeSign) "H$h1$tag1: $s1 · $r1" else "H$h1$tag1: ${String.format("%.1f°", d1)} $s1"
                                                val isH1Highlighted = if (!isWholeSign) (h1 == 1 || h1 == 7) else (h1 == 1 || h1 == mcHouseNum)
                                                Text(
                                                    text = text1,
                                                    fontSize = 12.sp,
                                                    color = if (isH1Highlighted) AccentPrimary else TextPrimary,
                                                    fontWeight = if (isH1Highlighted) FontWeight.SemiBold else FontWeight.Normal,
                                                    modifier = Modifier.weight(1f)
                                                )

                                                if (pair.size > 1) {
                                                    val h2 = rowIdx * 2 + 2
                                                    val deg2 = pair[1]
                                                    val sIdx2 = ((deg2 / 30.0).toInt() % 12 + 12) % 12
                                                    val s2 = getZodiacSignName(sIdx2)
                                                    val r2 = getZodiacSignRuler(sIdx2)
                                                    val d2 = deg2 % 30.0
                                                    val tag2 = if (!isWholeSign) {
                                                        when (h2) { 4 -> " (IC)"; 10 -> " (MC)"; else -> "" }
                                                    } else if (h2 == mcHouseNum) {
                                                        " [MC]"
                                                    } else {
                                                        ""
                                                    }
                                                    val text2 = if (isWholeSign) "H$h2$tag2: $s2 · $r2" else "H$h2$tag2: ${String.format("%.1f°", d2)} $s2"
                                                    val isH2Highlighted = if (!isWholeSign) (h2 == 4 || h2 == 10) else (h2 == mcHouseNum)
                                                    Text(
                                                        text = text2,
                                                        fontSize = 12.sp,
                                                        color = if (isH2Highlighted) AccentPrimary else TextPrimary,
                                                        fontWeight = if (isH2Highlighted) FontWeight.SemiBold else FontWeight.Normal,
                                                        modifier = Modifier.weight(1f)
                                                    )
                                                }
                                            }
                                            if (rowIdx < chunks.size - 1) {
                                                HorizontalDivider(color = BorderColor.copy(alpha = 0.5f))
                                            }
                                        }
                                    }
                                }
                                Spacer(modifier = Modifier.height(14.dp))
                            }

                            if (westernChart.aspects.isNotEmpty()) {
                                Card(
                                    shape = RoundedCornerShape(12.dp),
                                    colors = CardDefaults.cardColors(containerColor = BackgroundWhite),
                                    border = BorderStroke(1.dp, BorderColor),
                                    modifier = Modifier.fillMaxWidth()
                                ) {
                                    Column(modifier = Modifier.padding(16.dp)) {
                                        Row(
                                            modifier = Modifier.fillMaxWidth(),
                                            horizontalArrangement = Arrangement.SpaceBetween,
                                            verticalAlignment = Alignment.CenterVertically
                                        ) {
                                            Text("Major Aspects", color = TextPrimary, fontWeight = FontWeight.Bold, fontSize = 15.sp)
                                            IconButton(
                                                onClick = {
                                                    val sb = StringBuilder()
                                                    sb.appendLine("[Western Tropical Chart - Major Aspects]")
                                                    westernChart.aspects.forEach { a ->
                                                        val dynStr = if (a.is_applying) "Applying" else "Separating"
                                                        sb.appendLine("• ${a.planet1} · ${a.planet2}: ${a.aspect_type} (Orb ${String.format("%.2f°", a.orb)}, $dynStr)")
                                                    }
                                                    clipboardManager.setText(AnnotatedString(sb.toString().trimEnd()))
                                                    Toast.makeText(context, "Major Aspects 복사 완료", Toast.LENGTH_SHORT).show()
                                                },
                                                modifier = Modifier.size(28.dp)
                                            ) {
                                                Icon(
                                                    Icons.Default.ContentCopy,
                                                    contentDescription = "Major Aspects 복사",
                                                    tint = TextSecondary,
                                                    modifier = Modifier.size(16.dp)
                                                )
                                            }
                                        }
                                        Spacer(modifier = Modifier.height(10.dp))
                                        westernChart.aspects.forEach { a ->
                                            val dynStr = if (a.is_applying) "App" else "Sep"
                                            Row(
                                                modifier = Modifier.fillMaxWidth().padding(vertical = 5.dp),
                                                horizontalArrangement = Arrangement.SpaceBetween,
                                                verticalAlignment = Alignment.CenterVertically
                                            ) {
                                                Row(verticalAlignment = Alignment.CenterVertically) {
                                                    Text(text = "${a.planet1} · ${a.planet2}", color = TextPrimary, fontSize = 13.sp)
                                                    Spacer(modifier = Modifier.width(6.dp))
                                                    Text(
                                                        text = dynStr,
                                                        color = if (a.is_applying) AccentPrimary else TextSecondary.copy(alpha = 0.7f),
                                                        fontSize = 11.sp,
                                                        fontWeight = FontWeight.Medium
                                                    )
                                                }
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

                            // 서양 전통 점성술 분석 섹션 (헬레니즘 / 릴리 분기)
                            WesternTraditionalSection(
                                traditional = westernChart.traditional,
                                isHorary = false,
                                hellenistic = westernChart.hellenistic
                            )

                            Spacer(modifier = Modifier.height(14.dp))

                            // 6대 서양 예측 기법 보조 메뉴 섹션
                            WesternPredictiveSection(
                                profile = profile,
                                predictiveData = predictiveData,
                                targetYear = predictiveTargetYear,
                                targetMonth = predictiveTargetMonth,
                                targetDay = predictiveTargetDay,
                                isLoading = isPredictiveLoading,
                                onYearChange = {
                                    predictiveTargetYear = it
                                },
                                onDateChange = { y, m, d ->
                                    predictiveTargetYear = y
                                    predictiveTargetMonth = m
                                    predictiveTargetDay = d
                                },
                                onTimeChange = { h, m ->
                                    predictiveTargetHour = h
                                    predictiveTargetMinute = m
                                },
                                onDateTimeChange = { y, m, d, h, min ->
                                    predictiveTargetYear = y
                                    predictiveTargetMonth = m
                                    predictiveTargetDay = d
                                    predictiveTargetHour = h
                                    predictiveTargetMinute = min
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
                                        text = "Lagna: ${getZodiacSignName(vargaLagnaSign)}",
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
                                                    text = "House $houseNum · ${getZodiacSignName(signIdx)}",
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
                                                    text = "${String.format(java.util.Locale.US, "%.2f°", displayDegree)} $degreeLabel",
                                                    color = TextSecondary,
                                                    fontSize = 12.sp,
                                                    modifier = Modifier.weight(1f, fill = false)
                                                )
                                                Spacer(modifier = Modifier.width(8.dp))
                                                Text(
                                                    text = nakshatraLabel,
                                                    color = CelestialBlue,
                                                    fontSize = 12.sp,
                                                    fontWeight = FontWeight.Medium,
                                                    textAlign = TextAlign.End,
                                                    modifier = Modifier.weight(1f, fill = false)
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
