package com.fortunexample.note.ui.components

import android.graphics.Paint
import android.graphics.Typeface
import androidx.compose.foundation.Canvas
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.aspectRatio
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.Path
import androidx.compose.ui.graphics.drawscope.DrawScope
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.graphics.nativeCanvas
import androidx.compose.ui.graphics.toArgb
import androidx.compose.ui.unit.dp
import com.fortunexample.note.data.VargaChartData
import com.fortunexample.note.data.VedicChartData
import com.fortunexample.note.ui.theme.*

private class DiamondPaints(density: Float) {
    val rasi = createMonoPaint(ChartSubtleInk.toArgb(), 9.0f * density)
    val lagnaRasi = createMonoPaint(CelestialBlue.toArgb(), 9.5f * density)
    val planet = createMonoPaint(ChartFrameInk.toArgb(), 10.0f * density)
}

private fun createMonoPaint(argb: Int, textSizePx: Float): Paint = Paint(Paint.ANTI_ALIAS_FLAG).apply {
    color = argb
    textSize = textSizePx
    textAlign = Paint.Align.CENTER
    typeface = Typeface.create(Typeface.MONOSPACE, Typeface.BOLD)
}

@Composable
fun VedicChartDiamond(
    data: VedicChartData,
    selectedVarga: VargaChartData? = null,
    modifier: Modifier = Modifier
) {
    val activeLagnaSign = selectedVarga?.lagna_sign ?: data.lagna_sign
    val housePlanets = groupDiamondPlanets(data, selectedVarga, activeLagnaSign)

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
            drawDiamondFrame()
            drawDiamondHouses(activeLagnaSign, housePlanets, DiamondPaints(density))
        }
    }
}

private fun groupDiamondPlanets(
    data: VedicChartData,
    selectedVarga: VargaChartData?,
    activeLagnaSign: Int
): Array<MutableList<String>> {
    val grouped = Array(13) { mutableListOf<String>() }
    if (selectedVarga != null) {
        selectedVarga.placements.forEach { p ->
            grouped[p.house.coerceIn(1, 12)].add(vedicGrahaCode(p.planet_name))
        }
    } else {
        data.planets.forEach { p ->
            val bhava = if (p.house in 1..12) p.house else ((p.sign_index - activeLagnaSign + 12) % 12) + 1
            val suffix = if (p.is_retrograde && p.name != "Rahu" && p.name != "Ketu") "R" else ""
            grouped[bhava].add("${vedicGrahaCode(p.name)}$suffix")
        }
    }
    return grouped
}

private fun DrawScope.drawDiamondFrame() {
    val w = size.width
    val h = size.height
    val strokeThin = 1.1f * density
    val strokeBold = 1.6f * density
    val kendraPath = Path().apply {
        moveTo(w / 2f, 0f)
        lineTo(w, h / 2f)
        lineTo(w / 2f, h)
        lineTo(0f, h / 2f)
        close()
    }
    drawPath(path = kendraPath, color = ChartKendraFill)
    drawRect(color = ChartFrameInk, topLeft = Offset.Zero, size = Size(w, h), style = Stroke(width = strokeBold))
    drawLine(color = ChartFrameInk, start = Offset(0f, 0f), end = Offset(w, h), strokeWidth = strokeThin)
    drawLine(color = ChartFrameInk, start = Offset(w, 0f), end = Offset(0f, h), strokeWidth = strokeThin)
    drawPath(path = kendraPath, color = ChartFrameInk, style = Stroke(width = strokeBold))
}

private fun bhavaAnchorsFor(w: Float, h: Float): List<Pair<Offset, Offset>> = listOf(
    Pair(Offset.Zero, Offset.Zero),
    Pair(Offset(w * 0.50f, h * 0.11f), Offset(w * 0.50f, h * 0.27f)),
    Pair(Offset(w * 0.25f, h * 0.055f), Offset(w * 0.25f, h * 0.13f)),
    Pair(Offset(w * 0.055f, h * 0.25f), Offset(w * 0.13f, h * 0.25f)),
    Pair(Offset(w * 0.14f, h * 0.50f), Offset(w * 0.28f, h * 0.50f)),
    Pair(Offset(w * 0.055f, h * 0.75f), Offset(w * 0.13f, h * 0.75f)),
    Pair(Offset(w * 0.25f, h * 0.945f), Offset(w * 0.25f, h * 0.86f)),
    Pair(Offset(w * 0.50f, h * 0.89f), Offset(w * 0.50f, h * 0.73f)),
    Pair(Offset(w * 0.75f, h * 0.945f), Offset(w * 0.75f, h * 0.86f)),
    Pair(Offset(w * 0.945f, h * 0.75f), Offset(w * 0.87f, h * 0.75f)),
    Pair(Offset(w * 0.86f, h * 0.50f), Offset(w * 0.72f, h * 0.50f)),
    Pair(Offset(w * 0.945f, h * 0.25f), Offset(w * 0.87f, h * 0.25f)),
    Pair(Offset(w * 0.75f, h * 0.055f), Offset(w * 0.75f, h * 0.13f))
)

private fun DrawScope.drawDiamondHouses(
    activeLagnaSign: Int,
    housePlanets: Array<MutableList<String>>,
    paints: DiamondPaints
) {
    val anchors = bhavaAnchorsFor(size.width, size.height)
    val pFm = paints.planet.fontMetrics
    val pBaseline = -(pFm.ascent + pFm.descent) / 2f
    val lineHeight = 11.5f * density

    for (bhava in 1..12) {
        val signIdx = (activeLagnaSign + bhava - 1) % 12
        val (rasiPos, planetCenter) = anchors[bhava]
        val rasiLabel = if (bhava == 1) "ASC ${signIdx + 1}·${ZODIAC_CODES[signIdx]}" else "${signIdx + 1}"
        val activeRasiPaint = if (bhava == 1) paints.lagnaRasi else paints.rasi
        val rasiFm = activeRasiPaint.fontMetrics
        val rasiBaseline = -(rasiFm.ascent + rasiFm.descent) / 2f
        drawContext.canvas.nativeCanvas.drawText(rasiLabel, rasiPos.x, rasiPos.y + rasiBaseline, activeRasiPaint)

        val pList = housePlanets[bhava]
        if (pList.isNotEmpty()) {
            val perRow = if (bhava in listOf(1, 4, 7, 10)) 3 else 2
            val rows = pList.chunked(perRow)
            val startY = planetCenter.y - (rows.size - 1) * lineHeight / 2f
            rows.forEachIndexed { idx, rowItems ->
                drawContext.canvas.nativeCanvas.drawText(
                    rowItems.joinToString(" "),
                    planetCenter.x,
                    startY + idx * lineHeight + pBaseline,
                    paints.planet
                )
            }
        }
    }
}
