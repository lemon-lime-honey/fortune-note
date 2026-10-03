package com.fortunexample.note.ui.components

import androidx.compose.animation.AnimatedVisibility
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.horizontalScroll
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.KeyboardArrowDown
import androidx.compose.material.icons.filled.KeyboardArrowUp
import androidx.compose.material.icons.filled.Remove
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
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

@Composable
fun WesternPredictiveSection(
    profile: ProfileDto,
    predictiveData: PredictiveSuiteData?,
    targetYear: Int,
    isLoading: Boolean,
    onYearChange: (Int) -> Unit
) {
    var selectedTechTab by remember { mutableStateOf(0) }
    val techTabs = listOf(
        "Profections",
        "Solar Arc",
        "Progressions",
        "Solar Return",
        "Transits",
        "Primary Dir"
    )

    Card(
        shape = RoundedCornerShape(12.dp),
        colors = CardDefaults.cardColors(containerColor = BackgroundWhite),
        border = BorderStroke(1.dp, BorderColor),
        modifier = Modifier.fillMaxWidth()
    ) {
        Column(modifier = Modifier.padding(16.dp)) {
            // Header with year picker
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically
            ) {
                Column {
                    Text(
                        text = "Predictive Astrology Suite",
                        color = TextPrimary,
                        fontWeight = FontWeight.Bold,
                        fontSize = 15.sp
                    )
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
                        label = { Text(title, fontSize = 12.sp, fontWeight = if (selectedTechTab == idx) FontWeight.Bold else FontWeight.Normal) },
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

            if (isLoading) {
                Box(
                    modifier = Modifier
                        .fillMaxWidth()
                        .height(140.dp),
                    contentAlignment = Alignment.Center
                ) {
                    CircularProgressIndicator(color = AccentPrimary, modifier = Modifier.size(32.dp))
                }
            } else if (predictiveData != null) {
                when (selectedTechTab) {
                    0 -> ProfectionCard(predictiveData)
                    1 -> SolarArcCard(predictiveData)
                    2 -> SecondaryProgressionCard(predictiveData)
                    3 -> SolarReturnCard(predictiveData)
                    4 -> TransitsCard(predictiveData)
                    5 -> PrimaryDirectionsCard(predictiveData)
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
private fun ProfectionCard(data: PredictiveSuiteData) {
    val prof = data.profection
    Column {
        Text("Annual Profections", fontWeight = FontWeight.Bold, fontSize = 14.sp, color = TextPrimary)
        Spacer(modifier = Modifier.height(8.dp))
        Row(modifier = Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.SpaceBetween) {
            Text("Profected House & Sign", color = TextSecondary, fontSize = 13.sp)
            Text("House ${prof.profected_house} · ${prof.profected_sign_name}", fontWeight = FontWeight.Bold, color = CelestialBlue, fontSize = 13.sp)
        }
        Row(modifier = Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.SpaceBetween) {
            Text("Lord of the Year", color = TextSecondary, fontSize = 13.sp)
            Text(prof.lord_of_the_year, fontWeight = FontWeight.Bold, color = TextPrimary, fontSize = 13.sp)
        }
        if (prof.active_natal_planets.isNotEmpty()) {
            Spacer(modifier = Modifier.height(4.dp))
            Row(modifier = Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.SpaceBetween) {
                Text("Natal Planets in Sign", color = TextSecondary, fontSize = 13.sp)
                Text(prof.active_natal_planets.joinToString(", "), fontWeight = FontWeight.Medium, color = VedicGreen, fontSize = 13.sp)
            }
        }
    }
}

@Composable
private fun SolarArcCard(data: PredictiveSuiteData) {
    val arc = data.solar_arc
    Column {
        Text("Solar Arc Directions", fontWeight = FontWeight.Bold, fontSize = 14.sp, color = TextPrimary)
        Text("Solar Arc Degrees: ${String.format("+%.2f°", arc.arc_degrees)}", color = TextSecondary, fontSize = 12.sp)
        Spacer(modifier = Modifier.height(8.dp))

        if (arc.contacts.isEmpty()) {
            Text("No major aspect contacts within 1° orb.", color = TextSecondary, fontSize = 12.sp)
        } else {
            Text("Contacts to Natal Chart (Orb ≤ 1°):", fontWeight = FontWeight.SemiBold, fontSize = 13.sp, color = TextPrimary)
            arc.contacts.forEach { c ->
                Row(
                    modifier = Modifier.fillMaxWidth().padding(vertical = 3.dp),
                    horizontalArrangement = Arrangement.SpaceBetween
                ) {
                    Text("${c.moving_body} · ${c.natal_body}", fontSize = 12.sp, color = TextPrimary)
                    Text("${c.aspect_type} (Orb ${String.format("%.2f°", c.orb)})", fontSize = 12.sp, color = CelestialBlue, fontWeight = FontWeight.Medium)
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
        Text("Secondary Progressions (1 day = 1 year)", fontWeight = FontWeight.Bold, fontSize = 14.sp, color = TextPrimary)
        Text("Progressed Date: ${sec.progressed_date_utc} UTC (${String.format("%.1f", sec.elapsed_tropical_years)} yrs elapsed)", color = TextSecondary, fontSize = 12.sp)
        Spacer(modifier = Modifier.height(6.dp))

        Surface(
            shape = RoundedCornerShape(4.dp),
            color = CelestialBlue.copy(alpha = 0.1f)
        ) {
            Text(
                text = "Progressed Moon Phase: ${sec.progressed_lunar_phase}",
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
            sec.contacts.forEach { c ->
                Row(
                    modifier = Modifier.fillMaxWidth().padding(vertical = 3.dp),
                    horizontalArrangement = Arrangement.SpaceBetween
                ) {
                    Text("${c.moving_body} · ${c.natal_body}", fontSize = 12.sp, color = TextPrimary)
                    Text("${c.aspect_type} (Orb ${String.format("%.2f°", c.orb)})", fontSize = 12.sp, color = CelestialBlue, fontWeight = FontWeight.Medium)
                }
            }
        }
    }
}

@Composable
private fun SolarReturnCard(data: PredictiveSuiteData) {
    val sr = data.solar_return
    if (sr == null) {
        Text("Solar return data unavailable.", color = TextSecondary, fontSize = 12.sp)
        return
    }

    val ascSign = ZODIAC_NAMES.getOrElse((sr.ascendant / 30.0).toInt() % 12) { "" }
    val mcSign = ZODIAC_NAMES.getOrElse((sr.midheaven / 30.0).toInt() % 12) { "" }

    Column {
        Text("Solar Return (${sr.return_year})", fontWeight = FontWeight.Bold, fontSize = 14.sp, color = TextPrimary)
        Text("Exact Return Moment: ${sr.return_date_utc}", color = TextSecondary, fontSize = 12.sp)
        Spacer(modifier = Modifier.height(8.dp))

        Row(modifier = Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.SpaceBetween) {
            Text("Return Ascendant", color = TextSecondary, fontSize = 13.sp)
            Text("${String.format("%.2f°", sr.ascendant)} ($ascSign)", fontWeight = FontWeight.Bold, color = TextPrimary, fontSize = 13.sp)
        }
        Row(modifier = Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.SpaceBetween) {
            Text("Return Midheaven (MC)", color = TextSecondary, fontSize = 13.sp)
            Text("${String.format("%.2f°", sr.midheaven)} ($mcSign)", fontWeight = FontWeight.Bold, color = TextPrimary, fontSize = 13.sp)
        }
    }
}

@Composable
private fun TransitsCard(data: PredictiveSuiteData) {
    Column {
        Text("Transits (${data.target_year})", fontWeight = FontWeight.Bold, fontSize = 14.sp, color = TextPrimary)
        Spacer(modifier = Modifier.height(8.dp))

        if (data.transits.isEmpty()) {
            Text("No major transiting aspects within 2° orb.", color = TextSecondary, fontSize = 12.sp)
        } else {
            data.transits.take(10).forEach { t ->
                Row(
                    modifier = Modifier.fillMaxWidth().padding(vertical = 3.dp),
                    horizontalArrangement = Arrangement.SpaceBetween
                ) {
                    Text("Tr ${t.transiting_planet} · Nat ${t.natal_planet}", fontSize = 12.sp, color = TextPrimary)
                    Text("${t.aspect_type} (Orb ${String.format("%.2f°", t.orb)})", fontSize = 12.sp, color = CelestialBlue, fontWeight = FontWeight.Medium)
                }
            }
        }
    }
}

@Composable
private fun PrimaryDirectionsCard(data: PredictiveSuiteData) {
    Column {
        Text("Primary Directions (Naibod Key)", fontWeight = FontWeight.Bold, fontSize = 14.sp, color = TextPrimary)
        Spacer(modifier = Modifier.height(8.dp))

        if (data.primary_directions.isEmpty()) {
            Text("No primary directions calculated.", color = TextSecondary, fontSize = 12.sp)
        } else {
            data.primary_directions.take(8).forEach { pd ->
                Row(
                    modifier = Modifier.fillMaxWidth().padding(vertical = 3.dp),
                    horizontalArrangement = Arrangement.SpaceBetween
                ) {
                    Text("${pd.promittor} → ${pd.significator} ${pd.aspect_type}", fontSize = 12.sp, color = TextPrimary)
                    Text("Realization: ~${String.format("%.1f", pd.realization_age)} yrs (Arc: ${String.format("%.2f°", pd.direction_arc_deg)})", fontSize = 12.sp, color = CelestialBlue, fontWeight = FontWeight.Medium)
                }
            }
        }
    }
}
