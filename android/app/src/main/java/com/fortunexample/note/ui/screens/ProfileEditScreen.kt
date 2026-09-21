package com.fortunexample.note.ui.screens

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.fortunexample.note.data.CreateProfileRequest
import com.fortunexample.note.ui.theme.*

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ProfileEditScreen(
    onBack: () -> Unit,
    onSave: (CreateProfileRequest) -> Unit
) {
    var name by remember { mutableStateOf("") }
    var isMale by remember { mutableStateOf(true) }
    var yearStr by remember { mutableStateOf("1995") }
    var monthStr by remember { mutableStateOf("10") }
    var dayStr by remember { mutableStateOf("25") }
    var hourStr by remember { mutableStateOf("14") }
    var minuteStr by remember { mutableStateOf("30") }
    var latStr by remember { mutableStateOf("37.5665") }
    var lonStr by remember { mutableStateOf("126.9780") }
    var notes by remember { mutableStateOf("") }

    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text("인물 등록", fontWeight = FontWeight.Bold) },
                navigationIcon = {
                    IconButton(onClick = onBack) {
                        Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = "뒤로")
                    }
                },
                colors = TopAppBarDefaults.topAppBarColors(
                    containerColor = DeepNavy,
                    titleContentColor = TextPrimary,
                    navigationIconContentColor = TextPrimary
                )
            )
        },
        containerColor = DeepNavy
    ) { padding ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(padding)
                .verticalScroll(rememberScrollState())
                .padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(14.dp)
        ) {
            // 이름
            OutlinedTextField(
                value = name,
                onValueChange = { name = it },
                label = { Text("이름 (필수)", color = TextSecondary) },
                modifier = Modifier.fillMaxWidth(),
                shape = RoundedCornerShape(10.dp),
                colors = OutlinedTextFieldDefaults.colors(focusedBorderColor = AccentGold, unfocusedBorderColor = BorderColor)
            )

            // 성별 선택
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.spacedBy(12.dp)
            ) {
                Button(
                    onClick = { isMale = true },
                    modifier = Modifier.weight(1f),
                    colors = ButtonDefaults.buttonColors(containerColor = if (isMale) AccentGold else SurfaceDark)
                ) {
                    Text("남성", color = if (isMale) DeepNavy else TextPrimary)
                }
                Button(
                    onClick = { isMale = false },
                    modifier = Modifier.weight(1f),
                    colors = ButtonDefaults.buttonColors(containerColor = if (!isMale) AccentGold else SurfaceDark)
                ) {
                    Text("여성", color = if (!isMale) DeepNavy else TextPrimary)
                }
            }

            // 생년월일
            Text("출생 날짜", color = TextSecondary, fontSize = 13.sp)
            Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                OutlinedTextField(
                    value = yearStr,
                    onValueChange = { yearStr = it },
                    label = { Text("년도") },
                    modifier = Modifier.weight(1.2f)
                )
                OutlinedTextField(
                    value = monthStr,
                    onValueChange = { monthStr = it },
                    label = { Text("월") },
                    modifier = Modifier.weight(1f)
                )
                OutlinedTextField(
                    value = dayStr,
                    onValueChange = { dayStr = it },
                    label = { Text("일") },
                    modifier = Modifier.weight(1f)
                )
            }

            // 출생 시각 (초 단위 배제)
            Text("출생 시각 (시/분 필수 입력, 00초 기본)", color = TextSecondary, fontSize = 13.sp)
            Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                OutlinedTextField(
                    value = hourStr,
                    onValueChange = { hourStr = it },
                    label = { Text("시 (0~23)") },
                    modifier = Modifier.weight(1f)
                )
                OutlinedTextField(
                    value = minuteStr,
                    onValueChange = { minuteStr = it },
                    label = { Text("분 (0~59)") },
                    modifier = Modifier.weight(1f)
                )
            }

            // 관측 좌표
            Text("출생지 좌표 (서울 기본값)", color = TextSecondary, fontSize = 13.sp)
            Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                OutlinedTextField(
                    value = latStr,
                    onValueChange = { latStr = it },
                    label = { Text("위도") },
                    modifier = Modifier.weight(1f)
                )
                OutlinedTextField(
                    value = lonStr,
                    onValueChange = { lonStr = it },
                    label = { Text("경도") },
                    modifier = Modifier.weight(1f)
                )
            }

            // 메모
            OutlinedTextField(
                value = notes,
                onValueChange = { notes = it },
                label = { Text("메모 (선택)") },
                modifier = Modifier.fillMaxWidth(),
                minLines = 2
            )

            Spacer(modifier = Modifier.height(16.dp))

            Button(
                onClick = {
                    val y = yearStr.toIntOrNull() ?: 1995
                    val m = monthStr.toIntOrNull() ?: 1
                    val d = dayStr.toIntOrNull() ?: 1
                    val h = hourStr.toIntOrNull() ?: 0
                    val min = minuteStr.toIntOrNull() ?: 0
                    val lat = latStr.toDoubleOrNull() ?: 37.5665
                    val lon = lonStr.toDoubleOrNull() ?: 126.9780

                    if (name.trim().isNotEmpty()) {
                        onSave(
                            CreateProfileRequest(
                                name = name.trim(),
                                gender = if (isMale) "male" else "female",
                                birth_year = y,
                                birth_month = m,
                                birth_day = d,
                                birth_hour = h,
                                birth_minute = min,
                                latitude = lat,
                                longitude = lon,
                                notes = notes.trim()
                            )
                        )
                    }
                },
                modifier = Modifier
                    .fillMaxWidth()
                    .height(50.dp),
                colors = ButtonDefaults.buttonColors(containerColor = AccentGold),
                shape = RoundedCornerShape(12.dp)
            ) {
                Text("저장하기", color = DeepNavy, fontWeight = FontWeight.Bold, fontSize = 16.sp)
            }
        }
    }
}
