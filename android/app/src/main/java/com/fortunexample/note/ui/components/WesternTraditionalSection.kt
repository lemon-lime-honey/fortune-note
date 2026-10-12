package com.fortunexample.note.ui.components

import android.widget.Toast
import androidx.compose.animation.AnimatedVisibility
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ContentCopy
import androidx.compose.material.icons.filled.KeyboardArrowDown
import androidx.compose.material.icons.filled.KeyboardArrowUp
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
import com.fortunexample.note.data.HellenisticData
import com.fortunexample.note.data.WesternTraditionalData
import com.fortunexample.note.ui.theme.*

private val ZODIAC_NAMES = listOf(
    "Aries", "Taurus", "Gemini", "Cancer", "Leo", "Virgo",
    "Libra", "Scorpio", "Sagittarius", "Capricorn", "Aquarius", "Pisces"
)

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

@OptIn(ExperimentalLayoutApi::class)
@Composable
fun WesternTraditionalSection(
    traditional: WesternTraditionalData?,
    isHorary: Boolean = false,
    hellenistic: HellenisticData? = null
) {
    if (hellenistic != null) {
        HellenisticSectionContent(hellenistic)
        return
    }

    if (traditional == null) return

    var isDignityExpanded by remember { mutableStateOf(true) }
    var isStarsExpanded by remember { mutableStateOf(true) }
    var isPartsExpanded by remember { mutableStateOf(false) }

    Column(modifier = Modifier.fillMaxWidth()) {
        // 1. Planetary Hours Card (호라리일 때만 Radicality 및 판단 전 고려사항 표시)
        Card(
            shape = RoundedCornerShape(12.dp),
            colors = CardDefaults.cardColors(containerColor = BackgroundWhite),
            border = BorderStroke(1.dp, BorderColor),
            modifier = Modifier.fillMaxWidth()
        ) {
            Column(modifier = Modifier.padding(16.dp)) {
                // 1행: 제목 + 복사 버튼
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.SpaceBetween,
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    Text(
                        text = if (isHorary) "Planetary Hours & Radicality" else "Planetary Hours",
                        color = TextPrimary,
                        fontWeight = FontWeight.Bold,
                        fontSize = 15.sp
                    )
                    SectionCopyButton(title = "Planetary Hours") {
                        val sb = StringBuilder()
                        if (isHorary) {
                            sb.appendLine("[Planetary Hours & Radicality]")
                            sb.appendLine("• Status: ${if (traditional.planetary_hours.is_radical) "Radical" else "Non-radical"}")
                            sb.appendLine("• Day of ${traditional.planetary_hours.day_ruler} · Hour of ${traditional.planetary_hours.hour_ruler} (Hour ${traditional.planetary_hours.hour_number}, ${if (traditional.planetary_hours.is_day_hour) "Day" else "Night"})")
                            sb.appendLine("• Reason: ${traditional.planetary_hours.radicality_reason}")
                            if (traditional.planetary_hours.considerations.isNotEmpty()) {
                                sb.appendLine("• Considerations Before Judgment:")
                                traditional.planetary_hours.considerations.forEach { c -> sb.appendLine("  - $c") }
                            }
                        } else {
                            sb.appendLine("[Planetary Hours]")
                            sb.appendLine("• Day of ${traditional.planetary_hours.day_ruler} · Hour of ${traditional.planetary_hours.hour_ruler} (Hour ${traditional.planetary_hours.hour_number}, ${if (traditional.planetary_hours.is_day_hour) "Day" else "Night"})")
                        }
                        sb.toString().trimEnd()
                    }
                }

                if (isHorary) {
                    Spacer(modifier = Modifier.height(6.dp))

                    // 2행: Radicality 배지 (호라리 전용)
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

                    Spacer(modifier = Modifier.height(8.dp))
                } else {
                    Spacer(modifier = Modifier.height(10.dp))
                }

                Text(
                    text = "Day of ${traditional.planetary_hours.day_ruler} · Hour of ${traditional.planetary_hours.hour_ruler} (Hour ${traditional.planetary_hours.hour_number}, ${if (traditional.planetary_hours.is_day_hour) "Day" else "Night"})",
                    color = TextPrimary,
                    fontSize = 13.sp
                )

                if (isHorary) {
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
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.SpaceBetween,
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    Text("Moon Condition", color = TextPrimary, fontWeight = FontWeight.Bold, fontSize = 15.sp)
                    SectionCopyButton(title = "Moon Condition") {
                        val sb = StringBuilder()
                        sb.appendLine("[Moon Condition]")
                        sb.appendLine("• Phase: ${traditional.moon_condition.lunar_phase_name}")
                        if (isHorary) {
                            if (traditional.moon_condition.is_void_of_course) sb.appendLine("• Void of Course: Yes")
                            if (traditional.moon_condition.is_via_combusta) sb.appendLine("• Via Combusta: Yes")
                        }
                        traditional.moon_condition.next_applying_aspect?.let {
                            sb.appendLine("• Next Applying Aspect: $it")
                        }
                        sb.toString().trimEnd()
                    }
                }
                Spacer(modifier = Modifier.height(10.dp))

                FlowRow(
                    horizontalArrangement = Arrangement.spacedBy(8.dp),
                    verticalArrangement = Arrangement.spacedBy(6.dp),
                    modifier = Modifier.fillMaxWidth()
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

                    if (isHorary && traditional.moon_condition.is_void_of_course) {
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

                    if (isHorary && traditional.moon_condition.is_via_combusta) {
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
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.SpaceBetween,
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    Row(
                        modifier = Modifier
                            .weight(1f)
                            .clickable { isDignityExpanded = !isDignityExpanded },
                        verticalAlignment = Alignment.CenterVertically,
                        horizontalArrangement = Arrangement.spacedBy(6.dp)
                    ) {
                        Text("Essential Dignities & Lilly Scores", color = TextPrimary, fontWeight = FontWeight.Bold, fontSize = 15.sp)
                        Icon(
                            if (isDignityExpanded) Icons.Default.KeyboardArrowUp else Icons.Default.KeyboardArrowDown,
                            contentDescription = null,
                            tint = TextSecondary
                        )
                    }
                    SectionCopyButton(title = "Dignities") {
                        val sb = StringBuilder()
                        sb.appendLine("[Essential Dignities & Lilly Scores]")
                        traditional.planet_dignities.forEach { d ->
                            val badges = mutableListOf<String>()
                            if (d.is_in_domicile) badges.add("Domicile +5")
                            if (d.is_in_exaltation) badges.add("Exaltation +4")
                            if (d.is_in_triplicity) badges.add("Triplicity +3")
                            if (d.is_in_term) badges.add("Term +2")
                            if (d.is_in_face) badges.add("Face +1")
                            if (d.is_in_detriment) badges.add("Detriment -5")
                            if (d.is_in_fall) badges.add("Fall -4")
                            if (d.is_peregrine) badges.add("Peregrine -5")
                            val badgeStr = if (badges.isEmpty()) "Neutral" else badges.joinToString(", ")
                            sb.appendLine("• ${d.planet} (House ${d.house_number}): Score ${String.format("%+d", d.total_lilly_score)} [Ess ${String.format("%+d", d.essential_score)} / Acc ${String.format("%+d", d.accidental_score)}] | $badgeStr")
                        }
                        sb.toString().trimEnd()
                    }
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
                                Column(modifier = Modifier.weight(1f).padding(end = 8.dp)) {
                                    Text(
                                        text = "${d.planet} · House ${d.house_number}",
                                        fontWeight = FontWeight.Bold,
                                        fontSize = 14.sp,
                                        color = TextPrimary
                                    )
                                    FlowRow(
                                        horizontalArrangement = Arrangement.spacedBy(4.dp),
                                        verticalArrangement = Arrangement.spacedBy(4.dp),
                                        modifier = Modifier.padding(top = 4.dp)
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
                                        text = "Ess ${String.format("%+d", d.essential_score)} / Acc ${String.format("%+d", d.accidental_score)}",
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

        // 4. Fixed Star Conjunctions Card (순수 수직 3단 컬럼 배치)
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
                    Row(
                        modifier = Modifier
                            .weight(1f)
                            .clickable { isStarsExpanded = !isStarsExpanded },
                        verticalAlignment = Alignment.CenterVertically,
                        horizontalArrangement = Arrangement.spacedBy(6.dp)
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
                    SectionCopyButton(title = "Fixed Stars") {
                        val sb = StringBuilder()
                        sb.appendLine("[Fixed Star Conjunctions (Orb <= 1.5°)]")
                        if (traditional.fixed_star_conjunctions.isEmpty()) {
                            sb.appendLine("  (None in orb)")
                        } else {
                            traditional.fixed_star_conjunctions.forEach { fs ->
                                sb.appendLine("• ${fs.star_name} · ${fs.body_name}")
                                sb.appendLine("  - Orb ${String.format("%.2f°", fs.orb)} · Nature: ${fs.nature}")
                                if (fs.meaning.isNotBlank()) {
                                    sb.appendLine("  - Meaning: ${fs.meaning}")
                                }
                            }
                        }
                        sb.toString().trimEnd()
                    }
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
                                Column(
                                    modifier = Modifier
                                        .fillMaxWidth()
                                        .padding(vertical = 6.dp)
                                ) {
                                    // 1행: 항성명 · 결합 천체 + 가시성 뱃지
                                    Row(
                                        verticalAlignment = Alignment.CenterVertically,
                                        horizontalArrangement = Arrangement.spacedBy(6.dp)
                                    ) {
                                        Text(
                                            text = "${fs.star_name} · ${fs.body_name}",
                                            fontWeight = FontWeight.Bold,
                                            fontSize = 13.sp,
                                            color = TextPrimary
                                        )
                                        Surface(
                                            shape = RoundedCornerShape(4.dp),
                                            color = if (fs.is_visible) AstrologicalGreen.copy(alpha = 0.12f) else BorderColor.copy(alpha = 0.5f)
                                        ) {
                                            Text(
                                                text = if (fs.is_visible) "Visible (${String.format("%+.1f°", fs.max_altitude)})" else "Below Horizon",
                                                fontSize = 10.sp,
                                                fontWeight = FontWeight.SemiBold,
                                                color = if (fs.is_visible) AstrologicalGreen else TextSecondary,
                                                modifier = Modifier.padding(horizontal = 4.dp, vertical = 2.dp)
                                            )
                                        }
                                    }
                                    Spacer(modifier = Modifier.height(2.dp))
                                    // 2행: 오차 · 성질
                                    Text(
                                        text = "Orb ${String.format("%.2f°", fs.orb)} · Nature: ${fs.nature}",
                                        color = CelestialBlue,
                                        fontSize = 12.sp,
                                        fontWeight = FontWeight.Medium
                                    )
                                    // 3행: 전통 의미 및 해석
                                    if (fs.meaning.isNotBlank()) {
                                        Spacer(modifier = Modifier.height(2.dp))
                                        Text(
                                            text = fs.meaning,
                                            color = TextSecondary,
                                            fontSize = 12.sp
                                        )
                                    }
                                }
                                HorizontalDivider(color = BorderColor.copy(alpha = 0.5f))
                            }
                        }
                    }
                }
            }
        }

        Spacer(modifier = Modifier.height(12.dp))

        // 5. Arabic Parts & Almutens Card (제목 축약 및 Almuten 중복 라벨 제거)
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
                    Row(
                        modifier = Modifier
                            .weight(1f)
                            .clickable { isPartsExpanded = !isPartsExpanded },
                        verticalAlignment = Alignment.CenterVertically,
                        horizontalArrangement = Arrangement.spacedBy(6.dp)
                    ) {
                        Text("Arabic Parts & Almutens", color = TextPrimary, fontWeight = FontWeight.Bold, fontSize = 15.sp)
                        Icon(
                            if (isPartsExpanded) Icons.Default.KeyboardArrowUp else Icons.Default.KeyboardArrowDown,
                            contentDescription = null,
                            tint = TextSecondary
                        )
                    }
                    SectionCopyButton(title = "Arabic Parts & Almutens") {
                        val sb = StringBuilder()
                        sb.appendLine("[Arabic Parts & Almutens]")
                        sb.appendLine("Arabic Parts (Lots):")
                        traditional.arabic_parts.forEach { p ->
                            val s = ZODIAC_NAMES.getOrElse(p.sign_index % 12) { "" }
                            sb.appendLine("• ${p.name}: ${String.format("%.2f°", p.longitude)} ($s) · House ${p.house_number}")
                        }
                        sb.appendLine("House Cusp Almutens:")
                        traditional.house_almutens.forEach { ha ->
                            sb.appendLine("• House ${ha.house_number}: ${ha.almuten} (${ha.almuten_score} pts)")
                        }
                        sb.toString().trimEnd()
                    }
                }

                AnimatedVisibility(visible = isPartsExpanded) {
                    Column(modifier = Modifier.padding(top = 10.dp)) {
                        Text("Arabic Parts (Lots):", fontWeight = FontWeight.Bold, fontSize = 13.sp, color = TextPrimary)
                        traditional.arabic_parts.forEach { part ->
                            val sign = ZODIAC_NAMES.getOrElse(part.sign_index % 12) { "" }
                            Row(
                                modifier = Modifier
                                    .fillMaxWidth()
                                    .padding(vertical = 4.dp),
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

                        Spacer(modifier = Modifier.height(10.dp))
                        Text("House Cusp Almutens:", fontWeight = FontWeight.Bold, fontSize = 13.sp, color = TextPrimary)
                        traditional.house_almutens.forEach { ha ->
                            Row(
                                modifier = Modifier
                                    .fillMaxWidth()
                                    .padding(vertical = 3.dp),
                                horizontalArrangement = Arrangement.SpaceBetween
                            ) {
                                Text(text = "House ${ha.house_number}", color = TextPrimary, fontSize = 12.sp)
                                Text(
                                    text = "${ha.almuten} (${ha.almuten_score} pts)",
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

@Composable
private fun HellenisticSectionContent(h: HellenisticData) {
    Column(modifier = Modifier.fillMaxWidth()) {
        Text(
            text = "Hellenistic Analysis (Vettius Valens / Whole Sign)",
            fontWeight = FontWeight.Bold,
            fontSize = 17.sp,
            color = TextPrimary,
            modifier = Modifier.padding(bottom = 12.dp)
        )

        // 1. Independent Angles & Action Points Card
        Card(
            shape = RoundedCornerShape(12.dp),
            colors = CardDefaults.cardColors(containerColor = BackgroundWhite),
            border = BorderStroke(1.dp, BorderColor),
            modifier = Modifier.fillMaxWidth().padding(bottom = 12.dp)
        ) {
            Column(modifier = Modifier.padding(16.dp)) {
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.SpaceBetween,
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    Text("Angles & Planetary Action Points", fontWeight = FontWeight.Bold, fontSize = 14.sp, color = TextPrimary)
                    SectionCopyButton(title = "Hellenistic Angles") {
                        val ascSign = ZODIAC_NAMES.getOrElse(h.angles.ascendant_sign % 12) { "" }
                        val mcSign = ZODIAC_NAMES.getOrElse(h.angles.midheaven_sign % 12) { "" }
                        val mcHouseStr = if (h.angles.is_mc_in_10th) "House 10 (Culmination)" else "House ${h.angles.midheaven_house} (Action Point)"
                        """
                        [Hellenistic Angles]
                        • Ascendant (ASC): ${String.format("%.2f°", h.angles.ascendant_degree)} $ascSign
                        • Midheaven (MC): ${String.format("%.2f°", h.angles.midheaven_degree)} $mcSign · $mcHouseStr
                        """.trimIndent()
                    }
                }
                Spacer(modifier = Modifier.height(10.dp))
                val ascSign = ZODIAC_NAMES.getOrElse(h.angles.ascendant_sign % 12) { "" }
                val mcSign = ZODIAC_NAMES.getOrElse(h.angles.midheaven_sign % 12) { "" }

                Row(modifier = Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.SpaceBetween) {
                    Text("Ascendant (ASC)", fontSize = 13.sp, color = TextPrimary, fontWeight = FontWeight.Medium)
                    Text("${String.format("%.2f°", h.angles.ascendant_degree)} $ascSign", fontSize = 13.sp, color = AccentPrimary, fontWeight = FontWeight.SemiBold)
                }
                Spacer(modifier = Modifier.height(6.dp))
                Row(modifier = Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.SpaceBetween) {
                    Column {
                        Text("Midheaven (MC)", fontSize = 13.sp, color = TextPrimary, fontWeight = FontWeight.Medium)
                        val mcActionTag = if (h.angles.is_mc_in_10th) "10H Culmination" else "${h.angles.midheaven_house}H Action Point"
                        Text(mcActionTag, fontSize = 11.sp, color = if (h.angles.is_mc_in_10th) TextSecondary else VedicGreen, fontWeight = FontWeight.Normal)
                    }
                    Text("${String.format("%.2f°", h.angles.midheaven_degree)} $mcSign", fontSize = 13.sp, color = AccentPrimary, fontWeight = FontWeight.SemiBold)
                }
            }
        }

        // 2. Qualitative Planet Conditions (Hellenistic Essential States)
        Card(
            shape = RoundedCornerShape(12.dp),
            colors = CardDefaults.cardColors(containerColor = BackgroundWhite),
            border = BorderStroke(1.dp, BorderColor),
            modifier = Modifier.fillMaxWidth().padding(bottom = 12.dp)
        ) {
            Column(modifier = Modifier.padding(16.dp)) {
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.SpaceBetween,
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    Text("Qualitative Dignities (States)", fontWeight = FontWeight.Bold, fontSize = 14.sp, color = TextPrimary)
                    SectionCopyButton(title = "Hellenistic Dignities") {
                        val sb = StringBuilder()
                        sb.appendLine("[Hellenistic Qualitative Dignities]")
                        h.planet_conditions.forEach { pc ->
                            val lordTag = if (pc.is_ruler_of_asc) " [Lord of ASC]" else ""
                            sb.appendLine("• ${pc.name}: ${pc.condition_label} · House ${pc.house_number}$lordTag")
                        }
                        sb.toString().trimEnd()
                    }
                }
                Spacer(modifier = Modifier.height(10.dp))
                h.planet_conditions.forEach { pc ->
                    Row(
                        modifier = Modifier
                            .fillMaxWidth()
                            .padding(vertical = 5.dp),
                        horizontalArrangement = Arrangement.SpaceBetween,
                        verticalAlignment = Alignment.CenterVertically
                    ) {
                        Row(verticalAlignment = Alignment.CenterVertically) {
                            Text(pc.name, fontWeight = FontWeight.Medium, fontSize = 13.sp, color = TextPrimary)
                            if (pc.is_ruler_of_asc) {
                                Spacer(modifier = Modifier.width(6.dp))
                                Surface(
                                    color = AccentPrimary.copy(alpha = 0.12f),
                                    shape = RoundedCornerShape(4.dp)
                                ) {
                                    Text(
                                        text = "Lord of ASC",
                                        color = AccentPrimary,
                                        fontSize = 10.sp,
                                        fontWeight = FontWeight.Bold,
                                        modifier = Modifier.padding(horizontal = 4.dp, vertical = 2.dp)
                                    )
                                }
                            }
                        }
                        Row(verticalAlignment = Alignment.CenterVertically) {
                            val badgeColor = when (pc.condition_label) {
                                "Domicile" -> VedicGreen
                                "Exaltation" -> CelestialBlue
                                "Fall" -> AstrologicalRed
                                else -> TextSecondary
                            }
                            Surface(
                                color = badgeColor.copy(alpha = 0.12f),
                                shape = RoundedCornerShape(4.dp)
                            ) {
                                Text(
                                    text = pc.condition_label,
                                    color = badgeColor,
                                    fontSize = 11.sp,
                                    fontWeight = FontWeight.SemiBold,
                                    modifier = Modifier.padding(horizontal = 6.dp, vertical = 2.dp)
                                )
                            }
                            Spacer(modifier = Modifier.width(8.dp))
                            Text("House ${pc.house_number}", fontSize = 12.sp, color = TextSecondary)
                        }
                    }
                    HorizontalDivider(color = BorderColor.copy(alpha = 0.5f))
                }
            }
        }

        // 3. Hermetic Lots (Lots of Soul & Body)
        if (h.lots.isNotEmpty()) {
            Card(
                shape = RoundedCornerShape(12.dp),
                colors = CardDefaults.cardColors(containerColor = BackgroundWhite),
                border = BorderStroke(1.dp, BorderColor),
                modifier = Modifier.fillMaxWidth().padding(bottom = 12.dp)
            ) {
                Column(modifier = Modifier.padding(16.dp)) {
                    Row(
                        modifier = Modifier.fillMaxWidth(),
                        horizontalArrangement = Arrangement.SpaceBetween,
                        verticalAlignment = Alignment.CenterVertically
                    ) {
                        Text("Hermetic Lots (Paulus Alexandrinus)", fontWeight = FontWeight.Bold, fontSize = 14.sp, color = TextPrimary)
                        SectionCopyButton(title = "Hermetic Lots") {
                            val sb = StringBuilder()
                            sb.appendLine("[Hermetic Lots]")
                            h.lots.forEach { lot ->
                                val sign = ZODIAC_NAMES.getOrElse(lot.sign_index % 12) { "" }
                                sb.appendLine("• ${lot.name}: ${String.format("%.2f°", lot.sign_degree)} $sign · House ${lot.house_number} · Ruler: ${lot.ruler}")
                            }
                            sb.toString().trimEnd()
                        }
                    }
                    Spacer(modifier = Modifier.height(10.dp))
                    h.lots.forEach { lot ->
                        val sign = ZODIAC_NAMES.getOrElse(lot.sign_index % 12) { "" }
                        Row(
                            modifier = Modifier
                                .fillMaxWidth()
                                .padding(vertical = 4.dp),
                            horizontalArrangement = Arrangement.SpaceBetween,
                            verticalAlignment = Alignment.CenterVertically
                        ) {
                            Text(lot.name, fontSize = 13.sp, color = TextPrimary)
                            Text(
                                text = "${String.format("%.2f°", lot.sign_degree)} $sign · H${lot.house_number}",
                                fontSize = 12.sp,
                                color = TextSecondary
                            )
                        }
                        HorizontalDivider(color = BorderColor.copy(alpha = 0.4f))
                    }
                }
            }
        }

        // 4. Sign-Based Aspects & Degree Contacts
        if (h.aspects.isNotEmpty()) {
            Card(
                shape = RoundedCornerShape(12.dp),
                colors = CardDefaults.cardColors(containerColor = BackgroundWhite),
                border = BorderStroke(1.dp, BorderColor),
                modifier = Modifier.fillMaxWidth().padding(bottom = 12.dp)
            ) {
                Column(modifier = Modifier.padding(16.dp)) {
                    Text("Sign-Based Aspects & Contacts", fontWeight = FontWeight.Bold, fontSize = 14.sp, color = TextPrimary)
                    Spacer(modifier = Modifier.height(10.dp))
                    h.aspects.forEach { asp ->
                        val appText = if (asp.is_applying) "App (${String.format("%.1f°", asp.degrees_to_perfection)})" else "Sep"
                        Row(
                            modifier = Modifier
                                .fillMaxWidth()
                                .padding(vertical = 4.dp),
                            horizontalArrangement = Arrangement.SpaceBetween
                        ) {
                            Text("${asp.planet1} ${asp.aspect_type} ${asp.planet2}", fontSize = 13.sp, color = TextPrimary)
                            Text(
                                text = "Orb ${String.format("%.2f°", asp.orb)} · $appText",
                                fontSize = 12.sp,
                                color = if (asp.is_partile) AccentPrimary else TextSecondary
                            )
                        }
                    }
                }
            }
        }

        // 5. Fixed Star Conjunctions
        if (h.fixed_star_conjunctions.isNotEmpty()) {
            Card(
                shape = RoundedCornerShape(12.dp),
                colors = CardDefaults.cardColors(containerColor = BackgroundWhite),
                border = BorderStroke(1.dp, BorderColor),
                modifier = Modifier.fillMaxWidth().padding(bottom = 12.dp)
            ) {
                Column(modifier = Modifier.padding(16.dp)) {
                    Text("Fixed Star Conjunctions (Orb ≤ 1.5°)", fontWeight = FontWeight.Bold, fontSize = 14.sp, color = TextPrimary)
                    Spacer(modifier = Modifier.height(10.dp))
                    h.fixed_star_conjunctions.forEach { fsc ->
                        Column(modifier = Modifier.padding(vertical = 4.dp)) {
                            Row(
                                verticalAlignment = Alignment.CenterVertically,
                                horizontalArrangement = Arrangement.spacedBy(6.dp)
                            ) {
                                Text(
                                    text = "★ ${fsc.star_name} (${fsc.korean_name}) ☌ ${fsc.body_name} (Orb ${String.format("%.2f°", fsc.orb)})",
                                    fontSize = 13.sp,
                                    fontWeight = FontWeight.SemiBold,
                                    color = TextPrimary
                                )
                                Surface(
                                    shape = RoundedCornerShape(4.dp),
                                    color = if (fsc.is_visible) AstrologicalGreen.copy(alpha = 0.12f) else BorderColor.copy(alpha = 0.5f)
                                ) {
                                    Text(
                                        text = if (fsc.is_visible) "Visible (${String.format("%+.1f°", fsc.max_altitude)})" else "Below Horizon",
                                        fontSize = 10.sp,
                                        fontWeight = FontWeight.SemiBold,
                                        color = if (fsc.is_visible) AstrologicalGreen else TextSecondary,
                                        modifier = Modifier.padding(horizontal = 4.dp, vertical = 2.dp)
                                    )
                                }
                            }
                            Text(text = fsc.meaning, fontSize = 12.sp, color = TextSecondary)
                        }
                    }
                }
            }
        }
    }
}
