package com.fortunexample.note.ui.components

import androidx.compose.foundation.Canvas
import androidx.compose.foundation.layout.aspectRatio
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.unit.dp
import com.fortunexample.note.data.WesternChartData
import com.fortunexample.note.ui.theme.AccentGold
import com.fortunexample.note.ui.theme.AstrologicalRed
import com.fortunexample.note.ui.theme.BorderColor
import com.fortunexample.note.ui.theme.CelestialBlue
import kotlin.math.cos
import kotlin.math.sin

@Composable
fun WesternChartWheel(
    chart: WesternChartData,
    modifier: Modifier = Modifier
) {
    Canvas(
        modifier = modifier
            .fillMaxWidth()
            .aspectRatio(1f)
            .padding(16.dp)
    ) {
        val center = Offset(size.width / 2f, size.height / 2f)
        val outerRadius = size.minDimension / 2f * 0.95f
        val innerRadius = outerRadius * 0.75f
        val aspectRadius = outerRadius * 0.55f

        // 1. 외곽 원 및 하우스 대역 원
        drawCircle(color = BorderColor, radius = outerRadius, center = center, style = Stroke(width = 2f))
        drawCircle(color = BorderColor, radius = innerRadius, center = center, style = Stroke(width = 1.5f))
        drawCircle(color = BorderColor.copy(alpha = 0.5f), radius = aspectRadius, center = center, style = Stroke(width = 1f))

        // 2. 12 하우스 커스프 분할선
        chart.house_cusps.forEachIndexed { index, cuspDeg ->
            val angleRad = Math.toRadians(cuspDeg - chart.ascendant + 180.0)
            val start = Offset(
                center.x + (innerRadius * cos(angleRad)).toFloat(),
                center.y + (innerRadius * sin(angleRad)).toFloat()
            )
            val end = Offset(
                center.x + (outerRadius * cos(angleRad)).toFloat(),
                center.y + (outerRadius * sin(angleRad)).toFloat()
            )
            val strokeColor = if (index == 0 || index == 9) AccentGold else BorderColor
            drawLine(color = strokeColor, start = start, end = end, strokeWidth = if (index == 0 || index == 9) 3f else 1.5f)
        }

        // 3. 메이저 아스펙트 연결선 (중심부)
        chart.aspects.forEach { aspect ->
            val p1 = chart.planets.find { it.name == aspect.planet1 }
            val p2 = chart.planets.find { it.name == aspect.planet2 }
            if (p1 != null && p2 != null) {
                val a1 = Math.toRadians(p1.longitude - chart.ascendant + 180.0)
                val a2 = Math.toRadians(p2.longitude - chart.ascendant + 180.0)
                val pos1 = Offset(center.x + (aspectRadius * 0.9f * cos(a1)).toFloat(), center.y + (aspectRadius * 0.9f * sin(a1)).toFloat())
                val pos2 = Offset(center.x + (aspectRadius * 0.9f * cos(a2)).toFloat(), center.y + (aspectRadius * 0.9f * sin(a2)).toFloat())

                val color = when (aspect.aspect_type) {
                    "Trine", "Sextile" -> CelestialBlue.copy(alpha = 0.6f)
                    "Square", "Opposition" -> AstrologicalRed.copy(alpha = 0.6f)
                    else -> AccentGold.copy(alpha = 0.5f)
                }
                drawLine(color = color, start = pos1, end = pos2, strokeWidth = 1.2f)
            }
        }

        // 4. 행성 위치 점 표시
        chart.planets.forEach { p ->
            val angleRad = Math.toRadians(p.longitude - chart.ascendant + 180.0)
            val pos = Offset(
                center.x + (innerRadius * 0.85f * cos(angleRad)).toFloat(),
                center.y + (innerRadius * 0.85f * sin(angleRad)).toFloat()
            )
            drawCircle(color = AccentGold, radius = 5f, center = pos)
        }
    }
}
