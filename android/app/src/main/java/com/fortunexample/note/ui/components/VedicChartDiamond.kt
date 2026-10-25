package com.fortunexample.note.ui.components

import android.graphics.Paint
import android.graphics.Typeface
import androidx.compose.foundation.Canvas
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.graphics.nativeCanvas
import androidx.compose.ui.graphics.toArgb
import androidx.compose.ui.unit.dp
import com.fortunexample.note.data.VargaChartData
import com.fortunexample.note.data.VedicChartData
import com.fortunexample.note.ui.theme.*

@Composable
fun VedicChartDiamond(
    data: VedicChartData,
    selectedVarga: VargaChartData? = null,
    modifier: Modifier = Modifier
) {
    val activeLagnaSign = selectedVarga?.lagna_sign ?: data.lagna_sign

    Box(
        modifier = modifier
            .fillMaxWidth()
            .aspectRatio(1f)
            .background(BackgroundWhite, RoundedCornerShape(8.dp))
            .border(1.dp, BorderColor, RoundedCornerShape(8.dp))
            .padding(12.dp),
        contentAlignment = Alignment.Center
    ) {
        Canvas(modifier = Modifier.fillMaxSize()) {
            val w = size.width
            val h = size.height
            val d = density

            val frameInk = Color(0xFF0F172A)
            val subInk = Color(0xFF64748B)
            val lagnaInk = CelestialBlue
            val strokeThin = 1.1f * d
            val strokeBold = 1.6f * d

            // Subtle background fill for Kendra (houses 1, 4, 7, 10) central diamond
            val kendraPath = androidx.compose.ui.graphics.Path().apply {
                moveTo(w / 2f, 0f)
                lineTo(w, h / 2f)
                lineTo(w / 2f, h)
                lineTo(0f, h / 2f)
                close()
            }
            drawPath(
                path = kendraPath,
                color = Color(0xFFF8FAFC)
            )

            // Outer Square
            drawRect(
                color = frameInk,
                topLeft = Offset.Zero,
                size = Size(w, h),
                style = Stroke(width = strokeBold)
            )

            // Diagonal X lines
            drawLine(
                color = frameInk,
                start = Offset(0f, 0f),
                end = Offset(w, h),
                strokeWidth = strokeThin
            )
            drawLine(
                color = frameInk,
                start = Offset(w, 0f),
                end = Offset(0f, h),
                strokeWidth = strokeThin
            )

            // Inner Diamond
            drawPath(
                path = kendraPath,
                color = frameInk,
                style = Stroke(width = strokeBold)
            )

            val rasiPaint = Paint(Paint.ANTI_ALIAS_FLAG).apply {
                color = subInk.toArgb()
                textSize = 9.0f * d
                textAlign = Paint.Align.CENTER
                typeface = Typeface.create(Typeface.MONOSPACE, Typeface.BOLD)
            }

            val lagnaRasiPaint = Paint(Paint.ANTI_ALIAS_FLAG).apply {
                color = lagnaInk.toArgb()
                textSize = 9.5f * d
                textAlign = Paint.Align.CENTER
                typeface = Typeface.create(Typeface.MONOSPACE, Typeface.BOLD)
            }

            val planetPaint = Paint(Paint.ANTI_ALIAS_FLAG).apply {
                color = frameInk.toArgb()
                textSize = 10.0f * d
                textAlign = Paint.Align.CENTER
                typeface = Typeface.create(Typeface.MONOSPACE, Typeface.BOLD)
            }

            // Group planet codes by house (1..12)
            val housePlanets = Array(13) { mutableListOf<String>() }

            if (selectedVarga != null) {
                selectedVarga.placements.forEach { p ->
                    val bhava = p.house.coerceIn(1, 12)
                    housePlanets[bhava].add(vedicGrahaCode(p.planet_name))
                }
            } else {
                data.planets.forEach { p ->
                    val bhava = if (p.house in 1..12) p.house else ((p.sign_index - activeLagnaSign + 12) % 12) + 1
                    val suffix = if (p.is_retrograde && p.name != "Rahu" && p.name != "Ketu") "R" else ""
                    housePlanets[bhava].add("${vedicGrahaCode(p.name)}$suffix")
                }
            }

            // Centroids for the 12 Bhavas in North Indian Diamond Chart
            // Pair(rasiLabelOffset, planetBlockCenterOffset)
            val bhavaAnchors = listOf(
                Pair(Offset(0f, 0f), Offset(0f, 0f)), // 0 unused
                Pair(Offset(w * 0.50f, h * 0.11f), Offset(w * 0.50f, h * 0.27f)), // H1 (Top Kendra)
                Pair(Offset(w * 0.25f, h * 0.055f), Offset(w * 0.25f, h * 0.13f)), // H2 (Top-Left Triangle)
                Pair(Offset(w * 0.055f, h * 0.25f), Offset(w * 0.13f, h * 0.25f)), // H3 (Left-Top Triangle)
                Pair(Offset(w * 0.14f, h * 0.50f), Offset(w * 0.28f, h * 0.50f)), // H4 (Left Kendra)
                Pair(Offset(w * 0.055f, h * 0.75f), Offset(w * 0.13f, h * 0.75f)), // H5 (Left-Bottom Triangle)
                Pair(Offset(w * 0.25f, h * 0.945f), Offset(w * 0.25f, h * 0.86f)), // H6 (Bottom-Left Triangle)
                Pair(Offset(w * 0.50f, h * 0.89f), Offset(w * 0.50f, h * 0.73f)), // H7 (Bottom Kendra)
                Pair(Offset(w * 0.75f, h * 0.945f), Offset(w * 0.75f, h * 0.86f)), // H8 (Bottom-Right Triangle)
                Pair(Offset(w * 0.945f, h * 0.75f), Offset(w * 0.87f, h * 0.75f)), // H9 (Right-Bottom Triangle)
                Pair(Offset(w * 0.86f, h * 0.50f), Offset(w * 0.72f, h * 0.50f)), // H10 (Right Kendra)
                Pair(Offset(w * 0.945f, h * 0.25f), Offset(w * 0.87f, h * 0.25f)), // H11 (Right-Top Triangle)
                Pair(Offset(w * 0.75f, h * 0.055f), Offset(w * 0.75f, h * 0.13f))  // H12 (Top-Right Triangle)
            )

            for (bhava in 1..12) {
                val signIdx = (activeLagnaSign + bhava - 1) % 12
                val signNum = signIdx + 1
                val signShort = ZODIAC_CODES[signIdx]
                val (rasiPos, planetCenter) = bhavaAnchors[bhava]

                // Draw Rasi Number (and ASC tag in House 1)
                val rasiLabel = if (bhava == 1) "ASC $signNum·$signShort" else "$signNum"
                val activeRasiPaint = if (bhava == 1) lagnaRasiPaint else rasiPaint
                val rasiFm = activeRasiPaint.fontMetrics
                val rasiBaseline = -(rasiFm.ascent + rasiFm.descent) / 2f
                drawContext.canvas.nativeCanvas.drawText(
                    rasiLabel,
                    rasiPos.x,
                    rasiPos.y + rasiBaseline,
                    activeRasiPaint
                )

                // Draw Planets in compact rows (max 2 or 3 per line so they never cross diagonal borders)
                val pList = housePlanets[bhava]
                if (pList.isNotEmpty()) {
                    val isKendra = (bhava == 1 || bhava == 4 || bhava == 7 || bhava == 10)
                    val perRow = if (isKendra) 3 else 2
                    val rows = pList.chunked(perRow)
                    val lineHeight = 11.5f * d
                    val totalHeight = (rows.size - 1) * lineHeight
                    val startY = planetCenter.y - totalHeight / 2f

                    val pFm = planetPaint.fontMetrics
                    val pBaseline = -(pFm.ascent + pFm.descent) / 2f

                    rows.forEachIndexed { idx, rowItems ->
                        val lineText = rowItems.joinToString(" ")
                        drawContext.canvas.nativeCanvas.drawText(
                            lineText,
                            planetCenter.x,
                            startY + idx * lineHeight + pBaseline,
                            planetPaint
                        )
                    }
                }
            }
        }
    }
}
