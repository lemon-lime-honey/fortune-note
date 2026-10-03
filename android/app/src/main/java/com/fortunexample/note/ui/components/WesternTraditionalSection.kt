package com.fortunexample.note.ui.components

import androidx.compose.animation.AnimatedVisibility
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.KeyboardArrowDown
import androidx.compose.material.icons.filled.KeyboardArrowUp
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.fortunexample.note.data.WesternTraditionalData
import com.fortunexample.note.ui.theme.*

private val ZODIAC_NAMES = listOf(
    "Aries", "Taurus", "Gemini", "Cancer", "Leo", "Virgo",
    "Libra", "Scorpio", "Sagittarius", "Capricorn", "Aquarius", "Pisces"
)

@Composable
fun WesternTraditionalSection(
    traditional: WesternTraditionalData?
) {
    if (traditional == null) return

    var isDignityExpanded by remember { mutableStateOf(true) }
    var isAspectsExpanded by remember { mutableStateOf(false) }
    var isStarsExpanded by remember { mutableStateOf(true) }
    var isPartsExpanded by remember { mutableStateOf(false) }

    Column(modifier = Modifier.fillMaxWidth()) {
        // 1. Planetary Hours & Radicality Card
        Card(
            shape = RoundedCornerShape(12.dp),
            colors = CardDefaults.cardColors(containerColor = BackgroundWhite),
            border = BorderStroke(1.dp, BorderColor),
            modifier = Modifier.fillMaxWidth()
        ) {
            Column(modifier = Modifier.padding(16.dp)) {
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.SpaceBetween,
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    Text("Planetary Hours & Radicality", color = TextPrimary, fontWeight = FontWeight.Bold, fontSize = 15.sp)
                    Surface(
                        shape = RoundedCornerShape(6.dp),
                        color = if (traditional.planetary_hours.is_radical) VedicGreen.copy(alpha = 0.12f) else AstrologicalRed.copy(alpha = 0.12f)
                    ) {
                        Text(
                            text = if (traditional.planetary_hours.is_radical) "Radical" else "Non-radical",
                            color = if (traditional.planetary_hours.is_radical) VedicGreen else AstrologicalRed,
                            fontWeight = FontWeight.Bold,
                            fontSize = 12.sp,
                            modifier = Modifier.padding(horizontal = 8.dp, vertical = 4.dp)
                        )
                    }
                }
                Spacer(modifier = Modifier.height(10.dp))
                Text(
                    text = "Day of ${traditional.planetary_hours.day_ruler} · Hour of ${traditional.planetary_hours.hour_ruler} (Hour ${traditional.planetary_hours.hour_number}, ${if (traditional.planetary_hours.is_day_hour) "Day" else "Night"})",
                    color = TextPrimary,
                    fontSize = 13.sp
                )
                Text(
                    text = traditional.planetary_hours.radicality_reason,
                    color = TextSecondary,
                    fontSize = 12.sp
                )

                if (traditional.planetary_hours.considerations.isNotEmpty()) {
                    Spacer(modifier = Modifier.height(8.dp))
                    Text(
                        text = "Considerations Before Judgment:",
                        color = AstrologicalRed,
                        fontWeight = FontWeight.SemiBold,
                        fontSize = 12.sp
                    )
                    traditional.planetary_hours.considerations.forEach { c ->
                        Text(
                            text = "• $c",
                            color = TextSecondary,
                            fontSize = 12.sp,
                            modifier = Modifier.padding(start = 4.dp, top = 2.dp)
                        )
                    }
                }
            }
        }

        Spacer(modifier = Modifier.height(12.dp))

        // 2. Moon Condition Card
        Card(
            shape = RoundedCornerShape(12.dp),
            colors = CardDefaults.cardColors(containerColor = BackgroundWhite),
            border = BorderStroke(1.dp, BorderColor),
            modifier = Modifier.fillMaxWidth()
        ) {
            Column(modifier = Modifier.padding(16.dp)) {
                Text("Moon Condition", color = TextPrimary, fontWeight = FontWeight.Bold, fontSize = 15.sp)
                Spacer(modifier = Modifier.height(10.dp))

                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.spacedBy(8.dp)
                ) {
                    Surface(
                        shape = RoundedCornerShape(6.dp),
                        color = CelestialBlue.copy(alpha = 0.1f)
                    ) {
                        Text(
                            text = traditional.moon_condition.lunar_phase_name,
                            color = CelestialBlue,
                            fontSize = 12.sp,
                            fontWeight = FontWeight.Medium,
                            modifier = Modifier.padding(horizontal = 8.dp, vertical = 4.dp)
                        )
                    }

                    if (traditional.moon_condition.is_void_of_course) {
                        Surface(
                            shape = RoundedCornerShape(6.dp),
                            color = AstrologicalRed.copy(alpha = 0.12f)
                        ) {
                            Text(
                                text = "Void of Course",
                                color = AstrologicalRed,
                                fontSize = 12.sp,
                                fontWeight = FontWeight.Bold,
                                modifier = Modifier.padding(horizontal = 8.dp, vertical = 4.dp)
                            )
                        }
                    }

                    if (traditional.moon_condition.is_via_combusta) {
                        Surface(
                            shape = RoundedCornerShape(6.dp),
                            color = AstrologicalRed.copy(alpha = 0.12f)
                        ) {
                            Text(
                                text = "Via Combusta",
                                color = AstrologicalRed,
                                fontSize = 12.sp,
                                fontWeight = FontWeight.Bold,
                                modifier = Modifier.padding(horizontal = 8.dp, vertical = 4.dp)
                            )
                        }
                    }
                }

                traditional.moon_condition.next_applying_aspect?.let { nextAsp ->
                    Spacer(modifier = Modifier.height(8.dp))
                    Text(
                        text = "Next Applying Aspect: $nextAsp",
                        color = TextSecondary,
                        fontSize = 13.sp
                    )
                }
            }
        }

        Spacer(modifier = Modifier.height(12.dp))

        // 3. Essential Dignities & Lilly Scores Table Card
        Card(
            shape = RoundedCornerShape(12.dp),
            colors = CardDefaults.cardColors(containerColor = BackgroundWhite),
            border = BorderStroke(1.dp, BorderColor),
            modifier = Modifier.fillMaxWidth()
        ) {
            Column(modifier = Modifier.padding(16.dp)) {
                Row(
                    modifier = Modifier
                        .fillMaxWidth()
                        .clickable { isDignityExpanded = !isDignityExpanded },
                    horizontalArrangement = Arrangement.SpaceBetween,
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    Text("Essential Dignities & Lilly Scores", color = TextPrimary, fontWeight = FontWeight.Bold, fontSize = 15.sp)
                    Icon(
                        if (isDignityExpanded) Icons.Default.KeyboardArrowUp else Icons.Default.KeyboardArrowDown,
                        contentDescription = null,
                        tint = TextSecondary
                    )
                }

                AnimatedVisibility(visible = isDignityExpanded) {
                    Column(modifier = Modifier.padding(top = 10.dp)) {
                        traditional.planet_dignities.forEach { d ->
                            val statusBadges = mutableListOf<Pair<String, Boolean>>()
                            if (d.is_in_domicile) statusBadges.add("Domicile +5" to true)
                            if (d.is_in_exaltation) statusBadges.add("Exaltation +4" to true)
                            if (d.is_in_triplicity) statusBadges.add("Triplicity +3" to true)
                            if (d.is_in_term) statusBadges.add("Term +2" to true)
                            if (d.is_in_face) statusBadges.add("Face +1" to true)
                            if (d.is_in_detriment) statusBadges.add("Detriment -5" to false)
                            if (d.is_in_fall) statusBadges.add("Fall -4" to false)
                            if (d.is_peregrine) statusBadges.add("Peregrine -5" to false)

                            Row(
                                modifier = Modifier
                                    .fillMaxWidth()
                                    .padding(vertical = 6.dp),
                                horizontalArrangement = Arrangement.SpaceBetween,
                                verticalAlignment = Alignment.CenterVertically
                            ) {
                                Column(modifier = Modifier.weight(1f)) {
                                    Text(
                                        text = "${d.planet} · House ${d.house_number}",
                                        fontWeight = FontWeight.Bold,
                                        fontSize = 14.sp,
                                        color = TextPrimary
                                    )
                                    Row(
                                        horizontalArrangement = Arrangement.spacedBy(4.dp),
                                        modifier = Modifier.padding(top = 2.dp)
                                    ) {
                                        if (statusBadges.isEmpty()) {
                                            Text("Neutral", color = TextSecondary, fontSize = 11.sp)
                                        } else {
                                            statusBadges.forEach { (badge, isPositive) ->
                                                Surface(
                                                    shape = RoundedCornerShape(4.dp),
                                                    color = if (isPositive) VedicGreen.copy(alpha = 0.1f) else AstrologicalRed.copy(alpha = 0.1f)
                                                ) {
                                                    Text(
                                                        text = badge,
                                                        color = if (isPositive) VedicGreen else AstrologicalRed,
                                                        fontSize = 10.sp,
                                                        fontWeight = FontWeight.SemiBold,
                                                        modifier = Modifier.padding(horizontal = 4.dp, vertical = 2.dp)
                                                    )
                                                }
                                            }
                                        }
                                    }
                                }

                                Column(horizontalAlignment = Alignment.End) {
                                    Text(
                                        text = String.format("%+d", d.total_lilly_score),
                                        fontWeight = FontWeight.Bold,
                                        fontSize = 15.sp,
                                        color = if (d.total_lilly_score >= 0) CelestialBlue else AstrologicalRed
                                    )
                                    Text(
                                        text = "Essential ${String.format("%+d", d.essential_score)} / Accidental ${String.format("%+d", d.accidental_score)}",
                                        color = TextSecondary,
                                        fontSize = 11.sp
                                    )
                                }
                            }
                            HorizontalDivider(color = BorderColor.copy(alpha = 0.5f))
                        }
                    }
                }
            }
        }

        Spacer(modifier = Modifier.height(12.dp))

        // 4. Fixed Star Conjunctions Card (Orb <= 1.5°)
        Card(
            shape = RoundedCornerShape(12.dp),
            colors = CardDefaults.cardColors(containerColor = BackgroundWhite),
            border = BorderStroke(1.dp, BorderColor),
            modifier = Modifier.fillMaxWidth()
        ) {
            Column(modifier = Modifier.padding(16.dp)) {
                Row(
                    modifier = Modifier
                        .fillMaxWidth()
                        .clickable { isStarsExpanded = !isStarsExpanded },
                    horizontalArrangement = Arrangement.SpaceBetween,
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    Text(
                        text = "Fixed Star Conjunctions (${traditional.fixed_star_conjunctions.size} in orb)",
                        color = TextPrimary,
                        fontWeight = FontWeight.Bold,
                        fontSize = 15.sp
                    )
                    Icon(
                        if (isStarsExpanded) Icons.Default.KeyboardArrowUp else Icons.Default.KeyboardArrowDown,
                        contentDescription = null,
                        tint = TextSecondary
                    )
                }

                AnimatedVisibility(visible = isStarsExpanded) {
                    Column(modifier = Modifier.padding(top = 10.dp)) {
                        if (traditional.fixed_star_conjunctions.isEmpty()) {
                            Text(
                                text = "No fixed stars conjunct within 1.5° orb.",
                                color = TextSecondary,
                                fontSize = 13.sp
                            )
                        } else {
                            traditional.fixed_star_conjunctions.forEach { fs ->
                                Column(modifier = Modifier.padding(vertical = 4.dp)) {
                                    Row(
                                        modifier = Modifier.fillMaxWidth(),
                                        horizontalArrangement = Arrangement.SpaceBetween
                                    ) {
                                        Text(
                                            text = "${fs.star_name} ★ · ${fs.planet_name}",
                                            fontWeight = FontWeight.Bold,
                                            fontSize = 13.sp,
                                            color = TextPrimary
                                        )
                                        Text(
                                            text = "Orb ${String.format("%.2f°", fs.orb)} · ${fs.nature}",
                                            color = CelestialBlue,
                                            fontSize = 12.sp,
                                            fontWeight = FontWeight.Medium
                                        )
                                    }
                                    Text(
                                        text = fs.description,
                                        color = TextSecondary,
                                        fontSize = 12.sp
                                    )
                                }
                                HorizontalDivider(color = BorderColor.copy(alpha = 0.5f))
                            }
                        }
                    }
                }
            }
        }

        Spacer(modifier = Modifier.height(12.dp))

        // 5. Arabic Parts & Cusp Almutens Card
        Card(
            shape = RoundedCornerShape(12.dp),
            colors = CardDefaults.cardColors(containerColor = BackgroundWhite),
            border = BorderStroke(1.dp, BorderColor),
            modifier = Modifier.fillMaxWidth()
        ) {
            Column(modifier = Modifier.padding(16.dp)) {
                Row(
                    modifier = Modifier
                        .fillMaxWidth()
                        .clickable { isPartsExpanded = !isPartsExpanded },
                    horizontalArrangement = Arrangement.SpaceBetween,
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    Text("Arabic Parts & House Cusp Almutens", color = TextPrimary, fontWeight = FontWeight.Bold, fontSize = 15.sp)
                    Icon(
                        if (isPartsExpanded) Icons.Default.KeyboardArrowUp else Icons.Default.KeyboardArrowDown,
                        contentDescription = null,
                        tint = TextSecondary
                    )
                }

                AnimatedVisibility(visible = isPartsExpanded) {
                    Column(modifier = Modifier.padding(top = 10.dp)) {
                        Text("Arabic Parts (Lots):", fontWeight = FontWeight.Bold, fontSize = 13.sp, color = TextPrimary)
                        traditional.arabic_parts.forEach { part ->
                            val sign = ZODIAC_NAMES.getOrElse(part.sign_index % 12) { "" }
                            Row(
                                modifier = Modifier.fillMaxWidth().padding(vertical = 4.dp),
                                horizontalArrangement = Arrangement.SpaceBetween
                            ) {
                                Text(text = part.name, color = TextPrimary, fontSize = 13.sp)
                                Text(
                                    text = "${String.format("%.2f°", part.longitude)} ($sign) · H${part.house_number}",
                                    color = TextSecondary,
                                    fontSize = 12.sp
                                )
                            }
                        }

                        Spacer(modifier = Modifier.height(8.dp))
                        Text("House Cusp Almutens:", fontWeight = FontWeight.Bold, fontSize = 13.sp, color = TextPrimary)
                        traditional.house_almutens.forEach { ha ->
                            Row(
                                modifier = Modifier.fillMaxWidth().padding(vertical = 3.dp),
                                horizontalArrangement = Arrangement.SpaceBetween
                            ) {
                                Text(text = "House ${ha.house_number} Cusp", color = TextPrimary, fontSize = 12.sp)
                                Text(
                                    text = "Almuten: ${ha.almuten_planet} (${ha.score} pts)",
                                    color = CelestialBlue,
                                    fontWeight = FontWeight.Medium,
                                    fontSize = 12.sp
                                )
                            }
                        }
                    }
                }
            }
        }
    }
}
