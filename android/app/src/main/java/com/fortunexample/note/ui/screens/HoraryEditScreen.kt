package com.fortunexample.note.ui.screens

import android.content.Context
import android.location.LocationManager
import android.widget.Toast
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.filled.FlashOn
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.fortunexample.note.data.HoraryDto
import com.fortunexample.note.data.ProfileDto
import com.fortunexample.note.ui.components.LocationPicker
import com.fortunexample.note.ui.theme.*
import java.util.Calendar

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun HoraryEditScreen(
    initialHorary: HoraryDto? = null,
    profiles: List<ProfileDto> = emptyList(),
    onBack: () -> Unit,
    onSave: (title: String, category: String, questionText: String, y: Int, m: Int, d: Int, h: Int, min: Int, sec: Double, lat: Double, lon: Double, querentProfileId: String?, resultNotes: String) -> Unit
) {
    val context = LocalContext.current
    val isEditMode = initialHorary != null

    var title by remember(initialHorary) { mutableStateOf(initialHorary?.title ?: "") }
    var category by remember(initialHorary) { mutableStateOf(initialHorary?.category ?: "Career") }
    var questionText by remember(initialHorary) { mutableStateOf(initialHorary?.question_text ?: "") }
    var querentProfileId by remember(initialHorary) { mutableStateOf(initialHorary?.querent_profile_id) }
    var resultNotes by remember(initialHorary) { mutableStateOf(initialHorary?.result_notes ?: "") }
    var profileMenuExpanded by remember { mutableStateOf(false) }

    val cal = Calendar.getInstance()
    var yearStr by remember(initialHorary) { mutableStateOf(initialHorary?.query_year?.toString() ?: cal.get(Calendar.YEAR).toString()) }
    var monthStr by remember(initialHorary) { mutableStateOf(initialHorary?.query_month?.toString() ?: (cal.get(Calendar.MONTH) + 1).toString()) }
    var dayStr by remember(initialHorary) { mutableStateOf(initialHorary?.query_day?.toString() ?: cal.get(Calendar.DAY_OF_MONTH).toString()) }
    var hourStr by remember(initialHorary) { mutableStateOf(initialHorary?.query_hour?.toString() ?: cal.get(Calendar.HOUR_OF_DAY).toString()) }
    var minuteStr by remember(initialHorary) { mutableStateOf(initialHorary?.query_minute?.toString() ?: cal.get(Calendar.MINUTE).toString()) }
    var secondStr by remember(initialHorary) { mutableStateOf(initialHorary?.query_second?.toInt()?.toString() ?: cal.get(Calendar.SECOND).toString()) }
    var latitude by remember(initialHorary) { mutableDoubleStateOf(initialHorary?.latitude ?: 37.5665) }
    var longitude by remember(initialHorary) { mutableDoubleStateOf(initialHorary?.longitude ?: 126.9780) }

    Scaffold(
        topBar = {
            TopAppBar(
                title = {
                    Text(
                        text = if (isEditMode) "질문 수정" else "질문 등록",
                        fontWeight = FontWeight.Bold,
                        color = TextPrimary
                    )
                },
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
        },
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
            // 원터치 일시 캡처 버튼
            OutlinedButton(
                onClick = {
                    val now = Calendar.getInstance()
                    yearStr = now.get(Calendar.YEAR).toString()
                    monthStr = (now.get(Calendar.MONTH) + 1).toString()
                    dayStr = now.get(Calendar.DAY_OF_MONTH).toString()
                    hourStr = now.get(Calendar.HOUR_OF_DAY).toString()
                    minuteStr = now.get(Calendar.MINUTE).toString()
                    secondStr = now.get(Calendar.SECOND).toString()

                    val lm = context.getSystemService(Context.LOCATION_SERVICE) as? LocationManager
                    if (lm != null) {
                        try {
                            val loc = lm.getLastKnownLocation(LocationManager.GPS_PROVIDER)
                                ?: lm.getLastKnownLocation(LocationManager.NETWORK_PROVIDER)
                            if (loc != null) {
                                latitude = loc.latitude
                                longitude = loc.longitude
                            }
                        } catch (_: SecurityException) {}
                    }
                    Toast.makeText(context, "현재 시각이 캡처되었습니다.", Toast.LENGTH_SHORT).show()
                },
                modifier = Modifier.fillMaxWidth(),
                colors = ButtonDefaults.outlinedButtonColors(containerColor = SurfaceLight),
                border = BorderStroke(1.dp, BorderColor),
                shape = RoundedCornerShape(10.dp)
            ) {
                Icon(Icons.Default.FlashOn, contentDescription = null, tint = CelestialBlue)
                Spacer(modifier = Modifier.width(6.dp))
                Text("지금 현재 시각으로 캡처", color = TextPrimary, fontWeight = FontWeight.SemiBold)
            }

            OutlinedTextField(
                value = title,
                onValueChange = { title = it },
                label = { Text("질문 제목 (필수)", color = TextSecondary) },
                singleLine = true,
                modifier = Modifier.fillMaxWidth(),
                shape = RoundedCornerShape(10.dp),
                colors = OutlinedTextFieldDefaults.colors(
                    focusedBorderColor = AccentPrimary,
                    unfocusedBorderColor = BorderColor
                )
            )

            OutlinedTextField(
                value = category,
                onValueChange = { category = it },
                label = { Text("카테고리 (예: Career, Relationship, Wealth)", color = TextSecondary) },
                singleLine = true,
                modifier = Modifier.fillMaxWidth(),
                shape = RoundedCornerShape(10.dp),
                colors = OutlinedTextFieldDefaults.colors(
                    focusedBorderColor = AccentPrimary,
                    unfocusedBorderColor = BorderColor
                )
            )

            // 질문자(Querent) 프로필 연결 선택기
            val selectedProfileName = profiles.find { it.id == querentProfileId }?.name ?: "연결 안 함 (독립 호라리)"
            ExposedDropdownMenuBox(
                expanded = profileMenuExpanded,
                onExpandedChange = { profileMenuExpanded = !profileMenuExpanded }
            ) {
                OutlinedTextField(
                    value = selectedProfileName,
                    onValueChange = {},
                    readOnly = true,
                    label = { Text("질문자 프로필 연결 (선택)", color = TextSecondary) },
                    trailingIcon = { ExposedDropdownMenuDefaults.TrailingIcon(expanded = profileMenuExpanded) },
                    modifier = Modifier
                        .menuAnchor()
                        .fillMaxWidth(),
                    shape = RoundedCornerShape(10.dp),
                    colors = OutlinedTextFieldDefaults.colors(
                        focusedBorderColor = AccentPrimary,
                        unfocusedBorderColor = BorderColor
                    )
                )
                ExposedDropdownMenu(
                    expanded = profileMenuExpanded,
                    onDismissRequest = { profileMenuExpanded = false }
                ) {
                    DropdownMenuItem(
                        text = { Text("연결 안 함 (독립 호라리)") },
                        onClick = {
                            querentProfileId = null
                            profileMenuExpanded = false
                        }
                    )
                    profiles.forEach { prof ->
                        DropdownMenuItem(
                            text = { Text("${prof.name} (${prof.birth_year}-${String.format("%02d", prof.birth_month)}-${String.format("%02d", prof.birth_day)})") },
                            onClick = {
                                querentProfileId = prof.id
                                profileMenuExpanded = false
                            }
                        )
                    }
                }
            }

            OutlinedTextField(
                value = questionText,
                onValueChange = { questionText = it },
                label = { Text("질문 상세 본문", color = TextSecondary) },
                modifier = Modifier.fillMaxWidth(),
                minLines = 3,
                shape = RoundedCornerShape(10.dp),
                colors = OutlinedTextFieldDefaults.colors(
                    focusedBorderColor = AccentPrimary,
                    unfocusedBorderColor = BorderColor
                )
            )

            OutlinedTextField(
                value = resultNotes,
                onValueChange = { resultNotes = it },
                label = { Text("판단 메모 및 실제 결과 기록 (선택)", color = TextSecondary) },
                modifier = Modifier.fillMaxWidth(),
                minLines = 2,
                shape = RoundedCornerShape(10.dp),
                colors = OutlinedTextFieldDefaults.colors(
                    focusedBorderColor = AccentPrimary,
                    unfocusedBorderColor = BorderColor
                )
            )

            // 질문 일시 (숫자 전용 키보드)
            Text("질문 일시", color = TextSecondary, fontSize = 13.sp, fontWeight = FontWeight.Medium)
            Row(horizontalArrangement = Arrangement.spacedBy(6.dp)) {
                OutlinedTextField(
                    value = yearStr,
                    onValueChange = { yearStr = it },
                    label = { Text("년") },
                    keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number, imeAction = ImeAction.Next),
                    modifier = Modifier.weight(1.2f),
                    singleLine = true
                )
                OutlinedTextField(
                    value = monthStr,
                    onValueChange = { monthStr = it },
                    label = { Text("월") },
                    keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number, imeAction = ImeAction.Next),
                    modifier = Modifier.weight(1f),
                    singleLine = true
                )
                OutlinedTextField(
                    value = dayStr,
                    onValueChange = { dayStr = it },
                    label = { Text("일") },
                    keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number, imeAction = ImeAction.Next),
                    modifier = Modifier.weight(1f),
                    singleLine = true
                )
            }
            Row(horizontalArrangement = Arrangement.spacedBy(6.dp)) {
                OutlinedTextField(
                    value = hourStr,
                    onValueChange = { hourStr = it },
                    label = { Text("시") },
                    keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number, imeAction = ImeAction.Next),
                    modifier = Modifier.weight(1f),
                    singleLine = true
                )
                OutlinedTextField(
                    value = minuteStr,
                    onValueChange = { minuteStr = it },
                    label = { Text("분") },
                    keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number, imeAction = ImeAction.Next),
                    modifier = Modifier.weight(1f),
                    singleLine = true
                )
                OutlinedTextField(
                    value = secondStr,
                    onValueChange = { secondStr = it },
                    label = { Text("초") },
                    keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number, imeAction = ImeAction.Done),
                    modifier = Modifier.weight(1f),
                    singleLine = true
                )
            }

            // 관측 좌표 선택기
            LocationPicker(
                latitude = latitude,
                longitude = longitude,
                onLocationChanged = { newLat, newLon ->
                    latitude = newLat
                    longitude = newLon
                }
            )

            var validationError by remember { mutableStateOf<String?>(null) }

            if (validationError != null) {
                Text(
                    text = validationError ?: "",
                    color = AccentWarning,
                    fontSize = 13.sp,
                    fontWeight = FontWeight.Medium,
                    modifier = Modifier.padding(horizontal = 4.dp)
                )
            }

            Spacer(modifier = Modifier.height(16.dp))

            Button(
                onClick = {
                    if (title.trim().isEmpty()) {
                        validationError = "질문 제목을 입력해 주세요."
                        return@Button
                    }
                    val y = yearStr.toIntOrNull()
                    val m = monthStr.toIntOrNull()
                    val d = dayStr.toIntOrNull()
                    val h = hourStr.toIntOrNull()
                    val min = minuteStr.toIntOrNull()
                    val sec = secondStr.toDoubleOrNull()

                    if (y == null || y !in -4000..3000) {
                        validationError = "올바른 연도를 입력해 주세요 (-4000~3000)."
                        return@Button
                    }
                    if (m == null || m !in 1..12) {
                        validationError = "올바른 월을 입력해 주세요 (1~12)."
                        return@Button
                    }
                    if (d == null || d !in 1..31) {
                        validationError = "올바른 일을 입력해 주세요 (1~31)."
                        return@Button
                    }
                    if (h == null || h !in 0..23) {
                        validationError = "올바른 시를 입력해 주세요 (0~23)."
                        return@Button
                    }
                    if (min == null || min !in 0..59) {
                        validationError = "올바른 분을 입력해 주세요 (0~59)."
                        return@Button
                    }
                    if (sec == null || sec < 0.0 || sec >= 60.0) {
                        validationError = "올바른 초를 입력해 주세요 (0~59.99)."
                        return@Button
                    }
                    try {
                        java.time.LocalDate.of(y, m, d)
                    } catch (_: Exception) {
                        validationError = "존재하지 않는 날짜입니다."
                        return@Button
                    }

                    validationError = null
                    onSave(
                        title.trim(),
                        category.trim(),
                        questionText.trim(),
                        y,
                        m,
                        d,
                        h,
                        min,
                        sec,
                        latitude,
                        longitude,
                        querentProfileId,
                        resultNotes.trim()
                    )
                },
                modifier = Modifier
                    .fillMaxWidth()
                    .height(50.dp),
                colors = ButtonDefaults.buttonColors(containerColor = AccentPrimary),
                shape = RoundedCornerShape(10.dp)
            ) {
                Text(
                    text = if (isEditMode) "수정 완료" else "저장하기",
                    color = BackgroundWhite,
                    fontWeight = FontWeight.Bold,
                    fontSize = 16.sp
                )
            }
        }
    }
}
