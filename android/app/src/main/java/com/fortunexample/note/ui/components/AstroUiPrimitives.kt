package com.fortunexample.note.ui.components

import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.horizontalScroll
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.TextUnit
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.fortunexample.note.ui.theme.*
import java.util.Locale
import kotlin.math.abs

/**
 * Standard 3-letter uppercase English zodiac sign codes.
 */
val ZODIAC_CODES = listOf(
    "ARI", "TAU", "GEM", "CAN", "LEO", "VIR",
    "LIB", "SCO", "SAG", "CAP", "AQU", "PIS"
)

val ZODIAC_FULL_NAMES = listOf(
    "Aries", "Taurus", "Gemini", "Cancer", "Leo", "Virgo",
    "Libra", "Scorpio", "Sagittarius", "Capricorn", "Aquarius", "Pisces"
)

val VEDIC_RASI_NAMES = listOf(
    "Mesha", "Vrishabha", "Mithuna", "Karka",
    "Simha", "Kanya", "Tula", "Vrishchika",
    "Dhanu", "Makara", "Kumbha", "Meena"
)

fun signCode(index: Int): String = ZODIAC_CODES.getOrElse(((index % 12) + 12) % 12) { "ARI" }

fun signFullName(index: Int): String = ZODIAC_FULL_NAMES.getOrElse(((index % 12) + 12) % 12) { "Aries" }

fun vedicRasiCode(index: Int): String {
    val idx = ((index % 12) + 12) % 12
    return "${VEDIC_RASI_NAMES[idx]} (${ZODIAC_CODES[idx]})"
}

/**
 * Standard 2/3-letter uppercase planetary codes (zero emoji usage).
 */
fun bodyCode(name: String): String = when (name.trim()) {
    "Sun", "Surya" -> "SUN"
    "Moon", "Chandra" -> "MON"
    "Mercury", "Budha" -> "MER"
    "Venus", "Shukra" -> "VEN"
    "Mars", "Mangala", "Kuja" -> "MAR"
    "Jupiter", "Guru", "Brihaspati" -> "JUP"
    "Saturn", "Shani" -> "SAT"
    "Uranus" -> "URA"
    "Neptune" -> "NEP"
    "Pluto" -> "PLU"
    "North Node", "NorthNode", "True Node", "Mean Node", "Rahu" -> "NN"
    "South Node", "SouthNode", "Ketu" -> "SN"
    "Ascendant", "ASC", "Lagna" -> "ASC"
    "Midheaven", "MC" -> "MC"
    "Descendant", "DSC" -> "DSC"
    "Imum Coeli", "IC" -> "IC"
    "Part of Fortune", "Lot of Fortune", "Fortune" -> "POF"
    "Part of Spirit", "Lot of Spirit", "Spirit" -> "POS"
    else -> name.trim().take(3).uppercase(Locale.US)
}

fun vedicGrahaCode(name: String): String = when (name.trim()) {
    "Sun", "Surya" -> "SUR"
    "Moon", "Chandra" -> "CHA"
    "Mars", "Mangala", "Kuja" -> "MAN"
    "Mercury", "Budha" -> "BUD"
    "Jupiter", "Guru" -> "GUR"
    "Venus", "Shukra" -> "SHU"
    "Saturn", "Shani" -> "SHA"
    "Rahu", "North Node", "NorthNode" -> "RAH"
    "Ketu", "South Node", "SouthNode" -> "KET"
    "Lagna", "Ascendant", "ASC" -> "LAG"
    else -> bodyCode(name)
}

fun aspectCode(aspectType: String): String = when (aspectType.trim().lowercase(Locale.US)) {
    "conjunction", "conj" -> "CNJ 0°"
    "sextile", "sext" -> "SXT 60°"
    "square", "sqr" -> "SQR 90°"
    "trine", "tri" -> "TRI 120°"
    "opposition", "opp" -> "OPP 180°"
    "quincunx", "inconjunct" -> "QNX 150°"
    "semisextile" -> "SSX 30°"
    "semisquare" -> "SSQ 45°"
    "sesquiquadrate" -> "SES 135°"
    else -> aspectType.take(7).uppercase(Locale.US)
}

fun aspectShortCode(aspectType: String): String = when (aspectType.trim().lowercase(Locale.US)) {
    "conjunction", "conj" -> "CNJ"
    "sextile", "sext" -> "SXT"
    "square", "sqr" -> "SQR"
    "trine", "tri" -> "TRI"
    "opposition", "opp" -> "OPP"
    else -> aspectType.take(3).uppercase(Locale.US)
}

fun isHardAspect(aspectType: String): Boolean {
    val lower = aspectType.trim().lowercase(Locale.US)
    return lower.contains("square") || lower.contains("opposition")
}

fun isSoftAspect(aspectType: String): Boolean {
    val lower = aspectType.trim().lowercase(Locale.US)
    return lower.contains("trine") || lower.contains("sextile")
}

/**
 * Formats degree within sign (0..30) into fixed-width DMS or DM string: e.g. "15°24'" or "04°08'"
 */
fun formatDegMin(degInSign: Double): String {
    val normalized = ((degInSign % 30.0) + 30.0) % 30.0
    val totalMinutes = (normalized * 60.0).toInt()
    val d = (totalMinutes / 60).coerceIn(0, 29)
    val m = (totalMinutes % 60).coerceIn(0, 59)
    return String.format(Locale.US, "%02d°%02d'", d, m)
}

fun formatSignAndDeg(signIndex: Int, degInSign: Double): String {
    return "${signCode(signIndex)} ${formatDegMin(degInSign)}"
}

fun formatLongitudeSignDeg(longitude: Double): String {
    val norm = ((longitude % 360.0) + 360.0) % 360.0
    val signIdx = (norm / 30.0).toInt().coerceIn(0, 11)
    val degInSign = norm - signIdx * 30.0
    return "${signCode(signIdx)} ${formatDegMin(degInSign)}"
}

fun formatOrb(orb: Double): String = String.format(Locale.US, "%04.2f°", abs(orb))

fun formatSignedScore(score: Int): String = when {
    score > 0 -> String.format(Locale.US, "+%d", score)
    score < 0 -> String.format(Locale.US, "%d", score)
    else -> "0"
}

val TabularMonoStyle = TextStyle(
    fontFamily = FontFamily.Monospace,
    fontFeatureSettings = "tnum"
)

val TabularSansStyle = TextStyle(
    fontFamily = FontFamily.SansSerif,
    fontFeatureSettings = "tnum"
)

/**
 * Clean architectural card container with 1dp border and zero drop-shadow clutter.
 */
@Composable
fun AstroPanelCard(
    title: String,
    modifier: Modifier = Modifier,
    subtitle: String? = null,
    headerAction: (@Composable () -> Unit)? = null,
    contentPadding: PaddingValues = PaddingValues(12.dp),
    content: @Composable ColumnScope.() -> Unit
) {
    Surface(
        modifier = modifier.fillMaxWidth(),
        shape = RoundedCornerShape(8.dp),
        color = BackgroundWhite,
        border = BorderStroke(1.dp, BorderColor)
    ) {
        Column(modifier = Modifier.fillMaxWidth()) {
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .background(SurfaceLight)
                    .padding(horizontal = 12.dp, vertical = 8.dp),
                verticalAlignment = Alignment.CenterVertically,
                horizontalArrangement = Arrangement.SpaceBetween
            ) {
                Column(modifier = Modifier.weight(1f)) {
                    Text(
                        text = title.uppercase(Locale.US),
                        fontSize = 11.sp,
                        fontWeight = FontWeight.Bold,
                        letterSpacing = 0.6.sp,
                        color = TextPrimary,
                        maxLines = 1,
                        overflow = TextOverflow.Ellipsis
                    )
                    if (!subtitle.isNullOrBlank()) {
                        Text(
                            text = subtitle,
                            fontSize = 11.sp,
                            color = TextSecondary,
                            maxLines = 1,
                            overflow = TextOverflow.Ellipsis,
                            style = TabularSansStyle
                        )
                    }
                }
                if (headerAction != null) {
                    Spacer(modifier = Modifier.width(8.dp))
                    headerAction()
                }
            }
            HorizontalDivider(color = BorderColor, thickness = 1.dp)
            Column(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(contentPadding),
                content = content
            )
        }
    }
}

/**
 * Sub-navigation bar with clean English tabs.
 */
@Composable
fun AstroSubTabBar(
    tabs: List<String>,
    selectedIndex: Int,
    onSelect: (Int) -> Unit,
    modifier: Modifier = Modifier
) {
    Row(
        modifier = modifier
            .fillMaxWidth()
            .horizontalScroll(rememberScrollState())
            .background(SurfaceLight, RoundedCornerShape(6.dp))
            .border(1.dp, BorderColor, RoundedCornerShape(6.dp))
            .padding(3.dp),
        horizontalArrangement = Arrangement.spacedBy(4.dp),
        verticalAlignment = Alignment.CenterVertically
    ) {
        tabs.forEachIndexed { index, label ->
            val selected = index == selectedIndex
            Box(
                modifier = Modifier
                    .clip(RoundedCornerShape(4.dp))
                    .background(if (selected) AccentPrimary else Color.Transparent)
                    .clickable { onSelect(index) }
                    .padding(horizontal = 12.dp, vertical = 6.dp),
                contentAlignment = Alignment.Center
            ) {
                Text(
                    text = label.uppercase(Locale.US),
                    fontSize = 11.sp,
                    fontWeight = if (selected) FontWeight.Bold else FontWeight.Medium,
                    letterSpacing = 0.4.sp,
                    color = if (selected) BackgroundWhite else TextSecondary,
                    maxLines = 1
                )
            }
        }
    }
}

/**
 * Column specification for strict single-line fixed-alignment tables.
 */
data class TableColSpec(
    val header: String,
    val weight: Float,
    val align: TextAlign = TextAlign.Start,
    val monospace: Boolean = false
)

data class TableCellData(
    val text: String,
    val color: Color = TextPrimary,
    val fontWeight: FontWeight = FontWeight.Normal,
    val badgeColor: Color? = null
)

@Composable
fun AstroTableHeader(
    columns: List<TableColSpec>,
    modifier: Modifier = Modifier
) {
    Row(
        modifier = modifier
            .fillMaxWidth()
            .height(28.dp)
            .background(SurfaceLight)
            .padding(horizontal = 10.dp),
        verticalAlignment = Alignment.CenterVertically
    ) {
        columns.forEach { col ->
            Text(
                text = col.header.uppercase(Locale.US),
                modifier = Modifier.weight(col.weight),
                textAlign = col.align,
                fontSize = 10.sp,
                fontWeight = FontWeight.Bold,
                letterSpacing = 0.5.sp,
                color = TextSecondary,
                maxLines = 1,
                overflow = TextOverflow.Clip
            )
        }
    }
    HorizontalDivider(color = BorderColor, thickness = 1.dp)
}

@Composable
fun AstroTableRow(
    columns: List<TableColSpec>,
    cells: List<TableCellData>,
    modifier: Modifier = Modifier,
    height: Dp = 34.dp,
    striped: Boolean = false,
    onClick: (() -> Unit)? = null
) {
    val bgColor = if (striped) SurfaceLight.copy(alpha = 0.45f) else BackgroundWhite
    val rowMod = modifier
        .fillMaxWidth()
        .height(height)
        .background(bgColor)
        .let { if (onClick != null) it.clickable { onClick() } else it }
        .padding(horizontal = 10.dp)

    Row(
        modifier = rowMod,
        verticalAlignment = Alignment.CenterVertically
    ) {
        columns.forEachIndexed { idx, col ->
            val cell = cells.getOrNull(idx) ?: TableCellData("")
            Box(
                modifier = Modifier.weight(col.weight),
                contentAlignment = when (col.align) {
                    TextAlign.End -> Alignment.CenterEnd
                    TextAlign.Center -> Alignment.Center
                    else -> Alignment.CenterStart
                }
            ) {
                if (cell.badgeColor != null && cell.text.isNotEmpty()) {
                    Surface(
                        shape = RoundedCornerShape(3.dp),
                        color = cell.badgeColor.copy(alpha = 0.12f),
                        border = BorderStroke(0.5.dp, cell.badgeColor.copy(alpha = 0.35f))
                    ) {
                        Text(
                            text = cell.text,
                            fontSize = 10.sp,
                            fontWeight = FontWeight.Bold,
                            color = cell.badgeColor,
                            style = if (col.monospace) TabularMonoStyle else TabularSansStyle,
                            maxLines = 1,
                            modifier = Modifier.padding(horizontal = 5.dp, vertical = 1.dp)
                        )
                    }
                } else {
                    Text(
                        text = cell.text,
                        textAlign = col.align,
                        fontSize = 12.sp,
                        fontWeight = cell.fontWeight,
                        color = cell.color,
                        style = if (col.monospace) TabularMonoStyle else TabularSansStyle,
                        maxLines = 1,
                        softWrap = false,
                        overflow = TextOverflow.Ellipsis
                    )
                }
            }
        }
    }
    HorizontalDivider(color = BorderColor.copy(alpha = 0.5f), thickness = 0.5.dp)
}

/**
 * Compact tile grid item (e.g. for 12 Houses, 12 Arudha Padas, 12 SAV Bindus, 8 Karakas).
 */
data class MetricTileItem(
    val tag: String,
    val primary: String,
    val secondary: String = "",
    val highlightColor: Color? = null
)

@Composable
fun CompactMetricGrid(
    items: List<MetricTileItem>,
    columns: Int = 4,
    modifier: Modifier = Modifier,
    tileHeight: Dp = 48.dp
) {
    val rows = items.chunked(columns)
    Column(
        modifier = modifier.fillMaxWidth(),
        verticalArrangement = Arrangement.spacedBy(6.dp)
    ) {
        rows.forEach { rowItems ->
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.spacedBy(6.dp)
            ) {
                for (i in 0 until columns) {
                    val item = rowItems.getOrNull(i)
                    if (item != null) {
                        val borderCol = item.highlightColor?.copy(alpha = 0.45f) ?: BorderColor
                        val bgCol = item.highlightColor?.copy(alpha = 0.05f) ?: SurfaceLight.copy(alpha = 0.55f)
                        Column(
                            modifier = Modifier
                                .weight(1f)
                                .height(tileHeight)
                                .background(bgCol, RoundedCornerShape(6.dp))
                                .border(1.dp, borderCol, RoundedCornerShape(6.dp))
                                .padding(horizontal = 8.dp, vertical = 4.dp),
                            verticalArrangement = Arrangement.Center
                        ) {
                            Row(
                                modifier = Modifier.fillMaxWidth(),
                                horizontalArrangement = Arrangement.SpaceBetween,
                                verticalAlignment = Alignment.CenterVertically
                            ) {
                                Text(
                                    text = item.tag,
                                    fontSize = 10.sp,
                                    fontWeight = FontWeight.Bold,
                                    color = item.highlightColor ?: TextSecondary,
                                    style = TabularMonoStyle,
                                    maxLines = 1
                                )
                                if (item.secondary.isNotEmpty()) {
                                    Text(
                                        text = item.secondary,
                                        fontSize = 10.sp,
                                        fontWeight = FontWeight.Medium,
                                        color = TextSecondary,
                                        style = TabularMonoStyle,
                                        maxLines = 1
                                    )
                                }
                            }
                            Spacer(modifier = Modifier.height(2.dp))
                            Text(
                                text = item.primary,
                                fontSize = 12.sp,
                                fontWeight = FontWeight.Bold,
                                color = TextPrimary,
                                style = TabularMonoStyle,
                                maxLines = 1,
                                overflow = TextOverflow.Ellipsis
                            )
                        }
                    } else {
                        Spacer(modifier = Modifier.weight(1f))
                    }
                }
            }
        }
    }
}

@Composable
fun AstroStatusBadge(
    text: String,
    color: Color,
    fontSize: TextUnit = 10.sp
) {
    Surface(
        shape = RoundedCornerShape(3.dp),
        color = color.copy(alpha = 0.12f),
        border = BorderStroke(0.5.dp, color.copy(alpha = 0.35f))
    ) {
        Text(
            text = text.uppercase(Locale.US),
            fontSize = fontSize,
            fontWeight = FontWeight.Bold,
            color = color,
            style = TabularMonoStyle,
            maxLines = 1,
            modifier = Modifier.padding(horizontal = 5.dp, vertical = 1.dp)
        )
    }
}
