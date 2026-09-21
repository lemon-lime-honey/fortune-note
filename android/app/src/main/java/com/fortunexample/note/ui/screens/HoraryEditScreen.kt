package com.fortunexample.note.ui.screens

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
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.fortunexample.note.data.CreateHoraryRequest
import com.fortunexample.note.ui.theme.*
import java.util.Calendar

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun HoraryEditScreen(
    onBack: () -> Unit,
    onSave: (CreateHoraryRequest) -> Unit
) {
    var title by remember { mutableStateOf("") }
    var category by remember { mutableStateOf("직업") }
    var questionText by remember { mutableStateOf("") }

    val cal = Calendar.getInstance()
    var yearStr by remember { mutableStateOf(cal.get(Calendar.YEAR).toString()) }
    var monthStr by remember { mutableStateOf((cal.get(Calendar.MONTH) + 1).toString()) }
    var dayStr by remember { mutableStateOf(cal.get(Calendar.DAY_OF_MONTH).toString()) }
    var hourStr by remember { mutableStateOf(cal.get(Calendar.HOUR_OF_DAY).toString()) }
    var minuteStr by remember { mutableStateOf(cal.get(Calendar.MINUTE).toString()) }
    var secondStr by remember { mutableStateOf(cal.get(Calendar.SECOND).toString()) }
    var latStr by remember { mutableStateOf("37.5665") }
    var lonStr by remember { mutableStateOf("126.9780") }

    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text("호라리 질문 등록", fontWeight = FontWeight.Bold) },
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
            // 즉시 캡처 버튼
            Button(
                onClick = {
                    val now = Calendar.getInstance()
                    yearStr = now.get(Calendar.YEAR).toString()
                    monthStr = (now.get(Calendar.MONTH) + 1).toString()
                    dayStr = now.get(Calendar.DAY_OF_MONTH).toString()
                    hourStr = now.get(Calendar.HOUR_OF_DAY).toString()
                    minuteStr = now.get(Calendar.MINUTE).toString()
                    secondStr = now.get(Calendar.SECOND).toString()
                },
                modifier = Modifier.fillMaxWidth(),
                colors = ButtonDefaults.buttonColors(containerColor = CelestialBlue),
                shape = RoundedCornerShape(10.dp)
            ) {
                Icon(Icons.Default.FlashOn, contentDescription = null, tint = DeepNavy)
                Spacer(modifier = Modifier.width(6.dp))
                Text("지금 현재 시각으로 캡처", color = DeepNavy, fontWeight = FontWeight.Bold)
            }

            OutlinedTextField(
                value = title,
                onValueChange = { title = it },
                label = { Text("질문 제목 (필수)") },
                modifier = Modifier.fillMaxWidth()
            )

            OutlinedTextField(
                value = category,
                onValueChange = { category = it },
                label = { Text("카테고리 (예: 직업, 연애, 재물)") },
                modifier = Modifier.fillMaxWidth()
            )

            OutlinedTextField(
                value = questionText,
                onValueChange = { questionText = it },
                label = { Text("질문 상세 본문") },
                modifier = Modifier.fillMaxWidth(),
                minLines = 3
            )

            Text("질문 일시 (초 단위 정밀 지원)", color = TextSecondary, fontSize = 13.sp)
            Row(horizontalArrangement = Arrangement.spacedBy(6.dp)) {
                OutlinedTextField(value = yearStr, onValueChange = { yearStr = it }, label = { Text("년") }, modifier = Modifier.weight(1.2f))
                OutlinedTextField(value = monthStr, onValueChange = { monthStr = it }, label = { Text("월") }, modifier = Modifier.weight(1f))
                OutlinedTextField(value = dayStr, onValueChange = { dayStr = it }, label = { Text("일") }, modifier = Modifier.weight(1f))
            }
            Row(horizontalArrangement = Arrangement.spacedBy(6.dp)) {
                OutlinedTextField(value = hourStr, onValueChange = { hourStr = it }, label = { Text("시") }, modifier = Modifier.weight(1f))
                OutlinedTextField(value = minuteStr, onValueChange = { minuteStr = it }, label = { Text("분") }, modifier = Modifier.weight(1f))
                OutlinedTextField(value = secondStr, onValueChange = { secondStr = it }, label = { Text("초") }, modifier = Modifier.weight(1f))
            }

            Spacer(modifier = Modifier.height(16.dp))

            Button(
                onClick = {
                    if (title.trim().isNotEmpty()) {
                        onSave(
                            CreateHoraryRequest(
                                title = title.trim(),
                                category = category.trim(),
                                question_text = questionText.trim(),
                                query_year = yearStr.toIntOrNull() ?: 2026,
                                query_month = monthStr.toIntOrNull() ?: 1,
                                query_day = dayStr.toIntOrNull() ?: 1,
                                query_hour = hourStr.toIntOrNull() ?: 0,
                                query_minute = minuteStr.toIntOrNull() ?: 0,
                                query_second = secondStr.toDoubleOrNull() ?: 0.0,
                                latitude = latStr.toDoubleOrNull() ?: 37.5665,
                                longitude = lonStr.toDoubleOrNull() ?: 126.9780
                            )
                        )
                    }
                },
                modifier = Modifier
                    .fillMaxWidth()
                    .height(50.dp),
                colors = ButtonDefaults.buttonColors(containerColor = CelestialBlue),
                shape = RoundedCornerShape(12.dp)
            ) {
                Text("호라리 질문 저장", color = DeepNavy, fontWeight = FontWeight.Bold, fontSize = 16.sp)
            }
        }
    }
}
