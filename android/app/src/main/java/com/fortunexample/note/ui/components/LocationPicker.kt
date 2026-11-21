package com.fortunexample.note.ui.components

import android.Manifest
import android.content.Context
import android.content.pm.PackageManager
import android.location.Location
import android.location.LocationManager
import android.os.Build
import android.os.CancellationSignal
import android.widget.Toast
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Close
import androidx.compose.material.icons.filled.GpsFixed
import androidx.compose.material.icons.filled.Search
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalSoftwareKeyboardController
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.core.content.ContextCompat
import androidx.core.location.LocationManagerCompat
import com.fortunexample.note.data.CachedLocation
import com.fortunexample.note.data.GeocodingCache
import com.fortunexample.note.ui.theme.*
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import java.util.Locale
import kotlin.math.abs

private const val LAST_KNOWN_MAX_AGE_MS = 5 * 60 * 1000L

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun LocationPicker(
    latitude: Double,
    longitude: Double,
    onLocationChanged: (Double, Double) -> Unit,
    modifier: Modifier = Modifier
) {
    val context = LocalContext.current
    val coroutineScope = rememberCoroutineScope()
    var showDialog by remember { mutableStateOf(false) }
    var showManualEdit by remember { mutableStateOf(false) }
    var isFetchingGps by remember { mutableStateOf(false) }

    var latStr by remember { mutableStateOf(String.format(Locale.US, "%.4f", latitude)) }
    var lonStr by remember { mutableStateOf(String.format(Locale.US, "%.4f", longitude)) }

    // Synchronize text fields only when coordinates change externally (search selection / GPS)
    LaunchedEffect(latitude) {
        val current = latStr.trim().toDoubleOrNull()
        if (current == null || abs(current - latitude) > 1e-6) {
            latStr = String.format(Locale.US, "%.4f", latitude)
        }
    }
    LaunchedEffect(longitude) {
        val current = lonStr.trim().toDoubleOrNull()
        if (current == null || abs(current - longitude) > 1e-6) {
            lonStr = String.format(Locale.US, "%.4f", longitude)
        }
    }

    val latParsed = latStr.trim().toDoubleOrNull()
    val lonParsed = lonStr.trim().toDoubleOrNull()
    val isLatInvalid = latStr.isNotBlank() && (latParsed == null || latParsed !in -90.0..90.0)
    val isLonInvalid = lonStr.isNotBlank() && (lonParsed == null || lonParsed !in -180.0..180.0)

    fun triggerGpsFetch() {
        isFetchingGps = true
        fetchCurrentGpsLocation(
            context = context,
            onSuccess = { lat, lon ->
                isFetchingGps = false
                onLocationChanged(lat, lon)
                Toast.makeText(
                    context,
                    "GPS 수신 성공: 위도 ${String.format(Locale.US, "%.4f", lat)}°, 경도 ${String.format(Locale.US, "%.4f", lon)}°",
                    Toast.LENGTH_SHORT
                ).show()
            },
            onError = { msg ->
                isFetchingGps = false
                Toast.makeText(context, msg, Toast.LENGTH_LONG).show()
            }
        )
    }

    val permissionLauncher = rememberLauncherForActivityResult(
        contract = ActivityResultContracts.RequestMultiplePermissions()
    ) { permissions ->
        val granted = permissions[Manifest.permission.ACCESS_FINE_LOCATION] == true ||
            permissions[Manifest.permission.ACCESS_COARSE_LOCATION] == true
        if (granted) {
            triggerGpsFetch()
        } else {
            Toast.makeText(context, "위치 권한이 거부되었습니다.", Toast.LENGTH_SHORT).show()
        }
    }

    Card(
        shape = RoundedCornerShape(12.dp),
        colors = CardDefaults.cardColors(containerColor = SurfaceLight),
        border = BorderStroke(1.dp, BorderColor),
        modifier = modifier.fillMaxWidth()
    ) {
        Column(modifier = Modifier.padding(16.dp)) {
            Text(
                text = "관측 위치",
                color = TextPrimary,
                fontWeight = FontWeight.Bold,
                fontSize = 15.sp
            )
            Spacer(modifier = Modifier.height(4.dp))
            Text(
                text = "현재 지정: 위도 ${String.format(Locale.US, "%.4f°", latitude)}, 경도 ${String.format(Locale.US, "%.4f°", longitude)}",
                color = TextSecondary,
                fontSize = 13.sp
            )

            Spacer(modifier = Modifier.height(12.dp))

            // 위치 검색 및 GPS 캡처 액션 버튼
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.spacedBy(8.dp)
            ) {
                OutlinedButton(
                    onClick = { showDialog = true },
                    modifier = Modifier.weight(1f),
                    colors = ButtonDefaults.outlinedButtonColors(containerColor = CardLight),
                    border = BorderStroke(1.dp, BorderColor),
                    shape = RoundedCornerShape(8.dp)
                ) {
                    Icon(Icons.Default.Search, contentDescription = null, tint = CelestialBlue, modifier = Modifier.size(18.dp))
                    Spacer(modifier = Modifier.width(6.dp))
                    Text("위치 검색", color = TextPrimary, fontSize = 13.sp)
                }

                OutlinedButton(
                    onClick = {
                        if (isFetchingGps) return@OutlinedButton
                        val fineGranted = ContextCompat.checkSelfPermission(context, Manifest.permission.ACCESS_FINE_LOCATION) == PackageManager.PERMISSION_GRANTED
                        val coarseGranted = ContextCompat.checkSelfPermission(context, Manifest.permission.ACCESS_COARSE_LOCATION) == PackageManager.PERMISSION_GRANTED
                        if (fineGranted || coarseGranted) {
                            triggerGpsFetch()
                        } else {
                            permissionLauncher.launch(
                                arrayOf(Manifest.permission.ACCESS_FINE_LOCATION, Manifest.permission.ACCESS_COARSE_LOCATION)
                            )
                        }
                    },
                    enabled = !isFetchingGps,
                    modifier = Modifier.weight(1f),
                    colors = ButtonDefaults.outlinedButtonColors(containerColor = CardLight),
                    border = BorderStroke(1.dp, BorderColor),
                    shape = RoundedCornerShape(8.dp)
                ) {
                    if (isFetchingGps) {
                        CircularProgressIndicator(
                            modifier = Modifier.size(16.dp),
                            color = VedicGreen,
                            strokeWidth = 2.dp
                        )
                    } else {
                        Icon(Icons.Default.GpsFixed, contentDescription = null, tint = VedicGreen, modifier = Modifier.size(18.dp))
                    }
                    Spacer(modifier = Modifier.width(6.dp))
                    Text(if (isFetchingGps) "수신 중..." else "현재 GPS", color = TextPrimary, fontSize = 13.sp)
                }
            }

            Spacer(modifier = Modifier.height(8.dp))

            // 수동 좌표 입력 접기/펼치기
            TextButton(
                onClick = { showManualEdit = !showManualEdit },
                contentPadding = PaddingValues(0.dp)
            ) {
                Text(
                    text = if (showManualEdit) "수동 좌표 입력 닫기 ▲" else "수동 좌표 직접 입력 ▼",
                    color = TextSecondary,
                    fontSize = 12.sp
                )
            }

            if (showManualEdit) {
                Spacer(modifier = Modifier.height(4.dp))
                Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                    OutlinedTextField(
                        value = latStr,
                        onValueChange = { raw ->
                            val filtered = raw.filter { it.isDigit() || it == '.' || it == '-' || it == '+' }
                            latStr = filtered
                            filtered.toDoubleOrNull()?.let { l ->
                                if (l in -90.0..90.0) {
                                    onLocationChanged(l, longitude)
                                }
                            }
                        },
                        label = { Text("위도 (-90~90)") },
                        isError = isLatInvalid,
                        supportingText = if (isLatInvalid) {
                            { Text("-90 ~ 90 범위 입력", fontSize = 11.sp) }
                        } else null,
                        keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Ascii, imeAction = ImeAction.Next),
                        modifier = Modifier.weight(1f),
                        singleLine = true
                    )
                    OutlinedTextField(
                        value = lonStr,
                        onValueChange = { raw ->
                            val filtered = raw.filter { it.isDigit() || it == '.' || it == '-' || it == '+' }
                            lonStr = filtered
                            filtered.toDoubleOrNull()?.let { l ->
                                if (l in -180.0..180.0) {
                                    onLocationChanged(latitude, l)
                                }
                            }
                        },
                        label = { Text("경도 (-180~180)") },
                        isError = isLonInvalid,
                        supportingText = if (isLonInvalid) {
                            { Text("-180 ~ 180 범위 입력", fontSize = 11.sp) }
                        } else null,
                        keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Ascii, imeAction = ImeAction.Done),
                        modifier = Modifier.weight(1f),
                        singleLine = true
                    )
                }
            }
        }
    }

    // 위치 검색 모달 다이얼로그
    if (showDialog) {
        val keyboardController = LocalSoftwareKeyboardController.current
        var query by remember { mutableStateOf("") }
        var isSearching by remember { mutableStateOf(false) }
        var locations by remember { mutableStateOf(emptyList<CachedLocation>()) }

        // 내장 오프라인 DB + 로컬 캐시 + 좌표 문자열 즉시 반영 (비동기 IO)
        LaunchedEffect(query) {
            locations = withContext(Dispatchers.IO) {
                GeocodingCache.searchLocal(context, query)
            }
        }

        fun executeOnlineSearch() {
            if (query.isBlank() || isSearching) return
            keyboardController?.hide()
            isSearching = true
            val searchTarget = query
            coroutineScope.launch {
                val onlineResult = GeocodingCache.searchOnline(context, searchTarget)
                val localResults = withContext(Dispatchers.IO) {
                    GeocodingCache.searchLocal(context, searchTarget)
                }
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
            onDismissRequest = { showDialog = false },
            containerColor = BackgroundWhite,
            title = {
                Text("위치 검색", color = TextPrimary, fontWeight = FontWeight.Bold)
            },
            text = {
                Column(modifier = Modifier.fillMaxWidth()) {
                    OutlinedTextField(
                        value = query,
                        onValueChange = { query = it },
                        placeholder = { Text("장소명(역·건물), 동·읍·면, 도로명 또는 좌표 입력", fontSize = 13.sp) },
                        leadingIcon = {
                            Icon(Icons.Default.Search, contentDescription = null, tint = TextSecondary)
                        },
                        trailingIcon = {
                            Row(verticalAlignment = Alignment.CenterVertically) {
                                if (query.isNotEmpty() && !isSearching) {
                                    IconButton(onClick = { query = "" }, modifier = Modifier.size(32.dp)) {
                                        Icon(Icons.Default.Close, contentDescription = "지우기", tint = TextSecondary, modifier = Modifier.size(18.dp))
                                    }
                                }
                                if (isSearching) {
                                    CircularProgressIndicator(
                                        modifier = Modifier
                                            .padding(end = 12.dp)
                                            .size(20.dp),
                                        color = CelestialBlue,
                                        strokeWidth = 2.dp
                                    )
                                } else if (query.isNotBlank()) {
                                    IconButton(onClick = { executeOnlineSearch() }) {
                                        Icon(Icons.Default.Search, contentDescription = "온라인 검색", tint = AccentPrimary)
                                    }
                                }
                            }
                        },
                        keyboardOptions = KeyboardOptions(imeAction = ImeAction.Search),
                        keyboardActions = KeyboardActions(onSearch = { executeOnlineSearch() }),
                        modifier = Modifier.fillMaxWidth(),
                        singleLine = true
                    )

                    Spacer(modifier = Modifier.height(12.dp))

                    if (locations.isEmpty() && !isSearching) {
                        Box(
                            modifier = Modifier
                                .fillMaxWidth()
                                .height(160.dp),
                            contentAlignment = Alignment.Center
                        ) {
                            Text(
                                text = "검색 버튼을 눌러 온라인으로 위치를 찾거나 좌표를 직접 붙여넣을 수 있습니다.",
                                color = TextSecondary,
                                fontSize = 13.sp
                            )
                        }
                    } else {
                        LazyColumn(
                            modifier = Modifier
                                .fillMaxWidth()
                                .height(280.dp)
                        ) {
                            items(locations) { loc ->
                                Column(
                                    modifier = Modifier
                                        .fillMaxWidth()
                                        .clickable {
                                            onLocationChanged(loc.latitude, loc.longitude)
                                            showDialog = false
                                            Toast.makeText(context, "${loc.displayName} 좌표가 적용되었습니다.", Toast.LENGTH_SHORT).show()
                                        }
                                        .padding(vertical = 10.dp, horizontal = 4.dp)
                                ) {
                                    Text(
                                        text = loc.displayName,
                                        color = TextPrimary,
                                        fontWeight = FontWeight.SemiBold,
                                        fontSize = 15.sp
                                    )
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
                }
            },
            confirmButton = {
                TextButton(onClick = { showDialog = false }) {
                    Text("닫기", color = TextPrimary)
                }
            }
        )
    }
}

fun fetchCurrentGpsLocation(
    context: Context,
    onSuccess: (Double, Double) -> Unit,
    onError: (String) -> Unit
) {
    val locationManager = context.getSystemService(Context.LOCATION_SERVICE) as? LocationManager
    if (locationManager == null) {
        onError("위치 서비스를 사용할 수 없습니다.")
        return
    }

    try {
        val now = System.currentTimeMillis()
        val candidateProviders = buildList {
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
                add(LocationManager.FUSED_PROVIDER)
            }
            add(LocationManager.GPS_PROVIDER)
            add(LocationManager.NETWORK_PROVIDER)
        }

        var bestCached: Location? = null
        for (provider in candidateProviders + LocationManager.PASSIVE_PROVIDER) {
            val loc = try {
                locationManager.getLastKnownLocation(provider)
            } catch (_: Exception) {
                null
            }
            if (loc != null && (bestCached == null || loc.time > bestCached.time)) {
                bestCached = loc
            }
        }

        // Use fresh cached location immediately if within 5 minutes
        if (bestCached != null && (now - bestCached.time) <= LAST_KNOWN_MAX_AGE_MS) {
            onSuccess(bestCached.latitude, bestCached.longitude)
            return
        }

        val activeProvider = candidateProviders.firstOrNull { provider ->
            try {
                locationManager.isProviderEnabled(provider)
            } catch (_: Exception) {
                false
            }
        }

        if (activeProvider == null) {
            if (bestCached != null) {
                onSuccess(bestCached.latitude, bestCached.longitude)
            } else {
                onError("위치 서비스(GPS/네트워크)가 비활성화되어 있습니다.")
            }
            return
        }

        LocationManagerCompat.getCurrentLocation(
            locationManager,
            activeProvider,
            CancellationSignal(),
            ContextCompat.getMainExecutor(context)
        ) { location ->
            when {
                location != null -> onSuccess(location.latitude, location.longitude)
                bestCached != null -> onSuccess(bestCached.latitude, bestCached.longitude)
                else -> onError("현재 GPS 위치를 조회할 수 없습니다. 잠시 후 다시 시도하십시오.")
            }
        }
    } catch (e: SecurityException) {
        onError("위치 권한 오류: ${e.localizedMessage}")
    } catch (e: Exception) {
        onError("위치 조회 오류: ${e.localizedMessage ?: "알 수 없는 오류"}")
    }
}
