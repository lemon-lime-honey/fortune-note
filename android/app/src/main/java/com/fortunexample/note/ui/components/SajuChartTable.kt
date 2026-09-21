package com.fortunexample.note.ui.components

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.horizontalScroll
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.fortunexample.note.data.PillarData
import com.fortunexample.note.data.SajuChartData
import com.fortunexample.note.ui.theme.*

@Composable
fun SajuChartTable(
    chart: SajuChartData,
    modifier: Modifier = Modifier
) {
    Column(
        modifier = modifier
            .fillMaxWidth()
            .padding(16.dp),
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        // 1. 4주 명식표
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceEvenly
        ) {
            PillarColumn(title = "시주", pillar = chart.hour_pillar)
            PillarColumn(title = "일주", pillar = chart.day_pillar)
            PillarColumn(title = "월주", pillar = chart.month_pillar)
            PillarColumn(title = "연주", pillar = chart.year_pillar)
        }

        Spacer(modifier = Modifier.height(24.dp))

        // 2. 대운수 및 방향 헤더
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            Text(
                text = "대운 흐름 (${if (chart.is_forward_daeun) "순행" else "역행"})",
                color = TextPrimary,
                fontWeight = FontWeight.Bold,
                fontSize = 16.sp
            )
            Text(
                text = "대운수: ${chart.daeun_number}세",
                color = AccentGold,
                fontWeight = FontWeight.SemiBold,
                fontSize = 14.sp
            )
        }

        Spacer(modifier = Modifier.height(12.dp))

        // 3. 10년 주기 대운 가로 스크롤 타임라인
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .horizontalScroll(rememberScrollState()),
            horizontalArrangement = Arrangement.spacedBy(8.dp)
        ) {
            chart.daeun_periods.forEach { period ->
                Card(
                    shape = RoundedCornerShape(8.dp),
                    colors = CardDefaults.cardColors(containerColor = CardDark),
                    modifier = Modifier.width(68.dp)
                ) {
                    Column(
                        modifier = Modifier.padding(8.dp),
                        horizontalAlignment = Alignment.CenterHorizontally
                    ) {
                        Text(
                            text = "${period.start_age}세",
                            color = TextSecondary,
                            fontSize = 11.sp
                        )
                        Spacer(modifier = Modifier.height(4.dp))
                        Text(
                            text = period.pillar.hanja,
                            color = SajuAmber,
                            fontWeight = FontWeight.Bold,
                            fontSize = 16.sp
                        )
                        Text(
                            text = period.pillar.korean,
                            color = TextPrimary,
                            fontSize = 12.sp
                        )
                    }
                }
            }
        }
    }
}

@Composable
private fun PillarColumn(
    title: String,
    pillar: PillarData
) {
    Card(
        shape = RoundedCornerShape(8.dp),
        colors = CardDefaults.cardColors(containerColor = CardDark),
        modifier = Modifier
            .width(72.dp)
            .border(1.dp, BorderColor, RoundedCornerShape(8.dp))
    ) {
        Column(
            modifier = Modifier.padding(vertical = 12.dp, horizontal = 4.dp),
            horizontalAlignment = Alignment.CenterHorizontally
        ) {
            Text(text = title, color = TextSecondary, fontSize = 12.sp)
            Spacer(modifier = Modifier.height(8.dp))
            // 천간
            Text(
                text = PillarData.STEMS_HANJA[pillar.stem_index % 10],
                color = AccentGold,
                fontSize = 20.sp,
                fontWeight = FontWeight.Bold
            )
            Text(
                text = PillarData.STEMS[pillar.stem_index % 10],
                color = TextPrimary,
                fontSize = 13.sp
            )
            Spacer(modifier = Modifier.height(12.dp))
            // 지지
            Text(
                text = PillarData.BRANCHES_HANJA[pillar.branch_index % 12],
                color = CelestialBlue,
                fontSize = 20.sp,
                fontWeight = FontWeight.Bold
            )
            Text(
                text = PillarData.BRANCHES[pillar.branch_index % 12],
                color = TextPrimary,
                fontSize = 13.sp
            )
        }
    }
}
