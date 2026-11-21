package com.fortunexample.note.ui.screens

import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.fortunexample.note.data.*
import com.fortunexample.note.ui.components.DateInputRow
import com.fortunexample.note.ui.components.LocationPicker
import com.fortunexample.note.ui.components.TimeInputRow
import com.fortunexample.note.ui.theme.*

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ProfileEditScreen(
    initialProfile: ProfileDto? = null,
    onBack: () -> Unit,
    onSave: (CreateProfileRequest) -> Unit
) {
    var name by remember(initialProfile) { mutableStateOf(initialProfile?.name ?: "") }
    var isMale by remember(initialProfile) { mutableStateOf(initialProfile?.gender != "female") }
    var yearStr by remember(initialProfile) { mutableStateOf(initialProfile?.birth_year?.toString() ?: DEFAULT_BIRTH_YEAR) }
    var monthStr by remember(initialProfile) { mutableStateOf(initialProfile?.birth_month?.toString() ?: DEFAULT_BIRTH_MONTH) }
    var dayStr by remember(initialProfile) { mutableStateOf(initialProfile?.birth_day?.toString() ?: DEFAULT_BIRTH_DAY) }
    var hourStr by remember(initialProfile) { mutableStateOf(initialProfile?.birth_hour?.toString() ?: DEFAULT_BIRTH_HOUR) }
    var minuteStr by remember(initialProfile) { mutableStateOf(initialProfile?.birth_minute?.toString() ?: DEFAULT_BIRTH_MINUTE) }
    var secondStr by remember(initialProfile) { mutableStateOf(initialProfile?.birth_second?.toInt()?.toString() ?: DEFAULT_BIRTH_SECOND) }
    var latitude by remember(initialProfile) { mutableDoubleStateOf(initialProfile?.latitude ?: DEFAULT_LATITUDE) }
    var longitude by remember(initialProfile) { mutableDoubleStateOf(initialProfile?.longitude ?: DEFAULT_LONGITUDE) }
    var notes by remember(initialProfile) { mutableStateOf(initialProfile?.notes ?: "") }
    var validationError by remember { mutableStateOf<String?>(null) }

    val isEditMode = initialProfile != null

    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text(if (isEditMode) "인물 수정" else "인물 등록", fontWeight = FontWeight.Bold, color = TextPrimary) },
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
            OutlinedTextField(
                value = name,
                onValueChange = { name = it },
                label = { Text("이름 (필수)", color = TextSecondary) },
                modifier = Modifier.fillMaxWidth(),
                shape = RoundedCornerShape(10.dp),
                singleLine = true,
                colors = OutlinedTextFieldDefaults.colors(focusedBorderColor = AccentPrimary, unfocusedBorderColor = BorderColor)
            )

            GenderSelectorRow(isMale = isMale, onSelect = { isMale = it })

            Text("출생 날짜", color = TextSecondary, fontSize = 13.sp, fontWeight = FontWeight.Medium)
            DateInputRow(yearStr, monthStr, dayStr, { yearStr = it }, { monthStr = it }, { dayStr = it })

            Text("출생 시각", color = TextSecondary, fontSize = 13.sp, fontWeight = FontWeight.Medium)
            TimeInputRow(hourStr, minuteStr, secondStr, { hourStr = it }, { minuteStr = it }, { secondStr = it })

            LocationPicker(
                latitude = latitude,
                longitude = longitude,
                onLocationChanged = { lat, lon ->
                    latitude = lat
                    longitude = lon
                }
            )

            OutlinedTextField(
                value = notes,
                onValueChange = { notes = it },
                label = { Text("메모 (선택)") },
                modifier = Modifier.fillMaxWidth(),
                minLines = 2,
                shape = RoundedCornerShape(10.dp),
                colors = OutlinedTextFieldDefaults.colors(focusedBorderColor = AccentPrimary, unfocusedBorderColor = BorderColor)
            )

            validationError?.let { err ->
                Text(err, color = AccentWarning, fontSize = 13.sp, fontWeight = FontWeight.Medium, modifier = Modifier.padding(horizontal = 4.dp))
            }

            Spacer(modifier = Modifier.height(16.dp))

            Button(
                onClick = {
                    val trimmedName = name.trim()
                    if (trimmedName.isEmpty()) {
                        validationError = "이름을 입력해 주세요."
                        return@Button
                    }
                    val fields = DateTimeFormFields(yearStr, monthStr, dayStr, hourStr, minuteStr, secondStr, allowBlankSecond = true)
                    when (val result = validateDateTimeAndCoordinates(fields, latitude, longitude)) {
                        is FormValidationResult.Invalid -> validationError = result.message
                        is FormValidationResult.Valid -> {
                            validationError = null
                            val dt = result.value
                            onSave(
                                CreateProfileRequest(
                                    name = trimmedName,
                                    gender = if (isMale) "male" else "female",
                                    birth_year = dt.year,
                                    birth_month = dt.month,
                                    birth_day = dt.day,
                                    birth_hour = dt.hour,
                                    birth_minute = dt.minute,
                                    birth_second = dt.second,
                                    latitude = latitude,
                                    longitude = longitude,
                                    notes = notes.trim()
                                )
                            )
                        }
                    }
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

@Composable
private fun GenderSelectorRow(isMale: Boolean, onSelect: (Boolean) -> Unit) {
    Row(modifier = Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.spacedBy(12.dp)) {
        listOf(true to "남성", false to "여성").forEach { (maleOption, label) ->
            val selected = isMale == maleOption
            Button(
                onClick = { onSelect(maleOption) },
                modifier = Modifier.weight(1f),
                colors = ButtonDefaults.buttonColors(
                    containerColor = if (selected) AccentPrimary else SurfaceLight,
                    contentColor = if (selected) BackgroundWhite else TextPrimary
                ),
                border = BorderStroke(1.dp, if (selected) AccentPrimary else BorderColor),
                shape = RoundedCornerShape(8.dp)
            ) {
                Text(label, fontWeight = if (selected) FontWeight.Bold else FontWeight.Normal)
            }
        }
    }
}
