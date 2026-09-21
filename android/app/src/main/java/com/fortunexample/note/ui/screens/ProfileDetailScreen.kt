package com.fortunexample.note.ui.screens

import android.content.ClipData
import android.content.ClipboardManager
import android.content.Context
import android.widget.Toast
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.background
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
    onEdit: () -> Unit,
    onDelete: () -> Unit
) {
    val context = LocalContext.current
    var selectedDomainTab by remember { mutableStateOf(0) }

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
                    IconButton(onClick = {
                        if (!markdown.isNullOrEmpty()) {
                            val clipboard = context.getSystemService(Context.CLIPBOARD_SERVICE) as ClipboardManager
                            clipboard.setPrimaryClip(ClipData.newPlainText("Fortune Note", markdown))
                            Toast.makeText(context, "LLM 마크다운 프롬프트가 복사되었습니다.", Toast.LENGTH_SHORT).show()
                        }
                    }) {
                        Icon(Icons.Default.ContentCopy, contentDescription = "LLM 마크다운 복사", tint = TextPrimary)
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
            // 서브 도메인 탭 (라이트 스타일)
            TabRow(
                selectedTabIndex = selectedDomainTab,
                containerColor = SurfaceLight,
                contentColor = AccentPrimary
            ) {
                Tab(selected = selectedDomainTab == 0, onClick = { selectedDomainTab = 0 }, text = { Text("사주명리", fontWeight = FontWeight.SemiBold) })
                Tab(selected = selectedDomainTab == 1, onClick = { selectedDomainTab = 1 }, text = { Text("Western", fontWeight = FontWeight.SemiBold) })
                Tab(selected = selectedDomainTab == 2, onClick = { selectedDomainTab = 2 }, text = { Text("Vedic", fontWeight = FontWeight.SemiBold) })
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
                    when (selectedDomainTab) {
                        0 -> SajuChartTable(chart = chart.saju)
                        1 -> {
                            WesternChartWheel(chart = chart.western)
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
                                    chart.western.planets.forEach { p ->
                                        val rx = if (p.is_retrograde) " [Rx]" else ""
                                        Row(
                                            modifier = Modifier.fillMaxWidth().padding(vertical = 4.dp),
                                            horizontalArrangement = Arrangement.SpaceBetween
                                        ) {
                                            Text(text = "${p.name}$rx", color = TextPrimary, fontWeight = FontWeight.Medium, fontSize = 14.sp)
                                            Text(
                                                text = "${String.format("%.2f°", p.sign_degree)} (${getSignName(p.sign_index)}) · House ${p.house_number}",
                                                color = TextSecondary,
                                                fontSize = 13.sp
                                            )
                                        }
                                        HorizontalDivider(color = BorderColor.copy(alpha = 0.5f))
                                    }
                                    chart.western.nodes.forEach { n ->
                                        Row(
                                            modifier = Modifier.fillMaxWidth().padding(vertical = 4.dp),
                                            horizontalArrangement = Arrangement.SpaceBetween
                                        ) {
                                            Text(text = n.name, color = TextPrimary, fontWeight = FontWeight.Medium, fontSize = 14.sp)
                                            Text(
                                                text = "${String.format("%.2f°", n.sign_degree)} (${getSignName(n.sign_index)}) · House ${n.house_number}",
                                                color = TextSecondary,
                                                fontSize = 13.sp
                                            )
                                        }
                                        HorizontalDivider(color = BorderColor.copy(alpha = 0.5f))
                                    }
                                }
                            }

                            Spacer(modifier = Modifier.height(14.dp))

                            // Major Aspects 카드
                            if (chart.western.aspects.isNotEmpty()) {
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
                                            Row(
                                                modifier = Modifier.fillMaxWidth().padding(vertical = 4.dp),
                                                horizontalArrangement = Arrangement.SpaceBetween
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
                            }
                        }
                        2 -> {
                            VedicChartDiamond(chart = chart.vedic)
                            Spacer(modifier = Modifier.height(20.dp))

                            // D1 Rasi Placements 카드
                            Card(
                                shape = RoundedCornerShape(12.dp),
                                colors = CardDefaults.cardColors(containerColor = BackgroundWhite),
                                border = BorderStroke(1.dp, BorderColor),
                                modifier = Modifier.fillMaxWidth()
                            ) {
                                Column(modifier = Modifier.padding(16.dp)) {
                                    Text(
                                        text = "D1 Rasi Placements (Lagna: ${String.format("%.2f°", chart.vedic.lagna_sidereal)} ${getSignName(chart.vedic.lagna_sign)})",
                                        color = TextPrimary,
                                        fontWeight = FontWeight.Bold,
                                        fontSize = 14.sp
                                    )
                                    Spacer(modifier = Modifier.height(10.dp))
                                    chart.vedic.planets.forEach { p ->
                                        val rx = if (p.is_retrograde) " [Rx]" else ""
                                        Row(
                                            modifier = Modifier.fillMaxWidth().padding(vertical = 4.dp),
                                            horizontalArrangement = Arrangement.SpaceBetween
                                        ) {
                                            Text(text = "${p.name}$rx", color = TextPrimary, fontWeight = FontWeight.Medium, fontSize = 14.sp)
                                            Text(
                                                text = "${String.format("%.2f°", p.sign_degree)} ${getSignName(p.sign_index)} · ${p.nakshatra_name} Pada ${p.nakshatra_pada}",
                                                color = TextSecondary,
                                                fontSize = 13.sp
                                            )
                                        }
                                        HorizontalDivider(color = BorderColor.copy(alpha = 0.5f))
                                    }
                                }
                            }

                            Spacer(modifier = Modifier.height(14.dp))

                            // Vimshottari Dasha 카드
                            Card(
                                shape = RoundedCornerShape(12.dp),
                                colors = CardDefaults.cardColors(containerColor = BackgroundWhite),
                                border = BorderStroke(1.dp, BorderColor),
                                modifier = Modifier.fillMaxWidth()
                            ) {
                                Column(modifier = Modifier.padding(16.dp)) {
                                    Text("Vimshottari Dasha Sequence", color = TextPrimary, fontWeight = FontWeight.Bold, fontSize = 15.sp)
                                    Spacer(modifier = Modifier.height(10.dp))
                                    chart.vedic.vimshottari_dashas.forEach { d ->
                                        Row(
                                            modifier = Modifier.fillMaxWidth().padding(vertical = 4.dp),
                                            horizontalArrangement = Arrangement.SpaceBetween
                                        ) {
                                            Text(text = "Mahadasha: ${d.lord}", color = TextPrimary, fontWeight = FontWeight.Medium, fontSize = 14.sp)
                                            Text(text = "Start JD ${String.format("%.1f", d.start_jd)}", color = TextSecondary, fontSize = 13.sp)
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
}

private fun getSignName(index: Int): String {
    val signs = listOf(
        "Aries", "Taurus", "Gemini", "Cancer", "Leo", "Virgo",
        "Libra", "Scorpio", "Sagittarius", "Capricorn", "Aquarius", "Pisces"
    )
    return signs.getOrElse((index % 12 + 12) % 12) { "" }
}
