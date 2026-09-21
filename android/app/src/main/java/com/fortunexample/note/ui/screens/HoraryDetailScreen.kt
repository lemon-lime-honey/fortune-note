package com.fortunexample.note.ui.screens

import android.content.ClipData
import android.content.ClipboardManager
import android.content.Context
import android.widget.Toast
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.filled.ContentCopy
import androidx.compose.material.icons.filled.Delete
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
import com.fortunexample.note.ui.components.VedicChartDiamond
import com.fortunexample.note.ui.components.WesternChartWheel
import com.fortunexample.note.ui.theme.*

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun HoraryDetailScreen(
    horary: HoraryDto,
    chart: FortuneChartOutput?,
    markdown: String?,
    onBack: () -> Unit,
    onDelete: () -> Unit
) {
    val context = LocalContext.current
    var selectedTab by remember { mutableStateOf(0) }

    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text(horary.title, fontWeight = FontWeight.Bold) },
                navigationIcon = {
                    IconButton(onClick = onBack) {
                        Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = "뒤로")
                    }
                },
                actions = {
                    IconButton(onClick = {
                        if (!markdown.isNullOrEmpty()) {
                            val clipboard = context.getSystemService(Context.CLIPBOARD_SERVICE) as ClipboardManager
                            clipboard.setPrimaryClip(ClipData.newPlainText("Horary Prompt", markdown))
                            Toast.makeText(context, "호라리 LLM 프롬프트가 복사되었습니다.", Toast.LENGTH_SHORT).show()
                        }
                    }) {
                        Icon(Icons.Default.ContentCopy, contentDescription = "복사", tint = AccentGold)
                    }
                    IconButton(onClick = onDelete) {
                        Icon(Icons.Default.Delete, contentDescription = "삭제", tint = AstrologicalRed)
                    }
                },
                colors = TopAppBarDefaults.topAppBarColors(
                    containerColor = DeepNavy,
                    titleContentColor = TextPrimary,
                    navigationIconContentColor = TextPrimary
                )
            )
        },
        containerColor = DeepNavy
    ) { padding ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(padding)
        ) {
            TabRow(
                selectedTabIndex = selectedTab,
                containerColor = SurfaceDark,
                contentColor = CelestialBlue
            ) {
                Tab(selected = selectedTab == 0, onClick = { selectedTab = 0 }, text = { Text("호라리 (서양)") })
                Tab(selected = selectedTab == 1, onClick = { selectedTab = 1 }, text = { Text("프라슈나 (베다)") })
            }

            if (chart == null) {
                Box(modifier = Modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
                    CircularProgressIndicator(color = CelestialBlue)
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
                        colors = CardDefaults.cardColors(containerColor = SurfaceDark),
                        modifier = Modifier.fillMaxWidth()
                    ) {
                        Column(modifier = Modifier.padding(14.dp)) {
                            Text(text = "분류: ${horary.category}", color = CelestialBlue, fontSize = 12.sp)
                            Spacer(modifier = Modifier.height(4.dp))
                            Text(text = horary.question_text, color = TextPrimary, fontSize = 15.sp)
                            Spacer(modifier = Modifier.height(6.dp))
                            Text(
                                text = "질문 시각: ${horary.query_year}-${horary.query_month}-${horary.query_day} ${String.format("%02d:%02d:%02d", horary.query_hour, horary.query_minute, horary.query_second.toInt())}",
                                color = TextSecondary,
                                fontSize = 12.sp
                            )
                        }
                    }

                    Spacer(modifier = Modifier.height(16.dp))

                    if (selectedTab == 0) {
                        WesternChartWheel(chart = chart.western)
                        Spacer(modifier = Modifier.height(12.dp))
                        Text("주요 아스펙트", color = TextPrimary, fontWeight = FontWeight.Bold, fontSize = 15.sp)
                        chart.western.aspects.forEach { a ->
                            Text(
                                text = "${a.planet1} - ${a.planet2}: ${a.aspect_type} (오차: ${String.format("%.2f°", a.orb)})",
                                color = TextSecondary,
                                fontSize = 13.sp
                            )
                        }
                    } else {
                        VedicChartDiamond(chart = chart.vedic)
                        Spacer(modifier = Modifier.height(12.dp))
                        Text("프라슈나 라그나: ${String.format("%.2f°", chart.vedic.lagna_sidereal)}", color = TextPrimary, fontSize = 15.sp)
                        chart.vedic.planets.forEach { p ->
                            Text(
                                text = "${p.name}: ${String.format("%.2f°", p.sidereal_longitude)} (${p.nakshatra_name})",
                                color = TextSecondary,
                                fontSize = 13.sp
                            )
                        }
                    }
                }
            }
        }
    }
}
