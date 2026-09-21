package com.fortunexample.note.ui.screens

import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.background
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.MoreVert
import androidx.compose.material.icons.filled.Search
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.fortunexample.note.data.HoraryDto
import com.fortunexample.note.data.ProfileDto
import com.fortunexample.note.ui.theme.*

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun HomeScreen(
    profiles: List<ProfileDto>,
    horaryQuestions: List<HoraryDto>,
    onProfileClick: (String) -> Unit,
    onHoraryClick: (String) -> Unit,
    onAddProfile: () -> Unit,
    onAddHorary: () -> Unit,
    onSearch: (String) -> Unit,
    onExportBackup: () -> Unit,
    onImportBackup: () -> Unit
) {
    var selectedTab by remember { mutableStateOf(0) }
    var searchQuery by remember { mutableStateOf("") }
    var showMenu by remember { mutableStateOf(false) }

    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text("Fortune Note", fontWeight = FontWeight.Bold, color = TextPrimary) },
                actions = {
                    IconButton(onClick = { showMenu = true }) {
                        Icon(Icons.Default.MoreVert, contentDescription = "메뉴", tint = TextPrimary)
                    }
                    DropdownMenu(
                        expanded = showMenu,
                        onDismissRequest = { showMenu = false },
                        modifier = Modifier.background(BackgroundWhite)
                    ) {
                        DropdownMenuItem(
                            text = { Text("데이터 백업 (DB 파일 내보내기)", color = TextPrimary, fontSize = 14.sp) },
                            onClick = {
                                showMenu = false
                                onExportBackup()
                            }
                        )
                        DropdownMenuItem(
                            text = { Text("데이터 복원 (DB 파일 불러오기)", color = TextPrimary, fontSize = 14.sp) },
                            onClick = {
                                showMenu = false
                                onImportBackup()
                            }
                        )
                    }
                },
                colors = TopAppBarDefaults.topAppBarColors(
                    containerColor = BackgroundWhite,
                    titleContentColor = TextPrimary
                )
            )
        },
        floatingActionButton = {
            FloatingActionButton(
                onClick = { if (selectedTab == 0) onAddProfile() else onAddHorary() },
                containerColor = AccentPrimary,
                contentColor = BackgroundWhite
            ) {
                Icon(Icons.Default.Add, contentDescription = "추가")
            }
        },
        containerColor = BackgroundWhite
    ) { padding ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(padding)
        ) {
            // 검색 필드
            OutlinedTextField(
                value = searchQuery,
                onValueChange = {
                    searchQuery = it
                    onSearch(it)
                },
                placeholder = { Text("이름 또는 메모 검색", color = TextSecondary) },
                leadingIcon = { Icon(Icons.Default.Search, contentDescription = null, tint = TextSecondary) },
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(horizontal = 16.dp, vertical = 8.dp),
                shape = RoundedCornerShape(10.dp),
                singleLine = true,
                colors = OutlinedTextFieldDefaults.colors(
                    focusedBorderColor = AccentPrimary,
                    unfocusedBorderColor = BorderColor,
                    focusedTextColor = TextPrimary,
                    unfocusedTextColor = TextPrimary
                )
            )

            // 탭 헤더
            TabRow(
                selectedTabIndex = selectedTab,
                containerColor = SurfaceLight,
                contentColor = AccentPrimary
            ) {
                Tab(
                    selected = selectedTab == 0,
                    onClick = { selectedTab = 0 },
                    text = { Text("인물 노트 (${profiles.size})", fontWeight = FontWeight.SemiBold) }
                )
                Tab(
                    selected = selectedTab == 1,
                    onClick = { selectedTab = 1 },
                    text = { Text("질문 노트 (${horaryQuestions.size})", fontWeight = FontWeight.SemiBold) }
                )
            }

            // 목록 영역
            if (selectedTab == 0) {
                if (profiles.isEmpty()) {
                    Box(modifier = Modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
                        Text("등록된 인물 정보가 없습니다.", color = TextSecondary, fontSize = 14.sp)
                    }
                } else {
                    LazyColumn(
                        modifier = Modifier.fillMaxSize(),
                        contentPadding = PaddingValues(16.dp),
                        verticalArrangement = Arrangement.spacedBy(10.dp)
                    ) {
                        items(profiles) { profile ->
                            ProfileCard(profile = profile, onClick = { onProfileClick(profile.id) })
                        }
                    }
                }
            } else {
                if (horaryQuestions.isEmpty()) {
                    Box(modifier = Modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
                        Text("등록된 질문 기록이 없습니다.", color = TextSecondary, fontSize = 14.sp)
                    }
                } else {
                    LazyColumn(
                        modifier = Modifier.fillMaxSize(),
                        contentPadding = PaddingValues(16.dp),
                        verticalArrangement = Arrangement.spacedBy(10.dp)
                    ) {
                        items(horaryQuestions) { horary ->
                            HoraryCard(horary = horary, onClick = { onHoraryClick(horary.id) })
                        }
                    }
                }
            }
        }
    }
}

@Composable
private fun ProfileCard(
    profile: ProfileDto,
    onClick: () -> Unit
) {
    Card(
        shape = RoundedCornerShape(12.dp),
        colors = CardDefaults.cardColors(containerColor = BackgroundWhite),
        border = BorderStroke(1.dp, BorderColor),
        modifier = Modifier
            .fillMaxWidth()
            .clickable(onClick = onClick)
    ) {
        Column(modifier = Modifier.padding(16.dp)) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically
            ) {
                Text(
                    text = profile.name,
                    color = TextPrimary,
                    fontWeight = FontWeight.Bold,
                    fontSize = 17.sp
                )
                Text(
                    text = if (profile.gender == "male") "남성" else "여성",
                    color = TextSecondary,
                    fontSize = 13.sp,
                    fontWeight = FontWeight.Medium
                )
            }
            Spacer(modifier = Modifier.height(6.dp))
            Text(
                text = "${profile.birth_year}-${String.format("%02d", profile.birth_month)}-${String.format("%02d", profile.birth_day)} ${String.format("%02d:%02d", profile.birth_hour, profile.birth_minute)}",
                color = TextSecondary,
                fontSize = 14.sp
            )
            if (profile.notes.isNotEmpty()) {
                Spacer(modifier = Modifier.height(4.dp))
                Text(
                    text = profile.notes,
                    color = TextSecondary.copy(alpha = 0.8f),
                    fontSize = 12.sp,
                    maxLines = 1
                )
            }
        }
    }
}

@Composable
private fun HoraryCard(
    horary: HoraryDto,
    onClick: () -> Unit
) {
    Card(
        shape = RoundedCornerShape(12.dp),
        colors = CardDefaults.cardColors(containerColor = BackgroundWhite),
        border = BorderStroke(1.dp, BorderColor),
        modifier = Modifier
            .fillMaxWidth()
            .clickable(onClick = onClick)
    ) {
        Column(modifier = Modifier.padding(16.dp)) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically
            ) {
                Text(
                    text = horary.title,
                    color = TextPrimary,
                    fontWeight = FontWeight.Bold,
                    fontSize = 16.sp
                )
                Text(
                    text = horary.category,
                    color = CelestialBlue,
                    fontSize = 12.sp,
                    fontWeight = FontWeight.Medium
                )
            }
            Spacer(modifier = Modifier.height(6.dp))
            Text(
                text = "${horary.query_year}-${String.format("%02d", horary.query_month)}-${String.format("%02d", horary.query_day)} ${String.format("%02d:%02d:%02d", horary.query_hour, horary.query_minute, horary.query_second.toInt())}",
                color = TextSecondary,
                fontSize = 13.sp
            )
            if (horary.result_notes.isNotEmpty()) {
                Spacer(modifier = Modifier.height(4.dp))
                Text(
                    text = horary.result_notes,
                    color = TextSecondary,
                    fontSize = 12.sp
                )
            }
        }
    }
}
