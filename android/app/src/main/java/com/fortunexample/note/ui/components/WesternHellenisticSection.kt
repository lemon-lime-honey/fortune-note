package com.fortunexample.note.ui.components

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.runtime.Composable
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import com.fortunexample.note.data.HellenisticData
import com.fortunexample.note.ui.theme.CelestialBlue
import com.fortunexample.note.ui.theme.CrimsonAlert
import com.fortunexample.note.ui.theme.TextPrimary
import com.fortunexample.note.ui.theme.TextSecondary

@Composable
internal fun HellenisticSubSection(h: HellenisticData) {
    Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
        HellenisticAnglesCard(h)
        HellenisticConditionsCard(h)
        ArabicLotsTableCard(
            title = "Hermetic Lots (Paulus Alexandrinus)",
            rulerColumnHeader = "LORD",
            lots = h.lots,
            copyLabel = "Hermetic Lots"
        )
        h.prenatal_syzygy?.let { PrenatalSyzygyCard(it) }
        FixedStarConjunctionsCard(h.fixed_star_conjunctions)
    }
}

@Composable
private fun HellenisticAnglesCard(h: HellenisticData) {
    AstroPanelCard(
        title = "Hellenistic Angles (Whole Sign)",
        headerAction = {
            SectionCopyButton("Hellenistic Angles") {
                "ASC: ${formatSignAndDeg(h.angles.ascendant_sign, h.angles.ascendant_degree)} (H1) | " +
                    "MC: ${formatSignAndDeg(h.angles.midheaven_sign, h.angles.midheaven_degree)} (H${h.angles.midheaven_house})"
            }
        }
    ) {
        CompactMetricGrid(
            items = listOf(
                MetricTileItem("ASCENDANT", formatSignAndDeg(h.angles.ascendant_sign, h.angles.ascendant_degree), "H1"),
                MetricTileItem("MIDHEAVEN", formatSignAndDeg(h.angles.midheaven_sign, h.angles.midheaven_degree), "H${h.angles.midheaven_house}"),
                MetricTileItem("MC IN 10TH", if (h.angles.is_mc_in_10th) "YES" else "NO (H${h.angles.midheaven_house})")
            ),
            columns = 3
        )
    }
}

@Composable
private fun HellenisticConditionsCard(h: HellenisticData) {
    if (h.planet_conditions.isEmpty()) return
    AstroPanelCard(
        title = "Planetary Conditions, Egyptian Bounds & Triplicity",
        contentPadding = PaddingValues(0.dp),
        headerAction = {
            SectionCopyButton("Hellenistic Conditions") {
                buildString {
                    appendLine("[Hellenistic Conditions, Bounds & Triplicity]")
                    h.planet_conditions.forEach { pc ->
                        val tripTag = if (pc.is_in_triplicity) " · In Triplicity" else ""
                        val lordTag = if (pc.is_ruler_of_asc) " · Lord of ASC" else ""
                        appendLine(
                            "${bodyCode(pc.name)} (${formatSignAndDeg(pc.sign_index, pc.sign_degree)}, H${pc.house_number}): " +
                                "${pc.condition_label}, Bound ${bodyCode(pc.bound_ruler)}$tripTag$lordTag"
                        )
                    }
                }.trimEnd()
            }
        }
    ) {
        val cols = listOf(
            TableColSpec("BODY", 0.8f, TextAlign.Start, monospace = true),
            TableColSpec("POSITION", 1.25f, TextAlign.Start, monospace = true),
            TableColSpec("H", 0.45f, TextAlign.Center, monospace = true),
            TableColSpec("BOUND", 0.7f, TextAlign.Center, monospace = true),
            TableColSpec("TRIP", 0.65f, TextAlign.Center, monospace = true),
            TableColSpec("CONDITION", 1.35f, TextAlign.End)
        )
        AstroTableHeader(cols)
        h.planet_conditions.forEachIndexed { idx, pc ->
            val condColor = when {
                pc.is_in_domicile || pc.is_in_exaltation -> CelestialBlue
                pc.is_in_fall -> CrimsonAlert
                else -> TextSecondary
            }
            AstroTableRow(
                columns = cols,
                striped = idx % 2 == 1,
                cells = listOf(
                    TableCellData(bodyCode(pc.name), if (pc.is_ruler_of_asc) CelestialBlue else TextPrimary, FontWeight.Bold),
                    TableCellData(formatSignAndDeg(pc.sign_index, pc.sign_degree), TextPrimary),
                    TableCellData("${pc.house_number}", TextSecondary),
                    TableCellData(if (pc.bound_ruler.isNotBlank()) bodyCode(pc.bound_ruler) else "-", TextSecondary),
                    TableCellData(
                        text = if (pc.is_in_triplicity) "YES" else "-",
                        color = if (pc.is_in_triplicity) CelestialBlue else TextSecondary,
                        fontWeight = if (pc.is_in_triplicity) FontWeight.Bold else FontWeight.Normal
                    ),
                    TableCellData(pc.condition_label.ifBlank { "Peregrine" }, condColor, FontWeight.Medium)
                )
            )
        }
    }
}
