package com.fortunexample.note.ui.screens

import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.fortunexample.note.data.ProfileDto
import com.fortunexample.note.ui.components.LocationPicker
import com.fortunexample.note.ui.theme.*

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ProfileEditScreen(
    initialProfile: ProfileDto? = null,
    onBack: () -> Unit,
    onSave: (name: String, isMale: Boolean, year: Int, month: Int, day: Int, hour: Int, minute: Int, lat: Double, lon: Double, notes: String) -> Unit
) {
    var name by remember(initialProfile) { mutableStateOf(initialProfile?.name ?: "") }
    var isMale by remember(initialProfile) { mutableStateOf(initialProfile?.gender != "female") }
    var yearStr by remember(initialProfile) { mutableStateOf(initialProfile?.birth_year?.toString() ?: "2000") }
    var monthStr by remember(initialProfile) { mutableStateOf(initialProfile?.birth_month?.toString() ?: "1") }
    var dayStr by remember(initialProfile) { mutableStateOf(initialProfile?.birth_day?.toString() ?: "1") }
    var hourStr by remember(initialProfile) { mutableStateOf(initialProfile?.birth_hour?.toString() ?: "12") }
    var minuteStr by remember(initialProfile) { mutableStateOf(initialProfile?.birth_minute?.toString() ?: "0") }
    var latitude by remember(initialProfile) { mutableDoubleStateOf(initialProfile?.latitude ?: 37.5665) }
    var longitude by remember(initialProfile) { mutableDoubleStateOf(initialProfile?.longitude ?: 126.9780) }
    var notes by remember(initialProfile) { mutableStateOf(initialProfile?.notes ?: "") }

    val isEditMode = initialProfile != null

    Scaffold(
        topBar = {
            TopAppBar(
                title = {
                    Text(
                        text = if (isEditMode) "인물 수정" else "인물 등록",
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
            // 이름 입력
            OutlinedTextField(
                value = name,
                onValueChange = { name = it },
                label = { Text("이름 (필수)", color = TextSecondary) },
                modifier = Modifier.fillMaxWidth(),
                shape = RoundedCornerShape(10.dp),
                singleLine = true,
                colors = OutlinedTextFieldDefaults.colors(
                    focusedBorderColor = AccentPrimary,
                    unfocusedBorderColor = BorderColor
                )
            )

            // 성별 선택
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.spacedBy(12.dp)
            ) {
                Button(
                    onClick = { isMale = true },
                    modifier = Modifier.weight(1f),
                    colors = ButtonDefaults.buttonColors(
                        containerColor = if (isMale) AccentPrimary else SurfaceLight,
                        contentColor = if (isMale) BackgroundWhite else TextPrimary
                    ),
                    border = BorderStroke(1.dp, if (isMale) AccentPrimary else BorderColor),
                    shape = RoundedCornerShape(8.dp)
                ) {
                    Text("남성", fontWeight = if (isMale) FontWeight.Bold else FontWeight.Normal)
                }
                Button(
                    onClick = { isMale = false },
                    modifier = Modifier.weight(1f),
                    colors = ButtonDefaults.buttonColors(
                        containerColor = if (!isMale) AccentPrimary else SurfaceLight,
                        contentColor = if (!isMale) BackgroundWhite else TextPrimary
                    ),
                    border = BorderStroke(1.dp, if (!isMale) AccentPrimary else BorderColor),
                    shape = RoundedCornerShape(8.dp)
                ) {
                    Text("여성", fontWeight = if (!isMale) FontWeight.Bold else FontWeight.Normal)
                }
            }

            // 출생 날짜 (숫자 전용 키보드)
            Text("출생 날짜", color = TextSecondary, fontSize = 13.sp, fontWeight = FontWeight.Medium)
            Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                OutlinedTextField(
                    value = yearStr,
                    onValueChange = { yearStr = it },
                    label = { Text("년도") },
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

            // 출생 시각 (숫자 전용 키보드)
            Text("출생 시각", color = TextSecondary, fontSize = 13.sp, fontWeight = FontWeight.Medium)
            Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
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
                    keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number, imeAction = ImeAction.Done),
                    modifier = Modifier.weight(1f),
                    singleLine = true
                )
            }

            // 관측 위치 선택기
            LocationPicker(
                latitude = latitude,
                longitude = longitude,
                onLocationChanged = { newLat, newLon ->
                    latitude = newLat
                    longitude = newLon
                }
            )

            // 메모 입력
            OutlinedTextField(
                value = notes,
                onValueChange = { notes = it },
                label = { Text("메모 (선택)") },
                modifier = Modifier.fillMaxWidth(),
                minLines = 2,
                shape = RoundedCornerShape(10.dp),
                colors = OutlinedTextFieldDefaults.colors(
                    focusedBorderColor = AccentPrimary,
                    unfocusedBorderColor = BorderColor
                )
            )

            Spacer(modifier = Modifier.height(16.dp))

            // 저장 버튼
            Button(
                onClick = {
                    val y = yearStr.toIntOrNull() ?: 2000
                    val m = (monthStr.toIntOrNull() ?: 1).coerceIn(1, 12)
                    val d = (dayStr.toIntOrNull() ?: 1).coerceIn(1, 31)
                    val h = (hourStr.toIntOrNull() ?: 0).coerceIn(0, 23)
                    val min = (minuteStr.toIntOrNull() ?: 0).coerceIn(0, 59)

                    if (name.trim().isNotEmpty()) {
                        onSave(
                            name.trim(),
                            isMale,
                            y,
                            m,
                            d,
                            h,
                            min,
                            latitude,
                            longitude,
                            notes.trim()
                        )
                    }
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
