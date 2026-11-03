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
    predictive: PredictiveSuiteData? = null,
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
    val isWholeSign = western?.house_system?.lowercase() in listOf("wholesign", "whole_sign", "whole", "hellenistic") ||
        (western?.hellenistic != null && western.traditional == null)

    // State for sections
    var incMeta by remember { mutableStateOf(true) }

    // Saju options
    var incSajuPillars by remember { mutableStateOf(true) }
    var incSajuDaeun by remember { mutableStateOf(true) }

    // Western Natal/Traditional options
    var incWesternPlanets by remember { mutableStateOf(true) }
    var incWesternAspects by remember { mutableStateOf(true) }
    var incWesternDignities by remember { mutableStateOf(true) }
    var incWesternMoon by remember { mutableStateOf(true) }
    var incWesternHours by remember { mutableStateOf(true) }
    var incWesternDynamicAspects by remember { mutableStateOf(true) }
    var incWesternReceptions by remember { mutableStateOf(true) }
    var incWesternStars by remember { mutableStateOf(true) }
    var incWesternParts by remember { mutableStateOf(true) }

    // Western Predictive options
    var incWesternProfection by remember { mutableStateOf(true) }
    var incWesternReturns by remember { mutableStateOf(true) }
    var incWesternProgressions by remember { mutableStateOf(true) }
    var incWesternTransitsDirections by remember { mutableStateOf(true) }

    // Vedic options
    var incVedicRasi by remember { mutableStateOf(true) }
    var incVedicIndicators by remember { mutableStateOf(true) }
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
                    incWesternProfection = true
                    incWesternReturns = true
                    incWesternProgressions = true
                    incWesternTransitsDirections = true
                    incVedicRasi = true
                    incVedicIndicators = true
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
                    incWesternProfection = false
                    incWesternReturns = false
                    incWesternProgressions = false
                    incWesternTransitsDirections = false
                    incVedicRasi = true
                    incVedicIndicators = false
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
                        if (isWholeSign) {
                            CopyCheckboxItem(label = "천체 및 홀사인 하우스 배치 (Whole Sign Placements)", checked = incWesternPlanets, onCheckedChange = { incWesternPlanets = it })
                            CopyCheckboxItem(label = "메이저 애스펙트 (Major Aspects)", checked = incWesternAspects, onCheckedChange = { incWesternAspects = it })
                            CopyCheckboxItem(label = "헬레니즘 행성 상태·이집트 바운드·트리플리시티", checked = incWesternDignities, onCheckedChange = { incWesternDignities = it })
                            CopyCheckboxItem(label = "사인 기반 애스펙트 및 접촉 (Sign-Based Aspects)", checked = incWesternDynamicAspects, onCheckedChange = { incWesternDynamicAspects = it })
                            CopyCheckboxItem(label = "헤르메스 랏 및 출생 전 합충 (Hermetic Lots & SAN)", checked = incWesternParts, onCheckedChange = { incWesternParts = it })
                            CopyCheckboxItem(label = "항성 회합 (Fixed Star Conjunctions ≤ 1.5°)", checked = incWesternStars, onCheckedChange = { incWesternStars = it })
                        } else {
                            CopyCheckboxItem(label = "천체 및 레기오몬타누스 커스프 (Regiomontanus Placements)", checked = incWesternPlanets, onCheckedChange = { incWesternPlanets = it })
                            CopyCheckboxItem(label = "메이저 애스펙트 (Major Aspects)", checked = incWesternAspects, onCheckedChange = { incWesternAspects = it })
                            CopyCheckboxItem(label = "필수·우연 품위 및 점수 (Essential & Accidental Dignities)", checked = incWesternDignities, onCheckedChange = { incWesternDignities = it })
                            CopyCheckboxItem(label = "달의 위상 및 출생 전 합충 (Moon Phase & SAN)", checked = incWesternMoon, onCheckedChange = { incWesternMoon = it })
                            CopyCheckboxItem(label = "행성시 (Planetary Hours)", checked = incWesternHours, onCheckedChange = { incWesternHours = it })
                            CopyCheckboxItem(label = "중세 역학 애스펙트 (Applying/Separating, Moiety)", checked = incWesternDynamicAspects, onCheckedChange = { incWesternDynamicAspects = it })
                            CopyCheckboxItem(label = "뮤추얼 리셉션 및 안티시아 (Receptions & Antiscia)", checked = incWesternReceptions, onCheckedChange = { incWesternReceptions = it })
                            CopyCheckboxItem(label = "항성 회합 (Fixed Star Conjunctions ≤ 1.5°)", checked = incWesternStars, onCheckedChange = { incWesternStars = it })
                            CopyCheckboxItem(label = "아라빅 파츠 및 하우스 알무텐 (Arabic Parts & Almutens)", checked = incWesternParts, onCheckedChange = { incWesternParts = it })
                        }
                        if (predictive != null) {
                            Spacer(modifier = Modifier.height(6.dp))
                            HorizontalDivider(color = BorderColor, thickness = 0.5.dp)
                            Spacer(modifier = Modifier.height(6.dp))
                            CopyCheckboxItem(label = "연간 프로펙션 (Annual Profection · ${predictive.target_year})", checked = incWesternProfection, onCheckedChange = { incWesternProfection = it })
                            CopyCheckboxItem(label = "태양 및 행성 회귀 (Solar & Planetary Returns · ${predictive.target_year})", checked = incWesternReturns, onCheckedChange = { incWesternReturns = it })
                            CopyCheckboxItem(label = "솔라 아크 및 세컨더리 프로그레션 (Solar Arc & Progressions)", checked = incWesternProgressions, onCheckedChange = { incWesternProgressions = it })
                            CopyCheckboxItem(label = "트랜짓 및 프라이머리 디렉션 (Transits & Primary Directions)", checked = incWesternTransitsDirections, onCheckedChange = { incWesternTransitsDirections = it })
                        }
                    }
                    2 -> {
                        CopyCheckboxItem(label = "라그나 및 D1 라시 차트 배치 (품위/연소 포함)", checked = incVedicRasi, onCheckedChange = { incVedicRasi = it })
                        CopyCheckboxItem(label = "베다 핵심 지표 (판창가, 카라카, 파다, 드리슈티, SAV)", checked = incVedicIndicators, onCheckedChange = { incVedicIndicators = it })
                        CopyCheckboxItem(label = "19종 분점도 (Shodashavarga + D6, D8, D11)", checked = incVedicVargas, onCheckedChange = { incVedicVargas = it })
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
                        predictive = predictive,
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
                        incWesternProfection = incWesternProfection,
                        incWesternReturns = incWesternReturns,
                        incWesternProgressions = incWesternProgressions,
                        incWesternTransitsDirections = incWesternTransitsDirections,
                        incVedicRasi = incVedicRasi,
                        incVedicIndicators = incVedicIndicators,
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
    predictive: PredictiveSuiteData?,
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
    incWesternProfection: Boolean,
    incWesternReturns: Boolean,
    incWesternProgressions: Boolean,
    incWesternTransitsDirections: Boolean,
    incVedicRasi: Boolean,
    incVedicIndicators: Boolean,
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
        sb.append("- 출생 일시: ${profile.birth_year}-${String.format("%02d", profile.birth_month)}-${String.format("%02d", profile.birth_day)} ${String.format("%02d:%02d:%02d", profile.birth_hour, profile.birth_minute, profile.birth_second.toInt())} (현지 기준)\n")
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
                val isWholeSign = w.house_system?.lowercase() in listOf("wholesign", "whole_sign", "whole", "hellenistic") ||
                    (w.hellenistic != null && w.traditional == null)

                if (incWesternPlanets) {
                    val ascSign = getZodiacSignName((w.ascendant / 30.0).toInt())
                    val mcSign = getZodiacSignName((w.midheaven / 30.0).toInt())
                    val mcHouseNum = if (w.house_cusps.isNotEmpty()) findHouseNumber(w.midheaven, w.house_cusps) else 10
                    val mcHouseDesc = if (isWholeSign) " [Located in House ${String.format("%02d", mcHouseNum)}]" else ""

                    sb.append("## 2. Celestial Placements & House Cusps\n")
                    sb.append("- Tradition & House System: ${if (isWholeSign) "Hellenistic (Whole Sign)" else "Medieval (Regiomontanus)"}\n")
                    sb.append("- Ascendant: ${String.format("%.2f°", w.ascendant)} ($ascSign)\n")
                    sb.append("- Midheaven (MC): ${String.format("%.2f°", w.midheaven)} ($mcSign)$mcHouseDesc\n")
                    if (w.house_cusps.isNotEmpty()) {
                        if (isWholeSign) {
                            sb.append("- House Signs (Whole Sign Houses):\n")
                            w.house_cusps.forEachIndexed { idx, cusp ->
                                val hNum = idx + 1
                                val signIdx = ((cusp / 30.0).toInt() % 12 + 12) % 12
                                val signName = getZodiacSignName(signIdx)
                                val ruler = getZodiacSignRuler(signIdx)
                                val mcTag = if (hNum == mcHouseNum) " [MC Location]" else ""
                                sb.append("  * House ${String.format("%02d", hNum)}$mcTag: $signName (Ruler: $ruler)\n")
                            }
                        } else {
                            sb.append("- House Cusps (12 Regiomontanus Cusps):\n")
                            w.house_cusps.forEachIndexed { idx, cusp ->
                                val hNum = idx + 1
                                val signIdx = ((cusp / 30.0).toInt() % 12 + 12) % 12
                                val signName = getZodiacSignName(signIdx)
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
                    }
                    sb.append("- Planetary Placements:\n")
                    w.planets.forEach { p ->
                        val sign = getZodiacSignName(p.sign_index)
                        val rx = if (p.is_retrograde) " [Rx]" else ""
                        val speedStr = String.format("%+.2f°/day", p.speed)
                        sb.append("  * ${p.name}: ${String.format("%.2f°", p.longitude)} ($sign) - House ${p.house_number}$rx (Speed: $speedStr)\n")
                    }
                    sb.append("- Lunar Nodes:\n")
                    w.nodes.forEach { n ->
                        val sign = getZodiacSignName(n.sign_index)
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

                if (isWholeSign) {
                    val h = w.hellenistic
                    if (h != null && (incWesternDignities || incWesternDynamicAspects || incWesternStars || incWesternParts)) {
                        sb.append("\n## 3. Hellenistic Traditional Analysis (Whole Sign)\n")
                        val ascSign = getZodiacSignName(h.angles.ascendant_sign)
                        val mcSign = getZodiacSignName(h.angles.midheaven_sign)
                        val mcHouseStr = if (h.angles.is_mc_in_10th) "House 10 (Culmination)" else "House ${h.angles.midheaven_house} (Action Point)"
                        sb.append("- Hellenistic Angles:\n")
                        sb.append("  * Ascendant (ASC): ${String.format("%.2f°", h.angles.ascendant_degree)} $ascSign\n")
                        sb.append("  * Midheaven (MC): ${String.format("%.2f°", h.angles.midheaven_degree)} $mcSign · $mcHouseStr\n")
                        if (incWesternDignities) {
                            sb.append("- Qualitative Planet Conditions, Egyptian Bounds & Dorothean Triplicity:\n")
                            h.planet_conditions.forEach { pc ->
                                val boundStr = if (pc.bound_ruler.isNotBlank()) ", Bound: ${pc.bound_ruler}" else ""
                                val tripRulers = if (pc.triplicity_day_ruler.isNotBlank()) {
                                    ", Triplicity(${pc.triplicity_day_ruler}/${pc.triplicity_night_ruler}/${pc.triplicity_participating_ruler}): ${if (pc.is_in_triplicity) "In Triplicity" else "No"}"
                                } else ""
                                val lordTag = if (pc.is_ruler_of_asc) " [Lord of ASC]" else ""
                                sb.append("  * ${pc.name}: ${pc.condition_label} · House ${pc.house_number}$boundStr$tripRulers$lordTag\n")
                            }
                        }
                        if (incWesternDynamicAspects) {
                            sb.append("- Sign-Based Aspects & Degree Contacts:\n")
                            if (h.aspects.isEmpty()) {
                                sb.append("  * No sign-based aspects within orb\n")
                            } else {
                                h.aspects.forEach { a ->
                                    val app = if (a.is_applying) "Applying (${String.format("%.1f°", a.degrees_to_perfection)})" else "Separating"
                                    val part = if (a.is_partile) " [Partile]" else ""
                                    sb.append("  * ${a.planet1} ${a.aspect_type} ${a.planet2}: $app (Orb: ${String.format("%.2f°", a.orb)})$part\n")
                                }
                            }
                        }
                        if (incWesternParts) {
                            if (h.lots.isNotEmpty()) {
                                sb.append("- Hermetic Lots (Paulus Alexandrinus):\n")
                                h.lots.forEach { lot ->
                                    val sName = getZodiacSignName(lot.sign_index)
                                    sb.append("  * ${lot.name}: ${String.format("%.2f°", lot.sign_degree)} ($sName) - House ${lot.house_number} (Ruler: ${lot.ruler})\n")
                                }
                            }
                            h.prenatal_syzygy?.let { san ->
                                val sanSign = getZodiacSignName(san.sign_index)
                                sb.append("- Prenatal Syzygy (SAN): ${san.syzygy_type} at ${String.format("%.2f°", san.sign_degree)} ($sanSign) on ${san.exact_datetime_utc}\n")
                            }
                        }
                        if (incWesternStars) {
                            sb.append("- Fixed Stars Conjunctions (Orb <= 1.5°):\n")
                            if (h.fixed_star_conjunctions.isEmpty()) {
                                sb.append("  * No major fixed star conjunctions within orb\n")
                            } else {
                                h.fixed_star_conjunctions.forEach { s ->
                                    val visTag = if (s.is_visible) " [Visible]" else " [Below Horizon]"
                                    sb.append("  * ${s.star_name} ☌ ${s.body_name} (Orb: ${String.format("%.2f°", s.orb)}, Nature: ${s.nature})$visTag: ${s.meaning}\n")
                                }
                            }
                        }
                    }
                } else if (w.traditional != null && (incWesternHours || incWesternMoon || incWesternDignities || incWesternDynamicAspects || incWesternReceptions || incWesternStars || incWesternParts)) {
                    val trad = w.traditional
                    sb.append("\n## 3. Medieval Traditional Analysis (Regiomontanus)\n")
                    if (incWesternHours) {
                        sb.append("- Planetary Hours: Day of ${trad.planetary_hours.day_ruler}, Hour of ${trad.planetary_hours.hour_ruler} (Hour ${trad.planetary_hours.hour_number}, ${if (trad.planetary_hours.is_day_hour) "Day" else "Night"})\n")
                    }
                    if (incWesternMoon) {
                        sb.append("- Moon Condition: Phase [${trad.moon_condition.lunar_phase_name}]\n")
                        trad.prenatal_syzygy?.let { san ->
                            val sanSign = getZodiacSignName(san.sign_index)
                            sb.append("- Prenatal Syzygy (SAN): ${san.syzygy_type} at ${String.format("%.2f°", san.sign_degree)} ($sanSign) on ${san.exact_datetime_utc}\n")
                        }
                    }
                    if (incWesternDignities) {
                        sb.append("- Essential & Accidental Dignities:\n")
                        trad.planet_dignities.forEach { d ->
                            val scoreSign = if (d.total_lilly_score >= 0) "+${d.total_lilly_score}" else "${d.total_lilly_score}"
                            val essSign = if (d.essential_score >= 0) "+${d.essential_score}" else "${d.essential_score}"
                            val accSign = if (d.accidental_score >= 0) "+${d.accidental_score}" else "${d.accidental_score}"
                            sb.append("  * ${d.planet}: $scoreSign ($essSign Ess, $accSign Acc)\n")
                        }
                    }
                    if (incWesternDynamicAspects) {
                        sb.append("- Dynamic Classical Aspects (Moiety Orb):\n")
                        trad.traditional_aspects.forEach { a ->
                            val app = if (a.is_applying) "Applying" else "Separating"
                            val part = if (a.is_partile) " [Partile]" else ""
                            sb.append("  * ${a.planet1} ${a.aspect_type} ${a.planet2}: $app (Orb: ${String.format("%.2f°", a.orb)})$part\n")
                        }
                    }
                    if (incWesternReceptions) {
                        sb.append("- Mutual Receptions:\n")
                        if (trad.mutual_receptions.isEmpty()) {
                            sb.append("  * No mutual receptions\n")
                        } else {
                            trad.mutual_receptions.forEach { r ->
                                sb.append("  * ${r.planet1} - ${r.planet2}: ${r.reception_type} (${r.description})\n")
                            }
                        }
                        if (trad.antiscia.isNotEmpty()) {
                            sb.append("- Antiscia & Contra-Antiscia:\n")
                            trad.antiscia.forEach { ant ->
                                val antSign = getZodiacSignName(ant.antiscion_sign_index)
                                sb.append("  * ${ant.body_name}: Antiscion ${String.format("%.2f°", ant.antiscion_degree)} ($antSign) | Contra-Antiscion ${String.format("%.2f°", ant.contra_antiscion_longitude)}\n")
                            }
                        }
                    }
                    if (incWesternStars) {
                        sb.append("- Fixed Stars Conjunctions (Orb <= 1.5°):\n")
                        if (trad.fixed_star_conjunctions.isEmpty()) {
                            sb.append("  * No major fixed star conjunctions within orb\n")
                        } else {
                            trad.fixed_star_conjunctions.forEach { s ->
                                val visTag = if (s.is_visible) " [Visible]" else " [Below Horizon]"
                                sb.append("  * ${s.star_name} ☌ ${s.body_name} (Orb: ${String.format("%.2f°", s.orb)}, Nature: ${s.nature})$visTag: ${s.meaning}\n")
                            }
                        }
                    }
                    if (incWesternParts) {
                        sb.append("- Arabic Parts:\n")
                        trad.arabic_parts.forEach { p ->
                            val sName = getZodiacSignName(p.sign_index)
                            sb.append("  * ${p.name}: ${String.format("%.2f°", p.longitude)} ($sName) - House ${p.house_number} (Ruler: ${p.ruler})\n")
                        }
                        if (trad.house_almutens.isNotEmpty()) {
                            sb.append("- House Cusp Almutens:\n")
                            trad.house_almutens.forEach { ha ->
                                sb.append("  * House ${ha.house_number}: ${ha.almuten} (${ha.almuten_score} pts)\n")
                            }
                        }
                    }
                }

                if (predictive != null && (incWesternProfection || incWesternReturns || incWesternProgressions || incWesternTransitsDirections)) {
                    sb.append("\n## 4. Predictive Astrology Analysis (Target: ${predictive.target_year}-${String.format("%02d", predictive.target_month)}-${String.format("%02d", predictive.target_day)} ${String.format("%02d:%02d", predictive.target_hour, predictive.target_minute)}, Age ${predictive.age})\n")

                    if (incWesternProfection) {
                        val prof = predictive.profection
                        val occ = if (prof.active_natal_planets.isEmpty()) "None" else prof.active_natal_planets.joinToString(", ")
                        sb.append("\n### Annual Profection\n")
                        sb.append("- Target Year: ${prof.target_year} (Age ${prof.target_age})\n")
                        sb.append("- Profected House: House ${prof.profected_house} (${prof.profected_sign_name})\n")
                        sb.append("- Lord of the Year (Time Lord): ${prof.lord_of_the_year}\n")
                        sb.append("- Active Natal Planets in Profected House: $occ\n")
                    }

                    if (incWesternReturns) {
                        val allReturns = buildList {
                            val sr = predictive.solar_return
                            val hasSun = predictive.planetary_returns.any { it.return_body.equals("Sun", ignoreCase = true) }
                            if (sr != null && !hasSun) {
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
                            addAll(predictive.planetary_returns)
                        }
                        if (allReturns.isNotEmpty()) {
                            sb.append("\n### Solar & Planetary Returns (${predictive.target_year})\n")
                            allReturns.forEach { ret ->
                                val ascSign = getZodiacSignName((ret.ascendant / 30.0).toInt())
                                val mcSign = getZodiacSignName((ret.midheaven / 30.0).toInt())
                                sb.append("- **${ret.return_body} Return (${ret.target_year})**: Exact UTC `${ret.return_date_utc}` | Return ASC ${String.format("%.2f°", ret.ascendant)} ($ascSign), Return MC ${String.format("%.2f°", ret.midheaven)} ($mcSign)\n")
                                if (ret.return_planets.isNotEmpty()) {
                                    val plStr = ret.return_planets.joinToString(", ") { p ->
                                        val sName = getZodiacSignName(p.sign_index)
                                        val hNum = findHouseNumber(p.progressed_longitude, ret.house_cusps)
                                        val rx = if (p.is_retrograde) " Rx" else ""
                                        "${p.name}: ${String.format("%.2f°", p.sign_degree)} $sName (H$hNum$rx)"
                                    }
                                    sb.append("  * Return Placements: $plStr\n")
                                }
                                if (ret.natal_aspects.isNotEmpty()) {
                                    val aspStr = ret.natal_aspects.joinToString(", ") { a ->
                                        "Return ${a.return_point} ${a.aspect_type} Natal ${a.natal_point} (Orb ${String.format("%.2f°", a.orb)})"
                                    }
                                    sb.append("  * Return-to-Natal Aspects: $aspStr\n")
                                }
                            }
                        }
                    }

                    if (incWesternProgressions) {
                        val sa = predictive.solar_arc
                        sb.append("\n### Solar Arc Directions (Offset: +${String.format("%.2f°", sa.arc_degrees)})\n")
                        sa.directed_planets.forEach { p ->
                            val sName = getZodiacSignName(p.sign_index)
                            sb.append("- SA ${p.name}: ${String.format("%.2f°", p.sign_degree)} ($sName) [Natal: ${String.format("%.2f°", p.natal_longitude)}]\n")
                        }
                        if (sa.contacts.isNotEmpty()) {
                            sb.append("- Solar Arc Contacts (Orb <= 1.5°):\n")
                            sa.contacts.forEach { c ->
                                sb.append("  * SA ${c.moving_body} ${c.aspect_type} Natal ${c.natal_body} (Orb: ${String.format("%.2f°", c.orb)})\n")
                            }
                        }

                        predictive.secondary_progression?.let { sp ->
                            sb.append("\n### Secondary Progressions (Ephemeris Date: ${sp.progressed_date_utc}, Phase: ${sp.progressed_lunar_phase})\n")
                            sp.planets.forEach { p ->
                                val sName = getZodiacSignName(p.sign_index)
                                val rx = if (p.is_retrograde) " [Rx]" else ""
                                sb.append("- Progressed ${p.name}: ${String.format("%.2f°", p.sign_degree)} ($sName)$rx\n")
                            }
                            if (sp.contacts.isNotEmpty()) {
                                sb.append("- Progressed-to-Natal Contacts (Orb <= 1.5°):\n")
                                sp.contacts.forEach { c ->
                                    sb.append("  * Progressed ${c.moving_body} ${c.aspect_type} Natal ${c.natal_body} (Orb: ${String.format("%.2f°", c.orb)})\n")
                                }
                            }
                        }
                    }

                    if (incWesternTransitsDirections) {
                        if (predictive.transits.isNotEmpty()) {
                            sb.append("\n### Active Transits to Natal (Orb <= 2.0°)\n")
                            predictive.transits.forEach { t ->
                                sb.append("- Transit ${t.transiting_planet} (${String.format("%.2f°", t.transiting_longitude)}) ${t.aspect_type} Natal ${t.natal_planet} (${String.format("%.2f°", t.natal_longitude)}) (Orb: ${String.format("%.2f°", t.orb)})\n")
                            }
                        }
                        if (predictive.primary_directions.isNotEmpty()) {
                            sb.append("\n### Primary Directions (Naibod Key)\n")
                            predictive.primary_directions.forEach { pd ->
                                sb.append("- ${pd.promittor} ${pd.aspect_type} ${pd.significator} (Arc: ${String.format("%.2f°", pd.direction_arc_deg)}, Age: ${String.format("%.1f", pd.realization_age)})\n")
                            }
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
                    if (v.ayanamsha_deg > 0.0) {
                        sb.append("- Ayanamsha: ${v.ayanamsha_name} (${String.format("%.4f°", v.ayanamsha_deg)})\n")
                    }
                    sb.append("- Lagna: ${String.format("%.2f°", v.lagna_sidereal)} ($lagnaSign)\n")
                    v.planets.forEach { p ->
                        val sign = VEDIC_SIGNS.getOrElse(p.sign_index % 12) { "" }
                        val rx = if (p.is_retrograde) " [Rx]" else ""
                        val comb = if (p.combustion != "None" && p.combustion.isNotEmpty()) " [${p.combustion}]" else ""
                        val dig = if (p.dignity.isNotEmpty()) ", Dignity: ${p.dignity}" else ""
                        sb.append("  * ${p.name}: ${String.format("%.2f°", p.sidereal_longitude)} (H${p.house}, $sign ${String.format("%.2f°", p.sign_degree)}, ${p.nakshatra_name} Pada ${p.nakshatra_pada}$dig)$rx$comb\n")
                    }
                }
                if (incVedicIndicators) {
                    sb.append("\n## 3. 베다 핵심 해석 지표 (Panchanga, Karakas, Padas, Drishti, Ashtakavarga)\n")
                    val pan = v.panchanga
                    if (pan.tithi_name.isNotEmpty()) {
                        sb.append("- Panchanga:\n")
                        sb.append("  * Tithi: ${pan.tithi_name} (#${pan.tithi_number}, ${pan.paksha})\n")
                        sb.append("  * Vara: ${pan.vara_name} (Lord: ${pan.vara_lord})\n")
                        sb.append("  * Nakshatra: ${pan.nakshatra_name} Pada ${pan.nakshatra_pada} (Lord: ${pan.nakshatra_lord})\n")
                        sb.append("  * Yoga: ${pan.yoga_name} | Karana: ${pan.karana_name}\n")
                    }
                    if (v.chara_karakas.isNotEmpty()) {
                        sb.append("- Jaimini Chara Karakas (7-Karaka Scheme):\n")
                        v.chara_karakas.forEach { ck ->
                            sb.append("  * ${ck.karaka}: ${ck.planet} (${String.format("%.2f°", ck.effective_degree)})\n")
                        }
                    }
                    if (v.arudha_padas.isNotEmpty()) {
                        sb.append("- Arudha Padas:\n")
                        v.arudha_padas.forEach { ap ->
                            sb.append("  * ${ap.code} (${ap.name}): ${ap.sign_name} (H${ap.house_from_lagna})\n")
                        }
                    }
                    val notableDrishti = v.drishti.filter { it.aspected_planets.isNotEmpty() || it.aspect_offset != 7 }
                    if (notableDrishti.isNotEmpty()) {
                        sb.append("- Graha Drishti (Full & Special Aspects):\n")
                        notableDrishti.forEach { d ->
                            val tSign = VEDIC_SIGNS.getOrElse(d.target_sign % 12) { "" }
                            val occ = if (d.aspected_planets.isEmpty()) "Empty Bhava" else d.aspected_planets.joinToString(", ")
                            sb.append("  * ${d.from_planet} (H${d.from_house}) -> ${d.aspect_offset}th on H${d.target_house} ($tSign): $occ\n")
                        }
                    }
                    if (v.ashtakavarga.total_bindus > 0) {
                        val savStr = v.ashtakavarga.sarva.mapIndexed { idx, pts -> "${VEDIC_SIGNS.getOrElse(idx) { "" }}: $pts" }.joinToString(", ")
                        sb.append("- Sarvashtakavarga (Total ${v.ashtakavarga.total_bindus}): $savStr\n")
                    }
                }
                if (incVedicVargas) {
                    sb.append("\n## 4. 분점도 (Vargas)\n")
                    v.vargas.forEach { (key, chart) ->
                        val lSign = VEDIC_SIGNS.getOrElse(chart.lagna_sign % 12) { "" }
                        sb.append("- $key: Lagna $lSign\n")
                        val pl = chart.placements.map { p ->
                            val pSign = VEDIC_SIGNS.getOrElse(p.sign_index % 12) { "" }
                            if (key == "D1") {
                                "${p.planet_name}: H${p.house} $pSign ${String.format("%.2f°", p.sign_degree)}"
                            } else {
                                "${p.planet_name}: H${p.house} $pSign"
                            }
                        }
                        if (pl.isNotEmpty()) {
                            sb.append("  * ${pl.joinToString(", ")}\n")
                        }
                    }
                }
                if (incVedicDasha) {
                    sb.append("\n## 5. 빔쇼타리 다샤 운로 (Vimshottari Dasha)\n")
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
