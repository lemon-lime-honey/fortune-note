package com.fortunexample.note.ui.components

import android.content.ClipData
import android.content.ClipboardManager
import android.content.Context
import android.widget.Toast
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

data class SelectiveCopyPayload(
    val profile: ProfileDto,
    val saju: SajuChartData? = null,
    val western: WesternChartData? = null,
    val predictive: PredictiveSuiteData? = null,
    val vedic: VedicChartData? = null,
    val timezoneId: String = profile.timezone_id,
    val utcOffsetHours: Double? = null
)

data class SelectiveCopyOptions(
    val incMeta: Boolean = true,
    val incSajuPillars: Boolean = true,
    val incSajuDaeun: Boolean = true,
    val incWesternPlanets: Boolean = true,
    val incWesternAspects: Boolean = true,
    val incWesternDignities: Boolean = true,
    val incWesternMoon: Boolean = true,
    val incWesternHours: Boolean = true,
    val incWesternDynamicAspects: Boolean = true,
    val incWesternReceptions: Boolean = true,
    val incWesternStars: Boolean = true,
    val incWesternParts: Boolean = true,
    val incWesternProfection: Boolean = true,
    val incWesternReturns: Boolean = true,
    val incWesternProgressions: Boolean = true,
    val incWesternTransitsDirections: Boolean = true,
    val incVedicRasi: Boolean = true,
    val incVedicIndicators: Boolean = true,
    val incVedicVargas: Boolean = true,
    val incVedicDasha: Boolean = true
) {
    companion object {
        fun allSelected() = SelectiveCopyOptions()
        fun basicOnly() = SelectiveCopyOptions(
            incMeta = true,
            incSajuPillars = true,
            incSajuDaeun = false,
            incWesternPlanets = true,
            incWesternAspects = true,
            incWesternDignities = false,
            incWesternMoon = false,
            incWesternHours = false,
            incWesternDynamicAspects = false,
            incWesternReceptions = false,
            incWesternStars = false,
            incWesternParts = false,
            incWesternProfection = false,
            incWesternReturns = false,
            incWesternProgressions = false,
            incWesternTransitsDirections = false,
            incVedicRasi = true,
            incVedicIndicators = false,
            incVedicVargas = false,
            incVedicDasha = false
        )
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun SelectiveCopyBottomSheet(
    isOpen: Boolean,
    onDismiss: () -> Unit,
    payload: SelectiveCopyPayload,
    selectedDomainTab: Int
) {
    val isReady = when (selectedDomainTab) {
        0 -> payload.saju != null
        1 -> payload.western != null
        else -> payload.vedic != null
    }
    if (!isOpen || !isReady) return

    val context = LocalContext.current
    var options by remember { mutableStateOf(SelectiveCopyOptions.allSelected()) }

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
            Text("복사 항목 선택", fontWeight = FontWeight.Bold, fontSize = 18.sp, color = TextPrimary)
            Spacer(modifier = Modifier.height(4.dp))
            Text("필요한 천체 데이터 항목만 선택하여 클립보드로 복사합니다.", fontSize = 13.sp, color = TextSecondary)
            Spacer(modifier = Modifier.height(16.dp))

            Row(modifier = Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.SpaceBetween) {
                TextButton(onClick = { options = SelectiveCopyOptions.allSelected() }) {
                    Text("모두 선택", color = CelestialBlue, fontSize = 13.sp)
                }
                TextButton(onClick = { options = SelectiveCopyOptions.basicOnly() }) {
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
                CopyCheckboxItem("인물 기본 정보 및 메타데이터", options.incMeta) { options = options.copy(incMeta = it) }
                DomainCopyCheckboxes(
                    selectedDomainTab = selectedDomainTab,
                    payload = payload,
                    options = options,
                    onChange = { options = it }
                )
            }

            Spacer(modifier = Modifier.height(16.dp))

            Button(
                onClick = {
                    val markdown = generateSelectiveMarkdown(payload, selectedDomainTab, options)
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
private fun DomainCopyCheckboxes(
    selectedDomainTab: Int,
    payload: SelectiveCopyPayload,
    options: SelectiveCopyOptions,
    onChange: (SelectiveCopyOptions) -> Unit
) {
    when (selectedDomainTab) {
        0 -> {
            CopyCheckboxItem("사주팔자 원국 명식 (연/월/일/시주)", options.incSajuPillars) { onChange(options.copy(incSajuPillars = it)) }
            CopyCheckboxItem("10년 주기 대운 흐름 및 대운수", options.incSajuDaeun) { onChange(options.copy(incSajuDaeun = it)) }
        }
        1 -> WesternCopyCheckboxes(payload, options, onChange)
        else -> {
            CopyCheckboxItem("라그나 및 D1 라시 차트 배치 (품위/연소 포함)", options.incVedicRasi) { onChange(options.copy(incVedicRasi = it)) }
            CopyCheckboxItem("베다 핵심 지표 (판창가, 카라카, 파다, 드리슈티, SAV)", options.incVedicIndicators) { onChange(options.copy(incVedicIndicators = it)) }
            CopyCheckboxItem("19종 분점도 (Shodashavarga + D6, D8, D11)", options.incVedicVargas) { onChange(options.copy(incVedicVargas = it)) }
            CopyCheckboxItem("빔쇼타리 다샤 운로 (Mahadasha/Antardasha)", options.incVedicDasha) { onChange(options.copy(incVedicDasha = it)) }
        }
    }
}

@Composable
private fun WesternCopyCheckboxes(
    payload: SelectiveCopyPayload,
    options: SelectiveCopyOptions,
    onChange: (SelectiveCopyOptions) -> Unit
) {
    val isWholeSign = isWholeSignHouseSystem(payload.western)
    if (isWholeSign) {
        CopyCheckboxItem("천체 및 홀사인 하우스 배치 (Whole Sign Placements)", options.incWesternPlanets) { onChange(options.copy(incWesternPlanets = it)) }
        CopyCheckboxItem("메이저 애스펙트 (Major Aspects)", options.incWesternAspects) { onChange(options.copy(incWesternAspects = it)) }
        CopyCheckboxItem("헬레니즘 행성 상태·이집트 바운드·트리플리시티", options.incWesternDignities) { onChange(options.copy(incWesternDignities = it)) }
        CopyCheckboxItem("사인 기반 애스펙트 및 접촉 (Sign-Based Aspects)", options.incWesternDynamicAspects) { onChange(options.copy(incWesternDynamicAspects = it)) }
        CopyCheckboxItem("헤르메스 랏 및 출생 전 합충 (Hermetic Lots & SAN)", options.incWesternParts) { onChange(options.copy(incWesternParts = it)) }
        CopyCheckboxItem("항성 회합 (Fixed Star Conjunctions ≤ 1.5°)", options.incWesternStars) { onChange(options.copy(incWesternStars = it)) }
    } else {
        CopyCheckboxItem("천체 및 레기오몬타누스 커스프 (Regiomontanus Placements)", options.incWesternPlanets) { onChange(options.copy(incWesternPlanets = it)) }
        CopyCheckboxItem("메이저 애스펙트 (Major Aspects)", options.incWesternAspects) { onChange(options.copy(incWesternAspects = it)) }
        CopyCheckboxItem("필수·우연 품위 및 점수 (Essential & Accidental Dignities)", options.incWesternDignities) { onChange(options.copy(incWesternDignities = it)) }
        CopyCheckboxItem("달의 위상 및 출생 전 합충 (Moon Phase & SAN)", options.incWesternMoon) { onChange(options.copy(incWesternMoon = it)) }
        CopyCheckboxItem("행성시 (Planetary Hours)", options.incWesternHours) { onChange(options.copy(incWesternHours = it)) }
        CopyCheckboxItem("중세 역학 애스펙트 (Applying/Separating, Moiety)", options.incWesternDynamicAspects) { onChange(options.copy(incWesternDynamicAspects = it)) }
        CopyCheckboxItem("뮤추얼 리셉션 및 안티시아 (Receptions & Antiscia)", options.incWesternReceptions) { onChange(options.copy(incWesternReceptions = it)) }
        CopyCheckboxItem("항성 회합 (Fixed Star Conjunctions ≤ 1.5°)", options.incWesternStars) { onChange(options.copy(incWesternStars = it)) }
        CopyCheckboxItem("아라빅 파츠 및 하우스 알무텐 (Arabic Parts & Almutens)", options.incWesternParts) { onChange(options.copy(incWesternParts = it)) }
    }
    payload.predictive?.let { pred ->
        Spacer(modifier = Modifier.height(6.dp))
        HorizontalDivider(color = BorderColor, thickness = 0.5.dp)
        Spacer(modifier = Modifier.height(6.dp))
        CopyCheckboxItem("연간 프로펙션 (Annual Profection · ${pred.target_year})", options.incWesternProfection) { onChange(options.copy(incWesternProfection = it)) }
        CopyCheckboxItem("태양 및 행성 회귀 (Solar & Planetary Returns · ${pred.target_year})", options.incWesternReturns) { onChange(options.copy(incWesternReturns = it)) }
        CopyCheckboxItem("솔라 아크 및 세컨더리 프로그레션 (Solar Arc & Progressions)", options.incWesternProgressions) { onChange(options.copy(incWesternProgressions = it)) }
        CopyCheckboxItem("트랜짓 및 프라이머리 디렉션 (Transits & Primary Directions)", options.incWesternTransitsDirections) { onChange(options.copy(incWesternTransitsDirections = it)) }
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
