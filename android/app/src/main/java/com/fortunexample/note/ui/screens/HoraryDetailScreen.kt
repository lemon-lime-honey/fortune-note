package com.fortunexample.note.ui.screens

import android.content.ClipData
import android.content.ClipboardManager
import android.content.Context
import android.widget.Toast
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.background
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.filled.ContentCopy
import androidx.compose.material.icons.filled.Delete
import androidx.compose.material.icons.filled.Edit
import androidx.compose.material.icons.filled.MoreVert
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.fortunexample.note.data.FortuneChartOutput
import com.fortunexample.note.data.HoraryDto
import com.fortunexample.note.data.getZodiacSignName
import com.fortunexample.note.ui.components.VedicChartDiamond
import com.fortunexample.note.ui.components.WesternChartWheel
import com.fortunexample.note.ui.components.WesternTraditionalSection
import com.fortunexample.note.ui.theme.*

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
    var selectedTab by remember { mutableStateOf(0) }
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
                title = { Text(horary.title, fontWeight = FontWeight.Bold, fontSize = 18.sp, color = TextPrimary) },
                navigationIcon = {
                    IconButton(onClick = onBack) {
                        Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = "뒤로", tint = TextPrimary)
                    }
                },
                actions = {
                    IconButton(onClick = onEdit) {
                        Icon(Icons.Default.Edit, contentDescription = "수정", tint = TextPrimary)
                    }
                    IconButton(onClick = {
                        if (selectedTab == 0) {
                            copyToClipboard("Western Horary Data", westernMarkdown, "호라리(서양) 분석 데이터가 복사되었습니다.")
                        } else {
                            copyToClipboard("Vedic Prashna Data", prashnaMarkdown, "프라슈나(베다) 분석 데이터가 복사되었습니다.")
                        }
                    }) {
                        Icon(Icons.Default.ContentCopy, contentDescription = "현재 탭 복사", tint = TextPrimary)
                    }
                    IconButton(onClick = { showMenu = true }) {
                        Icon(Icons.Default.MoreVert, contentDescription = "옵션 메뉴", tint = TextPrimary)
                    }
                    DropdownMenu(
                        expanded = showMenu,
                        onDismissRequest = { showMenu = false },
                        modifier = Modifier.background(BackgroundWhite)
                    ) {
                        DropdownMenuItem(
                            text = { Text("호라리 (서양) 데이터 복사", color = TextPrimary, fontSize = 14.sp) },
                            onClick = {
                                showMenu = false
                                copyToClipboard("Western Horary Data", westernMarkdown, "호라리(서양) 분석 데이터가 복사되었습니다.")
                            }
                        )
                        DropdownMenuItem(
                            text = { Text("프라슈나 (베다) 데이터 복사", color = TextPrimary, fontSize = 14.sp) },
                            onClick = {
                                showMenu = false
                                copyToClipboard("Vedic Prashna Data", prashnaMarkdown, "프라슈나(베다) 분석 데이터가 복사되었습니다.")
                            }
                        )
                        HorizontalDivider(color = BorderColor)
                        DropdownMenuItem(
                            text = { Text("질문 기록 삭제", color = AstrologicalRed, fontSize = 14.sp) },
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
                Tab(selected = selectedTab == 0, onClick = { selectedTab = 0 }, text = { Text("Horary (Western)", fontWeight = FontWeight.SemiBold) })
                Tab(selected = selectedTab == 1, onClick = { selectedTab = 1 }, text = { Text("Prashna (Vedic)", fontWeight = FontWeight.SemiBold) })
            }

            if (chart == null) {
                Box(modifier = Modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
                    CircularProgressIndicator(color = AccentPrimary)
                }
            } else {
                Column(
                    modifier = Modifier
                        .fillMaxSize()
                        .verticalScroll(rememberScrollState())
                        .padding(16.dp)
                ) {
                    // 질문 내용 카드
                    Card(
                        shape = RoundedCornerShape(12.dp),
                        colors = CardDefaults.cardColors(containerColor = BackgroundWhite),
                        border = BorderStroke(1.dp, BorderColor),
                        modifier = Modifier.fillMaxWidth()
                    ) {
                        Column(modifier = Modifier.padding(16.dp)) {
                            Text(text = "Category: ${horary.category}", color = CelestialBlue, fontWeight = FontWeight.SemiBold, fontSize = 13.sp)
                            Spacer(modifier = Modifier.height(6.dp))
                            Text(text = horary.question_text, color = TextPrimary, fontSize = 16.sp, fontWeight = FontWeight.Medium)
                            Spacer(modifier = Modifier.height(8.dp))
                            Text(
                                text = "Query Time: ${horary.query_year}-${String.format("%02d", horary.query_month)}-${String.format("%02d", horary.query_day)} ${String.format("%02d:%02d:%02d", horary.query_hour, horary.query_minute, horary.query_second.toInt())} (${horary.timezone_id})",
                                color = TextSecondary,
                                fontSize = 12.sp
                            )
                        }
                    }

                    Spacer(modifier = Modifier.height(16.dp))

                    if (selectedTab == 0) {
                        WesternChartWheel(chart = chart.western)
                        Spacer(modifier = Modifier.height(16.dp))

                        // 12 하우스 커스프 카드
                        if (chart.western.house_cusps.isNotEmpty()) {
                            Card(
                                shape = RoundedCornerShape(12.dp),
                                colors = CardDefaults.cardColors(containerColor = BackgroundWhite),
                                border = BorderStroke(1.dp, BorderColor),
                                modifier = Modifier.fillMaxWidth()
                            ) {
                                Column(modifier = Modifier.padding(16.dp)) {
                                    val houseSysName = chart.western.house_system ?: "Regiomontanus"
                                    Row(
                                        modifier = Modifier.fillMaxWidth(),
                                        horizontalArrangement = Arrangement.SpaceBetween,
                                        verticalAlignment = Alignment.CenterVertically
                                    ) {
                                        Text(
                                            text = "House Cusps ($houseSysName)",
                                            color = TextPrimary,
                                            fontWeight = FontWeight.Bold,
                                            fontSize = 15.sp
                                        )
                                        IconButton(
                                            onClick = {
                                                val sb = StringBuilder()
                                                sb.appendLine("[Horary Chart - House Cusps ($houseSysName)]")
                                                chart.western.house_cusps.forEachIndexed { idx, deg ->
                                                    val hNum = idx + 1
                                                    val signIdx = ((deg / 30.0).toInt() % 12 + 12) % 12
                                                    val signName = getZodiacSignName(signIdx)
                                                    val signDeg = deg % 30.0
                                                    val angleTag = when (hNum) {
                                                        1 -> " (ASC)"
                                                        4 -> " (IC)"
                                                        7 -> " (DSC)"
                                                        10 -> " (MC)"
                                                        else -> ""
                                                    }
                                                    sb.appendLine("• House ${String.format("%02d", hNum)}$angleTag: ${String.format("%.2f°", signDeg)} $signName")
                                                }
                                                copyToClipboard("House Cusps", sb.toString().trimEnd(), "House Cusps 복사 완료")
                                            },
                                            modifier = Modifier.size(28.dp)
                                        ) {
                                            Icon(
                                                Icons.Default.ContentCopy,
                                                contentDescription = "House Cusps 복사",
                                                tint = TextSecondary,
                                                modifier = Modifier.size(16.dp)
                                            )
                                        }
                                    }
                                    Spacer(modifier = Modifier.height(10.dp))
                                    val chunks = chart.western.house_cusps.chunked(2)
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
                                            val d1 = deg1 % 30.0
                                            val tag1 = when (h1) { 1 -> " (ASC)"; 7 -> " (DSC)"; else -> "" }
                                            Text(
                                                text = "H$h1$tag1: ${String.format("%.1f°", d1)} $s1",
                                                fontSize = 12.sp,
                                                color = if (h1 == 1 || h1 == 7) CelestialBlue else TextPrimary,
                                                fontWeight = if (h1 == 1 || h1 == 7) FontWeight.SemiBold else FontWeight.Normal,
                                                modifier = Modifier.weight(1f)
                                            )

                                            if (pair.size > 1) {
                                                val h2 = rowIdx * 2 + 2
                                                val deg2 = pair[1]
                                                val sIdx2 = ((deg2 / 30.0).toInt() % 12 + 12) % 12
                                                val s2 = getZodiacSignName(sIdx2)
                                                val d2 = deg2 % 30.0
                                                val tag2 = when (h2) { 4 -> " (IC)"; 10 -> " (MC)"; else -> "" }
                                                Text(
                                                    text = "H$h2$tag2: ${String.format("%.1f°", d2)} $s2",
                                                    fontSize = 12.sp,
                                                    color = if (h2 == 4 || h2 == 10) CelestialBlue else TextPrimary,
                                                    fontWeight = if (h2 == 4 || h2 == 10) FontWeight.SemiBold else FontWeight.Normal,
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

                        Card(
                            shape = RoundedCornerShape(12.dp),
                            colors = CardDefaults.cardColors(containerColor = BackgroundWhite),
                            border = BorderStroke(1.dp, BorderColor),
                            modifier = Modifier.fillMaxWidth()
                        ) {
                            Column(modifier = Modifier.padding(16.dp)) {
                                Text("Major Aspects", color = TextPrimary, fontWeight = FontWeight.Bold, fontSize = 15.sp)
                                Spacer(modifier = Modifier.height(10.dp))
                                chart.western.aspects.forEach { a ->
                                    val dynStr = if (a.is_applying) "App" else "Sep"
                                    Row(
                                        modifier = Modifier.fillMaxWidth().padding(vertical = 4.dp),
                                        horizontalArrangement = Arrangement.SpaceBetween,
                                        verticalAlignment = Alignment.CenterVertically
                                    ) {
                                        Row(verticalAlignment = Alignment.CenterVertically) {
                                            Text(text = "${a.planet1} · ${a.planet2}", color = TextPrimary, fontSize = 13.sp)
                                            Spacer(modifier = Modifier.width(6.dp))
                                            Text(
                                                text = dynStr,
                                                color = if (a.is_applying) CelestialBlue else TextSecondary.copy(alpha = 0.7f),
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
                        WesternTraditionalSection(traditional = chart.western.traditional, isHorary = true)
                    } else {
                        VedicChartDiamond(chart = chart.vedic)
                        Spacer(modifier = Modifier.height(16.dp))

                        Card(
                            shape = RoundedCornerShape(12.dp),
                            colors = CardDefaults.cardColors(containerColor = BackgroundWhite),
                            border = BorderStroke(1.dp, BorderColor),
                            modifier = Modifier.fillMaxWidth()
                        ) {
                            Column(modifier = Modifier.padding(16.dp)) {
                                Text(
                                    text = "Prashna Lagna: ${String.format("%.2f°", chart.vedic.lagna_sidereal)}",
                                    color = TextPrimary,
                                    fontWeight = FontWeight.Bold,
                                    fontSize = 15.sp
                                )
                                Spacer(modifier = Modifier.height(10.dp))
                                chart.vedic.planets.forEach { p ->
                                    Row(
                                        modifier = Modifier.fillMaxWidth().padding(vertical = 4.dp),
                                        horizontalArrangement = Arrangement.SpaceBetween
                                    ) {
                                        Text(text = p.name, color = TextPrimary, fontWeight = FontWeight.Medium, fontSize = 14.sp)
                                        Text(
                                            text = "${String.format("%.2f°", p.sidereal_longitude)} · ${p.nakshatra_name}",
                                            color = TextSecondary,
                                            fontSize = 13.sp
                                        )
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
