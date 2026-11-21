package com.fortunexample.note.ui.components

import android.content.ClipData
import android.content.ClipboardManager
import android.content.Context
import android.widget.Toast
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ContentCopy
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.fortunexample.note.data.*
import com.fortunexample.note.ui.theme.*
import java.util.Locale

@Composable
fun SectionCopyButton(
    title: String,
    textProvider: () -> String
) {
    val context = LocalContext.current
    IconButton(
        onClick = {
            val clipboard = context.getSystemService(Context.CLIPBOARD_SERVICE) as ClipboardManager
            clipboard.setPrimaryClip(ClipData.newPlainText(title, textProvider()))
            Toast.makeText(context, "Copied $title", Toast.LENGTH_SHORT).show()
        },
        modifier = Modifier.size(22.dp)
    ) {
        Icon(
            imageVector = Icons.Default.ContentCopy,
            contentDescription = "Copy $title",
            tint = TextSecondary,
            modifier = Modifier.size(13.dp)
        )
    }
}

@Composable
fun WesternTraditionalSection(
    data: WesternTraditionalData?,
    hellenistic: HellenisticData? = null,
    isHorary: Boolean = false
) {
    if (hellenistic != null) {
        HellenisticSubSection(hellenistic)
        return
    }
    if (data == null) return

    var selectedSubTab by remember { mutableIntStateOf(0) }
    val subTabs = listOf("Dignities", "Hours & Moon", "Receptions & Cusps", "Lots & Stars")

    Column(
        modifier = Modifier.fillMaxWidth(),
        verticalArrangement = Arrangement.spacedBy(10.dp)
    ) {
        AstroSubTabBar(
            tabs = subTabs,
            selectedIndex = selectedSubTab.coerceIn(0, subTabs.lastIndex),
            onSelect = { selectedSubTab = it }
        )

        when (selectedSubTab.coerceIn(0, subTabs.lastIndex)) {
            0 -> DignitiesMatrixSubSection(data)
            1 -> HoursAndMoonSubSection(data, isHorary)
            2 -> ReceptionsAndCuspsSubSection(data)
            else -> LotsAndStarsSubSection(data)
        }
    }
}

@Composable
internal fun PrenatalSyzygyCard(san: PrenatalSyzygyData) {
    AstroPanelCard(title = "Prenatal Syzygy (SAN)") {
        val tiles = listOf(
            MetricTileItem("TYPE", san.syzygy_type.uppercase(Locale.US)),
            MetricTileItem("DEGREE", formatSignAndDeg(san.sign_index, san.sign_degree)),
            MetricTileItem("UTC DATE", san.exact_datetime_utc.take(16))
        )
        CompactMetricGrid(items = tiles, columns = 3)
    }
}

@Composable
internal fun ArabicLotsTableCard(
    title: String,
    rulerColumnHeader: String,
    lots: List<ArabicPartData>,
    copyLabel: String? = null
) {
    if (lots.isEmpty()) return
    AstroPanelCard(
        title = title,
        contentPadding = PaddingValues(0.dp),
        headerAction = copyLabel?.let { label ->
            {
                SectionCopyButton(label) {
                    buildString {
                        appendLine("[$label]")
                        lots.forEach { lot ->
                            appendLine("${lot.name}: ${formatSignAndDeg(lot.sign_index, lot.sign_degree)} (H${lot.house_number}, $rulerColumnHeader: ${bodyCode(lot.ruler)})")
                        }
                    }.trimEnd()
                }
            }
        }
    ) {
        val cols = listOf(
            TableColSpec("LOT", 1.5f, TextAlign.Start),
            TableColSpec("POSITION", 1.25f, TextAlign.Start, monospace = true),
            TableColSpec("H", 0.55f, TextAlign.Center, monospace = true),
            TableColSpec(rulerColumnHeader, 0.75f, TextAlign.Center, monospace = true),
            TableColSpec("FORMULA", 1.4f, TextAlign.End, monospace = true)
        )
        AstroTableHeader(cols)
        lots.forEachIndexed { idx, part ->
            AstroTableRow(
                columns = cols,
                striped = idx % 2 == 1,
                cells = listOf(
                    TableCellData(part.name, TextPrimary, FontWeight.SemiBold),
                    TableCellData(formatSignAndDeg(part.sign_index, part.sign_degree), TextPrimary),
                    TableCellData("${part.house_number}", TextSecondary),
                    TableCellData(bodyCode(part.ruler), CelestialBlue, FontWeight.Bold),
                    TableCellData(part.formula.ifBlank { "-" }, TextSecondary)
                )
            )
        }
    }
}

@Composable
internal fun FixedStarConjunctionsCard(conjunctions: List<FixedStarConjunctionData>) {
    var expandedStar by remember { mutableStateOf<String?>(null) }

    AstroPanelCard(
        title = "Fixed Star Conjunctions",
        subtitle = "Tap row to view traditional nature & meaning",
        contentPadding = PaddingValues(0.dp)
    ) {
        if (conjunctions.isEmpty()) {
            Box(modifier = Modifier.padding(12.dp)) {
                Text("No active fixed star conjunctions within orb.", fontSize = 12.sp, color = TextSecondary)
            }
        } else {
            val cols = listOf(
                TableColSpec("STAR", 1.5f, TextAlign.Start),
                TableColSpec("BODY", 0.75f, TextAlign.Center, monospace = true),
                TableColSpec("ORB", 0.85f, TextAlign.End, monospace = true),
                TableColSpec("NATURE", 0.95f, TextAlign.Center, monospace = true),
                TableColSpec("STATUS", 0.95f, TextAlign.End, monospace = true)
            )
            AstroTableHeader(cols)
            conjunctions.forEachIndexed { idx, fsc ->
                val key = "${fsc.star_name}_${fsc.body_name}"
                val isExp = expandedStar == key
                AstroTableRow(
                    columns = cols,
                    striped = idx % 2 == 1,
                    onClick = { expandedStar = if (isExp) null else key },
                    cells = listOf(
                        TableCellData(fsc.star_name, TextPrimary, FontWeight.Bold),
                        TableCellData(bodyCode(fsc.body_name), CelestialBlue, FontWeight.Bold),
                        TableCellData(formatOrb(fsc.orb), TextPrimary),
                        TableCellData(fsc.nature, TextSecondary),
                        TableCellData(
                            text = if (fsc.is_qualified) "QUAL" else "FILT",
                            color = if (fsc.is_qualified) CelestialBlue else TextSecondary,
                            badgeColor = if (fsc.is_qualified) CelestialBlue else null
                        )
                    )
                )
                if (isExp) {
                    FixedStarDetailDrawer(fsc)
                }
            }
        }
    }
}

@Composable
private fun FixedStarDetailDrawer(fsc: FixedStarConjunctionData) {
    Column(
        modifier = Modifier
            .fillMaxWidth()
            .background(SurfaceLight)
            .padding(horizontal = 12.dp, vertical = 8.dp),
        verticalArrangement = Arrangement.spacedBy(2.dp)
    ) {
        Text(
            text = "Mag ${fsc.star_magnitude} · Lat ${String.format(Locale.US, "%+.1f°", fsc.star_latitude)} · Max Alt ${String.format(Locale.US, "%+.1f°", fsc.max_altitude)}",
            fontSize = 10.sp,
            color = TextSecondary,
            style = TabularMonoStyle
        )
        if (fsc.meaning.isNotBlank()) {
            Text(text = fsc.meaning, fontSize = 11.sp, color = TextPrimary)
        }
    }
    HorizontalDivider(color = BorderColor, thickness = 0.5.dp)
}
