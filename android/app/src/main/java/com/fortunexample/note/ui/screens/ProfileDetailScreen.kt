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
import com.fortunexample.note.data.ProfileDto
import com.fortunexample.note.ui.components.SajuChartTable
import com.fortunexample.note.ui.components.VedicChartDiamond
import com.fortunexample.note.ui.components.WesternChartWheel
import com.fortunexample.note.ui.theme.*

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ProfileDetailScreen(
    profile: ProfileDto,
    chart: FortuneChartOutput?,
    markdown: String?,
    onBack: () -> Unit,
    onDelete: () -> Unit
) {
    val context = LocalContext.current
    var selectedDomainTab by remember { mutableStateOf(0) }

    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text(profile.name, fontWeight = FontWeight.Bold) },
                navigationIcon = {
                    IconButton(onClick = onBack) {
                        Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = "뒤로")
                    }
                },
                actions = {
                    IconButton(onClick = {
                        if (!markdown.isNullOrEmpty()) {
                            val clipboard = context.getSystemService(Context.CLIPBOARD_SERVICE) as ClipboardManager
                            clipboard.setPrimaryClip(ClipData.newPlainText("Fortune Note", markdown))
                            Toast.makeText(context, "LLM 마크다운 프롬프트가 복사되었습니다.", Toast.LENGTH_SHORT).show()
                        }
                    }) {
                        Icon(Icons.Default.ContentCopy, contentDescription = "LLM 마크다운 복사", tint = AccentGold)
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
            // 서브 도메인 탭
            TabRow(
                selectedTabIndex = selectedDomainTab,
                containerColor = SurfaceDark,
                contentColor = AccentGold
            ) {
                Tab(selected = selectedDomainTab == 0, onClick = { selectedDomainTab = 0 }, text = { Text("사주명리학") })
                Tab(selected = selectedDomainTab == 1, onClick = { selectedDomainTab = 1 }, text = { Text("서양 점성술") })
                Tab(selected = selectedDomainTab == 2, onClick = { selectedDomainTab = 2 }, text = { Text("베다 점성술") })
            }

            if (chart == null) {
                Box(modifier = Modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
                    CircularProgressIndicator(color = AccentGold)
                }
            } else {
                Column(
                    modifier = Modifier
                        .fillMaxSize()
                        .verticalScroll(rememberScrollState())
                        .padding(16.dp)
                ) {
                    when (selectedDomainTab) {
                        0 -> SajuChartTable(chart = chart.saju)
                        1 -> {
                            WesternChartWheel(chart = chart.western)
                            Spacer(modifier = Modifier.height(16.dp))
                            Text("7개 고전 행성 배치", color = TextPrimary, fontWeight = FontWeight.Bold, fontSize = 16.sp)
                            Spacer(modifier = Modifier.height(8.dp))
                            chart.western.planets.forEach { p ->
                                val retroStr = if (p.is_retrograde) " [역행]" else ""
                                Text(
                                    text = "${p.name}: ${String.format("%.2f°", p.longitude)} (제${p.house_number}하우스)${retroStr}",
                                    color = TextSecondary,
                                    fontSize = 14.sp
                                )
                            }
                            Spacer(modifier = Modifier.height(12.dp))
                            Text("달의 교점 (Nodes)", color = TextPrimary, fontWeight = FontWeight.Bold, fontSize = 16.sp)
                            chart.western.nodes.forEach { n ->
                                Text(
                                    text = "${n.name}: ${String.format("%.2f°", n.longitude)} (제${n.house_number}하우스)",
                                    color = TextSecondary,
                                    fontSize = 14.sp
                                )
                            }
                        }
                        2 -> {
                            VedicChartDiamond(chart = chart.vedic)
                            Spacer(modifier = Modifier.height(16.dp))
                            Text("D1 Rasi 행성 배치", color = TextPrimary, fontWeight = FontWeight.Bold, fontSize = 16.sp)
                            chart.vedic.planets.forEach { p ->
                                Text(
                                    text = "${p.name}: ${String.format("%.2f°", p.sidereal_longitude)} (${p.nakshatra_name} ${p.nakshatra_pada}파다)",
                                    color = TextSecondary,
                                    fontSize = 14.sp
                                )
                            }
                            Spacer(modifier = Modifier.height(16.dp))
                            Text("빔쇼타리 다샤 타임라인", color = TextPrimary, fontWeight = FontWeight.Bold, fontSize = 16.sp)
                            chart.vedic.vimshottari_dashas.forEach { d ->
                                Text(
                                    text = "대운 주성 ${d.lord}: JD ${String.format("%.1f", d.start_jd)} ~ ${String.format("%.1f", d.end_jd)}",
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
}
