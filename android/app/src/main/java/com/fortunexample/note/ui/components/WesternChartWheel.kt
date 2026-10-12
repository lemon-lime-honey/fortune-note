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
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.graphics.nativeCanvas
import androidx.compose.ui.unit.dp
import com.fortunexample.note.data.WesternChartData
import com.fortunexample.note.ui.theme.*
import kotlin.math.cos
import kotlin.math.sin

private val ZODIAC_SYMBOLS = listOf("♈", "♉", "♊", "♋", "♌", "♍", "♎", "♏", "♐", "♑", "♒", "♓")

// 4원소 은은한 배경색 (불, 흙, 공기, 물)
private val ELEMENT_COLORS = listOf(
    Color(0xFFFEE2E2), // 0 Aries: Fire
    Color(0xFFDCFCE7), // 1 Taurus: Earth
    Color(0xFFFEF9C3), // 2 Gemini: Air
    Color(0xFFE0F2FE), // 3 Cancer: Water
    Color(0xFFFEE2E2), // 4 Leo: Fire
    Color(0xFFDCFCE7), // 5 Virgo: Earth
    Color(0xFFFEF9C3), // 6 Libra: Air
    Color(0xFFE0F2FE), // 7 Scorpio: Water
    Color(0xFFFEE2E2), // 8 Sagittarius: Fire
    Color(0xFFDCFCE7), // 9 Capricorn: Earth
    Color(0xFFFEF9C3), // 10 Aquarius: Air
    Color(0xFFE0F2FE)  // 11 Pisces: Water
)

@Composable
fun WesternChartWheel(
    chart: WesternChartData,
    modifier: Modifier = Modifier
) {
    val symbolPaint = remember {
        Paint().apply {
            isAntiAlias = true
            textAlign = Paint.Align.CENTER
            textSize = 30f
            typeface = Typeface.create(Typeface.DEFAULT, Typeface.BOLD)
            color = android.graphics.Color.parseColor("#0F172A")
        }
    }

    val degreePaint = remember {
        Paint().apply {
            isAntiAlias = true
            textAlign = Paint.Align.CENTER
            textSize = 18f
            typeface = Typeface.create(Typeface.DEFAULT, Typeface.NORMAL)
            color = android.graphics.Color.parseColor("#475569")
        }
    }

    val retroPaint = remember {
        Paint().apply {
            isAntiAlias = true
            textAlign = Paint.Align.LEFT
            textSize = 16f
            typeface = Typeface.create(Typeface.DEFAULT, Typeface.BOLD)
            color = android.graphics.Color.parseColor("#DC2626")
        }
    }

    val zodiacSymbolPaint = remember {
        Paint().apply {
            isAntiAlias = true
            textAlign = Paint.Align.CENTER
            textSize = 28f
            typeface = Typeface.create(Typeface.DEFAULT, Typeface.BOLD)
            color = android.graphics.Color.parseColor("#334155")
        }
    }

    val houseNumPaint = remember {
        Paint().apply {
            isAntiAlias = true
            textAlign = Paint.Align.CENTER
            textSize = 20f
            typeface = Typeface.create(Typeface.DEFAULT, Typeface.BOLD)
            color = android.graphics.Color.parseColor("#64748B")
        }
    }

    val angleLabelPaint = remember {
        Paint().apply {
            isAntiAlias = true
            textAlign = Paint.Align.CENTER
            textSize = 22f
            typeface = Typeface.create(Typeface.DEFAULT, Typeface.BOLD)
            color = android.graphics.Color.parseColor("#D97706") // Accent primary
        }
    }

    Canvas(
        modifier = modifier
            .fillMaxWidth()
            .aspectRatio(1f)
            .padding(8.dp)
    ) {
        val center = Offset(size.width / 2f, size.height / 2f)
        val outerRadius = size.minDimension / 2f * 0.96f
        val zodiacInnerRadius = outerRadius * 0.82f
        val planetBandInnerRadius = outerRadius * 0.58f
        val aspectRadius = outerRadius * 0.38f

        // -------------------------------------------------------------
        // 1. 외곽 황도 12궁 링 (Outer Zodiac Sign Ring)
        // -------------------------------------------------------------
        val zodiacDiameter = outerRadius * 2f
        val zodiacTopLeft = Offset(center.x - outerRadius, center.y - outerRadius)
        val zodiacSize = Size(zodiacDiameter, zodiacDiameter)
        val zodiacThickness = outerRadius - zodiacInnerRadius

        // 12 별자리 부채꼴 배경색 채우기
        for (signIdx in 0 until 12) {
            val signStartDeg = signIdx * 30.0
            // 각도 변환: ASC가 180도(9시 방향)에 오도록 반시계방향 배치
            // Canvas drawArc에서 startAngle은 3시 방향이 0도이고 시계방향이 양수이므로:
            // Math 각도 A는 Canvas 각도 -A에 해당
            val chartAngle = (signStartDeg - chart.ascendant + 180.0).remEuclid(360.0)
            val canvasStartAngle = (-chartAngle - 30.0).remEuclid(360.0).toFloat()

            drawArc(
                color = ELEMENT_COLORS[signIdx],
                startAngle = canvasStartAngle,
                sweepAngle = 30f,
                useCenter = false,
                topLeft = Offset(center.x - (outerRadius + zodiacInnerRadius) / 2f, center.y - (outerRadius + zodiacInnerRadius) / 2f),
                size = Size(outerRadius + zodiacInnerRadius, outerRadius + zodiacInnerRadius),
                style = Stroke(width = zodiacThickness)
            )
        }

        // 황도대 경계 원선
        drawCircle(color = BorderColor.copy(alpha = 0.8f), radius = outerRadius, center = center, style = Stroke(width = 2.5f))
        drawCircle(color = BorderColor.copy(alpha = 0.8f), radius = zodiacInnerRadius, center = center, style = Stroke(width = 2.0f))
        drawCircle(color = BorderColor.copy(alpha = 0.6f), radius = planetBandInnerRadius, center = center, style = Stroke(width = 1.5f))
        drawCircle(color = BorderColor.copy(alpha = 0.5f), radius = aspectRadius, center = center, style = Stroke(width = 1.5f))

        // 12 별자리 구획선 및 심볼 렌더링
        val zodiacMidRadius = (outerRadius + zodiacInnerRadius) / 2f
        for (signIdx in 0 until 12) {
            val signStartDeg = signIdx * 30.0
            val angleRad = Math.toRadians(signStartDeg - chart.ascendant + 180.0)

            // 별자리 경계선
            val start = Offset(
                center.x + (zodiacInnerRadius * cos(angleRad)).toFloat(),
                center.y - (zodiacInnerRadius * sin(angleRad)).toFloat()
            )
            val end = Offset(
                center.x + (outerRadius * cos(angleRad)).toFloat(),
                center.y - (outerRadius * sin(angleRad)).toFloat()
            )
            drawLine(color = BorderColor.copy(alpha = 0.9f), start = start, end = end, strokeWidth = 1.5f)

            // 별자리 심볼 (별자리 중앙 15도 지점)
            val midAngleRad = Math.toRadians(signStartDeg + 15.0 - chart.ascendant + 180.0)
            val symX = center.x + (zodiacMidRadius * cos(midAngleRad)).toFloat()
            val symY = center.y - (zodiacMidRadius * sin(midAngleRad)).toFloat() + 10f
            drawContext.canvas.nativeCanvas.drawText(ZODIAC_SYMBOLS[signIdx], symX, symY, zodiacSymbolPaint)
        }

        // -------------------------------------------------------------
        // 2. 12 하우스 커스프 분할선 및 번호
        // -------------------------------------------------------------
        val cusps = chart.house_cusps
        for (i in cusps.indices) {
            val cuspDeg = cusps[i]
            val angleRad = Math.toRadians(cuspDeg - chart.ascendant + 180.0)
            val start = Offset(
                center.x + (aspectRadius * cos(angleRad)).toFloat(),
                center.y - (aspectRadius * sin(angleRad)).toFloat()
            )
            val end = Offset(
                center.x + (zodiacInnerRadius * cos(angleRad)).toFloat(),
                center.y - (zodiacInnerRadius * sin(angleRad)).toFloat()
            )

            drawLine(
                color = TextSecondary.copy(alpha = 0.40f),
                start = start,
                end = end,
                strokeWidth = 1.2f
            )

            // 하우스 번호 (중간 스팬)
            val nextCuspDeg = cusps[(i + 1) % 12]
            var span = nextCuspDeg - cuspDeg
            if (span < 0.0) span += 360.0
            val midAngleRad = Math.toRadians(cuspDeg + span / 2.0 - chart.ascendant + 180.0)
            val numRadius = (planetBandInnerRadius + aspectRadius) / 2f
            val numX = center.x + (numRadius * cos(midAngleRad)).toFloat()
            val numY = center.y - (numRadius * sin(midAngleRad)).toFloat() + 7f
            drawContext.canvas.nativeCanvas.drawText("${i + 1}", numX, numY, houseNumPaint)
        }

        // -------------------------------------------------------------
        // 3. 독립 앵글 마커 (ASC, DSC, MC, IC)
        // -------------------------------------------------------------
        // ASC는 항상 180도 (9시 방향 좌측 수평축)
        val ascRad = Math.toRadians(180.0)
        val ascLineEnd = Offset(center.x - outerRadius, center.y)
        val ascLineStart = Offset(center.x - aspectRadius, center.y)
        drawLine(color = AccentPrimary, start = ascLineStart, end = ascLineEnd, strokeWidth = 3.2f)
        drawContext.canvas.nativeCanvas.drawText("ASC", center.x - outerRadius - 16f, center.y + 7f, angleLabelPaint)

        // DSC (3시 방향 우측 수평축)
        val dscLineEnd = Offset(center.x + outerRadius, center.y)
        val dscLineStart = Offset(center.x + aspectRadius, center.y)
        drawLine(color = AccentPrimary.copy(alpha = 0.6f), start = dscLineStart, end = dscLineEnd, strokeWidth = 2.0f)
        drawContext.canvas.nativeCanvas.drawText("DSC", center.x + outerRadius + 18f, center.y + 7f, angleLabelPaint)

        // MC (천문학적 실제 위치)
        val mcAngleRad = Math.toRadians(chart.midheaven - chart.ascendant + 180.0)
        val mcStart = Offset(
            center.x + (aspectRadius * cos(mcAngleRad)).toFloat(),
            center.y - (aspectRadius * sin(mcAngleRad)).toFloat()
        )
        val mcEnd = Offset(
            center.x + (outerRadius * cos(mcAngleRad)).toFloat(),
            center.y - (outerRadius * sin(mcAngleRad)).toFloat()
        )
        drawLine(color = CelestialBlue, start = mcStart, end = mcEnd, strokeWidth = 3.0f)
        val mcLabelPos = Offset(
            center.x + ((outerRadius + 16f) * cos(mcAngleRad)).toFloat(),
            center.y - ((outerRadius + 16f) * sin(mcAngleRad)).toFloat() + 8f
        )
        drawContext.canvas.nativeCanvas.drawText("MC", mcLabelPos.x, mcLabelPos.y, angleLabelPaint.apply { color = android.graphics.Color.parseColor("#0284C7") })

        // -------------------------------------------------------------
        // 4. 메이저 아스펙트 연결선 (정밀도에 따른 차등 투명도/두께)
        // -------------------------------------------------------------
        chart.aspects.forEach { aspect ->
            val p1 = chart.planets.find { it.name == aspect.planet1 }
            val p2 = chart.planets.find { it.name == aspect.planet2 }
            if (p1 != null && p2 != null) {
                val a1 = Math.toRadians(p1.longitude - chart.ascendant + 180.0)
                val a2 = Math.toRadians(p2.longitude - chart.ascendant + 180.0)
                val pos1 = Offset(
                    center.x + (aspectRadius * cos(a1)).toFloat(),
                    center.y - (aspectRadius * sin(a1)).toFloat()
                )
                val pos2 = Offset(
                    center.x + (aspectRadius * cos(a2)).toFloat(),
                    center.y - (aspectRadius * sin(a2)).toFloat()
                )

                val (baseColor, strokeW) = when (aspect.aspect_type) {
                    "Trine", "Sextile" -> CelestialBlue to if (aspect.orb <= 1.5) 2.2f else 1.4f
                    "Square", "Opposition" -> AstrologicalRed to if (aspect.orb <= 1.5) 2.2f else 1.4f
                    else -> SajuSlate to 1.2f
                }
                val alpha = if (aspect.orb <= 1.0) 0.90f else if (aspect.orb <= 3.0) 0.65f else 0.40f
                drawLine(color = baseColor.copy(alpha = alpha), start = pos1, end = pos2, strokeWidth = strokeW)
            }
        }

        // -------------------------------------------------------------
        // 5. 행성 및 노드 심볼과 도수 표시 (3단계 밀집 분산 배치 알고리즘)
        // -------------------------------------------------------------
        data class BodyRenderItem(
            val name: String,
            val symbol: String,
            val longitude: Double,
            val signDegree: Double,
            val isRetrograde: Boolean
        )

        val items = (chart.planets.map {
            BodyRenderItem(it.name, getPlanetSymbol(it.name), it.longitude, it.sign_degree, it.is_retrograde)
        } + chart.nodes.map {
            BodyRenderItem(it.name, getPlanetSymbol(it.name), it.longitude, it.sign_degree, it.is_retrograde)
        }).sortedBy { it.longitude }

        val planetBandMid = (zodiacInnerRadius + planetBandInnerRadius) / 2f
        val radialOffsets = FloatArray(items.size) { 0f }

        // 스텔리움 대응 3단계 방사상 오프셋 분산 (-20f, 0f, +20f)
        var level = 0
        for (i in items.indices) {
            val curr = items[i]
            val prev = if (i > 0) items[i - 1] else items.last()
            var diff = (curr.longitude - prev.longitude).remEuclid(360.0)
            if (diff < 7.0 && i > 0) {
                level = (level + 1) % 3
                radialOffsets[i] = when (level) {
                    1 -> 20f
                    2 -> -20f
                    else -> 0f
                }
            } else {
                level = 0
                radialOffsets[i] = 0f
            }
        }

        items.forEachIndexed { idx, body ->
            val angleRad = Math.toRadians(body.longitude - chart.ascendant + 180.0)
            val dotRadius = planetBandInnerRadius + 4f
            val dotPos = Offset(
                center.x + (dotRadius * cos(angleRad)).toFloat(),
                center.y - (dotRadius * sin(angleRad)).toFloat()
            )
            drawCircle(color = AccentPrimary, radius = 3.5f, center = dotPos)

            val textRadius = planetBandMid + radialOffsets[idx]
            val textX = center.x + (textRadius * cos(angleRad)).toFloat()
            val textY = center.y - (textRadius * sin(angleRad)).toFloat() + 4f

            // 행성 심볼
            drawContext.canvas.nativeCanvas.drawText(body.symbol, textX, textY, symbolPaint)

            // 도수 텍스트 (심볼 아래 작게 표시: 예: 17°)
            val degText = "${body.signDegree.toInt()}°"
            drawContext.canvas.nativeCanvas.drawText(degText, textX, textY + 16f, degreePaint)

            if (body.isRetrograde) {
                drawContext.canvas.nativeCanvas.drawText("℞", textX + 14f, textY - 12f, retroPaint)
            }
        }
    }
}

private fun Double.remEuclid(mod: Double): Double {
    val r = this % mod
    return if (r < 0.0) r + mod else r
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
