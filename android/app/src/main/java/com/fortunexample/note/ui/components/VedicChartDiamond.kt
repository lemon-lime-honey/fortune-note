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
import com.fortunexample.note.data.VedicChartData
import com.fortunexample.note.ui.theme.*

@Composable
fun VedicChartDiamond(
    chart: VedicChartData,
    modifier: Modifier = Modifier
) {
    val planetPaint = remember {
        Paint().apply {
            isAntiAlias = true
            textAlign = Paint.Align.CENTER
            textSize = 26f
            typeface = Typeface.create(Typeface.DEFAULT, Typeface.BOLD)
            color = android.graphics.Color.parseColor("#0F172A")
        }
    }

    val signNumPaint = remember {
        Paint().apply {
            isAntiAlias = true
            textAlign = Paint.Align.CENTER
            textSize = 20f
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
        val w = size.width
        val h = size.height

        // 1. 외곽 사각형
        drawRect(
            color = BorderColor,
            topLeft = Offset.Zero,
            size = size,
            style = Stroke(width = 2.5f)
        )

        // 2. 대각선 (X자)
        drawLine(color = BorderColor, start = Offset(0f, 0f), end = Offset(w, h), strokeWidth = 2f)
        drawLine(color = BorderColor, start = Offset(w, 0f), end = Offset(0f, h), strokeWidth = 2f)

        // 3. 내부 마름모 (다이아몬드)
        val topMid = Offset(w / 2f, 0f)
        val rightMid = Offset(w, h / 2f)
        val bottomMid = Offset(w / 2f, h)
        val leftMid = Offset(0f, h / 2f)

        drawLine(color = AccentPrimary, start = topMid, end = rightMid, strokeWidth = 2.5f)
        drawLine(color = AccentPrimary, start = rightMid, end = bottomMid, strokeWidth = 2.5f)
        drawLine(color = AccentPrimary, start = bottomMid, end = leftMid, strokeWidth = 2.5f)
        drawLine(color = AccentPrimary, start = leftMid, end = topMid, strokeWidth = 2.5f)

        // 4. 12개 하우스 영역 중심점 정의 (북인도 차트 표준)
        val houseCenters = listOf(
            Offset(w * 0.50f, h * 0.24f),  // H1 (Top Diamond)
            Offset(w * 0.25f, h * 0.13f),  // H2
            Offset(w * 0.13f, h * 0.25f),  // H3
            Offset(w * 0.24f, h * 0.50f),  // H4 (Left Diamond)
            Offset(w * 0.13f, h * 0.75f),  // H5
            Offset(w * 0.25f, h * 0.87f),  // H6
            Offset(w * 0.50f, h * 0.76f),  // H7 (Bottom Diamond)
            Offset(w * 0.75f, h * 0.87f),  // H8
            Offset(w * 0.87f, h * 0.75f),  // H9
            Offset(w * 0.76f, h * 0.50f),  // H10 (Right Diamond)
            Offset(w * 0.87f, h * 0.25f),  // H11
            Offset(w * 0.75f, h * 0.13f)   // H12
        )

        // 5. 각 하우스별 행성 배치 맵 구성
        val housePlanets = Array(12) { mutableListOf<String>() }
        housePlanets[0].add("As") // H1 always contains Lagna Ascendant

        chart.planets.forEach { p ->
            val hIdx = ((p.sign_index - chart.lagna_sign + 12) % 12)
            val abbr = when (p.name) {
                "Sun" -> "Su"
                "Moon" -> "Mo"
                "Mercury" -> "Me"
                "Venus" -> "Ve"
                "Mars" -> "Ma"
                "Jupiter" -> "Ju"
                "Saturn" -> "Sa"
                "Rahu" -> "Ra"
                "Ketu" -> "Ke"
                else -> p.name.take(2)
            }
            val label = if (p.is_retrograde) "$abbrⓇ" else abbr
            housePlanets[hIdx].add(label)
        }

        // 6. 각 하우스에 Sign 번호 및 행성 렌더링
        for (i in 0..11) {
            val center = houseCenters[i]
            // Rasi Sign Number (1 = Aries, 2 = Taurus ...)
            val signNum = ((chart.lagna_sign + i) % 12) + 1
            drawContext.canvas.nativeCanvas.drawText(
                "$signNum",
                center.x,
                center.y - 18f,
                signNumPaint
            )

            // 행성 목록
            val planetsInHouse = housePlanets[i]
            if (planetsInHouse.isNotEmpty()) {
                val line1 = planetsInHouse.take(3).joinToString(" ")
                drawContext.canvas.nativeCanvas.drawText(
                    line1,
                    center.x,
                    center.y + 12f,
                    planetPaint
                )
                if (planetsInHouse.size > 3) {
                    val line2 = planetsInHouse.drop(3).joinToString(" ")
                    drawContext.canvas.nativeCanvas.drawText(
                        line2,
                        center.x,
                        center.y + 36f,
                        planetPaint
                    )
                }
            }
        }
    }
}
