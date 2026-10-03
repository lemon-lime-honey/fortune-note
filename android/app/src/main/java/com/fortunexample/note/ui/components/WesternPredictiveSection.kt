package com.fortunexample.note.ui.components

import android.widget.Toast
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.background
import androidx.compose.foundation.horizontalScroll
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.ContentCopy
import androidx.compose.material.icons.filled.Remove
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalClipboardManager
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.AnnotatedString
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.fortunexample.note.data.PredictiveSuiteData
import com.fortunexample.note.data.ProfileDto
import com.fortunexample.note.ui.theme.*

private val ZODIAC_NAMES = listOf(
    "Aries", "Taurus", "Gemini", "Cancer", "Leo", "Virgo",
    "Libra", "Scorpio", "Sagittarius", "Capricorn", "Aquarius", "Pisces"
)

private fun formatWesternBodyName(rawName: String): String {
    return rawName
        .replace("Rahu", "North Node")
        .replace("Ketu", "South Node")
}

private fun getZodiacSign(lon: Double): String {
    val idx = ((lon % 360.0 + 360.0) % 360.0 / 30.0).toInt() % 12
    return ZODIAC_NAMES.getOrElse(idx) { "" }
}

private fun getSignDegree(lon: Double): Double {
    val normalized = (lon % 360.0 + 360.0) % 360.0
    return normalized % 30.0
}

private fun findHouseNumber(lon: Double, cusps: List<Double>): Int {
    if (cusps.size < 12) return 1
    val normalizedLon = (lon % 360.0 + 360.0) % 360.0
    for (i in 0 until 12) {
        val start = (cusps[i] % 360.0 + 360.0) % 360.0
        val end = (cusps[(i + 1) % 12] % 360.0 + 360.0) % 360.0
        if (start < end) {
            if (normalizedLon >= start && normalizedLon < end) return i + 1
        } else {
            if (normalizedLon >= start || normalizedLon < end) return i + 1
        }
    }
    return 1
}

@Composable
fun WesternPredictiveSection(
    profile: ProfileDto,
    predictiveData: PredictiveSuiteData?,
    targetYear: Int,
    targetMonth: Int = profile.birth_month.toInt(),
    targetDay: Int = profile.birth_day.toInt(),
    isLoading: Boolean,
    onYearChange: (Int) -> Unit,
    onDateChange: ((Int, Int, Int) -> Unit)? = null,
    onTimeChange: ((Int?, Int?) -> Unit)? = null
) {
    var selectedTechTab by remember { mutableStateOf(0) }
    val techTabs = listOf(
        "Profections",
        "Solar Arc",
        "Progressions",
        "Planetary Returns",
        "Transits",
        "Primary Directions"
    )

    Card(
        shape = RoundedCornerShape(12.dp),
        colors = CardDefaults.cardColors(containerColor = BackgroundWhite),
        border = BorderStroke(1.dp, BorderColor),
        modifier = Modifier.fillMaxWidth()
    ) {
        Column(modifier = Modifier.padding(16.dp)) {
            // Header with year picker & subtle inline loading
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically
            ) {
                Column {
                    Row(
                        verticalAlignment = Alignment.CenterVertically,
                        horizontalArrangement = Arrangement.spacedBy(8.dp)
                    ) {
                        Text(
                            text = "Predictive Astrology Suite",
                            color = TextPrimary,
                            fontWeight = FontWeight.Bold,
                            fontSize = 15.sp
                        )
                        if (isLoading) {
                            CircularProgressIndicator(
                                color = AccentPrimary,
                                strokeWidth = 2.dp,
                                modifier = Modifier.size(16.dp)
                            )
                        }
                    }
                    Text(
                        text = "Target: Year $targetYear (${targetYear - profile.birth_year} yrs)",
                        color = TextSecondary,
                        fontSize = 12.sp
                    )
                }

                // Year selector controls
                Row(
                    verticalAlignment = Alignment.CenterVertically,
                    horizontalArrangement = Arrangement.spacedBy(4.dp)
                ) {
                    IconButton(
                        onClick = { onYearChange(targetYear - 1) },
                        modifier = Modifier.size(32.dp)
                    ) {
                        Icon(Icons.Default.Remove, contentDescription = "이전 해", tint = TextPrimary)
                    }

                    Text(
                        text = "$targetYear",
                        fontWeight = FontWeight.Bold,
                        fontSize = 15.sp,
                        color = CelestialBlue
                    )

                    IconButton(
                        onClick = { onYearChange(targetYear + 1) },
                        modifier = Modifier.size(32.dp)
                    ) {
                        Icon(Icons.Default.Add, contentDescription = "다음 해", tint = TextPrimary)
                    }
                }
            }

            Spacer(modifier = Modifier.height(12.dp))

            // Tech Navigation Chips
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .horizontalScroll(rememberScrollState()),
                horizontalArrangement = Arrangement.spacedBy(6.dp)
            ) {
                techTabs.forEachIndexed { idx, title ->
                    FilterChip(
                        selected = selectedTechTab == idx,
                        onClick = { selectedTechTab = idx },
                        label = {
                            Text(
                                text = title,
                                fontSize = 12.sp,
                                fontWeight = if (selectedTechTab == idx) FontWeight.Bold else FontWeight.Normal,
                                maxLines = 1,
                                softWrap = false
                            )
                        },
                        colors = FilterChipDefaults.filterChipColors(
                            selectedContainerColor = AccentPrimary,
                            selectedLabelColor = BackgroundWhite,
                            containerColor = SurfaceLight,
                            labelColor = TextSecondary
                        ),
                        border = FilterChipDefaults.filterChipBorder(
                            enabled = true,
                            selected = selectedTechTab == idx,
                            borderColor = BorderColor,
                            selectedBorderColor = AccentPrimary
                        )
                    )
                }
            }

            Spacer(modifier = Modifier.height(14.dp))

            if (predictiveData != null) {
                when (selectedTechTab) {
                    0 -> ProfectionCard(predictiveData)
                    1 -> SolarArcCard(predictiveData)
                    2 -> SecondaryProgressionCard(predictiveData)
                    3 -> PlanetaryReturnsCard(predictiveData)
                    4 -> TransitsCard(predictiveData, profile, onDateChange, onTimeChange)
                    5 -> PrimaryDirectionsCard(predictiveData)
                }
            } else if (isLoading) {
                Box(
                    modifier = Modifier
                        .fillMaxWidth()
                        .height(140.dp),
                    contentAlignment = Alignment.Center
                ) {
                    CircularProgressIndicator(color = AccentPrimary, modifier = Modifier.size(32.dp))
                }
            } else {
                Text(
                    text = "해당 연도의 예측 데이터를 불러올 수 없습니다.",
                    color = TextSecondary,
                    fontSize = 13.sp
                )
            }
        }
    }
}

@Composable
private fun SectionCopyButton(
    title: String,
    onCopy: () -> String
) {
    val clipboardManager = LocalClipboardManager.current
    val context = LocalContext.current

    IconButton(
        onClick = {
            val text = onCopy()
            clipboardManager.setText(AnnotatedString(text))
            Toast.makeText(context, "$title 복사 완료", Toast.LENGTH_SHORT).show()
        },
        modifier = Modifier.size(28.dp)
    ) {
        Icon(
            imageVector = Icons.Default.ContentCopy,
            contentDescription = "$title 복사",
            tint = TextSecondary,
            modifier = Modifier.size(16.dp)
        )
    }
}

@Composable
private fun ProfectionCard(data: PredictiveSuiteData) {
    val prof = data.profection
    val formattedPlanets = prof.active_natal_planets.map { formatWesternBodyName(it) }

    Column {
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            Text("Annual Profections", fontWeight = FontWeight.Bold, fontSize = 14.sp, color = TextPrimary)
            SectionCopyButton(title = "Profection") {
                val sb = StringBuilder()
                sb.appendLine("[Annual Profections - ${data.target_year}년 (${data.age}세)]")
                sb.appendLine("• Profected House: House ${prof.profected_house}")
                sb.appendLine("• Profected Sign: ${prof.profected_sign_name}")
                sb.appendLine("• Lord of the Year: ${formatWesternBodyName(prof.lord_of_the_year)}")
                if (formattedPlanets.isNotEmpty()) {
                    sb.appendLine("• Natal Planets in Sign: ${formattedPlanets.joinToString(", ")}")
                }
                sb.toString().trimEnd()
            }
        }
        Spacer(modifier = Modifier.height(6.dp))
        Row(modifier = Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.SpaceBetween) {
            Text("Profected House & Sign", color = TextSecondary, fontSize = 13.sp)
            Text("House ${prof.profected_house} · ${prof.profected_sign_name}", fontWeight = FontWeight.Bold, color = CelestialBlue, fontSize = 13.sp)
        }
        Spacer(modifier = Modifier.height(4.dp))
        Row(modifier = Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.SpaceBetween) {
            Text("Lord of the Year", color = TextSecondary, fontSize = 13.sp)
            Text(formatWesternBodyName(prof.lord_of_the_year), fontWeight = FontWeight.Bold, color = TextPrimary, fontSize = 13.sp)
        }
        if (formattedPlanets.isNotEmpty()) {
            Spacer(modifier = Modifier.height(4.dp))
            Row(modifier = Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.SpaceBetween) {
                Text("Natal Planets in Sign", color = TextSecondary, fontSize = 13.sp)
                Text(formattedPlanets.joinToString(", "), fontWeight = FontWeight.Medium, color = VedicGreen, fontSize = 13.sp)
            }
        }
    }
}

@Composable
private fun SolarArcCard(data: PredictiveSuiteData) {
    val arc = data.solar_arc
    Column {
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            Text("Solar Arc Directions", fontWeight = FontWeight.Bold, fontSize = 14.sp, color = TextPrimary)
            SectionCopyButton(title = "Solar Arc") {
                val sb = StringBuilder()
                sb.appendLine("[Solar Arc Directions - ${data.target_year}년]")
                sb.appendLine("• Solar Arc Degrees: ${String.format("+%.2f°", arc.arc_degrees)}")
                sb.appendLine("• Contacts to Natal Chart (Orb <= 1°):")
                if (arc.contacts.isEmpty()) {
                    sb.appendLine("  (None)")
                } else {
                    arc.contacts.forEach { c ->
                        val moving = formatWesternBodyName(c.moving_body)
                        val natal = formatWesternBodyName(c.natal_body)
                        sb.appendLine("  - $moving · $natal : ${c.aspect_type} (Orb ${String.format("%.2f°", c.orb)})")
                    }
                }
                sb.toString().trimEnd()
            }
        }
        Text("Solar Arc Degrees: ${String.format("+%.2f°", arc.arc_degrees)}", color = TextSecondary, fontSize = 12.sp)
        Spacer(modifier = Modifier.height(8.dp))

        if (arc.contacts.isEmpty()) {
            Text("No major aspect contacts within 1° orb.", color = TextSecondary, fontSize = 12.sp)
        } else {
            Text("Contacts to Natal Chart (Orb ≤ 1°):", fontWeight = FontWeight.SemiBold, fontSize = 13.sp, color = TextPrimary)
            Spacer(modifier = Modifier.height(4.dp))
            arc.contacts.forEach { c ->
                val moving = formatWesternBodyName(c.moving_body)
                val natal = formatWesternBodyName(c.natal_body)
                Row(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(vertical = 3.dp),
                    horizontalArrangement = Arrangement.SpaceBetween
                ) {
                    Text("$moving · $natal", fontSize = 12.sp, color = TextPrimary)
                    Text(
                        "${c.aspect_type} (Orb ${String.format("%.2f°", c.orb)})",
                        fontSize = 12.sp,
                        color = CelestialBlue,
                        fontWeight = FontWeight.Medium
                    )
                }
            }
        }
    }
}

@Composable
private fun SecondaryProgressionCard(data: PredictiveSuiteData) {
    val sec = data.secondary_progression
    if (sec == null) {
        Text("Secondary progression data unavailable.", color = TextSecondary, fontSize = 12.sp)
        return
    }

    Column {
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            Text("Secondary Progressions (1 day = 1 year)", fontWeight = FontWeight.Bold, fontSize = 14.sp, color = TextPrimary)
            SectionCopyButton(title = "Secondary Progressions") {
                val sb = StringBuilder()
                sb.appendLine("[Secondary Progressions - ${data.target_year}년]")
                sb.appendLine("• Progressed Date: ${sec.progressed_date_utc}")
                sb.appendLine("• Progressed Moon Phase: ${sec.progressed_lunar_phase}")
                sb.appendLine("• Contacts to Natal Chart:")
                if (sec.contacts.isEmpty()) {
                    sb.appendLine("  (None)")
                } else {
                    sec.contacts.forEach { c ->
                        val moving = formatWesternBodyName(c.moving_body)
                        val natal = formatWesternBodyName(c.natal_body)
                        sb.appendLine("  - $moving · $natal : ${c.aspect_type} (Orb ${String.format("%.2f°", c.orb)})")
                    }
                }
                sb.toString().trimEnd()
            }
        }
        // 불필요한 Progressed Date 라벨 제거하고 날짜와 경과 연도만 명시
        Text(
            text = "${sec.progressed_date_utc} · ${String.format("%.1f", sec.elapsed_tropical_years)} yrs elapsed",
            color = TextSecondary,
            fontSize = 12.sp
        )
        Spacer(modifier = Modifier.height(6.dp))

        // 긴 중복 접두어 제거하여 배지 잘림 방지
        val cleanPhase = sec.progressed_lunar_phase.removePrefix("Progressed ")
        Surface(
            shape = RoundedCornerShape(4.dp),
            color = CelestialBlue.copy(alpha = 0.1f)
        ) {
            Text(
                text = "Moon Phase: $cleanPhase",
                color = CelestialBlue,
                fontSize = 12.sp,
                fontWeight = FontWeight.SemiBold,
                modifier = Modifier.padding(horizontal = 8.dp, vertical = 4.dp)
            )
        }

        Spacer(modifier = Modifier.height(8.dp))
        if (sec.contacts.isEmpty()) {
            Text("No contacts to natal chart within 1° orb.", color = TextSecondary, fontSize = 12.sp)
        } else {
            Text("Contacts to Natal Chart:", fontWeight = FontWeight.SemiBold, fontSize = 13.sp, color = TextPrimary)
            Spacer(modifier = Modifier.height(4.dp))
            sec.contacts.forEach { c ->
                val moving = formatWesternBodyName(c.moving_body)
                val natal = formatWesternBodyName(c.natal_body)
                Row(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(vertical = 3.dp),
                    horizontalArrangement = Arrangement.SpaceBetween
                ) {
                    Text("$moving · $natal", fontSize = 12.sp, color = TextPrimary)
                    Text("${c.aspect_type} (Orb ${String.format("%.2f°", c.orb)})", fontSize = 12.sp, color = CelestialBlue, fontWeight = FontWeight.Medium)
                }
            }
        }
    }
}

@Composable
private fun PlanetaryReturnsCard(data: PredictiveSuiteData) {
    val sr = data.solar_return
    var selectedPlanetIdx by remember { mutableStateOf(0) }
    val planetOptions = listOf("Sun", "Moon", "Mercury", "Venus", "Mars", "Jupiter", "Saturn")

    if (sr == null) {
        Text("Planetary return data unavailable.", color = TextSecondary, fontSize = 12.sp)
        return
    }

    val ascSign = getZodiacSign(sr.ascendant)
    val ascDeg = getSignDegree(sr.ascendant)
    val mcSign = getZodiacSign(sr.midheaven)
    val mcDeg = getSignDegree(sr.midheaven)

    val classicalPlanets = sr.planets.filter { 
        it.name != "Rahu" && it.name != "Ketu" && it.name != "North Node" && it.name != "South Node" 
    }
    val lunarNodes = sr.planets.filter { 
        it.name == "Rahu" || it.name == "Ketu" || it.name == "North Node" || it.name == "South Node" 
    }

    Column {
        // 1행: 카드 제목과 복사 버튼
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            Text("Planetary Returns", fontWeight = FontWeight.Bold, fontSize = 15.sp, color = TextPrimary)
            SectionCopyButton(title = "Planetary Returns") {
                val sb = StringBuilder()
                sb.appendLine("[Planetary Returns (${planetOptions[selectedPlanetIdx]}) - ${sr.return_year}년]")
                sb.appendLine("• Return Moment: ${sr.return_date_utc}")
                sb.appendLine("• ASC: ${String.format("%.2f°", ascDeg)} $ascSign (Abs: ${String.format("%.2f°", sr.ascendant)})")
                sb.appendLine("• MC: ${String.format("%.2f°", mcDeg)} $mcSign (Abs: ${String.format("%.2f°", sr.midheaven)})")
                if (sr.house_cusps.isNotEmpty()) {
                    sb.appendLine("• House Cusps:")
                    sr.house_cusps.forEachIndexed { idx, c ->
                        val s = getZodiacSign(c)
                        val d = getSignDegree(c)
                        sb.appendLine("  - H${idx + 1}: ${String.format("%.2f°", d)} $s")
                    }
                }
                if (classicalPlanets.isNotEmpty()) {
                    sb.appendLine("• Return Planetary Placements:")
                    classicalPlanets.forEach { p ->
                        val rx = if (p.is_retrograde) " [Rx]" else ""
                        val s = getZodiacSign(p.progressed_longitude)
                        val h = if (sr.house_cusps.isNotEmpty()) "House ${findHouseNumber(p.progressed_longitude, sr.house_cusps)}" else ""
                        sb.appendLine("  - ${p.name}$rx: ${String.format("%.2f°", p.sign_degree)} $s · $h")
                    }
                }
                if (lunarNodes.isNotEmpty()) {
                    sb.appendLine("• Return Lunar Nodes:")
                    lunarNodes.forEach { n ->
                        val displayName = formatWesternBodyName(n.name)
                        val s = getZodiacSign(n.progressed_longitude)
                        val h = if (sr.house_cusps.isNotEmpty()) "House ${findHouseNumber(n.progressed_longitude, sr.house_cusps)}" else ""
                        sb.appendLine("  - $displayName: ${String.format("%.2f°", n.sign_degree)} $s · $h")
                    }
                }
                sb.toString().trimEnd()
            }
        }

        Spacer(modifier = Modifier.height(6.dp))

        // 행성 선택 칩 인터페이스
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .horizontalScroll(rememberScrollState()),
            horizontalArrangement = Arrangement.spacedBy(6.dp)
        ) {
            planetOptions.forEachIndexed { idx, pName ->
                FilterChip(
                    selected = selectedPlanetIdx == idx,
                    onClick = { selectedPlanetIdx = idx },
                    label = {
                        Text(
                            text = if (idx == 0) "$pName (Solar)" else pName,
                            fontSize = 11.sp,
                            fontWeight = if (selectedPlanetIdx == idx) FontWeight.Bold else FontWeight.Normal
                        )
                    },
                    colors = FilterChipDefaults.filterChipColors(
                        selectedContainerColor = CelestialBlue,
                        selectedLabelColor = BackgroundWhite,
                        containerColor = SurfaceLight,
                        labelColor = TextSecondary
                    ),
                    border = FilterChipDefaults.filterChipBorder(
                        enabled = true,
                        selected = selectedPlanetIdx == idx,
                        borderColor = BorderColor,
                        selectedBorderColor = CelestialBlue
                    )
                )
            }
        }

        if (selectedPlanetIdx != 0) {
            Surface(
                color = SurfaceLight,
                shape = RoundedCornerShape(6.dp),
                border = BorderStroke(1.dp, BorderColor),
                modifier = Modifier.fillMaxWidth().padding(vertical = 4.dp)
            ) {
                Text(
                    text = "${planetOptions[selectedPlanetIdx]} Return is currently in development. Solar Return chart is displayed.",
                    color = TextSecondary,
                    fontSize = 11.sp,
                    modifier = Modifier.padding(horizontal = 8.dp, vertical = 4.dp)
                )
            }
        }

        Spacer(modifier = Modifier.height(6.dp))

        // 중복 Return Year 라벨 제거하고 정확한 회귀 시각만 표시
        Text(
            text = "Exact Return Moment: ${sr.return_date_utc}",
            color = TextSecondary,
            fontSize = 12.sp
        )

        Spacer(modifier = Modifier.height(10.dp))

        // Return Chart Angles (ASC / MC)
        Card(
            shape = RoundedCornerShape(8.dp),
            colors = CardDefaults.cardColors(containerColor = SurfaceLight),
            modifier = Modifier.fillMaxWidth()
        ) {
            Column(modifier = Modifier.padding(10.dp)) {
                Row(modifier = Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.SpaceBetween) {
                    Text("Return Ascendant", color = TextSecondary, fontSize = 12.sp)
                    Text("${String.format("%.2f°", ascDeg)} $ascSign", fontWeight = FontWeight.Bold, color = TextPrimary, fontSize = 12.sp)
                }
                Spacer(modifier = Modifier.height(4.dp))
                Row(modifier = Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.SpaceBetween) {
                    Text("Return Midheaven (MC)", color = TextSecondary, fontSize = 12.sp)
                    Text("${String.format("%.2f°", mcDeg)} $mcSign", fontWeight = FontWeight.Bold, color = TextPrimary, fontSize = 12.sp)
                }
            }
        }

        // Return House Cusps (1~12)
        if (sr.house_cusps.isNotEmpty()) {
            Spacer(modifier = Modifier.height(10.dp))
            Text("Return House Cusps", fontWeight = FontWeight.SemiBold, fontSize = 12.sp, color = TextPrimary)
            Spacer(modifier = Modifier.height(4.dp))
            Column(modifier = Modifier.fillMaxWidth()) {
                val chunks = sr.house_cusps.chunked(2)
                chunks.forEachIndexed { rowIdx, pair ->
                    Row(
                        modifier = Modifier
                            .fillMaxWidth()
                            .padding(vertical = 2.dp),
                        horizontalArrangement = Arrangement.SpaceBetween
                    ) {
                        val h1 = rowIdx * 2 + 1
                        val s1 = getZodiacSign(pair[0])
                        val d1 = getSignDegree(pair[0])
                        Text("H$h1: ${String.format("%.1f°", d1)} $s1", fontSize = 11.sp, color = TextSecondary, modifier = Modifier.weight(1f))

                        if (pair.size > 1) {
                            val h2 = rowIdx * 2 + 2
                            val s2 = getZodiacSign(pair[1])
                            val d2 = getSignDegree(pair[1])
                            Text("H$h2: ${String.format("%.1f°", d2)} $s2", fontSize = 11.sp, color = TextSecondary, modifier = Modifier.weight(1f))
                        }
                    }
                }
            }
        }

        // 7대 고전 행성 배치 표
        if (classicalPlanets.isNotEmpty()) {
            Spacer(modifier = Modifier.height(10.dp))
            Text("Return Planetary Placements", fontWeight = FontWeight.SemiBold, fontSize = 12.sp, color = TextPrimary)
            Spacer(modifier = Modifier.height(4.dp))
            classicalPlanets.forEach { p ->
                val rx = if (p.is_retrograde) " [Rx]" else ""
                val signName = getZodiacSign(p.progressed_longitude)
                val houseNum = if (sr.house_cusps.isNotEmpty()) findHouseNumber(p.progressed_longitude, sr.house_cusps) else null

                Row(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(vertical = 3.dp),
                    horizontalArrangement = Arrangement.SpaceBetween,
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    Text("${p.name}$rx", fontSize = 12.sp, fontWeight = FontWeight.Medium, color = TextPrimary)
                    Text(
                        text = "${String.format("%.2f°", p.sign_degree)} $signName" + (if (houseNum != null) " · H$houseNum" else ""),
                        fontSize = 12.sp,
                        color = CelestialBlue
                    )
                }
                HorizontalDivider(color = BorderColor.copy(alpha = 0.4f))
            }
        }

        // 달의 교점 (North Node & South Node) 분리 표
        if (lunarNodes.isNotEmpty()) {
            Spacer(modifier = Modifier.height(10.dp))
            Text("Return Lunar Nodes", fontWeight = FontWeight.SemiBold, fontSize = 12.sp, color = TextPrimary)
            Spacer(modifier = Modifier.height(4.dp))
            lunarNodes.forEach { n ->
                val displayName = formatWesternBodyName(n.name)
                val signName = getZodiacSign(n.progressed_longitude)
                val houseNum = if (sr.house_cusps.isNotEmpty()) findHouseNumber(n.progressed_longitude, sr.house_cusps) else null

                Row(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(vertical = 3.dp),
                    horizontalArrangement = Arrangement.SpaceBetween,
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    Text(displayName, fontSize = 12.sp, fontWeight = FontWeight.Medium, color = TextPrimary)
                    Text(
                        text = "${String.format("%.2f°", n.sign_degree)} $signName" + (if (houseNum != null) " · H$houseNum" else ""),
                        fontSize = 12.sp,
                        color = CelestialBlue
                    )
                }
                HorizontalDivider(color = BorderColor.copy(alpha = 0.4f))
            }
        }
    }
}

@Composable
private fun TransitsCard(
    data: PredictiveSuiteData,
    profile: ProfileDto,
    onDateChange: ((Int, Int, Int) -> Unit)? = null,
    onTimeChange: ((Int?, Int?) -> Unit)? = null
) {
    val context = LocalContext.current

    val openDatePicker = {
        val dpd = android.app.DatePickerDialog(
            context,
            { _, year, monthOfYear, dayOfMonth ->
                onDateChange?.invoke(year, monthOfYear + 1, dayOfMonth)
            },
            data.target_year,
            (data.target_month.toInt() - 1).coerceIn(0, 11),
            data.target_day.toInt()
        )
        dpd.show()
    }

    val shiftDays = { days: Int ->
        val cal = java.util.Calendar.getInstance()
        cal.set(data.target_year, (data.target_month.toInt() - 1).coerceIn(0, 11), data.target_day.toInt())
        cal.add(java.util.Calendar.DAY_OF_MONTH, days)
        onDateChange?.invoke(
            cal.get(java.util.Calendar.YEAR),
            cal.get(java.util.Calendar.MONTH) + 1,
            cal.get(java.util.Calendar.DAY_OF_MONTH)
        )
    }

    val setToToday = {
        val now = java.util.Calendar.getInstance()
        onDateChange?.invoke(
            now.get(java.util.Calendar.YEAR),
            now.get(java.util.Calendar.MONTH) + 1,
            now.get(java.util.Calendar.DAY_OF_MONTH)
        )
    }

    val setToBirthDay = {
        onDateChange?.invoke(
            data.target_year,
            profile.birth_month.toInt(),
            profile.birth_day.toInt()
        )
    }

    Column {
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            Column {
                Text("Transits (${data.target_year})", fontWeight = FontWeight.Bold, fontSize = 14.sp, color = TextPrimary)
                // 트랜짓 계산 기준 시점 명시 (출생 시각 및 분 기준)
                Text(
                    text = "${data.target_year}-${String.format("%02d", data.target_month)}-${String.format("%02d", data.target_day)} ${String.format("%02d:%02d", data.target_hour, data.target_minute)} Local",
                    color = CelestialBlue,
                    fontWeight = FontWeight.SemiBold,
                    fontSize = 12.sp,
                    modifier = Modifier.padding(top = 2.dp)
                )
            }
            SectionCopyButton(title = "Transits") {
                val sb = StringBuilder()
                sb.appendLine("[Transits - ${data.target_year}년 (${data.target_year}-${String.format("%02d", data.target_month)}-${String.format("%02d", data.target_day)} ${String.format("%02d:%02d", data.target_hour, data.target_minute)} Local)]")
                if (data.transits.isEmpty()) {
                    sb.appendLine("  (No major aspects within 2° orb)")
                } else {
                    data.transits.forEach { t ->
                        val trPlanet = formatWesternBodyName(t.transiting_planet)
                        val natPlanet = formatWesternBodyName(t.natal_planet)
                        sb.appendLine("• Tr $trPlanet · Nat $natPlanet: ${t.aspect_type} (Orb ${String.format("%.2f°", t.orb)})")
                    }
                }
                sb.toString().trimEnd()
            }
        }

        Spacer(modifier = Modifier.height(8.dp))

        // 트랜짓 날짜 조정 빠른 칩 (오늘, 날짜 선택, 출생일, -1일, +1일)
        Text(
            text = "날짜 (Date)",
            fontSize = 11.sp,
            fontWeight = FontWeight.Medium,
            color = TextSecondary
        )
        Spacer(modifier = Modifier.height(4.dp))
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .horizontalScroll(rememberScrollState()),
            horizontalArrangement = Arrangement.spacedBy(6.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            FilterChip(
                selected = false,
                onClick = { setToToday() },
                label = { Text("오늘", fontSize = 11.sp, fontWeight = FontWeight.SemiBold) }
            )
            FilterChip(
                selected = false,
                onClick = { openDatePicker() },
                label = { Text("날짜 선택...", fontSize = 11.sp) }
            )
            FilterChip(
                selected = data.target_month.toInt() == profile.birth_month.toInt() && data.target_day.toInt() == profile.birth_day.toInt(),
                onClick = { setToBirthDay() },
                label = { Text("출생일", fontSize = 11.sp) }
            )
            FilterChip(
                selected = false,
                onClick = { shiftDays(-1) },
                label = { Text("-1일", fontSize = 11.sp) }
            )
            FilterChip(
                selected = false,
                onClick = { shiftDays(1) },
                label = { Text("+1일", fontSize = 11.sp) }
            )
        }

        Spacer(modifier = Modifier.height(8.dp))

        // 트랜짓 시간 조정 빠른 칩
        Text(
            text = "시각 (Time)",
            fontSize = 11.sp,
            fontWeight = FontWeight.Medium,
            color = TextSecondary
        )
        Spacer(modifier = Modifier.height(4.dp))
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .horizontalScroll(rememberScrollState()),
            horizontalArrangement = Arrangement.spacedBy(6.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            FilterChip(
                selected = false,
                onClick = { onTimeChange?.invoke(null, null) },
                label = { Text("출생시각", fontSize = 11.sp) }
            )
            FilterChip(
                selected = data.target_hour == 12 && data.target_minute == 0,
                onClick = { onTimeChange?.invoke(12, 0) },
                label = { Text("12:00 정오", fontSize = 11.sp) }
            )
            FilterChip(
                selected = false,
                onClick = {
                    val prev = (data.target_hour + 23) % 24
                    onTimeChange?.invoke(prev, data.target_minute)
                },
                label = { Text("-1시간", fontSize = 11.sp) }
            )
            FilterChip(
                selected = false,
                onClick = {
                    val next = (data.target_hour + 1) % 24
                    onTimeChange?.invoke(next, data.target_minute)
                },
                label = { Text("+1시간", fontSize = 11.sp) }
            )
        }

        Spacer(modifier = Modifier.height(10.dp))

        if (data.transits.isEmpty()) {
            Text("No major transiting aspects within 2° orb.", color = TextSecondary, fontSize = 12.sp)
        } else {
            data.transits.take(10).forEach { t ->
                val trPlanet = formatWesternBodyName(t.transiting_planet)
                val natPlanet = formatWesternBodyName(t.natal_planet)
                Row(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(vertical = 3.dp),
                    horizontalArrangement = Arrangement.SpaceBetween
                ) {
                    Text("Tr $trPlanet · Nat $natPlanet", fontSize = 12.sp, color = TextPrimary)
                    Text("${t.aspect_type} (Orb ${String.format("%.2f°", t.orb)})", fontSize = 12.sp, color = CelestialBlue, fontWeight = FontWeight.Medium)
                }
            }
        }
    }
}

@Composable
private fun PrimaryDirectionsCard(data: PredictiveSuiteData) {
    Column {
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            Text("Primary Directions (Naibod Key)", fontWeight = FontWeight.Bold, fontSize = 14.sp, color = TextPrimary)
            SectionCopyButton(title = "Primary Directions") {
                val sb = StringBuilder()
                sb.appendLine("[Primary Directions (Naibod Key) - ${data.target_year}년]")
                if (data.primary_directions.isEmpty()) {
                    sb.appendLine("  (None calculated)")
                } else {
                    data.primary_directions.forEach { pd ->
                        val prom = formatWesternBodyName(pd.promittor)
                        val sig = formatWesternBodyName(pd.significator)
                        sb.appendLine("• $prom → $sig ${pd.aspect_type} | Realization: ~${String.format("%.1f", pd.realization_age)} yrs (Arc: ${String.format("%.2f°", pd.direction_arc_deg)})")
                    }
                }
                sb.toString().trimEnd()
            }
        }
        Spacer(modifier = Modifier.height(8.dp))

        if (data.primary_directions.isEmpty()) {
            Text("No primary directions calculated.", color = TextSecondary, fontSize = 12.sp)
        } else {
            data.primary_directions.take(8).forEach { pd ->
                val prom = formatWesternBodyName(pd.promittor)
                val sig = formatWesternBodyName(pd.significator)
                Column(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(vertical = 5.dp)
                ) {
                    Row(
                        modifier = Modifier.fillMaxWidth(),
                        horizontalArrangement = Arrangement.SpaceBetween,
                        verticalAlignment = Alignment.CenterVertically
                    ) {
                        Text(
                            text = "$prom → $sig",
                            fontSize = 13.sp,
                            fontWeight = FontWeight.Bold,
                            color = TextPrimary
                        )
                        Surface(
                            shape = RoundedCornerShape(4.dp),
                            color = CelestialBlue.copy(alpha = 0.12f)
                        ) {
                            Text(
                                text = pd.aspect_type,
                                color = CelestialBlue,
                                fontSize = 11.sp,
                                fontWeight = FontWeight.SemiBold,
                                maxLines = 1,
                                softWrap = false,
                                modifier = Modifier.padding(horizontal = 6.dp, vertical = 2.dp)
                            )
                        }
                    }
                    Spacer(modifier = Modifier.height(2.dp))
                    Text(
                        text = "Realization: ~${String.format("%.1f", pd.realization_age)} yrs · Arc: ${String.format("%.2f°", pd.direction_arc_deg)}",
                        fontSize = 12.sp,
                        color = TextSecondary
                    )
                }
                HorizontalDivider(color = BorderColor.copy(alpha = 0.4f))
            }
        }
    }
}
