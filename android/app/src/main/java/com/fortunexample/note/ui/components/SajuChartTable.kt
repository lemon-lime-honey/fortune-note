package com.fortunexample.note.ui.components

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.horizontalScroll
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
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
            .padding(horizontal = 16.dp, vertical = 12.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp)
    ) {
        AstroPanelCard(
            title = "Four Pillars (원국 명식)",
            contentPadding = PaddingValues(12.dp)
        ) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.spacedBy(8.dp)
            ) {
                PillarBox("HOUR · 시주", chart.hour_pillar, Modifier.weight(1f))
                PillarBox("DAY · 일주", chart.day_pillar, Modifier.weight(1f))
                PillarBox("MONTH · 월주", chart.month_pillar, Modifier.weight(1f))
                PillarBox("YEAR · 연주", chart.year_pillar, Modifier.weight(1f))
            }
        }

        AstroPanelCard(
            title = "Major Cycles (대운 · ${if (chart.is_forward_daeun) "순행" else "역행"})",
            headerAction = {
                AstroStatusBadge(
                    text = "DAEUN ${chart.daeun_number}",
                    color = CelestialBlue
                )
            }
        ) {
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .horizontalScroll(rememberScrollState()),
                horizontalArrangement = Arrangement.spacedBy(6.dp)
            ) {
                chart.daeun_periods.forEach { period ->
                    Column(
                        modifier = Modifier
                            .width(56.dp)
                            .background(SurfaceLight, RoundedCornerShape(6.dp))
                            .border(1.dp, BorderColor, RoundedCornerShape(6.dp))
                            .padding(vertical = 8.dp, horizontal = 4.dp),
                        horizontalAlignment = Alignment.CenterHorizontally
                    ) {
                        Text(
                            text = "${period.start_age}",
                            fontSize = 11.sp,
                            fontWeight = FontWeight.Bold,
                            color = CelestialBlue,
                            style = TabularMonoStyle,
                            textAlign = TextAlign.Center
                        )
                        Spacer(modifier = Modifier.height(4.dp))
                        Text(
                            text = period.pillar.hanja,
                            fontSize = 16.sp,
                            fontWeight = FontWeight.Bold,
                            color = TextPrimary,
                            textAlign = TextAlign.Center
                        )
                        Text(
                            text = period.pillar.korean,
                            fontSize = 11.sp,
                            color = TextSecondary,
                            textAlign = TextAlign.Center
                        )
                    }
                }
            }
        }
    }
}

@Composable
private fun PillarBox(
    label: String,
    pillar: PillarData,
    modifier: Modifier = Modifier
) {
    Column(
        modifier = modifier
            .background(SurfaceLight, RoundedCornerShape(6.dp))
            .border(1.dp, BorderColor, RoundedCornerShape(6.dp))
            .padding(vertical = 10.dp, horizontal = 4.dp),
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        Text(
            text = label,
            fontSize = 10.sp,
            fontWeight = FontWeight.Bold,
            color = TextSecondary,
            style = TabularMonoStyle,
            textAlign = TextAlign.Center,
            maxLines = 1
        )
        Spacer(modifier = Modifier.height(8.dp))
        Text(
            text = PillarData.STEMS_HANJA[pillar.stem_index % 10],
            fontSize = 22.sp,
            fontWeight = FontWeight.Bold,
            color = TextPrimary,
            textAlign = TextAlign.Center
        )
        Text(
            text = PillarData.STEMS[pillar.stem_index % 10],
            fontSize = 11.sp,
            color = TextSecondary,
            textAlign = TextAlign.Center
        )
        HorizontalDivider(
            color = BorderColor,
            thickness = 0.5.dp,
            modifier = Modifier.padding(vertical = 6.dp, horizontal = 10.dp)
        )
        Text(
            text = PillarData.BRANCHES_HANJA[pillar.branch_index % 12],
            fontSize = 22.sp,
            fontWeight = FontWeight.Bold,
            color = TextPrimary,
            textAlign = TextAlign.Center
        )
        Text(
            text = PillarData.BRANCHES[pillar.branch_index % 12],
            fontSize = 11.sp,
            color = TextSecondary,
            textAlign = TextAlign.Center
        )
    }
}
