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
            val canvasSize = size.minDimension
            val center = Offset(size.width / 2f, size.height / 2f)
            val d = density

            // Radii proportional to canvas size
            val outerRadius = canvasSize / 2f - 4f * d
            val signInnerRadius = outerRadius * 0.855f
            val tickInnerRadius = outerRadius * 0.82f
            val planetCodeRadius = outerRadius * 0.725f
            val planetDegRadius = outerRadius * 0.625f
            val houseRingRadius = outerRadius * 0.51f
            val innerAspectRadius = outerRadius * 0.42f

            // Colors
            val frameInk = Color(0xFF0F172A)
            val subtleInk = Color(0xFF475569)
            val mutedLine = Color(0xFFCBD5E1)
            val faintFill = Color(0xFFF8FAFC)
            val ascColor = CrimsonAlert // #B91C1C
            val mcColor = CelestialBlue // #1D4ED8

            val asc = data.ascendant

            // Coordinate mapping: ASC is fixed at 9 o'clock (180° in Android Canvas where 0° is 3 o'clock).
            // Zodiac longitude increases counter-clockwise on the wheel.
            fun lonToScreenDeg(lon: Double): Float {
                val rel = (lon - asc + 360.0) % 360.0
                return ((180.0 - rel + 360.0) % 360.0).toFloat()
            }

            fun lonToOffset(lon: Double, r: Float): Offset {
                val deg = lonToScreenDeg(lon)
                val rad = deg * (PI / 180.0)
                return Offset(
                    x = center.x + (r * cos(rad)).toFloat(),
                    y = center.y + (r * sin(rad)).toFloat()
                )
            }

            // Native Paints (isolated instances, zero in-place color mutation)
            val signPaint = Paint(Paint.ANTI_ALIAS_FLAG).apply {
                color = frameInk.toArgb()
                textSize = 10.5f * d
                textAlign = Paint.Align.CENTER
                typeface = Typeface.create(Typeface.MONOSPACE, Typeface.BOLD)
            }

            val houseNumPaint = Paint(Paint.ANTI_ALIAS_FLAG).apply {
                color = subtleInk.toArgb()
                textSize = 9.5f * d
                textAlign = Paint.Align.CENTER
                typeface = Typeface.create(Typeface.MONOSPACE, Typeface.BOLD)
            }

            val ascLabelPaint = Paint(Paint.ANTI_ALIAS_FLAG).apply {
                color = ascColor.toArgb()
                textSize = 9.5f * d
                textAlign = Paint.Align.CENTER
                typeface = Typeface.create(Typeface.MONOSPACE, Typeface.BOLD)
            }

            val mcLabelPaint = Paint(Paint.ANTI_ALIAS_FLAG).apply {
                color = mcColor.toArgb()
                textSize = 9.5f * d
                textAlign = Paint.Align.CENTER
                typeface = Typeface.create(Typeface.MONOSPACE, Typeface.BOLD)
            }

            val bodyCodePaint = Paint(Paint.ANTI_ALIAS_FLAG).apply {
                color = frameInk.toArgb()
                textSize = 9.5f * d
                textAlign = Paint.Align.CENTER
                typeface = Typeface.create(Typeface.MONOSPACE, Typeface.BOLD)
            }

            val nodeCodePaint = Paint(Paint.ANTI_ALIAS_FLAG).apply {
                color = subtleInk.toArgb()
                textSize = 9.0f * d
                textAlign = Paint.Align.CENTER
                typeface = Typeface.create(Typeface.MONOSPACE, Typeface.BOLD)
            }

            val retroBodyCodePaint = Paint(Paint.ANTI_ALIAS_FLAG).apply {
                color = ascColor.toArgb()
                textSize = 9.5f * d
                textAlign = Paint.Align.CENTER
                typeface = Typeface.create(Typeface.MONOSPACE, Typeface.BOLD)
            }

            val bodyDegPaint = Paint(Paint.ANTI_ALIAS_FLAG).apply {
                color = subtleInk.toArgb()
                textSize = 8.2f * d
                textAlign = Paint.Align.CENTER
                typeface = Typeface.create(Typeface.MONOSPACE, Typeface.NORMAL)
            }

            // 1. Draw alternating monochrome sectors for the 12 signs
            for (i in 0 until 12) {
                val signStartLon = i * 30.0
                val signEndLon = (i + 1) * 30.0
                val endScreenDeg = lonToScreenDeg(signEndLon)

                if (i % 2 == 1) {
                    drawArc(
                        color = faintFill,
                        startAngle = endScreenDeg,
                        sweepAngle = 30f,
                        useCenter = true,
                        topLeft = Offset(center.x - outerRadius, center.y - outerRadius),
                        size = Size(outerRadius * 2f, outerRadius * 2f)
                    )
                }
            }

            // Clear inner area inside signInnerRadius with pure white
            drawCircle(
                color = BackgroundWhite,
                radius = signInnerRadius,
                center = center
            )

            // 2. Draw Zodiac Ring Borders & 3-Letter Sign Codes
            drawCircle(
                color = frameInk,
                radius = outerRadius,
                center = center,
                style = Stroke(width = 1.5f * d)
            )
            drawCircle(
                color = frameInk,
                radius = signInnerRadius,
                center = center,
                style = Stroke(width = 1.2f * d)
            )

            for (i in 0 until 12) {
                val signStartLon = i * 30.0
                val pOuter = lonToOffset(signStartLon, outerRadius)
                val pInner = lonToOffset(signStartLon, signInnerRadius)
                drawLine(
                    color = frameInk,
                    start = pInner,
                    end = pOuter,
                    strokeWidth = 1.0f * d
                )

                // Sign 3-letter code at midpoint of sector
                val midLon = signStartLon + 15.0
                val textRadius = (outerRadius + signInnerRadius) / 2f
                val textPos = lonToOffset(midLon, textRadius)
                drawCenteredText(ZODIAC_CODES[i], textPos, signPaint)
            }

            // 3. Draw Precision Ephemeris Degree Ticks (every 1°, 5°, 10°) along signInnerRadius
            for (deg in 0 until 360) {
                val lon = deg.toDouble()
                val isTen = deg % 10 == 0
                val isFive = deg % 5 == 0
                val tickLen = when {
                    isTen -> 5.5f * d
                    isFive -> 3.8f * d
                    else -> 2.0f * d
                }
                val tickWidth = when {
                    isTen -> 1.1f * d
                    isFive -> 0.8f * d
                    else -> 0.5f * d
                }
                val tickColor = if (isTen || isFive) frameInk else mutedLine
                val pStart = lonToOffset(lon, signInnerRadius)
                val pEnd = lonToOffset(lon, signInnerRadius - tickLen)
                drawLine(
                    color = tickColor,
                    start = pStart,
                    end = pEnd,
                    strokeWidth = tickWidth
                )
            }

            // 4. Draw Inner Rings (House Number Band & Aspect Core)
            drawCircle(
                color = faintFill,
                radius = houseRingRadius,
                center = center
            )
            drawCircle(
                color = BackgroundWhite,
                radius = innerAspectRadius,
                center = center
            )
            drawCircle(
                color = mutedLine,
                radius = houseRingRadius,
                center = center,
                style = Stroke(width = 1.0f * d)
            )
            drawCircle(
                color = frameInk,
                radius = innerAspectRadius,
                center = center,
                style = Stroke(width = 1.1f * d)
            )

            // 5. Draw House Cusps & House Numbers
            val cusps = data.house_cusps
            if (cusps.size == 12) {
                for (i in 0 until 12) {
                    val cuspLon = cusps[i]
                    val nextCuspLon = cusps[(i + 1) % 12]
                    val isAngle = (i == 0 || i == 3 || i == 6 || i == 9)

                    val pInner = lonToOffset(cuspLon, innerAspectRadius)
                    val pOuter = lonToOffset(cuspLon, signInnerRadius)

                    if (!isAngle) {
                        drawLine(
                            color = mutedLine,
                            start = pInner,
                            end = pOuter,
                            strokeWidth = 0.9f * d
                        )
                    }

                    // Midpoint of house for number label (1..12)
                    val span = (nextCuspLon - cuspLon + 360.0) % 360.0
                    val midLon = (cuspLon + span / 2.0) % 360.0
                    val numRadius = (houseRingRadius + innerAspectRadius) / 2f
                    val numPos = lonToOffset(midLon, numRadius)
                    drawCenteredText("${i + 1}", numPos, houseNumPaint)
                }
            }

            // 6. Draw Primary Angles (ASC/DSC and MC/IC) with sharp contrast
            val dsc = (asc + 180.0) % 360.0
            val mc = data.midheaven
            val ic = (mc + 180.0) % 360.0

            // ASC-DSC Axis
            drawLine(
                color = ascColor,
                start = lonToOffset(asc, innerAspectRadius),
                end = lonToOffset(asc, signInnerRadius),
                strokeWidth = 2.0f * d
            )
            drawLine(
                color = ascColor.copy(alpha = 0.65f),
                start = lonToOffset(dsc, innerAspectRadius),
                end = lonToOffset(dsc, signInnerRadius),
                strokeWidth = 1.4f * d
            )

            // MC-IC Axis
            drawLine(
                color = mcColor,
                start = lonToOffset(mc, innerAspectRadius),
                end = lonToOffset(mc, signInnerRadius),
                strokeWidth = 2.0f * d
            )
            drawLine(
                color = mcColor.copy(alpha = 0.65f),
                start = lonToOffset(ic, innerAspectRadius),
                end = lonToOffset(ic, signInnerRadius),
                strokeWidth = 1.4f * d
            )

            // Angle markers inside the aspect core rim
            val angleBadgeRadius = innerAspectRadius - 10f * d
            drawCenteredText("ASC", lonToOffset(asc, angleBadgeRadius), ascLabelPaint)
            drawCenteredText("MC", lonToOffset(mc, angleBadgeRadius), mcLabelPaint)

            // 7. Draw Aspect Lines inside innerAspectRadius
            val bodyLonMap = mutableMapOf<String, Double>()
            data.planets.forEach { bodyLonMap[it.name] = it.longitude }
            data.nodes.forEach { bodyLonMap[it.name] = it.longitude }

            data.aspects.forEach { asp ->
                val lon1 = bodyLonMap[asp.planet1]
                val lon2 = bodyLonMap[asp.planet2]
                if (lon1 != null && lon2 != null) {
                    val p1 = lonToOffset(lon1, innerAspectRadius - 2f * d)
                    val p2 = lonToOffset(lon2, innerAspectRadius - 2f * d)

                    val isHard = isHardAspect(asp.aspect_type)
                    val isSoft = isSoftAspect(asp.aspect_type)

                    val lineColor = when {
                        isHard -> ascColor.copy(alpha = 0.55f)
                        isSoft -> mcColor.copy(alpha = 0.55f)
                        else -> subtleInk.copy(alpha = 0.45f)
                    }
                    val pathEffect = if (asp.aspect_type.equals("Sextile", ignoreCase = true)) {
                        PathEffect.dashPathEffect(floatArrayOf(5f * d, 4f * d), 0f)
                    } else null

                    drawLine(
                        color = lineColor,
                        start = p1,
                        end = p2,
                        strokeWidth = if (asp.orb <= 2.0) 1.4f * d else 0.9f * d,
                        pathEffect = pathEffect
                    )
                }
            }

            // 8. Angular Collision-Avoidance Solver for Planets & Nodes
            val rawBodies = mutableListOf<WheelBodyItem>()
            data.planets.forEach { p ->
                val deg = ((p.sign_degree % 30.0 + 30.0) % 30.0).toInt().coerceIn(0, 29)
                rawBodies.add(
                    WheelBodyItem(
                        code = bodyCode(p.name),
                        trueLon = (p.longitude % 360.0 + 360.0) % 360.0,
                        degInSign = deg,
                        isRetrograde = p.is_retrograde,
                        isNode = false,
                        displayLon = (p.longitude % 360.0 + 360.0) % 360.0
                    )
                )
            }
            data.nodes.forEach { n ->
                val deg = ((n.sign_degree % 30.0 + 30.0) % 30.0).toInt().coerceIn(0, 29)
                rawBodies.add(
                    WheelBodyItem(
                        code = bodyCode(n.name),
                        trueLon = (n.longitude % 360.0 + 360.0) % 360.0,
                        degInSign = deg,
                        isRetrograde = n.is_retrograde,
                        isNode = true,
                        displayLon = (n.longitude % 360.0 + 360.0) % 360.0
                    )
                )
            }

            val solvedBodies = resolveAngularCollisions(rawBodies, minSepDeg = 7.2)

            // 9. Draw True Longitude Dots, Leader Lines, Planet Codes, and Degree Labels
            solvedBodies.forEach { b ->
                val exactRimPoint = lonToOffset(b.trueLon, signInnerRadius - 2.5f * d)
                val leaderInnerPoint = lonToOffset(b.displayLon, tickInnerRadius - 2f * d)
                val codePos = lonToOffset(b.displayLon, planetCodeRadius)
                val degPos = lonToOffset(b.displayLon, planetDegRadius)

                // Exact point dot on inner zodiac rim
                drawCircle(
                    color = if (b.isRetrograde && !b.isNode) ascColor else frameInk,
                    radius = 2.0f * d,
                    center = exactRimPoint
                )

                // Thin leader line connecting exact degree on rim to displaced label azimuth
                drawLine(
                    color = subtleInk.copy(alpha = 0.65f),
                    start = exactRimPoint,
                    end = leaderInnerPoint,
                    strokeWidth = 0.8f * d
                )

                // Planet 2/3-letter code
                val codePaint = when {
                    b.isNode -> nodeCodePaint
                    b.isRetrograde -> retroBodyCodePaint
                    else -> bodyCodePaint
                }
                drawCenteredText(b.code, codePos, codePaint)

                // Degree + Retrograde indicator on inner layer
                val degLabel = String.format(
                    Locale.US,
                    "%02d°%s",
                    b.degInSign,
                    if (b.isRetrograde && !b.isNode) "R" else ""
                )
                drawCenteredText(degLabel, degPos, bodyDegPaint)
            }
        }
    }
}

private fun DrawScope.drawCenteredText(text: String, pos: Offset, paint: Paint) {
    val fm = paint.fontMetrics
    val baselineOffset = -(fm.ascent + fm.descent) / 2f
    drawContext.canvas.nativeCanvas.drawText(
        text,
        pos.x,
        pos.y + baselineOffset,
        paint
    )
}

/**
 * Iterative circular relaxation solver that spreads clustered planets along the azimuth
 * so labels never overlap while staying anchored as close as possible to their true longitudes.
 */
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
            val j = (i + 1) % n
            val a = sorted[i]
            val b = sorted[j]
            // Signed angular distance from a to b in [0, 360)
            var diff = (b.displayLon - a.displayLon + 360.0) % 360.0
            if (diff > 180.0) diff -= 360.0

            if (diff >= 0.0 && diff < minSepDeg) {
                val push = (minSepDeg - diff) * 0.52
                a.displayLon = (a.displayLon - push + 360.0) % 360.0
                b.displayLon = (b.displayLon + push + 360.0) % 360.0
                moved = true
            } else if (diff < 0.0 && abs(diff) < minSepDeg) {
                // Crossed over during push: restore order
                val mid = (a.displayLon + diff / 2.0 + 360.0) % 360.0
                a.displayLon = (mid - minSepDeg * 0.5 + 360.0) % 360.0
                b.displayLon = (mid + minSepDeg * 0.5 + 360.0) % 360.0
                moved = true
            }
        }
        // Gentle spring pull back toward true longitude so clusters remain centered on their real degree
        for (item in sorted) {
            var delta = (item.trueLon - item.displayLon + 540.0) % 360.0 - 180.0
            if (abs(delta) > 0.2) {
                item.displayLon = (item.displayLon + delta * 0.08 + 360.0) % 360.0
            }
        }
        if (!moved) return@repeat
    }
    return sorted
}
