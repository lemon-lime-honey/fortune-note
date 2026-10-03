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
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.graphics.nativeCanvas
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
    val symbolPaint = remember {
        Paint().apply {
            isAntiAlias = true
            textAlign = Paint.Align.CENTER
            textSize = 34f
            typeface = Typeface.create(Typeface.DEFAULT, Typeface.BOLD)
            color = android.graphics.Color.parseColor("#0F172A")
        }
    }

    val retroPaint = remember {
        Paint().apply {
            isAntiAlias = true
            textAlign = Paint.Align.LEFT
            textSize = 18f
            typeface = Typeface.create(Typeface.DEFAULT, Typeface.BOLD)
            color = android.graphics.Color.parseColor("#DC2626")
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
        val innerRadius = outerRadius * 0.70f
        val aspectRadius = outerRadius * 0.45f

        // 1. 외곽 원 및 하우스 대역 (안쪽 원 및 링 선명화)
        drawCircle(color = TextSecondary.copy(alpha = 0.85f), radius = outerRadius, center = center, style = Stroke(width = 3.5f))
        drawCircle(color = TextSecondary.copy(alpha = 0.75f), radius = innerRadius, center = center, style = Stroke(width = 2.5f))
        drawCircle(color = TextSecondary.copy(alpha = 0.70f), radius = aspectRadius, center = center, style = Stroke(width = 2.5f))

        // 2. 12 하우스 커스프 분할선 및 번호
        val cusps = chart.house_cusps
        for (i in cusps.indices) {
            val cuspDeg = cusps[i]
            val angleRad = Math.toRadians(cuspDeg - chart.ascendant + 180.0)
            val start = Offset(
                center.x + (aspectRadius * cos(angleRad)).toFloat(),
                center.y - (aspectRadius * sin(angleRad)).toFloat()
            )
            val end = Offset(
                center.x + (outerRadius * cos(angleRad)).toFloat(),
                center.y - (outerRadius * sin(angleRad)).toFloat()
            )
            val isAxis = (i == 0 || i == 9)
            val strokeColor = if (isAxis) AccentPrimary else TextSecondary.copy(alpha = 0.45f)
            drawLine(
                color = strokeColor,
                start = start,
                end = end,
                strokeWidth = if (isAxis) 3.5f else 1.8f
            )

            val nextCuspDeg = cusps[(i + 1) % 12]
            var span = nextCuspDeg - cuspDeg
            if (span < 0.0) span += 360.0
            val midAngleRad = Math.toRadians(cuspDeg + span / 2.0 - chart.ascendant + 180.0)
            val numRadius = (innerRadius + aspectRadius) / 2f
            val numX = center.x + (numRadius * cos(midAngleRad)).toFloat()
            val numY = center.y - (numRadius * sin(midAngleRad)).toFloat() + 8f
            drawContext.canvas.nativeCanvas.drawText("${i + 1}", numX, numY, houseNumPaint)
        }

        // 3. 메이저 아스펙트 연결선
        chart.aspects.forEach { aspect ->
            val p1 = chart.planets.find { it.name == aspect.planet1 }
            val p2 = chart.planets.find { it.name == aspect.planet2 }
            if (p1 != null && p2 != null) {
                val a1 = Math.toRadians(p1.longitude - chart.ascendant + 180.0)
                val a2 = Math.toRadians(p2.longitude - chart.ascendant + 180.0)
                val pos1 = Offset(
                    center.x + (aspectRadius * 0.95f * cos(a1)).toFloat(),
                    center.y - (aspectRadius * 0.95f * sin(a1)).toFloat()
                )
                val pos2 = Offset(
                    center.x + (aspectRadius * 0.95f * cos(a2)).toFloat(),
                    center.y - (aspectRadius * 0.95f * sin(a2)).toFloat()
                )

                val color = when (aspect.aspect_type) {
                    "Trine", "Sextile" -> CelestialBlue.copy(alpha = 0.8f)
                    "Square", "Opposition" -> AstrologicalRed.copy(alpha = 0.8f)
                    else -> SajuSlate.copy(alpha = 0.6f)
                }
                drawLine(color = color, start = pos1, end = pos2, strokeWidth = 1.8f)
            }
        }

        // 4. 행성 및 노드 유니코드 기호 표시 (겹침 방지 방사상 오프셋 알고리즘 적용)
        data class BodyRenderItem(
            val name: String,
            val symbol: String,
            val longitude: Double,
            val isRetrograde: Boolean
        )

        val items = (chart.planets.map {
            BodyRenderItem(it.name, getPlanetSymbol(it.name), it.longitude, it.is_retrograde)
        } + chart.nodes.map {
            BodyRenderItem(it.name, getPlanetSymbol(it.name), it.longitude, it.is_retrograde)
        }).sortedBy { it.longitude }

        val baseRadius = (outerRadius + innerRadius) / 2f
        val radialOffsets = FloatArray(items.size) { 0f }

        // 근접 행성(각도차 8도 이내) 지그재그 분산 오프셋 계산
        for (i in items.indices) {
            val curr = items[i]
            val prev = if (i > 0) items[i - 1] else items.last()
            var diff = (curr.longitude - prev.longitude).let { if (it < 0.0) it + 360.0 else it }
            if (diff < 8.0) {
                // 이전 항목과 엇갈리게 레벨 변경 (-18f, +18f, 0f)
                val prevOffset = if (i > 0) radialOffsets[i - 1] else 0f
                radialOffsets[i] = if (prevOffset <= 0f) 18f else -18f
            }
        }

        items.forEachIndexed { idx, body ->
            val angleRad = Math.toRadians(body.longitude - chart.ascendant + 180.0)
            val dotPos = Offset(
                center.x + (innerRadius * cos(angleRad)).toFloat(),
                center.y - (innerRadius * sin(angleRad)).toFloat()
            )
            drawCircle(color = AccentPrimary, radius = 4f, center = dotPos)

            val textRadius = baseRadius + radialOffsets[idx]
            val textX = center.x + (textRadius * cos(angleRad)).toFloat()
            val textY = center.y - (textRadius * sin(angleRad)).toFloat() + 10f

            drawContext.canvas.nativeCanvas.drawText(body.symbol, textX, textY, symbolPaint)
            if (body.isRetrograde) {
                drawContext.canvas.nativeCanvas.drawText("℞", textX + 14f, textY - 14f, retroPaint)
            }
        }
    }
}

private fun getPlanetSymbol(name: String): String {
    return when (name) {
        "Sun" -> "☉"
        "Moon" -> "☽"
        "Mercury" -> "☿"
        "Venus" -> "♀"
        "Mars" -> "♂"
        "Jupiter" -> "♃"
        "Saturn" -> "♄"
        "North Node" -> "☊"
        "South Node" -> "☋"
        else -> name.take(2)
    }
}
