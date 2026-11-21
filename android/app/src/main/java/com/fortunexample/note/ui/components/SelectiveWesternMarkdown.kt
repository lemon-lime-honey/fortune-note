package com.fortunexample.note.ui.components

import com.fortunexample.note.data.*

internal fun StringBuilder.appendWesternMarkdown(
    w: WesternChartData,
    predictive: PredictiveSuiteData?,
    options: SelectiveCopyOptions
) {
    val isWholeSign = isWholeSignHouseSystem(w)
    if (options.incWesternPlanets) appendWesternPlacementsMd(w, isWholeSign)
    if (options.incWesternAspects) appendWesternAspectsMd(w)
    if (isWholeSign) {
        w.hellenistic?.let { appendHellenisticMd(it, options) }
    } else {
        w.traditional?.let { appendMedievalMd(it, options) }
    }
    predictive?.let { appendPredictiveMd(it, options) }
}

private fun StringBuilder.appendWesternPlacementsMd(w: WesternChartData, isWholeSign: Boolean) {
    val ascSign = getZodiacSignName((w.ascendant / 30.0).toInt())
    val mcSign = getZodiacSignName((w.midheaven / 30.0).toInt())
    val mcHouseNum = if (w.house_cusps.isNotEmpty()) findHouseNumber(w.midheaven, w.house_cusps) else 10
    val mcHouseDesc = if (isWholeSign) " [Located in House ${fmtUs("%02d", mcHouseNum)}]" else ""

    append("## 2. Celestial Placements & House Cusps\n")
    append("- Tradition & House System: ${if (isWholeSign) "Hellenistic (Whole Sign)" else "Medieval (Regiomontanus)"}\n")
    append("- Ascendant: ${fmtUs("%.2f°", w.ascendant)} ($ascSign)\n")
    append("- Midheaven (MC): ${fmtUs("%.2f°", w.midheaven)} ($mcSign)$mcHouseDesc\n")
    appendHouseCuspsMd(w.house_cusps, isWholeSign, mcHouseNum)
    append("- Planetary Placements:\n")
    w.planets.forEach { p ->
        val rx = if (p.is_retrograde) " [Rx]" else ""
        append("  * ${p.name}: ${fmtUs("%.2f°", p.longitude)} (${getZodiacSignName(p.sign_index)}) - House ${p.house_number}$rx (Speed: ${fmtUs("%+.2f°/day", p.speed)})\n")
    }
    append("- Lunar Nodes:\n")
    w.nodes.forEach { n ->
        append("  * ${n.name}: ${fmtUs("%.2f°", n.longitude)} (${getZodiacSignName(n.sign_index)}) - House ${n.house_number}\n")
    }
}

private fun StringBuilder.appendHouseCuspsMd(
    cusps: List<Double>,
    isWholeSign: Boolean,
    mcHouseNum: Int
) {
    if (cusps.isEmpty()) return
    if (isWholeSign) {
        append("- House Signs (Whole Sign Houses):\n")
        cusps.forEachIndexed { idx, cusp ->
            val hNum = idx + 1
            val signIdx = ((cusp / 30.0).toInt() % 12 + 12) % 12
            val mcTag = if (hNum == mcHouseNum) " [MC Location]" else ""
            append("  * House ${fmtUs("%02d", hNum)}$mcTag: ${getZodiacSignName(signIdx)} (Ruler: ${getZodiacSignRuler(signIdx)})\n")
        }
        return
    }
    append("- House Cusps (12 Regiomontanus Cusps):\n")
    cusps.forEachIndexed { idx, cusp ->
        val hNum = idx + 1
        val signIdx = ((cusp / 30.0).toInt() % 12 + 12) % 12
        val tag = when (hNum) { 1 -> " (ASC)"; 4 -> " (IC)"; 7 -> " (DSC)"; 10 -> " (MC)"; else -> "" }
        append("  * House ${fmtUs("%02d", hNum)}$tag: ${fmtUs("%.2f°", cusp % 30.0)} (${getZodiacSignName(signIdx)})\n")
    }
}

private fun StringBuilder.appendWesternAspectsMd(w: WesternChartData) {
    append("- Major Aspects:\n")
    if (w.aspects.isEmpty()) {
        append("  * No major aspects within orb\n")
        return
    }
    w.aspects.forEach { a ->
        val dyn = if (a.is_applying) "Applying" else "Separating"
        append("  * ${a.planet1} - ${a.planet2}: ${a.aspect_type} (Orb: ${fmtUs("%.2f°", a.orb)}, $dyn)\n")
    }
}

private fun StringBuilder.appendFixedStarsMd(stars: List<FixedStarConjunctionData>) {
    append("- Fixed Stars Conjunctions (Orb <= 1.5°):\n")
    if (stars.isEmpty()) {
        append("  * No major fixed star conjunctions within orb\n")
        return
    }
    stars.forEach { s ->
        val visTag = if (s.is_visible) " [Visible]" else " [Below Horizon]"
        append("  * ${s.star_name} ☌ ${s.body_name} (Orb: ${fmtUs("%.2f°", s.orb)}, Nature: ${s.nature})$visTag: ${s.meaning}\n")
    }
}

private fun StringBuilder.appendSanMd(san: PrenatalSyzygyData?) {
    if (san == null) return
    append("- Prenatal Syzygy (SAN): ${san.syzygy_type} at ${fmtUs("%.2f°", san.sign_degree)} (${getZodiacSignName(san.sign_index)}) on ${san.exact_datetime_utc}\n")
}

private fun StringBuilder.appendHellenisticMd(h: HellenisticData, options: SelectiveCopyOptions) {
    val anySelected = options.incWesternDignities || options.incWesternDynamicAspects || options.incWesternStars || options.incWesternParts
    if (!anySelected) return
    append("\n## 3. Hellenistic Traditional Analysis (Whole Sign)\n")
    val mcHouseStr = if (h.angles.is_mc_in_10th) "House 10 (Culmination)" else "House ${h.angles.midheaven_house} (Action Point)"
    append("- Hellenistic Angles:\n")
    append("  * Ascendant (ASC): ${fmtUs("%.2f°", h.angles.ascendant_degree)} ${getZodiacSignName(h.angles.ascendant_sign)}\n")
    append("  * Midheaven (MC): ${fmtUs("%.2f°", h.angles.midheaven_degree)} ${getZodiacSignName(h.angles.midheaven_sign)} · $mcHouseStr\n")

    if (options.incWesternDignities) {
        append("- Qualitative Planet Conditions, Egyptian Bounds & Dorothean Triplicity:\n")
        h.planet_conditions.forEach { pc ->
            val boundStr = if (pc.bound_ruler.isNotBlank()) ", Bound: ${pc.bound_ruler}" else ""
            val tripStr = if (pc.triplicity_day_ruler.isNotBlank()) {
                ", Triplicity(${pc.triplicity_day_ruler}/${pc.triplicity_night_ruler}/${pc.triplicity_participating_ruler}): ${if (pc.is_in_triplicity) "In Triplicity" else "No"}"
            } else ""
            val lordTag = if (pc.is_ruler_of_asc) " [Lord of ASC]" else ""
            append("  * ${pc.name}: ${pc.condition_label} · House ${pc.house_number}$boundStr$tripStr$lordTag\n")
        }
    }
    if (options.incWesternDynamicAspects) {
        append("- Sign-Based Aspects & Degree Contacts:\n")
        if (h.aspects.isEmpty()) append("  * No sign-based aspects within orb\n")
        else h.aspects.forEach { a ->
            val app = if (a.is_applying) "Applying (${fmtUs("%.1f°", a.degrees_to_perfection)})" else "Separating"
            val part = if (a.is_partile) " [Partile]" else ""
            append("  * ${a.planet1} ${a.aspect_type} ${a.planet2}: $app (Orb: ${fmtUs("%.2f°", a.orb)})$part\n")
        }
    }
    if (options.incWesternParts) {
        if (h.lots.isNotEmpty()) {
            append("- Hellenistic Lots (Vettius Valens):\n")
            h.lots.forEach { lot ->
                append("  * ${lot.name}: ${fmtUs("%.2f°", lot.sign_degree)} (${getZodiacSignName(lot.sign_index)}) - House ${lot.house_number} (Ruler: ${lot.ruler})\n")
            }
        }
        appendSanMd(h.prenatal_syzygy)
    }
    if (options.incWesternStars) appendFixedStarsMd(h.fixed_star_conjunctions)
}

private fun StringBuilder.appendMedievalMd(trad: WesternTraditionalData, options: SelectiveCopyOptions) {
    val anySelected = options.incWesternHours || options.incWesternMoon || options.incWesternDignities ||
        options.incWesternDynamicAspects || options.incWesternReceptions || options.incWesternStars || options.incWesternParts
    if (!anySelected) return
    append("\n## 3. Medieval Traditional Analysis (Regiomontanus)\n")
    if (options.incWesternHours) {
        val ph = trad.planetary_hours
        append("- Planetary Hours: Day of ${ph.day_ruler}, Hour of ${ph.hour_ruler} (Hour ${ph.hour_number}, ${if (ph.is_day_hour) "Day" else "Night"})\n")
    }
    if (options.incWesternMoon) {
        append("- Moon Condition: Phase [${trad.moon_condition.lunar_phase_name}]\n")
        appendSanMd(trad.prenatal_syzygy)
    }
    if (options.incWesternDignities) {
        append("- Essential & Accidental Dignities:\n")
        trad.planet_dignities.forEach { d ->
            append("  * ${d.planet}: ${formatSignedScore(d.total_lilly_score)} (${formatSignedScore(d.essential_score)} Ess, ${formatSignedScore(d.accidental_score)} Acc)\n")
        }
    }
    if (options.incWesternDynamicAspects) {
        append("- Dynamic Classical Aspects (Moiety Orb):\n")
        trad.traditional_aspects.forEach { a ->
            val app = if (a.is_applying) "Applying" else "Separating"
            val part = if (a.is_partile) " [Partile]" else ""
            append("  * ${a.planet1} ${a.aspect_type} ${a.planet2}: $app (Orb: ${fmtUs("%.2f°", a.orb)})$part\n")
        }
    }
    if (options.incWesternReceptions) appendMedievalReceptionsMd(trad)
    if (options.incWesternStars) appendFixedStarsMd(trad.fixed_star_conjunctions)
    if (options.incWesternParts) appendMedievalPartsMd(trad)
}

private fun StringBuilder.appendMedievalReceptionsMd(trad: WesternTraditionalData) {
    append("- Mutual Receptions:\n")
    if (trad.mutual_receptions.isEmpty()) append("  * No mutual receptions\n")
    else trad.mutual_receptions.forEach { r -> append("  * ${r.planet1} - ${r.planet2}: ${r.reception_type} (${r.description})\n") }
    if (trad.antiscia.isNotEmpty()) {
        append("- Antiscia & Contra-Antiscia:\n")
        trad.antiscia.forEach { ant ->
            append("  * ${ant.body_name}: Antiscion ${fmtUs("%.2f°", ant.antiscion_degree)} (${getZodiacSignName(ant.antiscion_sign_index)}) | Contra-Antiscion ${fmtUs("%.2f°", ant.contra_antiscion_longitude)}\n")
        }
    }
}

private fun StringBuilder.appendMedievalPartsMd(trad: WesternTraditionalData) {
    append("- Arabic Parts:\n")
    trad.arabic_parts.forEach { p ->
        append("  * ${p.name}: ${fmtUs("%.2f°", p.longitude)} (${getZodiacSignName(p.sign_index)}) - House ${p.house_number} (Ruler: ${p.ruler})\n")
    }
    if (trad.cusp_dignities.isNotEmpty()) {
        append("- House Cusp Almutens:\n")
        trad.cusp_dignities.forEach { ha ->
            append("  * House ${ha.house_number}: ${ha.almuten} (${ha.almuten_score} pts)\n")
        }
    }
}

private fun StringBuilder.appendPredictiveMd(pred: PredictiveSuiteData, options: SelectiveCopyOptions) {
    val anySelected = options.incWesternProfection || options.incWesternReturns ||
        options.incWesternProgressions || options.incWesternTransitsDirections
    if (!anySelected) return
    val ts = fmtUs("%04d-%02d-%02d %02d:%02d", pred.target_year, pred.target_month, pred.target_day, pred.target_hour, pred.target_minute)
    append("\n## 4. Predictive Astrology Analysis (Target: $ts, Age ${pred.age})\n")
    if (options.incWesternProfection) appendProfectionMd(pred.profection)
    if (options.incWesternReturns) appendReturnsMd(pred)
    if (options.incWesternProgressions) appendProgressionsMd(pred)
    if (options.incWesternTransitsDirections) appendTransitsAndDirectionsMd(pred)
}

private fun StringBuilder.appendProfectionMd(prof: AnnualProfectionData) {
    val occ = if (prof.active_natal_planets.isEmpty()) "None" else prof.active_natal_planets.joinToString(", ")
    append("\n### Annual Profection\n")
    append("- Target Year: ${prof.target_year} (Age ${prof.target_age})\n")
    append("- Profected House: House ${prof.profected_house} (${prof.profected_sign_name})\n")
    append("- Lord of the Year (Time Lord): ${prof.lord_of_the_year}\n")
    append("- Active Natal Planets in Profected House: $occ\n")
}

private fun StringBuilder.appendReturnsMd(pred: PredictiveSuiteData) {
    val allReturns = mergeSolarAndPlanetaryReturns(pred)
    if (allReturns.isEmpty()) return
    append("\n### Solar & Planetary Returns (${pred.target_year})\n")
    allReturns.forEach { ret ->
        val ascSign = getZodiacSignName((ret.ascendant / 30.0).toInt())
        val mcSign = getZodiacSignName((ret.midheaven / 30.0).toInt())
        append("- **${ret.return_body} Return (${ret.target_year})**: Exact UTC `${ret.return_date_utc}` | Return ASC ${fmtUs("%.2f°", ret.ascendant)} ($ascSign), Return MC ${fmtUs("%.2f°", ret.midheaven)} ($mcSign)\n")
        if (ret.return_planets.isNotEmpty()) {
            val plStr = ret.return_planets.joinToString(", ") { p ->
                val hNum = findHouseNumber(p.progressed_longitude, ret.house_cusps)
                val rx = if (p.is_retrograde) " Rx" else ""
                "${p.name}: ${fmtUs("%.2f°", p.sign_degree)} ${getZodiacSignName(p.sign_index)} (H$hNum$rx)"
            }
            append("  * Return Placements: $plStr\n")
        }
        if (ret.natal_aspects.isNotEmpty()) {
            val aspStr = ret.natal_aspects.joinToString(", ") { a ->
                "Return ${a.return_point} ${a.aspect_type} Natal ${a.natal_point} (Orb ${fmtUs("%.2f°", a.orb)})"
            }
            append("  * Return-to-Natal Aspects: $aspStr\n")
        }
    }
}

private fun StringBuilder.appendProgressionsMd(pred: PredictiveSuiteData) {
    val sa = pred.solar_arc
    append("\n### Solar Arc Directions (Offset: +${fmtUs("%.2f°", sa.arc_degrees)})\n")
    sa.directed_planets.forEach { p ->
        append("- SA ${p.name}: ${fmtUs("%.2f°", p.sign_degree)} (${getZodiacSignName(p.sign_index)}) [Natal: ${fmtUs("%.2f°", p.natal_longitude)}]\n")
    }
    if (sa.contacts.isNotEmpty()) {
        append("- Solar Arc Contacts (Orb <= 1.0°):\n")
        sa.contacts.forEach { c ->
            append("  * SA ${stripBodyPrefix(c.moving_body)} ${c.aspect_type} Natal ${stripBodyPrefix(c.natal_body)} (Orb: ${fmtUs("%.2f°", c.orb)})\n")
        }
    }
    pred.secondary_progression?.let { sp ->
        append("\n### Secondary Progressions (Ephemeris Date: ${sp.progressed_date_utc}, Phase: ${sp.progressed_lunar_phase})\n")
        sp.planets.forEach { p ->
            val rx = if (p.is_retrograde) " [Rx]" else ""
            append("- Progressed ${p.name}: ${fmtUs("%.2f°", p.sign_degree)} (${getZodiacSignName(p.sign_index)})$rx\n")
        }
        if (sp.contacts.isNotEmpty()) {
            append("- Progressed-to-Natal Contacts (Orb <= 1.0°):\n")
            sp.contacts.forEach { c ->
                append("  * Progressed ${stripBodyPrefix(c.moving_body)} ${c.aspect_type} Natal ${stripBodyPrefix(c.natal_body)} (Orb: ${fmtUs("%.2f°", c.orb)})\n")
            }
        }
    }
}

private fun StringBuilder.appendTransitsAndDirectionsMd(pred: PredictiveSuiteData) {
    if (pred.transits.isNotEmpty()) {
        append("\n### Active Transits to Natal (Orb <= 2.0°)\n")
        pred.transits.forEach { t ->
            append("- Transit ${t.transiting_planet} (${fmtUs("%.2f°", t.transiting_longitude)}) ${t.aspect_type} Natal ${t.natal_planet} (${fmtUs("%.2f°", t.natal_longitude)}) (Orb: ${fmtUs("%.2f°", t.orb)})\n")
        }
    }
    if (pred.primary_directions.isNotEmpty()) {
        append("\n### Primary Directions (Naibod Key)\n")
        pred.primary_directions.forEach { pd ->
            append("- ${pd.promittor} ${pd.aspect_type} ${pd.significator} (Arc: ${fmtUs("%.2f°", pd.direction_arc_deg)}, Age: ${fmtUs("%.1f", pd.realization_age)})\n")
        }
    }
}
