package com.fortunexample.note.ui.screens

import android.Manifest
import android.content.pm.PackageManager
import android.widget.Toast
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.filled.FlashOn
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.core.content.ContextCompat
import com.fortunexample.note.data.*
import com.fortunexample.note.ui.components.DateInputRow
import com.fortunexample.note.ui.components.GpsRequestHandle
import com.fortunexample.note.ui.components.LocationPicker
import com.fortunexample.note.ui.components.TimeInputRow
import com.fortunexample.note.ui.components.fetchCurrentGpsLocation
import com.fortunexample.note.ui.theme.*
import java.util.Calendar
import java.util.Locale

private data class HoraryFormDraft(
    val title: String,
    val category: String,
    val questionText: String,
    val querentProfileId: String?,
    val resultNotes: String,
    val fields: DateTimeFormFields,
    val latitude: Double,
    val longitude: Double,
    val isCoordinateInputValid: Boolean
)

private fun buildHoraryRequest(draft: HoraryFormDraft): Result<CreateHoraryRequest> {
    val trimmedTitle = draft.title.trim()
    if (trimmedTitle.isEmpty()) return Result.failure(IllegalArgumentException("질문 제목을 입력해 주세요."))
    if (!draft.isCoordinateInputValid) {
        return Result.failure(IllegalArgumentException("올바른 위도(-90~90)와 경도(-180~180)를 입력해 주세요."))
    }
    return when (val result = validateDateTimeAndCoordinates(draft.fields, draft.latitude, draft.longitude)) {
        is FormValidationResult.Invalid -> Result.failure(IllegalArgumentException(result.message))
        is FormValidationResult.Valid -> {
            val dt = result.value
            Result.success(
                CreateHoraryRequest(
                    title = trimmedTitle,
                    category = draft.category.trim(),
                    question_text = draft.questionText.trim(),
                    query_year = dt.year,
                    query_month = dt.month,
                    query_day = dt.day,
                    query_hour = dt.hour,
                    query_minute = dt.minute,
                    query_second = dt.second,
                    latitude = draft.latitude,
                    longitude = draft.longitude,
                    querent_profile_id = draft.querentProfileId,
                    result_notes = draft.resultNotes.trim()
                )
            )
        }
    }
}

@Composable
fun HoraryEditScreen(
    initialHorary: HoraryDto? = null,
    profiles: List<ProfileDto> = emptyList(),
    onBack: () -> Unit,
    onSave: (CreateHoraryRequest) -> Unit
) {
    val context = LocalContext.current
    val isEditMode = initialHorary != null

    var title by remember(initialHorary) { mutableStateOf(initialHorary?.title ?: "") }
    var category by remember(initialHorary) { mutableStateOf(initialHorary?.category ?: DEFAULT_HORARY_CATEGORY) }
    var questionText by remember(initialHorary) { mutableStateOf(initialHorary?.question_text ?: "") }
    var querentProfileId by remember(initialHorary) { mutableStateOf(initialHorary?.querent_profile_id) }
    var resultNotes by remember(initialHorary) { mutableStateOf(initialHorary?.result_notes ?: "") }
    var validationError by remember { mutableStateOf<String?>(null) }

    val initCal = remember(initialHorary) { Calendar.getInstance() }
    var yearStr by remember(initialHorary) { mutableStateOf(initialHorary?.query_year?.toString() ?: initCal.get(Calendar.YEAR).toString()) }
    var monthStr by remember(initialHorary) { mutableStateOf(initialHorary?.query_month?.toString() ?: (initCal.get(Calendar.MONTH) + 1).toString()) }
    var dayStr by remember(initialHorary) { mutableStateOf(initialHorary?.query_day?.toString() ?: initCal.get(Calendar.DAY_OF_MONTH).toString()) }
    var hourStr by remember(initialHorary) { mutableStateOf(initialHorary?.query_hour?.toString() ?: initCal.get(Calendar.HOUR_OF_DAY).toString()) }
    var minuteStr by remember(initialHorary) { mutableStateOf(initialHorary?.query_minute?.toString() ?: initCal.get(Calendar.MINUTE).toString()) }
    var secondStr by remember(initialHorary) { mutableStateOf(initialHorary?.query_second?.toInt()?.toString() ?: initCal.get(Calendar.SECOND).toString()) }
    var latitude by remember(initialHorary) { mutableDoubleStateOf(initialHorary?.latitude ?: DEFAULT_LATITUDE) }
    var longitude by remember(initialHorary) { mutableDoubleStateOf(initialHorary?.longitude ?: DEFAULT_LONGITUDE) }
    var isCoordinateInputValid by remember { mutableStateOf(true) }
    var activeGpsHandle by remember { mutableStateOf<GpsRequestHandle?>(null) }

    DisposableEffect(Unit) {
        onDispose { activeGpsHandle?.cancel() }
    }

    fun triggerCaptureGps() {
        activeGpsHandle?.cancel()
        activeGpsHandle = fetchCurrentGpsLocation(context, { lat, lon -> latitude = lat; longitude = lon }, {})
    }

    val capturePermissionLauncher = rememberLauncherForActivityResult(
        contract = ActivityResultContracts.RequestMultiplePermissions()
    ) { permissions ->
        val granted = permissions[Manifest.permission.ACCESS_FINE_LOCATION] == true ||
            permissions[Manifest.permission.ACCESS_COARSE_LOCATION] == true
        if (granted) triggerCaptureGps()
    }

    Scaffold(
        topBar = { HoraryEditTopBar(isEditMode = isEditMode, onBack = onBack) },
        containerColor = BackgroundWhite
    ) { padding ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(padding)
                .verticalScroll(rememberScrollState())
                .padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(14.dp)
        ) {
            HoraryCaptureNowButton {
                val now = Calendar.getInstance()
                yearStr = now.get(Calendar.YEAR).toString()
                monthStr = (now.get(Calendar.MONTH) + 1).toString()
                dayStr = now.get(Calendar.DAY_OF_MONTH).toString()
                hourStr = now.get(Calendar.HOUR_OF_DAY).toString()
                minuteStr = now.get(Calendar.MINUTE).toString()
                secondStr = now.get(Calendar.SECOND).toString()

                val hasFine = ContextCompat.checkSelfPermission(context, Manifest.permission.ACCESS_FINE_LOCATION) == PackageManager.PERMISSION_GRANTED
                val hasCoarse = ContextCompat.checkSelfPermission(context, Manifest.permission.ACCESS_COARSE_LOCATION) == PackageManager.PERMISSION_GRANTED
                if (hasFine || hasCoarse) triggerCaptureGps()
                else capturePermissionLauncher.launch(arrayOf(Manifest.permission.ACCESS_FINE_LOCATION, Manifest.permission.ACCESS_COARSE_LOCATION))
                Toast.makeText(context, "현재 시각이 캡처되었습니다.", Toast.LENGTH_SHORT).show()
            }

            HoraryTextField("질문 제목 (필수)", title, { title = it }, singleLine = true)
            HoraryTextField("카테고리 (예: Career, Relationship, Wealth)", category, { category = it }, singleLine = true)
            QuerentProfileDropdown(profiles = profiles, selectedProfileId = querentProfileId, onSelectProfile = { querentProfileId = it })
            HoraryTextField("질문 상세 본문", questionText, { questionText = it }, minLines = 3)
            HoraryTextField("판단 메모 및 실제 결과 기록 (선택)", resultNotes, { resultNotes = it }, minLines = 2)

            Text("질문 일시", color = TextSecondary, fontSize = 13.sp, fontWeight = FontWeight.Medium)
            DateInputRow(yearStr, monthStr, dayStr, { yearStr = it }, { monthStr = it }, { dayStr = it }, yearLabel = "년", spacing = 6.dp)
            TimeInputRow(hourStr, minuteStr, secondStr, { hourStr = it }, { minuteStr = it }, { secondStr = it }, spacing = 6.dp)

            LocationPicker(
                latitude = latitude,
                longitude = longitude,
                onLocationChanged = { lat, lon -> latitude = lat; longitude = lon },
                onValidationChanged = { isCoordinateInputValid = it }
            )

            validationError?.let { err ->
                Text(err, color = AccentWarning, fontSize = 13.sp, fontWeight = FontWeight.Medium, modifier = Modifier.padding(horizontal = 4.dp))
            }

            Spacer(modifier = Modifier.height(16.dp))

            Button(
                onClick = {
                    val draft = HoraryFormDraft(
                        title = title,
                        category = category,
                        questionText = questionText,
                        querentProfileId = querentProfileId,
                        resultNotes = resultNotes,
                        fields = DateTimeFormFields(yearStr, monthStr, dayStr, hourStr, minuteStr, secondStr, allowBlankSecond = false),
                        latitude = latitude,
                        longitude = longitude,
                        isCoordinateInputValid = isCoordinateInputValid
                    )
                    buildHoraryRequest(draft)
                        .onSuccess { validationError = null; onSave(it) }
                        .onFailure { validationError = it.message }
                },
                modifier = Modifier.fillMaxWidth().height(50.dp),
                colors = ButtonDefaults.buttonColors(containerColor = AccentPrimary),
                shape = RoundedCornerShape(10.dp)
            ) {
                Text(if (isEditMode) "수정 완료" else "저장하기", color = BackgroundWhite, fontWeight = FontWeight.Bold, fontSize = 16.sp)
            }
        }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
private fun HoraryEditTopBar(isEditMode: Boolean, onBack: () -> Unit) {
    TopAppBar(
        title = { Text(if (isEditMode) "질문 수정" else "질문 등록", fontWeight = FontWeight.Bold, color = TextPrimary) },
        navigationIcon = {
            IconButton(onClick = onBack) {
                Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = "뒤로", tint = TextPrimary)
            }
        },
        colors = TopAppBarDefaults.topAppBarColors(
            containerColor = BackgroundWhite,
            titleContentColor = TextPrimary,
            navigationIconContentColor = TextPrimary
        )
    )
}

@Composable
private fun HoraryTextField(
    label: String,
    value: String,
    onValueChange: (String) -> Unit,
    singleLine: Boolean = false,
    minLines: Int = 1
) {
    OutlinedTextField(
        value = value,
        onValueChange = onValueChange,
        label = { Text(label, color = TextSecondary) },
        singleLine = singleLine,
        minLines = minLines,
        modifier = Modifier.fillMaxWidth(),
        shape = RoundedCornerShape(10.dp),
        colors = OutlinedTextFieldDefaults.colors(focusedBorderColor = AccentPrimary, unfocusedBorderColor = BorderColor)
    )
}

@Composable
private fun HoraryCaptureNowButton(onClick: () -> Unit) {
    OutlinedButton(
        onClick = onClick,
        modifier = Modifier.fillMaxWidth(),
        colors = ButtonDefaults.outlinedButtonColors(containerColor = SurfaceLight),
        border = BorderStroke(1.dp, BorderColor),
        shape = RoundedCornerShape(10.dp)
    ) {
        Icon(Icons.Default.FlashOn, contentDescription = null, tint = CelestialBlue)
        Spacer(modifier = Modifier.width(6.dp))
        Text("지금 현재 시각으로 캡처", color = TextPrimary, fontWeight = FontWeight.SemiBold)
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
private fun QuerentProfileDropdown(
    profiles: List<ProfileDto>,
    selectedProfileId: String?,
    onSelectProfile: (String?) -> Unit
) {
    var expanded by remember { mutableStateOf(false) }
    val selectedName = profiles.find { it.id == selectedProfileId }?.name ?: "연결 안 함 (독립 호라리)"

    ExposedDropdownMenuBox(expanded = expanded, onExpandedChange = { expanded = !expanded }) {
        OutlinedTextField(
            value = selectedName,
            onValueChange = {},
            readOnly = true,
            label = { Text("질문자 프로필 연결 (선택)", color = TextSecondary) },
            trailingIcon = { ExposedDropdownMenuDefaults.TrailingIcon(expanded = expanded) },
            modifier = Modifier.menuAnchor(MenuAnchorType.PrimaryNotEditable).fillMaxWidth(),
            shape = RoundedCornerShape(10.dp),
            colors = OutlinedTextFieldDefaults.colors(focusedBorderColor = AccentPrimary, unfocusedBorderColor = BorderColor)
        )
        ExposedDropdownMenu(expanded = expanded, onDismissRequest = { expanded = false }) {
            DropdownMenuItem(
                text = { Text("연결 안 함 (독립 호라리)") },
                onClick = { onSelectProfile(null); expanded = false }
            )
            profiles.forEach { prof ->
                val dateLabel = String.format(Locale.US, "%04d-%02d-%02d", prof.birth_year, prof.birth_month, prof.birth_day)
                DropdownMenuItem(
                    text = { Text("${prof.name} ($dateLabel)") },
                    onClick = { onSelectProfile(prof.id); expanded = false }
                )
            }
        }
    }
}
