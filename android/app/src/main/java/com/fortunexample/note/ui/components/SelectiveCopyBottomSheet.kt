package com.fortunexample.note.ui.components

import android.content.ClipData
import android.content.ClipboardManager
import android.content.Context
import android.widget.Toast
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.fortunexample.note.data.*
import com.fortunexample.note.ui.theme.*

private val ZODIAC_NAMES = listOf(
    "Aries", "Taurus", "Gemini", "Cancer", "Leo", "Virgo",
    "Libra", "Scorpio", "Sagittarius", "Capricorn", "Aquarius", "Pisces"
)

private val VEDIC_SIGNS = listOf(
    "Mesha (Aries)", "Vrishabha (Taurus)", "Mithuna (Gemini)", "Karka (Cancer)",
    "Simha (Leo)", "Kanya (Virgo)", "Tula (Libra)", "Vrishchika (Scorpio)",
    "Dhanu (Sagittarius)", "Makara (Capricorn)", "Kumbha (Aquarius)", "Meena (Pisces)"
)

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun SelectiveCopyBottomSheet(
    isOpen: Boolean,
    onDismiss: () -> Unit,
    profile: ProfileDto,
    chart: FortuneChartOutput? = null,
    saju: SajuChartData? = chart?.saju,
    western: WesternChartData? = chart?.western,
    vedic: VedicChartData? = chart?.vedic,
    timezoneId: String = chart?.timezone_id ?: profile.timezone_id,
    utcOffsetHours: Double = chart?.utc_offset_hours ?: 9.0,
    selectedDomainTab: Int // 0: Saju, 1: Western, 2: Vedic
) {
    val isDomainDataReady = when (selectedDomainTab) {
        0 -> saju != null
        1 -> western != null
        else -> vedic != null
    }
    if (!isOpen || !isDomainDataReady) return

    val context = LocalContext.current

    // State for sections
    var incMeta by remember { mutableStateOf(true) }

    // Saju options
    var incSajuPillars by remember { mutableStateOf(true) }
    var incSajuDaeun by remember { mutableStateOf(true) }

    // Western options
    var incWesternPlanets by remember { mutableStateOf(true) }
    var incWesternAspects by remember { mutableStateOf(true) }
    var incWesternDignities by remember { mutableStateOf(true) }
    var incWesternMoon by remember { mutableStateOf(true) }
    var incWesternHours by remember { mutableStateOf(true) }
    var incWesternDynamicAspects by remember { mutableStateOf(true) }
    var incWesternReceptions by remember { mutableStateOf(true) }
    var incWesternStars by remember { mutableStateOf(true) }
    var incWesternParts by remember { mutableStateOf(true) }

    // Vedic options
    var incVedicRasi by remember { mutableStateOf(true) }
    var incVedicVargas by remember { mutableStateOf(true) }
    var incVedicDasha by remember { mutableStateOf(true) }

    ModalBottomSheet(
        onDismissRequest = onDismiss,
        sheetState = rememberModalBottomSheetState(skipPartiallyExpanded = true),
        containerColor = BackgroundWhite
    ) {
        Column(
            modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 20.dp)
                .padding(bottom = 32.dp)
        ) {
            Text(
                text = "복사 항목 선택",
                fontWeight = FontWeight.Bold,
                fontSize = 18.sp,
                color = TextPrimary
            )
            Spacer(modifier = Modifier.height(4.dp))
            Text(
                text = "필요한 천체 데이터 항목만 선택하여 클립보드로 복사합니다.",
                fontSize = 13.sp,
                color = TextSecondary
            )

            Spacer(modifier = Modifier.height(16.dp))

            // Select All / Deselect All Row
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween
            ) {
                TextButton(onClick = {
                    incMeta = true
                    incSajuPillars = true
                    incSajuDaeun = true
                    incWesternPlanets = true
                    incWesternAspects = true
                    incWesternDignities = true
                    incWesternMoon = true
                    incWesternHours = true
                    incWesternDynamicAspects = true
                    incWesternReceptions = true
                    incWesternStars = true
                    incWesternParts = true
                    incVedicRasi = true
                    incVedicVargas = true
                    incVedicDasha = true
                }) {
                    Text("모두 선택", color = CelestialBlue, fontSize = 13.sp)
                }

                TextButton(onClick = {
                    incMeta = true
                    incSajuPillars = true
                    incSajuDaeun = false
                    incWesternPlanets = true
                    incWesternAspects = true
                    incWesternDignities = false
                    incWesternMoon = false
                    incWesternHours = false
                    incWesternDynamicAspects = false
                    incWesternReceptions = false
                    incWesternStars = false
                    incWesternParts = false
                    incVedicRasi = true
                    incVedicVargas = false
                    incVedicDasha = false
                }) {
                    Text("기본 배치만", color = TextSecondary, fontSize = 13.sp)
                }
            }

            HorizontalDivider(color = BorderColor)
            Spacer(modifier = Modifier.height(12.dp))

            Column(
                modifier = Modifier
                    .fillMaxWidth()
                    .weight(1f, fill = false)
                    .verticalScroll(rememberScrollState())
            ) {
                CopyCheckboxItem(label = "인물 기본 정보 및 메타데이터", checked = incMeta, onCheckedChange = { incMeta = it })

                when (selectedDomainTab) {
                    0 -> {
                        CopyCheckboxItem(label = "사주팔자 원국 명식 (연/월/일/시주)", checked = incSajuPillars, onCheckedChange = { incSajuPillars = it })
                        CopyCheckboxItem(label = "10년 주기 대운 흐름 및 대운수", checked = incSajuDaeun, onCheckedChange = { incSajuDaeun = it })
                    }
                    1 -> {
                        CopyCheckboxItem(label = "Placements & House Cusps (Planets, Nodes, Angles)", checked = incWesternPlanets, onCheckedChange = { incWesternPlanets = it })
                        CopyCheckboxItem(label = "Major Aspects", checked = incWesternAspects, onCheckedChange = { incWesternAspects = it })
                        CopyCheckboxItem(label = "Essential Dignities & Lilly Scores", checked = incWesternDignities, onCheckedChange = { incWesternDignities = it })
                        CopyCheckboxItem(label = "Moon Condition (Phase, Applying)", checked = incWesternMoon, onCheckedChange = { incWesternMoon = it })
                        CopyCheckboxItem(label = "Planetary Hours", checked = incWesternHours, onCheckedChange = { incWesternHours = it })
                        CopyCheckboxItem(label = "Traditional Dynamic Aspects (Applying/Separating, Moiety)", checked = incWesternDynamicAspects, onCheckedChange = { incWesternDynamicAspects = it })
                        CopyCheckboxItem(label = "Mutual Receptions & Besiegement", checked = incWesternReceptions, onCheckedChange = { incWesternReceptions = it })
                        CopyCheckboxItem(label = "Fixed Star Conjunctions (Orb ≤ 1.5°)", checked = incWesternStars, onCheckedChange = { incWesternStars = it })
                        CopyCheckboxItem(label = "Arabic Parts & House Cusp Almutens", checked = incWesternParts, onCheckedChange = { incWesternParts = it })
                    }
                    2 -> {
                        CopyCheckboxItem(label = "라그나 및 D1 라시 차트 배치", checked = incVedicRasi, onCheckedChange = { incVedicRasi = it })
                        CopyCheckboxItem(label = "핵심 분점도 (Vargas: D9, D10 등 11개)", checked = incVedicVargas, onCheckedChange = { incVedicVargas = it })
                        CopyCheckboxItem(label = "빔쇼타리 다샤 운로 (Mahadasha/Antardasha)", checked = incVedicDasha, onCheckedChange = { incVedicDasha = it })
                    }
                }
            }

            Spacer(modifier = Modifier.height(16.dp))

            Button(
                onClick = {
                    val markdown = generateSelectiveMarkdown(
                        profile = profile,
                        timezoneId = timezoneId,
                        utcOffsetHours = utcOffsetHours,
                        saju = saju,
                        western = western,
                        vedic = vedic,
                        selectedDomainTab = selectedDomainTab,
                        incMeta = incMeta,
                        incSajuPillars = incSajuPillars,
                        incSajuDaeun = incSajuDaeun,
                        incWesternPlanets = incWesternPlanets,
                        incWesternAspects = incWesternAspects,
                        incWesternDignities = incWesternDignities,
                        incWesternMoon = incWesternMoon,
                        incWesternHours = incWesternHours,
                        incWesternDynamicAspects = incWesternDynamicAspects,
                        incWesternReceptions = incWesternReceptions,
                        incWesternStars = incWesternStars,
                        incWesternParts = incWesternParts,
                        incVedicRasi = incVedicRasi,
                        incVedicVargas = incVedicVargas,
                        incVedicDasha = incVedicDasha
                    )

                    val clipboard = context.getSystemService(Context.CLIPBOARD_SERVICE) as ClipboardManager
                    clipboard.setPrimaryClip(ClipData.newPlainText("Fortune Note Data", markdown))
                    Toast.makeText(context, "선택한 데이터가 클립보드에 복사되었습니다.", Toast.LENGTH_SHORT).show()
                    onDismiss()
                },
                modifier = Modifier
                    .fillMaxWidth()
                    .height(48.dp),
                shape = RoundedCornerShape(8.dp),
                colors = ButtonDefaults.buttonColors(containerColor = AccentPrimary)
            ) {
                Text("선택한 내용 복사하기", color = BackgroundWhite, fontWeight = FontWeight.Bold, fontSize = 15.sp)
            }
        }
    }
}

@Composable
private fun CopyCheckboxItem(
    label: String,
    checked: Boolean,
    onCheckedChange: (Boolean) -> Unit
) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .clickable { onCheckedChange(!checked) }
            .padding(vertical = 6.dp),
        verticalAlignment = Alignment.CenterVertically
    ) {
        Checkbox(
            checked = checked,
            onCheckedChange = onCheckedChange,
            colors = CheckboxDefaults.colors(checkedColor = AccentPrimary)
        )
        Spacer(modifier = Modifier.width(8.dp))
        Text(text = label, color = TextPrimary, fontSize = 14.sp)
    }
}

private fun generateSelectiveMarkdown(
    profile: ProfileDto,
    timezoneId: String,
    utcOffsetHours: Double,
    saju: SajuChartData?,
    western: WesternChartData?,
    vedic: VedicChartData?,
    selectedDomainTab: Int,
    incMeta: Boolean,
    incSajuPillars: Boolean,
    incSajuDaeun: Boolean,
    incWesternPlanets: Boolean,
    incWesternAspects: Boolean,
    incWesternDignities: Boolean,
    incWesternMoon: Boolean,
    incWesternHours: Boolean,
    incWesternDynamicAspects: Boolean,
    incWesternReceptions: Boolean,
    incWesternStars: Boolean,
    incWesternParts: Boolean,
    incVedicRasi: Boolean,
    incVedicVargas: Boolean,
    incVedicDasha: Boolean
): String {
    val sb = StringBuilder()

    if (incMeta) {
        val domainTitle = when (selectedDomainTab) {
            0 -> "사주명리 분석 데이터"
            1 -> "서양 점성술 (Western Tropical Natal Chart)"
            2 -> "베다 점성술 (Vedic Sidereal Chart)"
            else -> "천체 분석 데이터"
        }
        sb.append("# $domainTitle: ${profile.name}\n\n")
        sb.append("## 1. 인물 기본 정보\n")
        sb.append("- 성별: ${if (profile.gender == "male") "남성" else "여성"}\n")
        sb.append("- 출생 일시: ${profile.birth_year}-${String.format("%02d", profile.birth_month)}-${String.format("%02d", profile.birth_day)} ${String.format("%02d:%02d", profile.birth_hour, profile.birth_minute)} (현지 기준)\n")
        sb.append("- 관측 위치: 위도 ${String.format("%.4f°", profile.latitude)}, 경도 ${String.format("%.4f°", profile.longitude)}\n")
        sb.append("- 시간대 보정: $timezoneId (UTC ${String.format("%+.1f시간", utcOffsetHours)})\n")
        if (profile.notes.isNotEmpty()) {
            sb.append("- 비고: ${profile.notes}\n")
        }
        sb.append("\n")
    }

    when (selectedDomainTab) {
        0 -> {
            saju?.let { s ->
                if (incSajuPillars) {
                    sb.append("## 2. 사주팔자 명식 (Four Pillars of Destiny)\n")
                    sb.append("- 명식: 연주 [${s.year_pillar.korean} (${s.year_pillar.hanja})], 월주 [${s.month_pillar.korean} (${s.month_pillar.hanja})], 일주 [${s.day_pillar.korean} (${s.day_pillar.hanja})], 시주 [${s.hour_pillar.korean} (${s.hour_pillar.hanja})]\n")
                    sb.append("- 대운 방향: ${if (s.is_forward_daeun) "순행" else "역행"} (대운수: ${s.daeun_number})\n")
                }
                if (incSajuDaeun) {
                    sb.append("- 10년 주기 대운 흐름:\n")
                    s.daeun_periods.forEach { d ->
                        sb.append("  * ${d.start_age}세 ~ ${d.start_age + 9}세: [${d.pillar.korean} (${d.pillar.hanja})]\n")
                    }
                }
            }
        }
        1 -> {
            western?.let { w ->
                if (incWesternPlanets) {
                    val ascSign = ZODIAC_NAMES.getOrElse((w.ascendant / 30.0).toInt() % 12) { "" }
                    val mcSign = ZODIAC_NAMES.getOrElse((w.midheaven / 30.0).toInt() % 12) { "" }
                    sb.append("## 2. Celestial Placements & House Cusps\n")
                    sb.append("- Ascendant: ${String.format("%.2f°", w.ascendant)} ($ascSign)\n")
                    sb.append("- Midheaven (MC): ${String.format("%.2f°", w.midheaven)} ($mcSign)\n")
                    w.house_system?.let { sb.append("- House System: $it\n") }
                    if (w.house_cusps.isNotEmpty()) {
                        sb.append("- House Cusps (12 Cusps):\n")
                        w.house_cusps.forEachIndexed { idx, cusp ->
                            val hNum = idx + 1
                            val signIdx = ((cusp / 30.0).toInt() % 12 + 12) % 12
                            val signName = ZODIAC_NAMES.getOrElse(signIdx) { "" }
                            val signDeg = cusp % 30.0
                            val tag = when (hNum) {
                                1 -> " (ASC)"
                                4 -> " (IC)"
                                7 -> " (DSC)"
                                10 -> " (MC)"
                                else -> ""
                            }
                            sb.append("  * House ${String.format("%02d", hNum)}$tag: ${String.format("%.2f°", signDeg)} ($signName)\n")
                        }
                    }
                    sb.append("- Planetary Placements:\n")
                    w.planets.forEach { p ->
                        val sign = ZODIAC_NAMES.getOrElse(p.sign_index % 12) { "" }
                        val rx = if (p.is_retrograde) " [Rx]" else ""
                        val speedStr = String.format("%+.2f°/day", p.speed)
                        sb.append("  * ${p.name}: ${String.format("%.2f°", p.longitude)} ($sign) - House ${p.house_number}$rx (Speed: $speedStr)\n")
                    }
                    sb.append("- Lunar Nodes:\n")
                    w.nodes.forEach { n ->
                        val sign = ZODIAC_NAMES.getOrElse(n.sign_index % 12) { "" }
                        sb.append("  * ${n.name}: ${String.format("%.2f°", n.longitude)} ($sign) - House ${n.house_number}\n")
                    }
                }
                if (incWesternAspects) {
                    sb.append("- Major Aspects:\n")
                    if (w.aspects.isEmpty()) {
                        sb.append("  * No major aspects within orb\n")
                    } else {
                        w.aspects.forEach { a ->
                            val dyn = if (a.is_applying) "Applying" else "Separating"
                            sb.append("  * ${a.planet1} - ${a.planet2}: ${a.aspect_type} (Orb: ${String.format("%.2f°", a.orb)}, $dyn)\n")
                        }
                    }
                }

                w.traditional?.let { trad ->
                    sb.append("\n## 3. William Lilly Traditional Analysis\n")
                    if (incWesternHours) {
                        sb.append("- Planetary Hours: Day of ${trad.planetary_hours.day_ruler}, Hour of ${trad.planetary_hours.hour_ruler} (Hour ${trad.planetary_hours.hour_number}, ${if (trad.planetary_hours.is_day_hour) "Day" else "Night"})\n")
                    }
                    if (incWesternMoon) {
                        sb.append("- Moon Condition: Phase [${trad.moon_condition.lunar_phase_name}]\n")
                        trad.moon_condition.next_applying_aspect?.let { next ->
                            sb.append("  * Next Applying Aspect: $next\n")
                        }
                    }
                    if (incWesternDignities) {
                        sb.append("- Essential Dignities & Lilly Scores:\n")
                        trad.planet_dignities.forEach { d ->
                            val scoreSign = if (d.total_lilly_score >= 0) "+${d.total_lilly_score}" else "${d.total_lilly_score}"
                            val essSign = if (d.essential_score >= 0) "+${d.essential_score}" else "${d.essential_score}"
                            val accSign = if (d.accidental_score >= 0) "+${d.accidental_score}" else "${d.accidental_score}"
                            sb.append("  * ${d.planet}: $scoreSign ($essSign Ess, $accSign Acc)\n")
                        }
                    }
                    if (incWesternDynamicAspects) {
                        sb.append("- Dynamic Classical Aspects:\n")
                        trad.traditional_aspects.forEach { a ->
                            val app = if (a.is_applying) "Applying" else "Separating"
                            val part = if (a.is_partile) " [Partile]" else ""
                            sb.append("  * ${a.planet1} ${a.aspect_type} ${a.planet2}: $app (Orb: ${String.format("%.2f°", a.orb)})$part\n")
                        }
                    }
                    if (incWesternReceptions) {
                        sb.append("- Mutual Receptions & Perfections:\n")
                        trad.mutual_receptions.forEach { r ->
                            sb.append("  * ${r.planet1} - ${r.planet2}: ${r.reception_type} (${r.description})\n")
                        }
                        trad.perfection_report.besiegement.forEach { b ->
                            sb.append("  * Besiegement: $b\n")
                        }
                    }
                    if (incWesternStars) {
                        sb.append("- Fixed Stars Conjunctions (Orb <= 1.5°):\n")
                        if (trad.fixed_star_conjunctions.isEmpty()) {
                            sb.append("  * No major fixed star conjunctions within orb\n")
                        } else {
                            trad.fixed_star_conjunctions.forEach { s ->
                                sb.append("  * ${s.star_name} ☌ ${s.body_name} (Orb: ${String.format("%.2f°", s.orb)}, Nature: ${s.nature}): ${s.meaning}\n")
                            }
                        }
                    }
                    if (incWesternParts) {
                        sb.append("- Arabic Parts (Hermetic Lots):\n")
                        trad.arabic_parts.forEach { p ->
                            val sName = ZODIAC_NAMES.getOrElse(p.sign_index % 12) { "" }
                            sb.append("  * ${p.name}: ${String.format("%.2f°", p.longitude)} ($sName) - House ${p.house_number}\n")
                        }
                    }
                }
            }
        }
        2 -> {
            vedic?.let { v ->
                if (incVedicRasi) {
                    val lagnaSign = VEDIC_SIGNS.getOrElse(v.lagna_sign % 12) { "" }
                    sb.append("## 2. 라시 차트 (Vedic Sidereal Placements)\n")
                    sb.append("- Lagna: ${String.format("%.2f°", v.lagna_sidereal)} ($lagnaSign)\n")
                    v.planets.forEach { p ->
                        val sign = VEDIC_SIGNS.getOrElse(p.sign_index % 12) { "" }
                        val rx = if (p.is_retrograde) " [Rx]" else ""
                        sb.append("  * ${p.name}: ${String.format("%.2f°", p.sidereal_longitude)} ($sign, ${p.nakshatra_name} Pada ${p.nakshatra_pada})$rx\n")
                    }
                }
                if (incVedicVargas) {
                    sb.append("\n## 3. 분점도 (Vargas)\n")
                    v.vargas.forEach { (key, chart) ->
                        val lSign = VEDIC_SIGNS.getOrElse(chart.lagna_sign % 12) { "" }
                        sb.append("- $key: Lagna $lSign\n")
                        val pl = chart.placements.map { p ->
                            val pSign = VEDIC_SIGNS.getOrElse(p.sign_index % 12) { "" }
                            "${p.planet_name}: H${p.house} $pSign"
                        }
                        if (pl.isNotEmpty()) {
                            sb.append("  * ${pl.joinToString(", ")}\n")
                        }
                    }
                }
                if (incVedicDasha) {
                    sb.append("\n## 4. 빔쇼타리 다샤 운로 (Vimshottari Dasha)\n")
                    v.vimshottari_dashas.forEach { d ->
                        sb.append("- Mahadasha ${d.lord}: ${d.startDateFormatted} ~ ${d.endDateFormatted}\n")
                        d.sub_periods.forEach { sub ->
                            sb.append("  * Antardasha ${sub.lord}: ${sub.startDateFormatted} ~ ${sub.endDateFormatted}\n")
                        }
                    }
                }
            }
        }
    }

    return sb.toString()
}
