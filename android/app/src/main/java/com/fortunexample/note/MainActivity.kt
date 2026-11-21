package com.fortunexample.note

import android.os.Bundle
import android.widget.Toast
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.platform.LocalContext
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavHostController
import androidx.navigation.NavType
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import androidx.navigation.navArgument
import com.fortunexample.note.ui.FortuneHomeUiState
import com.fortunexample.note.ui.FortuneViewModel
import com.fortunexample.note.ui.screens.*
import com.fortunexample.note.ui.theme.FortuneNoteTheme

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        setContent {
            FortuneNoteTheme {
                val vm: FortuneViewModel = viewModel(
                    factory = FortuneViewModel.factory(FortuneApp.repository)
                )
                FortuneNavGraph(vm)
            }
        }
    }
}

@Composable
private fun FortuneNavGraph(vm: FortuneViewModel) {
    val context = LocalContext.current
    val navController = rememberNavController()
    val homeState by vm.homeState.collectAsState()
    val showToast = { msg: String -> Toast.makeText(context, msg, Toast.LENGTH_SHORT).show() }

    NavHost(navController = navController, startDestination = "home") {
        composable("home") {
            HomeScreen(
                profiles = homeState.displayedProfiles,
                horaryQuestions = homeState.displayedHorary,
                searchQuery = homeState.searchQuery,
                onProfileClick = { id -> navController.navigate("profile_detail/$id") },
                onHoraryClick = { id -> navController.navigate("horary_detail/$id") },
                onAddProfile = { navController.navigate("profile_edit") },
                onAddHorary = { navController.navigate("horary_edit") },
                onSearch = vm::updateSearchQuery,
                onNavigateToDataManagement = { navController.navigate("data_management") }
            )
        }

        composable("data_management") {
            DataManagementScreen(
                profileCount = homeState.allProfiles.size,
                horaryCount = homeState.horaryList.size,
                backupRepository = vm.backupDataSource,
                onBack = { navController.popBackStack() },
                onDataRestored = {
                    vm.refreshAll()
                    navController.popBackStack()
                }
            )
        }

        composable(
            "profile_edit?id={id}",
            arguments = listOf(navArgument("id") { type = NavType.StringType; nullable = true; defaultValue = null })
        ) { entry ->
            ProfileEditRoute(entry.arguments?.getString("id"), homeState, vm, navController, showToast)
        }

        composable(
            "profile_detail/{id}",
            arguments = listOf(navArgument("id") { type = NavType.StringType })
        ) { entry ->
            val id = entry.arguments?.getString("id") ?: return@composable
            val profile = homeState.allProfiles.find { it.id == id } ?: return@composable
            ProfileDetailScreen(
                profile = profile,
                profileRepository = vm.profileDataSource,
                onBack = { navController.popBackStack() },
                onEdit = { navController.navigate("profile_edit?id=$id") },
                onDelete = { vm.deleteProfile(id, { navController.popBackStack() }, showToast) }
            )
        }

        composable(
            "horary_edit?id={id}",
            arguments = listOf(navArgument("id") { type = NavType.StringType; nullable = true; defaultValue = null })
        ) { entry ->
            HoraryEditRoute(entry.arguments?.getString("id"), homeState, vm, navController, showToast)
        }

        composable(
            "horary_detail/{id}",
            arguments = listOf(navArgument("id") { type = NavType.StringType })
        ) { entry ->
            val id = entry.arguments?.getString("id") ?: return@composable
            HoraryDetailRoute(id, homeState, vm, navController, showToast)
        }
    }
}

@Composable
private fun ProfileEditRoute(
    editId: String?,
    homeState: FortuneHomeUiState,
    vm: FortuneViewModel,
    navController: NavHostController,
    showToast: (String) -> Unit
) {
    val existing = homeState.allProfiles.find { it.id == editId }
    ProfileEditScreen(
        initialProfile = existing,
        onBack = { navController.popBackStack() },
        onSave = { req -> vm.saveProfile(editId, req, { navController.popBackStack() }, showToast) }
    )
}

@Composable
private fun HoraryEditRoute(
    editId: String?,
    homeState: FortuneHomeUiState,
    vm: FortuneViewModel,
    navController: NavHostController,
    showToast: (String) -> Unit
) {
    val existing = homeState.horaryList.find { it.id == editId }
    HoraryEditScreen(
        initialHorary = existing,
        profiles = homeState.allProfiles,
        onBack = { navController.popBackStack() },
        onSave = { req -> vm.saveHorary(editId, req, { navController.popBackStack() }, showToast) }
    )
}

@Composable
private fun HoraryDetailRoute(
    id: String,
    homeState: FortuneHomeUiState,
    vm: FortuneViewModel,
    navController: NavHostController,
    showToast: (String) -> Unit
) {
    val horary = homeState.horaryList.find { it.id == id } ?: return
    val detailState by vm.horaryDetailState.collectAsState()

    LaunchedEffect(id, horary.updated_at) {
        vm.loadHoraryDetail(id)
    }

    HoraryDetailScreen(
        horary = horary,
        state = detailState,
        onRetry = { vm.loadHoraryDetail(id) },
        onBack = { navController.popBackStack() },
        onEdit = { navController.navigate("horary_edit?id=$id") },
        onDelete = { vm.deleteHorary(id, { navController.popBackStack() }, showToast) }
    )
}
