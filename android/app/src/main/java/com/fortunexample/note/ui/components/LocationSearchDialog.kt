package com.fortunexample.note.ui.components

import android.widget.Toast
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Close
import androidx.compose.material.icons.filled.Search
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalSoftwareKeyboardController
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.fortunexample.note.data.CachedLocation
import com.fortunexample.note.data.GeocodingCache
import com.fortunexample.note.ui.theme.*
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job
import kotlinx.coroutines.currentCoroutineContext
import kotlinx.coroutines.ensureActive
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import java.util.Locale

@Composable
internal fun LocationSearchDialog(
    onLocationSelected: (Double, Double) -> Unit,
    onDismiss: () -> Unit
) {
    val context = LocalContext.current
    val keyboardController = LocalSoftwareKeyboardController.current
    val dialogScope = rememberCoroutineScope()

    var query by remember { mutableStateOf("") }
    var isSearching by remember { mutableStateOf(false) }
    var locations by remember { mutableStateOf(emptyList<CachedLocation>()) }
    var searchSeq by remember { mutableIntStateOf(0) }
    var activeOnlineJob by remember { mutableStateOf<Job?>(null) }

    LaunchedEffect(query) {
        searchSeq += 1
        activeOnlineJob?.cancel()
        isSearching = false
        val local = withContext(Dispatchers.IO) { GeocodingCache.searchLocal(context, query) }
        currentCoroutineContext().ensureActive()
        locations = local
    }

    fun executeOnlineSearch() {
        if (query.isBlank() || isSearching) return
        keyboardController?.hide()
        val requestSeq = ++searchSeq
        val searchTarget = query
        isSearching = true
        activeOnlineJob?.cancel()
        activeOnlineJob = dialogScope.launch {
            val onlineResult = GeocodingCache.searchOnline(context, searchTarget)
            val localResults = withContext(Dispatchers.IO) { GeocodingCache.searchLocal(context, searchTarget) }
            currentCoroutineContext().ensureActive()
            if (requestSeq != searchSeq) return@launch
            onlineResult
                .onSuccess { fetched ->
                    locations = GeocodingCache.mergeLocations(fetched, localResults)
                    if (fetched.isEmpty() && locations.isEmpty()) {
                        Toast.makeText(context, "검색 결과가 없습니다.", Toast.LENGTH_SHORT).show()
                    }
                }
                .onFailure {
                    locations = localResults
                    Toast.makeText(
                        context,
                        "온라인 위치 검색에 실패했습니다. 네트워크 연결을 확인해 주세요.",
                        Toast.LENGTH_SHORT
                    ).show()
                }
            isSearching = false
        }
    }

    AlertDialog(
        onDismissRequest = onDismiss,
        containerColor = BackgroundWhite,
        title = { Text("위치 검색", color = TextPrimary, fontWeight = FontWeight.Bold) },
        text = {
            Column(modifier = Modifier.fillMaxWidth()) {
                LocationSearchField(
                    query = query,
                    isSearching = isSearching,
                    onQueryChange = { query = it },
                    onClear = { query = "" },
                    onSearch = ::executeOnlineSearch
                )
                Spacer(modifier = Modifier.height(12.dp))
                LocationResultsList(
                    locations = locations,
                    isSearching = isSearching,
                    onSelect = { loc ->
                        onLocationSelected(loc.latitude, loc.longitude)
                        Toast.makeText(context, "${loc.displayName} 좌표가 적용되었습니다.", Toast.LENGTH_SHORT).show()
                    }
                )
            }
        },
        confirmButton = {
            TextButton(onClick = onDismiss) { Text("닫기", color = TextPrimary) }
        }
    )
}

@Composable
private fun LocationSearchField(
    query: String,
    isSearching: Boolean,
    onQueryChange: (String) -> Unit,
    onClear: () -> Unit,
    onSearch: () -> Unit
) {
    OutlinedTextField(
        value = query,
        onValueChange = onQueryChange,
        placeholder = { Text("장소명(역·건물), 동·읍·면, 도로명 또는 좌표 입력", fontSize = 13.sp) },
        leadingIcon = { Icon(Icons.Default.Search, contentDescription = null, tint = TextSecondary) },
        trailingIcon = {
            Row(verticalAlignment = Alignment.CenterVertically) {
                if (query.isNotEmpty() && !isSearching) {
                    IconButton(onClick = onClear, modifier = Modifier.size(32.dp)) {
                        Icon(Icons.Default.Close, contentDescription = "지우기", tint = TextSecondary, modifier = Modifier.size(18.dp))
                    }
                }
                if (isSearching) {
                    CircularProgressIndicator(
                        modifier = Modifier.padding(end = 12.dp).size(20.dp),
                        color = CelestialBlue,
                        strokeWidth = 2.dp
                    )
                } else if (query.isNotBlank()) {
                    IconButton(onClick = onSearch) {
                        Icon(Icons.Default.Search, contentDescription = "온라인 검색", tint = AccentPrimary)
                    }
                }
            }
        },
        keyboardOptions = KeyboardOptions(imeAction = ImeAction.Search),
        keyboardActions = KeyboardActions(onSearch = { onSearch() }),
        modifier = Modifier.fillMaxWidth(),
        singleLine = true
    )
}

@Composable
private fun LocationResultsList(
    locations: List<CachedLocation>,
    isSearching: Boolean,
    onSelect: (CachedLocation) -> Unit
) {
    if (locations.isEmpty() && !isSearching) {
        Box(
            modifier = Modifier.fillMaxWidth().height(160.dp),
            contentAlignment = Alignment.Center
        ) {
            Text(
                text = "검색 버튼을 눌러 온라인으로 위치를 찾거나 좌표를 직접 붙여넣을 수 있습니다.",
                color = TextSecondary,
                fontSize = 13.sp
            )
        }
        return
    }
    LazyColumn(modifier = Modifier.fillMaxWidth().height(280.dp)) {
        items(locations) { loc ->
            Column(
                modifier = Modifier
                    .fillMaxWidth()
                    .clickable { onSelect(loc) }
                    .padding(vertical = 10.dp, horizontal = 4.dp)
            ) {
                Text(text = loc.displayName, color = TextPrimary, fontWeight = FontWeight.SemiBold, fontSize = 15.sp)
                Spacer(modifier = Modifier.height(2.dp))
                Text(
                    text = "위도 ${String.format(Locale.US, "%.4f", loc.latitude)}°, 경도 ${String.format(Locale.US, "%.4f", loc.longitude)}°",
                    color = TextSecondary,
                    fontSize = 12.sp
                )
            }
            HorizontalDivider(color = BorderColor)
        }
    }
}
