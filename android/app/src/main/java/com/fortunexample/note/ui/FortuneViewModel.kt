package com.fortunexample.note.ui

import androidx.lifecycle.ViewModel
import androidx.lifecycle.ViewModelProvider
import androidx.lifecycle.viewModelScope
import com.fortunexample.note.data.*
import com.fortunexample.note.ui.screens.HoraryDetailUiState
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

data class FortuneHomeUiState(
    val allProfiles: List<ProfileDto> = emptyList(),
    val horaryList: List<HoraryDto> = emptyList(),
    val searchQuery: String = ""
) {
    val displayedProfiles: List<ProfileDto>
        get() {
            val q = searchQuery.trim()
            if (q.isEmpty()) return allProfiles
            return allProfiles.filter {
                it.name.contains(q, ignoreCase = true) || it.notes.contains(q, ignoreCase = true)
            }
        }

    val displayedHorary: List<HoraryDto>
        get() {
            val q = searchQuery.trim()
            if (q.isEmpty()) return horaryList
            val nameById = allProfiles.associate { it.id to it.name }
            return horaryList.filter { item ->
                item.title.contains(q, ignoreCase = true) ||
                    item.category.contains(q, ignoreCase = true) ||
                    item.question_text.contains(q, ignoreCase = true) ||
                    item.result_notes.contains(q, ignoreCase = true) ||
                    (item.querent_profile_id?.let { nameById[it]?.contains(q, ignoreCase = true) } == true)
            }
        }
}

class FortuneViewModel(
    private val repository: FortuneRepository
) : ViewModel() {
    val profileDataSource: ProfileDataSource get() = repository.profiles
    val backupDataSource: BackupDataSource get() = repository.backup

    private val _homeState = MutableStateFlow(FortuneHomeUiState())
    val homeState: StateFlow<FortuneHomeUiState> = _homeState.asStateFlow()

    private val _horaryDetailState = MutableStateFlow(HoraryDetailUiState())
    val horaryDetailState: StateFlow<HoraryDetailUiState> = _horaryDetailState.asStateFlow()

    init {
        refreshAll()
    }

    fun refreshAll() {
        viewModelScope.launch {
            repository.profiles.listProfiles().onSuccess { profiles ->
                _homeState.update { it.copy(allProfiles = profiles) }
            }
            repository.horary.listHorary().onSuccess { horaries ->
                _homeState.update { it.copy(horaryList = horaries) }
            }
        }
    }

    fun updateSearchQuery(query: String) {
        _homeState.update { it.copy(searchQuery = query) }
    }

    fun saveProfile(
        editId: String?,
        request: CreateProfileRequest,
        onSuccess: () -> Unit,
        onError: (String) -> Unit
    ) {
        viewModelScope.launch {
            val result = if (editId != null) {
                repository.profiles.updateProfile(editId, request)
            } else {
                repository.profiles.createProfile(request)
            }
            result.onSuccess {
                refreshAll()
                onSuccess()
            }.onFailure { err ->
                val fallback = if (editId != null) "인물 정보 수정에 실패했습니다." else "인물 등록에 실패했습니다."
                onError(err.message ?: fallback)
            }
        }
    }

    fun deleteProfile(id: String, onSuccess: () -> Unit, onError: (String) -> Unit) {
        viewModelScope.launch {
            repository.profiles.deleteProfile(id).onSuccess {
                refreshAll()
                onSuccess()
            }.onFailure { err ->
                onError(err.message ?: "인물 삭제에 실패했습니다.")
            }
        }
    }

    fun saveHorary(
        editId: String?,
        request: CreateHoraryRequest,
        onSuccess: () -> Unit,
        onError: (String) -> Unit
    ) {
        viewModelScope.launch {
            val result = if (editId != null) {
                repository.horary.updateHorary(editId, request)
            } else {
                repository.horary.createHorary(request)
            }
            result.onSuccess {
                refreshAll()
                onSuccess()
            }.onFailure { err ->
                val fallback = if (editId != null) "호라리 질문 수정에 실패했습니다." else "호라리 질문 등록에 실패했습니다."
                onError(err.message ?: fallback)
            }
        }
    }

    fun deleteHorary(id: String, onSuccess: () -> Unit, onError: (String) -> Unit) {
        viewModelScope.launch {
            repository.horary.deleteHorary(id).onSuccess {
                refreshAll()
                onSuccess()
            }.onFailure { err ->
                onError(err.message ?: "호라리 질문 삭제에 실패했습니다.")
            }
        }
    }

    fun loadHoraryDetail(id: String) {
        viewModelScope.launch {
            _horaryDetailState.value = HoraryDetailUiState(isLoading = true)
            val chartResult = repository.horary.calculateChart(id)
            chartResult.onFailure { err ->
                _horaryDetailState.value = HoraryDetailUiState(
                    isLoading = false,
                    errorMessage = err.message ?: "호라리 차트 계산에 실패했습니다."
                )
                return@launch
            }
            val westernMd = repository.horary.exportWesternMarkdown(id).getOrNull()
            val prashnaMd = repository.horary.exportPrashnaMarkdown(id).getOrNull()
            _horaryDetailState.value = HoraryDetailUiState(
                chart = chartResult.getOrNull(),
                westernMarkdown = westernMd,
                prashnaMarkdown = prashnaMd,
                isLoading = false,
                errorMessage = null
            )
        }
    }

    companion object {
        fun factory(repository: FortuneRepository): ViewModelProvider.Factory =
            object : ViewModelProvider.Factory {
                @Suppress("UNCHECKED_CAST")
                override fun <T : ViewModel> create(modelClass: Class<T>): T {
                    return FortuneViewModel(repository) as T
                }
            }
    }
}
