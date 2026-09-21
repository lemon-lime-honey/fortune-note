package com.fortunexample.note.ui.components

import android.graphics.Paint
import android.graphics.Typeface
import androidx.compose.foundation.Canvas
import androidx.compose.foundation.layout.aspectRatio
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.graphics.nativeCanvas
import androidx.compose.ui.graphics.toArgb
import androidx.compose.ui.unit.dp
import com.fortunexample.note.data.WesternChartData
import com.fortunexample.note.ui.theme.*
import kotlin.math.cos
import kotlin.math.sin

@Composable
fun WesternChartWheel(
    chart: WesternChartData,
    modifier: Modifier = Modifier
) {
    val textPaint = remember {
        Paint().apply {
            isAntiAlias = true
            textAlign = Paint.Align.CENTER
            textSize = 28f
            typeface = Typeface.create(Typeface.DEFAULT, Typeface.BOLD)
            color = android.graphics.Color.parseColor("#0F172A")
        }
    }

    val subTextPaint = remember {
        Paint().apply {
            isAntiAlias = true
            textAlign = Paint.Align.CENTER
            textSize = 22f
            color = android.graphics.Color.parseColor("#475569")
        }
    }

    val houseNumPaint = remember {
        Paint().apply {
            isAntiAlias = true
            textAlign = Paint.Align.CENTER
            textSize = 22f
            typeface = Typeface.create(Typeface.DEFAULT, Typeface.BOLD)
            color = android.graphics.Color.parseColor("#64748B")
        }
    }

    Canvas(
        modifier = modifier
            .fillMaxWidth()
            .aspectRatio(1f)
            .padding(12.dp)
    ) {
        val center = Offset(size.width / 2f, size.height / 2f)
        val outerRadius = size.minDimension / 2f * 0.95f
        val innerRadius = outerRadius * 0.72f
        val aspectRadius = outerRadius * 0.48f

        // 1. 외곽 원 및 하우스 밴드
        drawCircle(color = BorderColor, radius = outerRadius, center = center, style = Stroke(width = 2.5f))
        drawCircle(color = BorderColor, radius = innerRadius, center = center, style = Stroke(width = 2f))
        drawCircle(color = BorderColor.copy(alpha = 0.6f), radius = aspectRadius, center = center, style = Stroke(width = 1.2f))

        // 2. 12 하우스 커스프 분할선 및 하우스 번호
        val cusps = chart.house_cusps
        for (i in cusps.indices) {
            val cuspDeg = cusps[i]
            val angleRad = Math.toRadians(cuspDeg - chart.ascendant + 180.0)
            val start = Offset(
                center.x + (aspectRadius * cos(angleRad)).toFloat(),
                center.y + (aspectRadius * sin(angleRad)).toFloat()
            )
            val end = Offset(
                center.x + (outerRadius * cos(angleRad)).toFloat(),
                center.y + (outerRadius * sin(angleRad)).toFloat()
            )
            val isAxis = (i == 0 || i == 9) // Ascendant (1H) or Midheaven (10H)
            val strokeColor = if (isAxis) AccentPrimary else BorderColor
            drawLine(
                color = strokeColor,
                start = start,
                end = end,
                strokeWidth = if (isAxis) 3f else 1.5f
            )

            // 하우스 번호 표시 (커스프 중간 위치)
            val nextCuspDeg = cusps[(i + 1) % 12]
            var span = nextCuspDeg - cuspDeg
            if (span < 0.0) span += 360.0
            val midAngleRad = Math.toRadians(cuspDeg + span / 2.0 - chart.ascendant + 180.0)
            val numRadius = (innerRadius + aspectRadius) / 2f
            val numX = center.x + (numRadius * cos(midAngleRad)).toFloat()
            val numY = center.y + (numRadius * sin(midAngleRad)).toFloat() + 8f
            drawContext.canvas.nativeCanvas.drawText("${i + 1}", numX, numY, houseNumPaint)
        }

        // 3. 메이저 아스펙트 연결선 (중심부)
        chart.aspects.forEach { aspect ->
            val p1 = chart.planets.find { it.name == aspect.planet1 }
            val p2 = chart.planets.find { it.name == aspect.planet2 }
            if (p1 != null && p2 != null) {
                val a1 = Math.toRadians(p1.longitude - chart.ascendant + 180.0)
                val a2 = Math.toRadians(p2.longitude - chart.ascendant + 180.0)
                val pos1 = Offset(
                    center.x + (aspectRadius * 0.95f * cos(a1)).toFloat(),
                    center.y + (aspectRadius * 0.95f * sin(a1)).toFloat()
                )
                val pos2 = Offset(
                    center.x + (aspectRadius * 0.95f * cos(a2)).toFloat(),
                    center.y + (aspectRadius * 0.95f * sin(a2)).toFloat()
                )

                val color = when (aspect.aspect_type) {
                    "Trine", "Sextile" -> CelestialBlue.copy(alpha = 0.7f)
                    "Square", "Opposition" -> AstrologicalRed.copy(alpha = 0.7f)
                    else -> SajuSlate.copy(alpha = 0.6f)
                }
                drawLine(color = color, start = pos1, end = pos2, strokeWidth = 1.5f)
            }
        }

        // 4. 행성 및 노드 표시 (점 + 표준 약어 텍스트)
        val allBodies = chart.planets.map {
            Triple(it.name, it.longitude, it.is_retrograde)
        } + chart.nodes.map {
            Triple(it.name, it.longitude, it.is_retrograde)
        }

        allBodies.forEach { (name, longitude, isRetro) ->
            val angleRad = Math.toRadians(longitude - chart.ascendant + 180.0)
            val dotPos = Offset(
                center.x + (innerRadius * cos(angleRad)).toFloat(),
                center.y + (innerRadius * sin(angleRad)).toFloat()
            )
            drawCircle(color = AccentPrimary, radius = 4f, center = dotPos)

            val textRadius = (outerRadius + innerRadius) / 2f
            val textX = center.x + (textRadius * cos(angleRad)).toFloat()
            val textY = center.y + (textRadius * sin(angleRad)).toFloat() + 8f

            val abbr = when (name) {
                "Sun" -> "Sun"
                "Moon" -> "Moon"
                "Mercury" -> "Mer"
                "Venus" -> "Ven"
                "Mars" -> "Mar"
                "Jupiter" -> "Jup"
                "Saturn" -> "Sat"
                "North Node" -> "NN"
                "South Node" -> "SN"
                else -> name.take(3)
            }
            val label = if (isRetro) "$abbrⓇ" else abbr
            drawContext.canvas.nativeCanvas.drawText(label, textX, textY, textPaint)
        }
    }
}
