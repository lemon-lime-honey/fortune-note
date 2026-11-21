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
import androidx.compose.ui.graphics.PathEffect
import androidx.compose.ui.graphics.drawscope.DrawScope
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.graphics.nativeCanvas
import androidx.compose.ui.graphics.toArgb
import androidx.compose.ui.unit.dp
import com.fortunexample.note.data.WesternChartData
import com.fortunexample.note.ui.theme.*
import java.util.Locale
import kotlin.math.PI
import kotlin.math.abs
import kotlin.math.cos
import kotlin.math.sin

private data class WheelBodyItem(
    val code: String,
    val trueLon: Double,
    val degInSign: Int,
    val isRetrograde: Boolean,
    val isNode: Boolean,
    var displayLon: Double
)

private class WheelGeometry(size: Size, val d: Float, val asc: Double) {
    val center = Offset(size.width / 2f, size.height / 2f)
    val outerRadius = size.minDimension / 2f - 4f * d
    val signInnerRadius = outerRadius * 0.855f
    val tickInnerRadius = outerRadius * 0.82f
    val planetCodeRadius = outerRadius * 0.725f
    val planetDegRadius = outerRadius * 0.625f
    val houseRingRadius = outerRadius * 0.51f
    val innerAspectRadius = outerRadius * 0.42f

    fun lonToScreenDeg(lon: Double): Float {
        val rel = (lon - asc + 360.0) % 360.0
        return ((180.0 - rel + 360.0) % 360.0).toFloat()
    }

    fun lonToOffset(lon: Double, r: Float): Offset {
        val rad = lonToScreenDeg(lon) * (PI / 180.0)
        return Offset(center.x + (r * cos(rad)).toFloat(), center.y + (r * sin(rad)).toFloat())
    }
}

private class WheelPaints(d: Float) {
    val sign = makePaint(ChartFrameInk.toArgb(), 10.5f * d, Typeface.BOLD)
    val houseNum = makePaint(TextSecondary.toArgb(), 9.5f * d, Typeface.BOLD)
    val ascLabel = makePaint(CrimsonAlert.toArgb(), 9.5f * d, Typeface.BOLD)
    val mcLabel = makePaint(CelestialBlue.toArgb(), 9.5f * d, Typeface.BOLD)
    val bodyCode = makePaint(ChartFrameInk.toArgb(), 9.5f * d, Typeface.BOLD)
    val nodeCode = makePaint(TextSecondary.toArgb(), 9.0f * d, Typeface.BOLD)
    val retroBodyCode = makePaint(CrimsonAlert.toArgb(), 9.5f * d, Typeface.BOLD)
    val bodyDeg = makePaint(TextSecondary.toArgb(), 8.2f * d, Typeface.NORMAL)

    private fun makePaint(argb: Int, sizePx: Float, style: Int): Paint =
        Paint(Paint.ANTI_ALIAS_FLAG).apply {
            color = argb
            textSize = sizePx
            textAlign = Paint.Align.CENTER
            typeface = Typeface.create(Typeface.MONOSPACE, style)
        }
}

@Composable
fun WesternChartWheel(
    data: WesternChartData,
    modifier: Modifier = Modifier
) {
    Box(
        modifier = modifier
            .fillMaxWidth()
            .aspectRatio(1f)
            .background(BackgroundWhite, RoundedCornerShape(8.dp))
            .border(1.dp, BorderColor, RoundedCornerShape(8.dp))
            .padding(10.dp),
        contentAlignment = Alignment.Center
    ) {
        Canvas(modifier = Modifier.fillMaxSize()) {
            val geom = WheelGeometry(size, density, data.ascendant)
            val paints = WheelPaints(density)
            drawZodiacRingAndTicks(geom, paints)
            drawInnerRingsAndHouses(geom, data, paints)
            drawPrimaryAngleAxes(geom, data, paints)
            drawAspectWeb(geom, data)
            drawWheelBodies(geom, data, paints)
        }
    }
}

private fun DrawScope.drawZodiacRingAndTicks(geom: WheelGeometry, paints: WheelPaints) {
    for (i in 0 until 12) {
        if (i % 2 == 1) {
            drawArc(
                color = ChartKendraFill,
                startAngle = geom.lonToScreenDeg((i + 1) * 30.0),
                sweepAngle = 30f,
                useCenter = true,
                topLeft = Offset(geom.center.x - geom.outerRadius, geom.center.y - geom.outerRadius),
                size = Size(geom.outerRadius * 2f, geom.outerRadius * 2f)
            )
        }
    }
    drawCircle(color = BackgroundWhite, radius = geom.signInnerRadius, center = geom.center)
    drawCircle(color = ChartFrameInk, radius = geom.outerRadius, center = geom.center, style = Stroke(1.5f * geom.d))
    drawCircle(color = ChartFrameInk, radius = geom.signInnerRadius, center = geom.center, style = Stroke(1.2f * geom.d))

    val textRadius = (geom.outerRadius + geom.signInnerRadius) / 2f
    for (i in 0 until 12) {
        val startLon = i * 30.0
        drawLine(
            color = ChartFrameInk,
            start = geom.lonToOffset(startLon, geom.signInnerRadius),
            end = geom.lonToOffset(startLon, geom.outerRadius),
            strokeWidth = 1.0f * geom.d
        )
        drawCenteredText(ZODIAC_CODES[i], geom.lonToOffset(startLon + 15.0, textRadius), paints.sign)
    }

    for (deg in 0 until 360) {
        val isTen = deg % 10 == 0
        val isFive = deg % 5 == 0
        val len = when { isTen -> 5.5f; isFive -> 3.8f; else -> 2.0f } * geom.d
        val width = when { isTen -> 1.1f; isFive -> 0.8f; else -> 0.5f } * geom.d
        drawLine(
            color = if (isTen || isFive) ChartFrameInk else ChartMutedLine,
            start = geom.lonToOffset(deg.toDouble(), geom.signInnerRadius),
            end = geom.lonToOffset(deg.toDouble(), geom.signInnerRadius - len),
            strokeWidth = width
        )
    }
}

private fun DrawScope.drawInnerRingsAndHouses(
    geom: WheelGeometry,
    data: WesternChartData,
    paints: WheelPaints
) {
    drawCircle(color = ChartKendraFill, radius = geom.houseRingRadius, center = geom.center)
    drawCircle(color = BackgroundWhite, radius = geom.innerAspectRadius, center = geom.center)
    drawCircle(color = ChartMutedLine, radius = geom.houseRingRadius, center = geom.center, style = Stroke(1.0f * geom.d))
    drawCircle(color = ChartFrameInk, radius = geom.innerAspectRadius, center = geom.center, style = Stroke(1.1f * geom.d))

    val cusps = data.house_cusps
    if (cusps.size != 12) return
    val numRadius = (geom.houseRingRadius + geom.innerAspectRadius) / 2f
    for (i in 0 until 12) {
        val cuspLon = cusps[i]
        val nextCuspLon = cusps[(i + 1) % 12]
        if (i !in listOf(0, 3, 6, 9)) {
            drawLine(
                color = ChartMutedLine,
                start = geom.lonToOffset(cuspLon, geom.innerAspectRadius),
                end = geom.lonToOffset(cuspLon, geom.signInnerRadius),
                strokeWidth = 0.9f * geom.d
            )
        }
        val midLon = (cuspLon + ((nextCuspLon - cuspLon + 360.0) % 360.0) / 2.0) % 360.0
        drawCenteredText("${i + 1}", geom.lonToOffset(midLon, numRadius), paints.houseNum)
    }
}

private fun DrawScope.drawPrimaryAngleAxes(
    geom: WheelGeometry,
    data: WesternChartData,
    paints: WheelPaints
) {
    val asc = data.ascendant
    val dsc = (asc + 180.0) % 360.0
    val mc = data.midheaven
    val ic = (mc + 180.0) % 360.0
    drawLine(CrimsonAlert, geom.lonToOffset(asc, geom.innerAspectRadius), geom.lonToOffset(asc, geom.signInnerRadius), 2.0f * geom.d)
    drawLine(CrimsonAlert.copy(alpha = 0.65f), geom.lonToOffset(dsc, geom.innerAspectRadius), geom.lonToOffset(dsc, geom.signInnerRadius), 1.4f * geom.d)
    drawLine(CelestialBlue, geom.lonToOffset(mc, geom.innerAspectRadius), geom.lonToOffset(mc, geom.signInnerRadius), 2.0f * geom.d)
    drawLine(CelestialBlue.copy(alpha = 0.65f), geom.lonToOffset(ic, geom.innerAspectRadius), geom.lonToOffset(ic, geom.signInnerRadius), 1.4f * geom.d)

    val badgeRadius = geom.innerAspectRadius - 10f * geom.d
    drawCenteredText("ASC", geom.lonToOffset(asc, badgeRadius), paints.ascLabel)
    drawCenteredText("MC", geom.lonToOffset(mc, badgeRadius), paints.mcLabel)
}

private fun DrawScope.drawAspectWeb(geom: WheelGeometry, data: WesternChartData) {
    val bodyLonMap = buildMap {
        data.planets.forEach { put(it.name, it.longitude) }
        data.nodes.forEach { put(it.name, it.longitude) }
    }
    val rim = geom.innerAspectRadius - 2f * geom.d
    data.aspects.forEach { asp ->
        val lon1 = bodyLonMap[asp.planet1] ?: return@forEach
        val lon2 = bodyLonMap[asp.planet2] ?: return@forEach
        val lineColor = when {
            isHardAspect(asp.aspect_type) -> CrimsonAlert.copy(alpha = 0.55f)
            isSoftAspect(asp.aspect_type) -> CelestialBlue.copy(alpha = 0.55f)
            else -> TextSecondary.copy(alpha = 0.45f)
        }
        val dash = if (asp.aspect_type.equals("Sextile", ignoreCase = true)) {
            PathEffect.dashPathEffect(floatArrayOf(5f * geom.d, 4f * geom.d), 0f)
        } else null
        drawLine(
            color = lineColor,
            start = geom.lonToOffset(lon1, rim),
            end = geom.lonToOffset(lon2, rim),
            strokeWidth = if (asp.orb <= 2.0) 1.4f * geom.d else 0.9f * geom.d,
            pathEffect = dash
        )
    }
}

private fun DrawScope.drawWheelBodies(
    geom: WheelGeometry,
    data: WesternChartData,
    paints: WheelPaints
) {
    val rawBodies = buildList {
        data.planets.forEach { p ->
            val lon = (p.longitude % 360.0 + 360.0) % 360.0
            val deg = ((p.sign_degree % 30.0 + 30.0) % 30.0).toInt().coerceIn(0, 29)
            add(WheelBodyItem(bodyCode(p.name), lon, deg, p.is_retrograde, false, lon))
        }
        data.nodes.forEach { n ->
            val lon = (n.longitude % 360.0 + 360.0) % 360.0
            val deg = ((n.sign_degree % 30.0 + 30.0) % 30.0).toInt().coerceIn(0, 29)
            add(WheelBodyItem(bodyCode(n.name), lon, deg, n.is_retrograde, true, lon))
        }
    }
    resolveAngularCollisions(rawBodies, minSepDeg = 7.2).forEach { b ->
        val rimPt = geom.lonToOffset(b.trueLon, geom.signInnerRadius - 2.5f * geom.d)
        val leaderPt = geom.lonToOffset(b.displayLon, geom.tickInnerRadius - 2f * geom.d)
        drawCircle(if (b.isRetrograde && !b.isNode) CrimsonAlert else ChartFrameInk, 2.0f * geom.d, rimPt)
        drawLine(TextSecondary.copy(alpha = 0.65f), rimPt, leaderPt, 0.8f * geom.d)
        val codePaint = when {
            b.isNode -> paints.nodeCode
            b.isRetrograde -> paints.retroBodyCode
            else -> paints.bodyCode
        }
        drawCenteredText(b.code, geom.lonToOffset(b.displayLon, geom.planetCodeRadius), codePaint)
        val degLabel = String.format(Locale.US, "%02d°%s", b.degInSign, if (b.isRetrograde && !b.isNode) "R" else "")
        drawCenteredText(degLabel, geom.lonToOffset(b.displayLon, geom.planetDegRadius), paints.bodyDeg)
    }
}

private fun DrawScope.drawCenteredText(text: String, pos: Offset, paint: Paint) {
    val fm = paint.fontMetrics
    val baselineOffset = -(fm.ascent + fm.descent) / 2f
    drawContext.canvas.nativeCanvas.drawText(text, pos.x, pos.y + baselineOffset, paint)
}

private fun resolveAngularCollisions(
    items: List<WheelBodyItem>,
    minSepDeg: Double
): List<WheelBodyItem> {
    if (items.size <= 1) return items
    val sorted = items.sortedBy { it.trueLon }.toMutableList()
    val n = sorted.size
    repeat(60) {
        var moved = false
        for (i in 0 until n) {
            val a = sorted[i]
            val b = sorted[(i + 1) % n]
            var diff = (b.displayLon - a.displayLon + 360.0) % 360.0
            if (diff > 180.0) diff -= 360.0
            if (diff >= 0.0 && diff < minSepDeg) {
                val push = (minSepDeg - diff) * 0.52
                a.displayLon = (a.displayLon - push + 360.0) % 360.0
                b.displayLon = (b.displayLon + push + 360.0) % 360.0
                moved = true
            } else if (diff < 0.0 && abs(diff) < minSepDeg) {
                val mid = (a.displayLon + diff / 2.0 + 360.0) % 360.0
                a.displayLon = (mid - minSepDeg * 0.5 + 360.0) % 360.0
                b.displayLon = (mid + minSepDeg * 0.5 + 360.0) % 360.0
                moved = true
            }
        }
        for (item in sorted) {
            val delta = (item.trueLon - item.displayLon + 540.0) % 360.0 - 180.0
            if (abs(delta) > 0.2) {
                item.displayLon = (item.displayLon + delta * 0.08 + 360.0) % 360.0
            }
        }
        if (!moved) return@repeat
    }
    return sorted
}
