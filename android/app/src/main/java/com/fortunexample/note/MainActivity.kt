package com.fortunexample.note

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.runtime.*
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
        val repository = FortuneApp.repository

        setContent {
            FortuneNoteTheme {
                val navController = rememberNavController()
                val scope = rememberCoroutineScope()

                var profiles by remember { mutableStateOf<List<ProfileDto>>(emptyList()) }
                var horaryList by remember { mutableStateOf<List<HoraryDto>>(emptyList()) }

                fun refreshData() {
                    scope.launch {
                        repository.listProfiles().onSuccess { profiles = it }
                        repository.listHorary().onSuccess { horaryList = it }
                    }
                }

                LaunchedEffect(Unit) {
                    refreshData()
                }

                NavHost(navController = navController, startDestination = "home") {
                    composable("home") {
                        HomeScreen(
                            profiles = profiles,
                            horaryQuestions = horaryList,
                            onProfileClick = { id -> navController.navigate("profile_detail/$id") },
                            onHoraryClick = { id -> navController.navigate("horary_detail/$id") },
                            onAddProfile = { navController.navigate("profile_edit") },
                            onAddHorary = { navController.navigate("horary_edit") },
                            onSearch = { query ->
                                scope.launch {
                                    repository.listProfiles(query).onSuccess { profiles = it }
                                }
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
                        val existingProfile = profiles.find { it.id == editId }

                        ProfileEditScreen(
                            initialProfile = existingProfile,
                            onBack = { navController.popBackStack() },
                            onSave = { name, isMale, y, m, d, h, min, lat, lon, notes ->
                                scope.launch {
                                    if (editId != null) {
                                        repository.updateProfile(
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
                                        }
                                    } else {
                                        repository.createProfile(
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
                        val profile = profiles.find { it.id == id }
                        var chart by remember { mutableStateOf<FortuneChartOutput?>(null) }
                        var markdown by remember { mutableStateOf<String?>(null) }

                        LaunchedEffect(id) {
                            repository.calculateProfileChart(id).onSuccess { chart = it }
                            repository.exportProfileMarkdown(id).onSuccess { markdown = it }
                        }

                        if (profile != null) {
                            ProfileDetailScreen(
                                profile = profile,
                                chart = chart,
                                markdown = markdown,
                                onBack = { navController.popBackStack() },
                                onEdit = { navController.navigate("profile_edit?id=$id") },
                                onDelete = {
                                    scope.launch {
                                        repository.deleteProfile(id).onSuccess {
                                            refreshData()
                                            navController.popBackStack()
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
                                        repository.updateHorary(
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
                                        }
                                    } else {
                                        repository.createHorary(
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
                        var markdown by remember { mutableStateOf<String?>(null) }

                        LaunchedEffect(id) {
                            repository.calculateHoraryChart(id).onSuccess { chart = it }
                            repository.exportHoraryMarkdown(id).onSuccess { markdown = it }
                        }

                        if (horary != null) {
                            HoraryDetailScreen(
                                horary = horary,
                                chart = chart,
                                markdown = markdown,
                                onBack = { navController.popBackStack() },
                                onEdit = { navController.navigate("horary_edit?id=$id") },
                                onDelete = {
                                    scope.launch {
                                        repository.deleteHorary(id).onSuccess {
                                            refreshData()
                                            navController.popBackStack()
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
