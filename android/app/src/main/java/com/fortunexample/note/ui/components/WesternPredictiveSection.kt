package com.fortunexample.note.ui.components

import android.app.DatePickerDialog
import android.app.TimePickerDialog
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.horizontalScroll
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.Remove
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.fortunexample.note.data.PlanetaryReturnData
import com.fortunexample.note.data.PredictiveSuiteData
import com.fortunexample.note.data.ProfileDto
import com.fortunexample.note.data.findHouseNumber
import com.fortunexample.note.ui.theme.*
import java.util.Calendar
import java.util.Locale

@Composable
fun WesternPredictiveSection(
    profile: ProfileDto,
    predictiveData: PredictiveSuiteData?,
    targetYear: Int,
    targetMonth: Int = profile.birth_month,
    targetDay: Int = profile.birth_day,
    isLoading: Boolean,
    onYearChange: (Int) -> Unit,
    onDateChange: ((Int, Int, Int) -> Unit)? = null,
    onTimeChange: ((Int?, Int?) -> Unit)? = null,
    onDateTimeChange: ((Int, Int, Int, Int?, Int?) -> Unit)? = null
) {
    var selectedTechTab by remember { mutableIntStateOf(0) }
    val techTabs = listOf(
        "Profections",
        "Solar Arc",
        "Progressions",
        "Returns",
        "Transits",
        "Directions"
    )

    Column(
        modifier = Modifier.fillMaxWidth(),
        verticalArrangement = Arrangement.spacedBy(10.dp)
    ) {
        // Top Target Year Control Bar
        Surface(
            shape = RoundedCornerShape(8.dp),
            color = BackgroundWhite,
            border = BorderStroke(1.dp, BorderColor),
            modifier = Modifier.fillMaxWidth()
        ) {
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(horizontal = 12.dp, vertical = 8.dp),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically
            ) {
                Column {
                    Row(
                        verticalAlignment = Alignment.CenterVertically,
                        horizontalArrangement = Arrangement.spacedBy(8.dp)
                    ) {
                        Text(
                            text = "PREDICTIVE EPHEMERIS",
                            fontSize = 11.sp,
                            fontWeight = FontWeight.Bold,
                            letterSpacing = 0.5.sp,
                            color = TextPrimary
                        )
                        if (isLoading) {
                            CircularProgressIndicator(
                                color = AccentPrimary,
                                strokeWidth = 1.8.dp,
                                modifier = Modifier.size(13.dp)
                            )
                        }
                    }
                    Text(
                        text = "Target Year $targetYear · Age ${targetYear - profile.birth_year}",
                        fontSize = 11.sp,
                        color = TextSecondary,
                        style = TabularMonoStyle
                    )
                }

                Row(
                    verticalAlignment = Alignment.CenterVertically,
                    horizontalArrangement = Arrangement.spacedBy(4.dp)
                ) {
                    IconButton(
                        onClick = { onYearChange(targetYear - 1) },
                        modifier = Modifier.size(28.dp)
                    ) {
                        Icon(Icons.Default.Remove, contentDescription = "Previous Year", tint = TextPrimary, modifier = Modifier.size(16.dp))
                    }
                    Surface(
                        shape = RoundedCornerShape(4.dp),
                        color = SurfaceLight,
                        border = BorderStroke(1.dp, BorderColor)
                    ) {
                        Text(
                            text = "$targetYear",
                            fontSize = 13.sp,
                            fontWeight = FontWeight.Bold,
                            color = TextPrimary,
                            style = TabularMonoStyle,
                            modifier = Modifier.padding(horizontal = 10.dp, vertical = 4.dp)
                        )
                    }
                    IconButton(
                        onClick = { onYearChange(targetYear + 1) },
                        modifier = Modifier.size(28.dp)
                    ) {
                        Icon(Icons.Default.Add, contentDescription = "Next Year", tint = TextPrimary, modifier = Modifier.size(16.dp))
                    }
                }
            }
        }

        AstroSubTabBar(
            tabs = techTabs,
            selectedIndex = selectedTechTab,
            onSelect = { selectedTechTab = it }
        )

        if (predictiveData == null) {
            AstroPanelCard(title = "Loading Predictive Data") {
                Box(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(vertical = 24.dp),
                    contentAlignment = Alignment.Center
                ) {
                    Text(
                        text = if (isLoading) "Computing ephemeris tables..." else "Predictive data unavailable.",
                        fontSize = 12.sp,
                        color = TextSecondary
                    )
                }
            }
        } else {
            when (selectedTechTab) {
                0 -> ProfectionTabContent(predictiveData)
                1 -> SolarArcTabContent(predictiveData)
                2 -> SecondaryProgressionTabContent(predictiveData)
                3 -> PlanetaryReturnsTabContent(predictiveData)
                4 -> TransitsTabContent(
                    data = predictiveData,
                    profile = profile,
                    onDateChange = onDateChange,
                    onTimeChange = onTimeChange,
                    onDateTimeChange = onDateTimeChange
                )
                5 -> PrimaryDirectionsTabContent(predictiveData)
            }
        }
    }
}

@Composable
private fun ProfectionTabContent(data: PredictiveSuiteData) {
    val prof = data.profection
    AstroPanelCard(
        title = "Annual Profection (${data.target_year})",
        headerAction = {
            SectionCopyButton("Annual Profection") {
                val occ = if (prof.active_natal_planets.isEmpty()) "None" else prof.active_natal_planets.joinToString(", ")
                buildString {
                    appendLine("[Annual Profection ${data.target_year}]")
                    appendLine("- Target Age: ${prof.target_age}")
                    appendLine("- Profected House: House ${prof.profected_house} (${prof.profected_sign_name})")
                    appendLine("- Lord of the Year (Time Lord): ${prof.lord_of_the_year}")
                    appendLine("- Natal Occupants in House ${prof.profected_house}: $occ")
                }.trimEnd()
            }
        }
    ) {
        val tiles = listOf(
            MetricTileItem("TARGET AGE", "${prof.target_age} YRS", "${prof.target_year}"),
            MetricTileItem("PROFECTED HOUSE", "H${prof.profected_house}", signCode(prof.profected_sign_index), CelestialBlue),
            MetricTileItem("TIME LORD", bodyCode(prof.lord_of_the_year), prof.lord_of_the_year, CelestialBlue),
            MetricTileItem(
                tag = "NATAL OCCUPANTS",
                primary = if (prof.active_natal_planets.isEmpty()) "NONE" else prof.active_natal_planets.joinToString(" ") { bodyCode(it) },
                secondary = "IN H${prof.profected_house}"
            )
        )
        CompactMetricGrid(items = tiles, columns = 2)
    }
}

@Composable
private fun SolarArcTabContent(data: PredictiveSuiteData) {
    val sa = data.solar_arc
    Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
        AstroPanelCard(
            title = "Solar Arc Contacts (Orb ≤ 1.5°)",
            subtitle = String.format(Locale.US, "Arc Offset: +%05.2f°", sa.arc_degrees),
            contentPadding = PaddingValues(0.dp),
            headerAction = {
                SectionCopyButton("Solar Arc") {
                    buildString {
                        appendLine(String.format(Locale.US, "[Solar Arc Directions (%d, Offset +%.2f°)]", data.target_year, sa.arc_degrees))
                        sa.directed_planets.forEach { p ->
                            appendLine("- SA ${p.name}: ${formatSignAndDeg(p.sign_index, p.sign_degree)} (Natal: ${formatLongitudeSignDeg(p.natal_longitude)})")
                        }
                        if (sa.contacts.isNotEmpty()) {
                            appendLine("Contacts (Orb <= 1.5°):")
                            sa.contacts.forEach { c ->
                                appendLine("  * SA ${c.moving_body} ${c.aspect_type} Natal ${c.natal_body} (Orb ${formatOrb(c.orb)})")
                            }
                        }
                    }.trimEnd()
                }
            }
        ) {
            if (sa.contacts.isEmpty()) {
                Box(modifier = Modifier.padding(12.dp)) {
                    Text("No exact Solar Arc aspects within 1.5° orb.", fontSize = 12.sp, color = TextSecondary)
                }
            } else {
                val cols = listOf(
                    TableColSpec("SA BODY", 1.0f, TextAlign.Start, monospace = true),
                    TableColSpec("ASPECT", 1.1f, TextAlign.Center, monospace = true),
                    TableColSpec("NATAL", 1.0f, TextAlign.Center, monospace = true),
                    TableColSpec("ORB", 0.9f, TextAlign.End, monospace = true)
                )
                AstroTableHeader(cols)
                sa.contacts.forEachIndexed { idx, c ->
                    val hard = isHardAspect(c.aspect_type)
                    val soft = isSoftAspect(c.aspect_type)
                    val aspColor = when {
                        hard -> CrimsonAlert
                        soft -> CelestialBlue
                        else -> TextPrimary
                    }
                    AstroTableRow(
                        columns = cols,
                        striped = idx % 2 == 1,
                        cells = listOf(
                            TableCellData("SA ${bodyCode(c.moving_body)}", TextPrimary, FontWeight.Bold),
                            TableCellData(aspectCode(c.aspect_type), aspColor, FontWeight.Bold),
                            TableCellData("N·${bodyCode(c.natal_body)}", TextPrimary, FontWeight.SemiBold),
                            TableCellData(formatOrb(c.orb), TextSecondary)
                        )
                    )
                }
            }
        }

        AstroPanelCard(
            title = "Solar Arc Directed Positions",
            contentPadding = PaddingValues(0.dp)
        ) {
            val cols = listOf(
                TableColSpec("BODY", 0.9f, TextAlign.Start, monospace = true),
                TableColSpec("NATAL POS", 1.4f, TextAlign.Start, monospace = true),
                TableColSpec("DIRECTED POS", 1.4f, TextAlign.End, monospace = true)
            )
            AstroTableHeader(cols)
            sa.directed_planets.forEachIndexed { idx, p ->
                AstroTableRow(
                    columns = cols,
                    striped = idx % 2 == 1,
                    cells = listOf(
                        TableCellData(bodyCode(p.name), TextPrimary, FontWeight.Bold),
                        TableCellData(formatLongitudeSignDeg(p.natal_longitude), TextSecondary),
                        TableCellData(formatSignAndDeg(p.sign_index, p.sign_degree), CelestialBlue, FontWeight.SemiBold)
                    )
                )
            }
        }
    }
}

@Composable
private fun SecondaryProgressionTabContent(data: PredictiveSuiteData) {
    val sp = data.secondary_progression
    if (sp == null) {
        AstroPanelCard(title = "Secondary Progressions") {
            Text("Secondary progression data unavailable.", fontSize = 12.sp, color = TextSecondary)
        }
        return
    }

    Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
        AstroPanelCard(
            title = "Secondary Progressions Summary",
            subtitle = "Ephemeris Date: ${sp.progressed_date_utc}",
            headerAction = {
                SectionCopyButton("Secondary Progressions") {
                    buildString {
                        appendLine("[Secondary Progressions (${sp.progressed_date_utc})]")
                        appendLine("- Lunar Phase: ${sp.progressed_lunar_phase}")
                        sp.planets.forEach { p ->
                            val rx = if (p.is_retrograde) " [Rx]" else ""
                            appendLine("- Progressed ${p.name}: ${formatSignAndDeg(p.sign_index, p.sign_degree)}$rx")
                        }
                        if (sp.contacts.isNotEmpty()) {
                            appendLine("Contacts (Orb <= 1.5°):")
                            sp.contacts.forEach { c ->
                                appendLine("  * Progressed ${c.moving_body} ${c.aspect_type} Natal ${c.natal_body} (Orb ${formatOrb(c.orb)})")
                            }
                        }
                    }.trimEnd()
                }
            }
        ) {
            val tiles = listOf(
                MetricTileItem("LUNAR PHASE", sp.progressed_lunar_phase.uppercase(Locale.US)),
                MetricTileItem("ELAPSED SPAN", String.format(Locale.US, "%.2f DAYS", sp.elapsed_tropical_years), "1D = 1YR")
            )
            CompactMetricGrid(items = tiles, columns = 2)
        }

        AstroPanelCard(
            title = "Progressed Planet Positions",
            contentPadding = PaddingValues(0.dp)
        ) {
            val cols = listOf(
                TableColSpec("BODY", 1.0f, TextAlign.Start, monospace = true),
                TableColSpec("PROGRESSED POS", 1.6f, TextAlign.Start, monospace = true),
                TableColSpec("STATE", 0.9f, TextAlign.End, monospace = true)
            )
            AstroTableHeader(cols)
            sp.planets.forEachIndexed { idx, p ->
                AstroTableRow(
                    columns = cols,
                    striped = idx % 2 == 1,
                    cells = listOf(
                        TableCellData(bodyCode(p.name), TextPrimary, FontWeight.Bold),
                        TableCellData(formatSignAndDeg(p.sign_index, p.sign_degree), TextPrimary),
                        TableCellData(
                            text = if (p.is_retrograde) "RETRO" else "DIR",
                            color = if (p.is_retrograde) CrimsonAlert else TextSecondary,
                            badgeColor = if (p.is_retrograde) CrimsonAlert else null
                        )
                    )
                )
            }
        }

        AstroPanelCard(
            title = "Progressed-to-Natal Contacts (Orb ≤ 1.5°)",
            contentPadding = PaddingValues(0.dp)
        ) {
            if (sp.contacts.isEmpty()) {
                Box(modifier = Modifier.padding(12.dp)) {
                    Text("No active Progressed-to-Natal aspects within 1.5° orb.", fontSize = 12.sp, color = TextSecondary)
                }
            } else {
                val cols = listOf(
                    TableColSpec("PROG", 1.0f, TextAlign.Start, monospace = true),
                    TableColSpec("ASPECT", 1.1f, TextAlign.Center, monospace = true),
                    TableColSpec("NATAL", 1.0f, TextAlign.Center, monospace = true),
                    TableColSpec("ORB", 0.9f, TextAlign.End, monospace = true)
                )
                AstroTableHeader(cols)
                sp.contacts.forEachIndexed { idx, c ->
                    val aspColor = when {
                        isHardAspect(c.aspect_type) -> CrimsonAlert
                        isSoftAspect(c.aspect_type) -> CelestialBlue
                        else -> TextPrimary
                    }
                    AstroTableRow(
                        columns = cols,
                        striped = idx % 2 == 1,
                        cells = listOf(
                            TableCellData("P·${bodyCode(c.moving_body)}", TextPrimary, FontWeight.Bold),
                            TableCellData(aspectCode(c.aspect_type), aspColor, FontWeight.Bold),
                            TableCellData("N·${bodyCode(c.natal_body)}", TextPrimary, FontWeight.SemiBold),
                            TableCellData(formatOrb(c.orb), TextSecondary)
                        )
                    )
                }
            }
        }
    }
}

@Composable
private fun PlanetaryReturnsTabContent(data: PredictiveSuiteData) {
    val returns = remember(data) {
        buildList {
            val sr = data.solar_return
            val hasSunInList = data.planetary_returns.any { it.return_body.equals("Sun", ignoreCase = true) }
            if (sr != null && !hasSunInList) {
                add(
                    PlanetaryReturnData(
                        target_year = sr.return_year,
                        return_body = "Sun",
                        return_date_utc = sr.return_date_utc,
                        exact_jd_ut1 = sr.exact_jd_ut1,
                        body_longitude = sr.sun_longitude,
                        is_retrograde = false,
                        speed = 1.0,
                        ascendant = sr.ascendant,
                        midheaven = sr.midheaven,
                        house_cusps = sr.house_cusps,
                        return_planets = sr.planets,
                        natal_aspects = sr.natal_aspects
                    )
                )
            }
            addAll(data.planetary_returns)
        }
    }
    var selectedBodyIdx by remember { mutableIntStateOf(0) }

    if (returns.isEmpty()) {
        AstroPanelCard(title = "Planetary Returns") {
            Text("No planetary returns available for ${data.target_year}.", fontSize = 12.sp, color = TextSecondary)
        }
        return
    }

    val safeIdx = selectedBodyIdx.coerceIn(0, returns.lastIndex)
    val currentReturn = returns[safeIdx]

    Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
        AstroSubTabBar(
            tabs = returns.map { "${bodyCode(it.return_body)} Return" },
            selectedIndex = safeIdx,
            onSelect = { selectedBodyIdx = it }
        )

        AstroPanelCard(
            title = "${currentReturn.return_body} Return (${currentReturn.target_year})",
            subtitle = "Exact UTC: ${currentReturn.return_date_utc}",
            headerAction = {
                SectionCopyButton("${currentReturn.return_body} Return") {
                    buildString {
                        appendLine("[${currentReturn.return_body} Return (${currentReturn.target_year})]")
                        appendLine("- Exact UTC: ${currentReturn.return_date_utc}")
                        appendLine("- Return ASC: ${formatLongitudeSignDeg(currentReturn.ascendant)} | Return MC: ${formatLongitudeSignDeg(currentReturn.midheaven)}")
                        appendLine("- Return Placements:")
                        currentReturn.return_planets.forEach { p ->
                            val hNum = findHouseNumber(p.progressed_longitude, currentReturn.house_cusps)
                            val rx = if (p.is_retrograde) " [Rx]" else ""
                            appendLine("  * ${p.name}: ${formatSignAndDeg(p.sign_index, p.sign_degree)} (H$hNum)$rx")
                        }
                        if (currentReturn.natal_aspects.isNotEmpty()) {
                            appendLine("- Return-to-Natal Aspects:")
                            currentReturn.natal_aspects.forEach { a ->
                                appendLine("  * Return ${a.return_point} ${a.aspect_type} Natal ${a.natal_point} (Orb ${formatOrb(a.orb)})")
                            }
                        }
                    }.trimEnd()
                }
            }
        ) {
            val tiles = listOf(
                MetricTileItem("RETURN ASC", formatLongitudeSignDeg(currentReturn.ascendant), "H1", CrimsonAlert),
                MetricTileItem("RETURN MC", formatLongitudeSignDeg(currentReturn.midheaven), "H10", CelestialBlue),
                MetricTileItem("RETURN ${bodyCode(currentReturn.return_body)}", formatLongitudeSignDeg(currentReturn.body_longitude), if (currentReturn.is_retrograde) "RX" else "DIR")
            )
            CompactMetricGrid(items = tiles, columns = 3)
        }

        AstroPanelCard(
            title = "Return Chart Placements",
            contentPadding = PaddingValues(0.dp)
        ) {
            val cols = listOf(
                TableColSpec("BODY", 0.9f, TextAlign.Start, monospace = true),
                TableColSpec("POSITION", 1.4f, TextAlign.Start, monospace = true),
                TableColSpec("RET H", 0.7f, TextAlign.Center, monospace = true),
                TableColSpec("STATE", 0.8f, TextAlign.End, monospace = true)
            )
            AstroTableHeader(cols)
            currentReturn.return_planets.forEachIndexed { idx, p ->
                val hNum = findHouseNumber(p.progressed_longitude, currentReturn.house_cusps)
                AstroTableRow(
                    columns = cols,
                    striped = idx % 2 == 1,
                    cells = listOf(
                        TableCellData(bodyCode(p.name), TextPrimary, FontWeight.Bold),
                        TableCellData(formatSignAndDeg(p.sign_index, p.sign_degree), TextPrimary),
                        TableCellData("H$hNum", TextSecondary),
                        TableCellData(
                            text = if (p.is_retrograde) "RETRO" else "DIR",
                            color = if (p.is_retrograde) CrimsonAlert else TextSecondary,
                            badgeColor = if (p.is_retrograde) CrimsonAlert else null
                        )
                    )
                )
            }
        }

        if (currentReturn.natal_aspects.isNotEmpty()) {
            AstroPanelCard(
                title = "Return-to-Natal Aspects",
                contentPadding = PaddingValues(0.dp)
            ) {
                val cols = listOf(
                    TableColSpec("RET", 1.0f, TextAlign.Start, monospace = true),
                    TableColSpec("ASPECT", 1.1f, TextAlign.Center, monospace = true),
                    TableColSpec("NATAL", 1.0f, TextAlign.Center, monospace = true),
                    TableColSpec("ORB", 0.9f, TextAlign.End, monospace = true)
                )
                AstroTableHeader(cols)
                currentReturn.natal_aspects.forEachIndexed { idx, a ->
                    val aspColor = when {
                        isHardAspect(a.aspect_type) -> CrimsonAlert
                        isSoftAspect(a.aspect_type) -> CelestialBlue
                        else -> TextPrimary
                    }
                    AstroTableRow(
                        columns = cols,
                        striped = idx % 2 == 1,
                        cells = listOf(
                            TableCellData("R·${bodyCode(a.return_point)}", TextPrimary, FontWeight.Bold),
                            TableCellData(aspectCode(a.aspect_type), aspColor, FontWeight.Bold),
                            TableCellData("N·${bodyCode(a.natal_point)}", TextPrimary, FontWeight.SemiBold),
                            TableCellData(formatOrb(a.orb), TextSecondary)
                        )
                    )
                }
            }
        }
    }
}

@Composable
private fun TransitsTabContent(
    data: PredictiveSuiteData,
    profile: ProfileDto,
    onDateChange: ((Int, Int, Int) -> Unit)?,
    onTimeChange: ((Int?, Int?) -> Unit)?,
    onDateTimeChange: ((Int, Int, Int, Int?, Int?) -> Unit)?
) {
    val context = LocalContext.current

    val applyDateTime = { y: Int, m: Int, d: Int, hr: Int?, min: Int? ->
        if (onDateTimeChange != null) {
            onDateTimeChange(y, m, d, hr, min)
        } else {
            onDateChange?.invoke(y, m, d)
            onTimeChange?.invoke(hr, min)
        }
    }

    val shiftDays = { deltaDays: Int ->
        val cal = Calendar.getInstance().apply {
            set(Calendar.YEAR, data.target_year)
            set(Calendar.MONTH, (data.target_month - 1).coerceIn(0, 11))
            set(Calendar.DAY_OF_MONTH, data.target_day.coerceIn(1, 31))
            add(Calendar.DAY_OF_MONTH, deltaDays)
        }
        applyDateTime(
            cal.get(Calendar.YEAR),
            cal.get(Calendar.MONTH) + 1,
            cal.get(Calendar.DAY_OF_MONTH),
            data.target_hour,
            data.target_minute
        )
    }

    val shiftHours = { deltaHours: Int ->
        val cal = Calendar.getInstance().apply {
            set(Calendar.YEAR, data.target_year)
            set(Calendar.MONTH, (data.target_month - 1).coerceIn(0, 11))
            set(Calendar.DAY_OF_MONTH, data.target_day.coerceIn(1, 31))
            set(Calendar.HOUR_OF_DAY, data.target_hour.coerceIn(0, 23))
            set(Calendar.MINUTE, data.target_minute.coerceIn(0, 59))
            add(Calendar.HOUR_OF_DAY, deltaHours)
        }
        applyDateTime(
            cal.get(Calendar.YEAR),
            cal.get(Calendar.MONTH) + 1,
            cal.get(Calendar.DAY_OF_MONTH),
            cal.get(Calendar.HOUR_OF_DAY),
            cal.get(Calendar.MINUTE)
        )
    }

    val openDatePicker = {
        DatePickerDialog(
            context,
            { _, year, monthZeroBased, dayOfMonth ->
                applyDateTime(year, monthZeroBased + 1, dayOfMonth, data.target_hour, data.target_minute)
            },
            data.target_year,
            (data.target_month - 1).coerceIn(0, 11),
            data.target_day.coerceIn(1, 31)
        ).show()
    }

    val openTimePicker = {
        TimePickerDialog(
            context,
            { _, hourOfDay, minute ->
                applyDateTime(data.target_year, data.target_month, data.target_day, hourOfDay, minute)
            },
            data.target_hour.coerceIn(0, 23),
            data.target_minute.coerceIn(0, 59),
            true
        ).show()
    }

    val setToNow = {
        val now = Calendar.getInstance()
        applyDateTime(
            now.get(Calendar.YEAR),
            now.get(Calendar.MONTH) + 1,
            now.get(Calendar.DAY_OF_MONTH),
            now.get(Calendar.HOUR_OF_DAY),
            now.get(Calendar.MINUTE)
        )
    }

    val timestampStr = String.format(
        Locale.US,
        "%04d-%02d-%02d %02d:%02d LOCAL",
        data.target_year,
        data.target_month,
        data.target_day,
        data.target_hour,
        data.target_minute
    )

    Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
        AstroPanelCard(
            title = "Transit Ephemeris Controls",
            subtitle = timestampStr
        ) {
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .horizontalScroll(rememberScrollState()),
                horizontalArrangement = Arrangement.spacedBy(6.dp)
            ) {
                TransitActionChip("NOW") { setToNow() }
                TransitActionChip("DATE...") { openDatePicker() }
                TransitActionChip("TIME...") { openTimePicker() }
                TransitActionChip("-1D") { shiftDays(-1) }
                TransitActionChip("+1D") { shiftDays(1) }
                TransitActionChip("-1H") { shiftHours(-1) }
                TransitActionChip("+1H") { shiftHours(1) }
            }
        }

        AstroPanelCard(
            title = "Active Transits to Natal (Orb ≤ 2.0°)",
            contentPadding = PaddingValues(0.dp),
            headerAction = {
                SectionCopyButton("Active Transits") {
                    buildString {
                        appendLine("[Active Transits ($timestampStr)]")
                        if (data.transits.isEmpty()) {
                            appendLine("No active major transits within 2.0° orb.")
                        } else {
                            data.transits.forEach { t ->
                                appendLine("- Transit ${t.transiting_planet} (${formatLongitudeSignDeg(t.transiting_longitude)}) ${t.aspect_type} Natal ${t.natal_planet} (${formatLongitudeSignDeg(t.natal_longitude)}) (Orb ${formatOrb(t.orb)})")
                            }
                        }
                    }.trimEnd()
                }
            }
        ) {
            if (data.transits.isEmpty()) {
                Box(modifier = Modifier.padding(12.dp)) {
                    Text("No active major transits within 2.0° orb.", fontSize = 12.sp, color = TextSecondary)
                }
            } else {
                val cols = listOf(
                    TableColSpec("TRANSIT", 1.3f, TextAlign.Start, monospace = true),
                    TableColSpec("ASPECT", 1.0f, TextAlign.Center, monospace = true),
                    TableColSpec("NATAL", 1.3f, TextAlign.Start, monospace = true),
                    TableColSpec("ORB", 0.8f, TextAlign.End, monospace = true)
                )
                AstroTableHeader(cols)
                data.transits.forEachIndexed { idx, t ->
                    val aspColor = when {
                        isHardAspect(t.aspect_type) -> CrimsonAlert
                        isSoftAspect(t.aspect_type) -> CelestialBlue
                        else -> TextPrimary
                    }
                    AstroTableRow(
                        columns = cols,
                        striped = idx % 2 == 1,
                        cells = listOf(
                            TableCellData("${bodyCode(t.transiting_planet)} ${formatLongitudeSignDeg(t.transiting_longitude)}", TextPrimary, FontWeight.Bold),
                            TableCellData(aspectShortCode(t.aspect_type), aspColor, FontWeight.Bold),
                            TableCellData("${bodyCode(t.natal_planet)} ${formatLongitudeSignDeg(t.natal_longitude)}", TextSecondary),
                            TableCellData(formatOrb(t.orb), TextPrimary, FontWeight.SemiBold)
                        )
                    )
                }
            }
        }
    }
}

@Composable
private fun TransitActionChip(
    label: String,
    onClick: () -> Unit
) {
    Box(
        modifier = Modifier
            .clip(RoundedCornerShape(4.dp))
            .background(SurfaceLight)
            .border(1.dp, BorderColor, RoundedCornerShape(4.dp))
            .clickable { onClick() }
            .padding(horizontal = 9.dp, vertical = 5.dp),
        contentAlignment = Alignment.Center
    ) {
        Text(
            text = label,
            fontSize = 10.sp,
            fontWeight = FontWeight.Bold,
            color = TextPrimary,
            style = TabularMonoStyle,
            maxLines = 1
        )
    }
}

@Composable
private fun PrimaryDirectionsTabContent(data: PredictiveSuiteData) {
    AstroPanelCard(
        title = "Primary Directions (Naibod Key)",
        contentPadding = PaddingValues(0.dp),
        headerAction = {
            SectionCopyButton("Primary Directions") {
                buildString {
                    appendLine("[Primary Directions (Naibod Key, Target Year ${data.target_year})]")
                    if (data.primary_directions.isEmpty()) {
                        appendLine("No Primary Directions hits in current window.")
                    } else {
                        data.primary_directions.forEach { pd ->
                            appendLine(String.format(Locale.US, "- %s %s %s (Arc %.2f°, Age %.1fY)", pd.promittor, pd.aspect_type, pd.significator, pd.direction_arc_deg, pd.realization_age))
                        }
                    }
                }.trimEnd()
            }
        }
    ) {
        if (data.primary_directions.isEmpty()) {
            Box(modifier = Modifier.padding(12.dp)) {
                Text("No Primary Directions hits in current window.", fontSize = 12.sp, color = TextSecondary)
            }
        } else {
            val cols = listOf(
                TableColSpec("PROMITTOR", 0.95f, TextAlign.Start, monospace = true),
                TableColSpec("ASPECT", 0.95f, TextAlign.Center, monospace = true),
                TableColSpec("SIGNIFICATOR", 1.05f, TextAlign.Center, monospace = true),
                TableColSpec("ARC", 0.85f, TextAlign.End, monospace = true),
                TableColSpec("AGE", 0.85f, TextAlign.End, monospace = true)
            )
            AstroTableHeader(cols)
            data.primary_directions.forEachIndexed { idx, pd ->
                val aspColor = when {
                    isHardAspect(pd.aspect_type) -> CrimsonAlert
                    isSoftAspect(pd.aspect_type) -> CelestialBlue
                    else -> TextPrimary
                }
                AstroTableRow(
                    columns = cols,
                    striped = idx % 2 == 1,
                    cells = listOf(
                        TableCellData(bodyCode(pd.promittor), TextPrimary, FontWeight.Bold),
                        TableCellData(aspectShortCode(pd.aspect_type), aspColor, FontWeight.Bold),
                        TableCellData(bodyCode(pd.significator), TextPrimary, FontWeight.SemiBold),
                        TableCellData(String.format(Locale.US, "%05.2f°", pd.direction_arc_deg), TextSecondary),
                        TableCellData(String.format(Locale.US, "%.1fY", pd.realization_age), CelestialBlue, FontWeight.Bold)
                    )
                )
            }
        }
    }
}
