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
import com.fortunexample.note.data.FortuneChartOutput
import com.fortunexample.note.data.ProfileDto
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
    chart: FortuneChartOutput?,
    selectedDomainTab: Int // 0: Saju, 1: Western, 2: Vedic
) {
    if (!isOpen || chart == null) return

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
                        CopyCheckboxItem(label = "천체 및 하우스 기본 배치 (행성/노드/ASC/MC)", checked = incWesternPlanets, onCheckedChange = { incWesternPlanets = it })
                        CopyCheckboxItem(label = "메이저 아스펙트 (Major Aspects)", checked = incWesternAspects, onCheckedChange = { incWesternAspects = it })
                        CopyCheckboxItem(label = "윌리엄 릴리 5중 본질적 품계 & 위계 총점", checked = incWesternDignities, onCheckedChange = { incWesternDignities = it })
                        CopyCheckboxItem(label = "달의 상태 (VOC, Via Combusta, 다음 각)", checked = incWesternMoon, onCheckedChange = { incWesternMoon = it })
                        CopyCheckboxItem(label = "플래너터리 아워 & 차트 적격성 (Radicality)", checked = incWesternHours, onCheckedChange = { incWesternHours = it })
                        CopyCheckboxItem(label = "전통 동적 아스펙트 (접근/분리, 모이어티)", checked = incWesternDynamicAspects, onCheckedChange = { incWesternDynamicAspects = it })
                        CopyCheckboxItem(label = "상호 리셉션 및 포위 (Mutual Reception)", checked = incWesternReceptions, onCheckedChange = { incWesternReceptions = it })
                        CopyCheckboxItem(label = "주요 항성 회합 (Fixed Star Conjunctions ≤ 1.5°)", checked = incWesternStars, onCheckedChange = { incWesternStars = it })
                        CopyCheckboxItem(label = "하우스 커스프 알무텐 & 아라빅 파트", checked = incWesternParts, onCheckedChange = { incWesternParts = it })
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
                        chart = chart,
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
    chart: FortuneChartOutput,
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
        sb.append("- 시간대 보정: ${chart.timezone_id} (UTC ${String.format("%+0.1f시간", chart.utc_offset_hours)})\n")
        if (profile.notes.isNotEmpty()) {
            sb.append("- 비고: ${profile.notes}\n")
        }
        sb.append("\n")
    }

    when (selectedDomainTab) {
        0 -> {
            if (incSajuPillars) {
                sb.append("## 2. 사주팔자 명식 (Four Pillars of Destiny)\n")
                sb.append("- 명식: 연주 [${chart.saju.year_pillar.korean} (${chart.saju.year_pillar.hanja})], 월주 [${chart.saju.month_pillar.korean} (${chart.saju.month_pillar.hanja})], 일주 [${chart.saju.day_pillar.korean} (${chart.saju.day_pillar.hanja})], 시주 [${chart.saju.hour_pillar.korean} (${chart.saju.hour_pillar.hanja})]\n")
                sb.append("- 대운 방향: ${if (chart.saju.is_forward_daeun) "순행" else "역행"} (대운수: ${chart.saju.daeun_number})\n")
            }
            if (incSajuDaeun) {
                sb.append("- 10년 주기 대운 흐름:\n")
                chart.saju.daeun_periods.forEach { d ->
                    sb.append("  * ${d.start_age}세 ~ ${d.start_age + 9}세: [${d.pillar.korean} (${d.pillar.hanja})]\n")
                }
            }
        }
        1 -> {
            if (incWesternPlanets) {
                val ascSign = ZODIAC_NAMES.getOrElse((chart.western.ascendant / 30.0).toInt() % 12) { "" }
                val mcSign = ZODIAC_NAMES.getOrElse((chart.western.midheaven / 30.0).toInt() % 12) { "" }
                sb.append("## 2. 천체 및 하우스 기본 배치\n")
                sb.append("- Ascendant: ${String.format("%.2f°", chart.western.ascendant)} ($ascSign)\n")
                sb.append("- Midheaven (MC): ${String.format("%.2f°", chart.western.midheaven)} ($mcSign)\n")
                chart.western.house_system?.let { sb.append("- 하우스 시스템: $it\n") }
                sb.append("- 행성 배치:\n")
                chart.western.planets.forEach { p ->
                    val sign = ZODIAC_NAMES.getOrElse(p.sign_index % 12) { "" }
                    val rx = if (p.is_retrograde) " [Rx]" else ""
                    sb.append("  * ${p.name}: ${String.format("%.2f°", p.longitude)} ($sign) - House ${p.house_number}$rx\n")
                }
                sb.append("- 노드 배치:\n")
                chart.western.nodes.forEach { n ->
                    val sign = ZODIAC_NAMES.getOrElse(n.sign_index % 12) { "" }
                    sb.append("  * ${n.name}: ${String.format("%.2f°", n.longitude)} ($sign) - House ${n.house_number}\n")
                }
            }
            if (incWesternAspects) {
                sb.append("- 메이저 아스펙트 (Major Aspects):\n")
                if (chart.western.aspects.isEmpty()) {
                    sb.append("  * No major aspects within orb\n")
                } else {
                    chart.western.aspects.forEach { a ->
                        sb.append("  * ${a.planet1} - ${a.planet2}: ${a.aspect_type} (오차: ${String.format("%.2f°", a.orb)})\n")
                    }
                }
            }

            chart.western.traditional?.let { trad ->
                sb.append("\n## 3. 윌리엄 릴리 전통 점성술 정밀 분석\n")
                if (incWesternHours) {
                    sb.append("- 플래너터리 아워: ${trad.planetary_hours.day_ruler}의 날, ${trad.planetary_hours.hour_ruler}의 시간 (적격성: ${if (trad.planetary_hours.is_radical) "적격" else "부적격"} - ${trad.planetary_hours.radicality_reason})\n")
                    if (trad.planetary_hours.considerations.isNotEmpty()) {
                        sb.append("- 판단 전 고려사항 (Considerations Before Judgment):\n")
                        trad.planetary_hours.considerations.forEach { c ->
                            sb.append("  * $c\n")
                        }
                    }
                }
                if (incWesternMoon) {
                    sb.append("- 달의 상태: 위상 [${trad.moon_condition.lunar_phase_name}], VOC [${if (trad.moon_condition.is_void_of_course) "보이드 오브 코스" else "정상"}], Via Combusta [${if (trad.moon_condition.is_via_combusta) "연소도" else "정상"}]\n")
                    trad.moon_condition.next_applying_aspect?.let {
                        sb.append("  * 달의 다음 접근 각: $it\n")
                    }
                }
                if (incWesternDignities && trad.planet_dignities.isNotEmpty()) {
                    sb.append("\n### 5중 본질적 품계 및 릴리 위계 총점\n")
                    sb.append("| 행성 | 황경 | 하우스 | 품계 상태 | 본질 점수 | 우발 점수 | 릴리 총점 |\n")
                    sb.append("| :--- | :--- | :---: | :--- | :---: | :---: | :---: |\n")
                    trad.planet_dignities.forEach { d ->
                        val st = mutableListOf<String>()
                        if (d.is_in_domicile) st.add("Domicile (+5)")
                        if (d.is_in_exaltation) st.add("Exaltation (+4)")
                        if (d.is_in_triplicity) st.add("Triplicity (+3)")
                        if (d.is_in_term) st.add("Term (+2)")
                        if (d.is_in_face) st.add("Face (+1)")
                        if (d.is_in_detriment) st.add("Detriment (-5)")
                        if (d.is_in_fall) st.add("Fall (-4)")
                        if (d.is_peregrine) st.add("Peregrine (-5)")
                        if (st.isEmpty()) st.add("Neutral")
                        sb.append("| ${d.planet} | ${String.format("%.2f°", d.longitude)} | H${d.house_number} | ${st.joinToString(", ")} | ${String.format("%+d", d.essential_score)} | ${String.format("%+d", d.accidental_score)} | **${String.format("%+d", d.total_lilly_score)}** |\n")
                    }
                }
                if (incWesternDynamicAspects && trad.traditional_aspects.isNotEmpty()) {
                    sb.append("\n### 전통 동적 아스펙트 (접근 / 분리 및 모이어티 오브)\n")
                    trad.traditional_aspects.forEach { ta ->
                        val dyn = if (ta.is_applying) "접근 (Applying)" else "분리 (Separating)"
                        val partile = if (ta.is_partile) " [Partile]" else ""
                        sb.append("- ${ta.planet1} - ${ta.planet2}: ${ta.aspect_type} (오차: ${String.format("%.2f°", ta.orb)} / 허용: ${String.format("%.2f°", ta.max_allowed_orb)})$partile, $dyn\n")
                    }
                }
                if (incWesternReceptions && trad.mutual_receptions.isNotEmpty()) {
                    sb.append("\n### 상호 리셉션 (Mutual Reception)\n")
                    trad.mutual_receptions.forEach { mr ->
                        sb.append("- ${mr.planet1} - ${mr.planet2}: [${mr.reception_type}] ${mr.description}\n")
                    }
                }
                if (incWesternStars && trad.fixed_star_conjunctions.isNotEmpty()) {
                    sb.append("\n### 주요 항성 회합 (Fixed Star Conjunctions, Orb ≤ 1.5°)\n")
                    trad.fixed_star_conjunctions.forEach { fs ->
                        sb.append("- ${fs.star_name} - ${fs.planet_name}: 오차 ${String.format("%.2f°", fs.orb)} (본성: ${fs.nature}, 광도: ${fs.magnitude}등급) - ${fs.description}\n")
                    }
                }
                if (incWesternParts && trad.arabic_parts.isNotEmpty()) {
                    sb.append("\n### 아라빅 파트 (Arabic Parts)\n")
                    trad.arabic_parts.forEach { part ->
                        val sign = ZODIAC_NAMES.getOrElse(part.sign_index % 12) { "" }
                        sb.append("- ${part.name}: ${String.format("%.2f°", part.longitude)} ($sign, House ${part.house_number}) [공식: ${part.formula}]\n")
                    }
                }
            }
        }
        2 -> {
            if (incVedicRasi) {
                val lagnaSign = VEDIC_SIGNS.getOrElse(chart.vedic.lagna_sign % 12) { "" }
                sb.append("## 2. 라시 차트 (Vedic Sidereal Placements)\n")
                sb.append("- Lagna: ${String.format("%.2f°", chart.vedic.lagna_sidereal)} ($lagnaSign)\n")
                chart.vedic.planets.forEach { p ->
                    val sign = VEDIC_SIGNS.getOrElse(p.sign_index % 12) { "" }
                    val rx = if (p.is_retrograde) " [Rx]" else ""
                    sb.append("  * ${p.name}: ${String.format("%.2f°", p.sidereal_longitude)} ($sign, ${p.nakshatra_name} Pada ${p.nakshatra_pada})$rx\n")
                }
            }
            if (incVedicVargas) {
                sb.append("\n## 3. 분점도 (Vargas)\n")
                chart.vedic.vargas.forEach { (key, v) ->
                    val lSign = VEDIC_SIGNS.getOrElse(v.lagna_sign % 12) { "" }
                    sb.append("- $key: Lagna $lSign\n")
                    val pl = v.placements.map { p ->
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
                chart.vedic.vimshottari_dashas.forEach { d ->
                    sb.append("- Mahadasha ${d.lord}: ${d.startDateFormatted} ~ ${d.endDateFormatted}\n")
                    d.sub_periods.forEach { sub ->
                        sb.append("  * Antardasha ${sub.lord}: ${sub.startDateFormatted} ~ ${sub.endDateFormatted}\n")
                    }
                }
            }
        }
    }

    return sb.toString()
}
