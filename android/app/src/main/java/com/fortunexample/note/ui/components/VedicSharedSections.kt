package com.fortunexample.note.ui.components

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.fortunexample.note.data.VedicArudhaPadaData
import com.fortunexample.note.data.VedicChartData
import com.fortunexample.note.data.VedicDrishtiData
import com.fortunexample.note.data.VedicPanchangaData
import com.fortunexample.note.ui.theme.CelestialBlue
import com.fortunexample.note.ui.theme.TextPrimary
import com.fortunexample.note.ui.theme.TextSecondary
import java.util.Locale

@Composable
fun VedicPlacementsTableCard(
    v: VedicChartData,
    title: String,
    subtitle: String
) {
    AstroPanelCard(title = title, subtitle = subtitle, contentPadding = PaddingValues(0.dp)) {
        val cols = listOf(
            TableColSpec("GRAHA", 0.8f, TextAlign.Start, monospace = true),
            TableColSpec("RASI & DEG", 1.3f, TextAlign.Start, monospace = true),
            TableColSpec("H", 0.5f, TextAlign.Center, monospace = true),
            TableColSpec("NAKSHATRA", 1.45f, TextAlign.Start, monospace = true),
            TableColSpec("STATUS", 1.1f, TextAlign.End, monospace = true)
        )
        AstroTableHeader(cols)
        v.planets.forEachIndexed { idx, p ->
            val status = formatVedicGrahaStatus(p)
            AstroTableRow(
                columns = cols,
                striped = idx % 2 == 1,
                cells = listOf(
                    TableCellData(vedicGrahaCode(p.name), TextPrimary, FontWeight.Bold),
                    TableCellData(formatSignAndDeg(p.sign_index, p.sign_degree), TextPrimary),
                    TableCellData("${p.house}", TextSecondary),
                    TableCellData("${p.nakshatra_name.take(8)} P${p.nakshatra_pada}", TextSecondary),
                    TableCellData(status.tag, status.color, FontWeight.Bold)
                )
            )
        }
    }
}

@Composable
fun VedicPanchangaCard(
    pan: VedicPanchangaData,
    title: String
) {
    if (pan.tithi_name.isEmpty()) return
    AstroPanelCard(title = title, contentPadding = PaddingValues(0.dp)) {
        val cols = listOf(
            TableColSpec("LIMB", 1.1f, TextAlign.Start, monospace = true),
            TableColSpec("VALUE", 1.9f, TextAlign.Start),
            TableColSpec("META / LORD", 1.2f, TextAlign.End, monospace = true)
        )
        AstroTableHeader(cols)
        val rows = listOf(
            Triple("TITHI", pan.tithi_name, "#${pan.tithi_number} · ${pan.paksha.take(6).uppercase(Locale.US)}" to CelestialBlue),
            Triple("VARA", pan.vara_name, "LORD ${vedicGrahaCode(pan.vara_lord)}" to TextSecondary),
            Triple("NAKSHATRA", "${pan.nakshatra_name} (Pada ${pan.nakshatra_pada})", "LORD ${vedicGrahaCode(pan.nakshatra_lord)}" to CelestialBlue),
            Triple("YOGA", pan.yoga_name, "-" to TextSecondary),
            Triple("KARANA", pan.karana_name, "-" to TextSecondary)
        )
        rows.forEachIndexed { idx, (limb, value, meta) ->
            AstroTableRow(
                columns = cols,
                striped = idx % 2 == 1,
                cells = listOf(
                    TableCellData(limb, TextSecondary, FontWeight.Bold),
                    TableCellData(value, TextPrimary, FontWeight.SemiBold),
                    TableCellData(meta.first, meta.second)
                )
            )
        }
    }
}

@Composable
fun VedicArudhaPadasCard(
    padas: List<VedicArudhaPadaData>,
    title: String
) {
    if (padas.isEmpty()) return
    AstroPanelCard(title = title) {
        val tiles = padas.map { ap ->
            MetricTileItem(
                tag = ap.code,
                primary = signCode(ap.sign_index),
                secondary = "H${ap.house_from_lagna}",
                highlightColor = if (ap.code == "AL" || ap.code == "UL") CelestialBlue else null
            )
        }
        CompactMetricGrid(items = tiles, columns = 4)
    }
}

@Composable
fun VedicDrishtiCard(
    drishti: List<VedicDrishtiData>,
    title: String,
    targetColumnHeader: String
) {
    val notable = drishti.filter { it.aspected_planets.isNotEmpty() || it.aspect_offset != 7 }
    AstroPanelCard(title = title, contentPadding = PaddingValues(0.dp)) {
        if (notable.isEmpty()) {
            Box(modifier = Modifier.padding(12.dp)) {
                Text("No active Graha Drishti.", fontSize = 12.sp, color = TextSecondary)
            }
            return@AstroPanelCard
        }
        val cols = listOf(
            TableColSpec("SOURCE", 0.95f, TextAlign.Start, monospace = true),
            TableColSpec("DRISHTI", 0.85f, TextAlign.Center, monospace = true),
            TableColSpec(targetColumnHeader, 1.15f, TextAlign.Center, monospace = true),
            TableColSpec("OCCUPANTS", 1.35f, TextAlign.End, monospace = true)
        )
        AstroTableHeader(cols)
        notable.forEachIndexed { idx, d ->
            val occText = if (d.aspected_planets.isEmpty()) "- (EMPTY)" else d.aspected_planets.joinToString(" ") { vedicGrahaCode(it) }
            AstroTableRow(
                columns = cols,
                striped = idx % 2 == 1,
                cells = listOf(
                    TableCellData("${vedicGrahaCode(d.from_planet)} (H${d.from_house})", TextPrimary, FontWeight.Bold),
                    TableCellData("${d.aspect_offset}TH", CelestialBlue, FontWeight.Bold),
                    TableCellData("H${d.target_house}·${signCode(d.target_sign)}", TextSecondary),
                    TableCellData(
                        text = occText,
                        color = if (d.aspected_planets.isEmpty()) TextSecondary else TextPrimary,
                        fontWeight = if (d.aspected_planets.isEmpty()) FontWeight.Normal else FontWeight.SemiBold
                    )
                )
            )
        }
    }
}
