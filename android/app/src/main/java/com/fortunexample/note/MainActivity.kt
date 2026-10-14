package com.fortunexample.note

import android.os.Bundle
import android.widget.Toast
import androidx.activity.ComponentActivity
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.compose.setContent
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.runtime.*
import androidx.compose.ui.platform.LocalContext
import androidx.navigation.NavType
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import androidx.navigation.navArgument
import com.fortunexample.note.data.*
import com.fortunexample.note.ui.screens.*
import com.fortunexample.note.ui.theme.FortuneNoteTheme
import kotlinx.coroutines.launch

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        setContent {
            FortuneNoteTheme {
                val context = LocalContext.current
                val navController = rememberNavController()
                val scope = rememberCoroutineScope()

                var allProfiles by remember { mutableStateOf<List<ProfileDto>>(emptyList()) }
                var profileSearchQuery by remember { mutableStateOf("") }
                var horaryList by remember { mutableStateOf<List<HoraryDto>>(emptyList()) }

                val displayedProfiles = remember(allProfiles, profileSearchQuery) {
                    if (profileSearchQuery.isBlank()) {
                        allProfiles
                    } else {
                        allProfiles.filter {
                            it.name.contains(profileSearchQuery, ignoreCase = true) ||
                            it.notes.contains(profileSearchQuery, ignoreCase = true)
                        }
                    }
                }

                fun refreshData() {
                    scope.launch {
                        FortuneApp.repository.listProfiles().onSuccess { allProfiles = it }
                        FortuneApp.repository.listHorary().onSuccess { horaryList = it }
                    }
                }

                LaunchedEffect(Unit) {
                    refreshData()
                }

                NavHost(navController = navController, startDestination = "home") {
                    composable("home") {
                        HomeScreen(
                            profiles = displayedProfiles,
                            horaryQuestions = horaryList,
                            onProfileClick = { id -> navController.navigate("profile_detail/$id") },
                            onHoraryClick = { id -> navController.navigate("horary_detail/$id") },
                            onAddProfile = { navController.navigate("profile_edit") },
                            onAddHorary = { navController.navigate("horary_edit") },
                            onSearch = { query -> profileSearchQuery = query },
                            onNavigateToDataManagement = {
                                navController.navigate("data_management")
                            }
                        )
                    }

                    composable("data_management") {
                        DataManagementScreen(
                            profileCount = allProfiles.size,
                            horaryCount = horaryList.size,
                            onBack = { navController.popBackStack() },
                            onDataRestored = {
                                refreshData()
                                navController.popBackStack()
                            }
                        )
                    }

                    composable(
                        "profile_edit?id={id}",
                        arguments = listOf(navArgument("id") {
                            type = NavType.StringType
                            nullable = true
                            defaultValue = null
                        })
                    ) { backStackEntry ->
                        val editId = backStackEntry.arguments?.getString("id")
                        val existingProfile = allProfiles.find { it.id == editId }

                        ProfileEditScreen(
                            initialProfile = existingProfile,
                            onBack = { navController.popBackStack() },
                            onSave = { name, isMale, y, m, d, h, min, lat, lon, notes ->
                                scope.launch {
                                    if (editId != null) {
                                        FortuneApp.repository.updateProfile(
                                            editId,
                                            UpdateProfileRequest(
                                                name = name,
                                                gender = if (isMale) "male" else "female",
                                                birth_year = y,
                                                birth_month = m,
                                                birth_day = d,
                                                birth_hour = h,
                                                birth_minute = min,
                                                latitude = lat,
                                                longitude = lon,
                                                notes = notes
                                            )
                                        ).onSuccess {
                                            refreshData()
                                            navController.popBackStack()
                                        }.onFailure {
                                            Toast.makeText(context, it.message ?: "인물 정보 수정에 실패했습니다.", Toast.LENGTH_SHORT).show()
                                        }
                                    } else {
                                        FortuneApp.repository.createProfile(
                                            CreateProfileRequest(
                                                name = name,
                                                gender = if (isMale) "male" else "female",
                                                birth_year = y,
                                                birth_month = m,
                                                birth_day = d,
                                                birth_hour = h,
                                                birth_minute = min,
                                                latitude = lat,
                                                longitude = lon,
                                                notes = notes
                                            )
                                        ).onSuccess {
                                            refreshData()
                                            navController.popBackStack()
                                        }.onFailure {
                                            Toast.makeText(context, it.message ?: "인물 등록에 실패했습니다.", Toast.LENGTH_SHORT).show()
                                        }
                                    }
                                }
                            }
                        )
                    }

                    composable(
                        "profile_detail/{id}",
                        arguments = listOf(navArgument("id") { type = NavType.StringType })
                    ) { backStackEntry ->
                        val id = backStackEntry.arguments?.getString("id") ?: return@composable
                        val profile = allProfiles.find { it.id == id }
                        if (profile != null) {
                            ProfileDetailScreen(
                                profile = profile,
                                onBack = { navController.popBackStack() },
                                onEdit = { navController.navigate("profile_edit?id=$id") },
                                onDelete = {
                                    scope.launch {
                                        FortuneApp.repository.deleteProfile(id).onSuccess {
                                            refreshData()
                                            navController.popBackStack()
                                        }.onFailure {
                                            Toast.makeText(context, it.message ?: "인물 삭제에 실패했습니다.", Toast.LENGTH_SHORT).show()
                                        }
                                    }
                                }
                            )
                        }
                    }

                    composable(
                        "horary_edit?id={id}",
                        arguments = listOf(navArgument("id") {
                            type = NavType.StringType
                            nullable = true
                            defaultValue = null
                        })
                    ) { backStackEntry ->
                        val editId = backStackEntry.arguments?.getString("id")
                        val existingHorary = horaryList.find { it.id == editId }

                        HoraryEditScreen(
                            initialHorary = existingHorary,
                            onBack = { navController.popBackStack() },
                            onSave = { title, category, text, y, m, d, h, min, sec, lat, lon ->
                                scope.launch {
                                    if (editId != null) {
                                        FortuneApp.repository.updateHorary(
                                            editId,
                                            UpdateHoraryRequest(
                                                title = title,
                                                category = category,
                                                question_text = text,
                                                query_year = y,
                                                query_month = m,
                                                query_day = d,
                                                query_hour = h,
                                                query_minute = min,
                                                query_second = sec,
                                                latitude = lat,
                                                longitude = lon
                                            )
                                        ).onSuccess {
                                            refreshData()
                                            navController.popBackStack()
                                        }.onFailure {
                                            Toast.makeText(context, it.message ?: "호라리 질문 수정에 실패했습니다.", Toast.LENGTH_SHORT).show()
                                        }
                                    } else {
                                        FortuneApp.repository.createHorary(
                                            CreateHoraryRequest(
                                                title = title,
                                                category = category,
                                                question_text = text,
                                                query_year = y,
                                                query_month = m,
                                                query_day = d,
                                                query_hour = h,
                                                query_minute = min,
                                                query_second = sec,
                                                latitude = lat,
                                                longitude = lon
                                            )
                                        ).onSuccess {
                                            refreshData()
                                            navController.popBackStack()
                                        }.onFailure {
                                            Toast.makeText(context, it.message ?: "호라리 질문 등록에 실패했습니다.", Toast.LENGTH_SHORT).show()
                                        }
                                    }
                                }
                            }
                        )
                    }

                    composable(
                        "horary_detail/{id}",
                        arguments = listOf(navArgument("id") { type = NavType.StringType })
                    ) { backStackEntry ->
                        val id = backStackEntry.arguments?.getString("id") ?: return@composable
                        val horary = horaryList.find { it.id == id }
                        var chart by remember { mutableStateOf<FortuneChartOutput?>(null) }
                        var westernMarkdown by remember { mutableStateOf<String?>(null) }
                        var prashnaMarkdown by remember { mutableStateOf<String?>(null) }

                        LaunchedEffect(id) {
                            FortuneApp.repository.calculateHoraryChart(id).onSuccess { chart = it }
                            FortuneApp.repository.exportHoraryWesternMarkdown(id).onSuccess { westernMarkdown = it }
                            FortuneApp.repository.exportPrashnaVedicMarkdown(id).onSuccess { prashnaMarkdown = it }
                        }

                        if (horary != null) {
                            HoraryDetailScreen(
                                horary = horary,
                                chart = chart,
                                westernMarkdown = westernMarkdown,
                                prashnaMarkdown = prashnaMarkdown,
                                onBack = { navController.popBackStack() },
                                onEdit = { navController.navigate("horary_edit?id=$id") },
                                onDelete = {
                                    scope.launch {
                                        FortuneApp.repository.deleteHorary(id).onSuccess {
                                            refreshData()
                                            navController.popBackStack()
                                        }.onFailure {
                                            Toast.makeText(context, it.message ?: "호라리 질문 삭제에 실패했습니다.", Toast.LENGTH_SHORT).show()
                                        }
                                    }
                                }
                            )
                        }
                    }
                }
            }
        }
    }
}
