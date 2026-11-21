package com.fortunexample.note.ui.components

import com.fortunexample.note.data.*
import java.util.Locale

internal fun fmtUs(pattern: String, vararg args: Any?): String =
    String.format(Locale.US, pattern, *args)

fun generateSelectiveMarkdown(
    payload: SelectiveCopyPayload,
    selectedDomainTab: Int,
    options: SelectiveCopyOptions
): String = buildString {
    if (options.incMeta) {
        appendProfileMetadata(payload, selectedDomainTab)
    }
    when (selectedDomainTab) {
        0 -> payload.saju?.let { appendSajuMarkdown(it, options) }
        1 -> payload.western?.let { appendWesternMarkdown(it, payload.predictive, options) }
        else -> payload.vedic?.let { appendVedicMarkdown(it, options) }
    }
}

private fun StringBuilder.appendProfileMetadata(
    payload: SelectiveCopyPayload,
    selectedDomainTab: Int
) {
    val profile = payload.profile
    val domainTitle = when (selectedDomainTab) {
        0 -> "사주명리 분석 데이터"
        1 -> "서양 점성술 (Western Tropical Natal Chart)"
        2 -> "베다 점성술 (Vedic Sidereal Chart)"
        else -> "천체 분석 데이터"
    }
    append("# $domainTitle: ${profile.name}\n\n")
    append("## 1. 인물 기본 정보\n")
    append("- 성별: ${if (profile.gender == "male") "남성" else "여성"}\n")
    append(
        "- 출생 일시: ${
            fmtUs(
                "%04d-%02d-%02d %02d:%02d:%02d",
                profile.birth_year, profile.birth_month, profile.birth_day,
                profile.birth_hour, profile.birth_minute, profile.birth_second.toInt()
            )
        } (현지 기준)\n"
    )
    append("- 관측 위치: 위도 ${fmtUs("%.4f°", profile.latitude)}, 경도 ${fmtUs("%.4f°", profile.longitude)}\n")
    val tzLine = payload.utcOffsetHours?.let { offset ->
        "${payload.timezoneId} (UTC ${fmtUs("%+.1f시간", offset)})"
    } ?: payload.timezoneId
    append("- 시간대 보정: $tzLine\n")
    if (profile.notes.isNotEmpty()) {
        append("- 비고: ${profile.notes}\n")
    }
    append("\n")
}

private fun StringBuilder.appendSajuMarkdown(s: SajuChartData, options: SelectiveCopyOptions) {
    if (options.incSajuPillars) {
        append("## 2. 사주팔자 명식 (Four Pillars of Destiny)\n")
        append(
            "- 명식: 연주 [${s.year_pillar.korean} (${s.year_pillar.hanja})], " +
                "월주 [${s.month_pillar.korean} (${s.month_pillar.hanja})], " +
                "일주 [${s.day_pillar.korean} (${s.day_pillar.hanja})], " +
                "시주 [${s.hour_pillar.korean} (${s.hour_pillar.hanja})]\n"
        )
        append("- 대운 방향: ${if (s.is_forward_daeun) "순행" else "역행"} (대운수: ${s.daeun_number})\n")
    }
    if (options.incSajuDaeun) {
        append("- 10년 주기 대운 흐름:\n")
        s.daeun_periods.forEach { d ->
            append("  * ${d.start_age}세 ~ ${d.start_age + 9}세: [${d.pillar.korean} (${d.pillar.hanja})]\n")
        }
    }
}

private fun StringBuilder.appendVedicMarkdown(v: VedicChartData, options: SelectiveCopyOptions) {
    if (options.incVedicRasi) appendVedicRasiSection(v)
    if (options.incVedicIndicators) appendVedicIndicatorsSection(v)
    if (options.incVedicVargas) appendVedicVargasSection(v)
    if (options.incVedicDasha) appendVedicDashaSection(v)
}

private fun StringBuilder.appendVedicRasiSection(v: VedicChartData) {
    append("## 2. 라시 차트 (Vedic Sidereal Placements)\n")
    if (v.ayanamsha_deg > 0.0) {
        append("- Ayanamsha: ${v.ayanamsha_name} (${fmtUs("%.4f°", v.ayanamsha_deg)})\n")
    }
    append("- Lagna: ${fmtUs("%.2f°", v.lagna_sidereal)} (${vedicSignFullLabel(v.lagna_sign)})\n")
    v.planets.forEach { p ->
        val rx = if (p.is_retrograde) " [Rx]" else ""
        val comb = if (p.combustion != "None" && p.combustion.isNotEmpty()) " [${p.combustion}]" else ""
        val dig = if (p.dignity.isNotEmpty()) ", Dignity: ${p.dignity}" else ""
        append(
            "  * ${p.name}: ${fmtUs("%.2f°", p.sidereal_longitude)} " +
                "(H${p.house}, ${vedicSignFullLabel(p.sign_index)} ${fmtUs("%.2f°", p.sign_degree)}, " +
                "${p.nakshatra_name} Pada ${p.nakshatra_pada}$dig)$rx$comb\n"
        )
    }
}

private fun StringBuilder.appendVedicIndicatorsSection(v: VedicChartData) {
    append("\n## 3. 베다 핵심 해석 지표 (Panchanga, Karakas, Padas, Drishti, Ashtakavarga)\n")
    val pan = v.panchanga
    if (pan.tithi_name.isNotEmpty()) {
        append("- Panchanga:\n")
        append("  * Tithi: ${pan.tithi_name} (#${pan.tithi_number}, ${pan.paksha})\n")
        append("  * Vara: ${pan.vara_name} (Lord: ${pan.vara_lord})\n")
        append("  * Nakshatra: ${pan.nakshatra_name} Pada ${pan.nakshatra_pada} (Lord: ${pan.nakshatra_lord})\n")
        append("  * Yoga: ${pan.yoga_name} | Karana: ${pan.karana_name}\n")
    }
    if (v.chara_karakas.isNotEmpty()) {
        append("- Jaimini Chara Karakas (7-Karaka Scheme):\n")
        v.chara_karakas.forEach { ck ->
            append("  * ${ck.karaka}: ${ck.planet} (${fmtUs("%.2f°", ck.effective_degree)})\n")
        }
    }
    if (v.arudha_padas.isNotEmpty()) {
        append("- Arudha Padas:\n")
        v.arudha_padas.forEach { ap ->
            append("  * ${ap.code} (${ap.name}): ${ap.sign_name} (H${ap.house_from_lagna})\n")
        }
    }
    val notableDrishti = v.drishti.filter { it.aspected_planets.isNotEmpty() || it.aspect_offset != 7 }
    if (notableDrishti.isNotEmpty()) {
        append("- Graha Drishti (Full & Special Aspects):\n")
        notableDrishti.forEach { d ->
            val occ = if (d.aspected_planets.isEmpty()) "Empty Bhava" else d.aspected_planets.joinToString(", ")
            append("  * ${d.from_planet} (H${d.from_house}) -> ${d.aspect_offset}th on H${d.target_house} (${vedicSignFullLabel(d.target_sign)}): $occ\n")
        }
    }
    if (v.ashtakavarga.total_bindus > 0) {
        val savStr = v.ashtakavarga.sarva.mapIndexed { idx, pts -> "${vedicSignFullLabel(idx)}: $pts" }.joinToString(", ")
        append("- Sarvashtakavarga (Total ${v.ashtakavarga.total_bindus}): $savStr\n")
    }
}

private fun StringBuilder.appendVedicVargasSection(v: VedicChartData) {
    append("\n## 4. 분점도 (Vargas)\n")
    v.vargas.forEach { (key, chart) ->
        append("- $key: Lagna ${vedicSignFullLabel(chart.lagna_sign)}\n")
        val placements = chart.placements.map { p ->
            val pSign = vedicSignFullLabel(p.sign_index)
            if (key == "D1") "${p.planet_name}: H${p.house} $pSign ${fmtUs("%.2f°", p.sign_degree)}"
            else "${p.planet_name}: H${p.house} $pSign"
        }
        if (placements.isNotEmpty()) {
            append("  * ${placements.joinToString(", ")}\n")
        }
    }
}

private fun StringBuilder.appendVedicDashaSection(v: VedicChartData) {
    append("\n## 5. 빔쇼타리 다샤 운로 (Vimshottari Dasha)\n")
    v.vimshottari_dashas.forEach { d ->
        append("- Mahadasha ${d.lord}: ${d.startDateFormatted} ~ ${d.endDateFormatted}\n")
        d.sub_periods.forEach { sub ->
            append("  * Antardasha ${sub.lord}: ${sub.startDateFormatted} ~ ${sub.endDateFormatted}\n")
        }
    }
}
