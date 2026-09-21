package com.fortunexample.note.ui.components

import android.Manifest
import android.content.Context
import android.content.pm.PackageManager
import android.location.LocationManager
import android.widget.Toast
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.icons.Icons
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
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.core.content.ContextCompat
import com.fortunexample.note.data.CachedLocation
import com.fortunexample.note.data.GeocodingCache
import com.fortunexample.note.ui.theme.*
import kotlinx.coroutines.launch

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

    var latStr by remember(latitude) { mutableStateOf(String.format("%.4f", latitude)) }
    var lonStr by remember(longitude) { mutableStateOf(String.format("%.4f", longitude)) }

    val permissionLauncher = rememberLauncherForActivityResult(
        contract = ActivityResultContracts.RequestMultiplePermissions()
    ) { permissions ->
        val granted = permissions[Manifest.permission.ACCESS_FINE_LOCATION] == true ||
                      permissions[Manifest.permission.ACCESS_COARSE_LOCATION] == true
        if (granted) {
            fetchGpsLocation(context, onLocationChanged)
        } else {
            Toast.makeText(context, "위치 권한이 거부되었습니다.", Toast.LENGTH_SHORT).show()
        }
    }

    Card(
        shape = RoundedCornerShape(12.dp),
        colors = CardDefaults.cardColors(containerColor = SurfaceDark),
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
                text = "현재 지정: 위도 ${String.format("%.4f°", latitude)}, 경도 ${String.format("%.4f°", longitude)}",
                color = AccentGold,
                fontSize = 13.sp
            )

            Spacer(modifier = Modifier.height(12.dp))

            // 위치 검색 및 GPS 캡처 액션 버튼
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.spacedBy(8.dp)
            ) {
                Button(
                    onClick = { showDialog = true },
                    modifier = Modifier.weight(1f),
                    colors = ButtonDefaults.buttonColors(containerColor = CardDark),
                    shape = RoundedCornerShape(8.dp)
                ) {
                    Icon(Icons.Default.Search, contentDescription = null, tint = CelestialBlue, modifier = Modifier.size(18.dp))
                    Spacer(modifier = Modifier.width(6.dp))
                    Text("위치 검색", color = TextPrimary, fontSize = 13.sp)
                }

                Button(
                    onClick = {
                        val fineGranted = ContextCompat.checkSelfPermission(context, Manifest.permission.ACCESS_FINE_LOCATION) == PackageManager.PERMISSION_GRANTED
                        val coarseGranted = ContextCompat.checkSelfPermission(context, Manifest.permission.ACCESS_COARSE_LOCATION) == PackageManager.PERMISSION_GRANTED
                        if (fineGranted || coarseGranted) {
                            fetchGpsLocation(context, onLocationChanged)
                        } else {
                            permissionLauncher.launch(
                                arrayOf(Manifest.permission.ACCESS_FINE_LOCATION, Manifest.permission.ACCESS_COARSE_LOCATION)
                            )
                        }
                    },
                    modifier = Modifier.weight(1f),
                    colors = ButtonDefaults.buttonColors(containerColor = CardDark),
                    shape = RoundedCornerShape(8.dp)
                ) {
                    Icon(Icons.Default.GpsFixed, contentDescription = null, tint = VedicGreen, modifier = Modifier.size(18.dp))
                    Spacer(modifier = Modifier.width(6.dp))
                    Text("현재 GPS", color = TextPrimary, fontSize = 13.sp)
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
                        onValueChange = {
                            latStr = it
                            it.toDoubleOrNull()?.let { l -> onLocationChanged(l, longitude) }
                        },
                        label = { Text("위도") },
                        modifier = Modifier.weight(1f),
                        singleLine = true
                    )
                    OutlinedTextField(
                        value = lonStr,
                        onValueChange = {
                            lonStr = it
                            it.toDoubleOrNull()?.let { l -> onLocationChanged(latitude, l) }
                        },
                        label = { Text("경도") },
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

        // 로컬 캐시 즉시 반영 (오프라인 0바이트)
        LaunchedEffect(query) {
            locations = GeocodingCache.searchLocal(context, query)
        }

        fun executeOnlineSearch() {
            if (query.isBlank() || isSearching) return
            keyboardController?.hide()
            isSearching = true
            coroutineScope.launch {
                val results = GeocodingCache.searchOnline(context, query)
                locations = GeocodingCache.searchLocal(context, query)
                isSearching = false
                if (results.isEmpty() && locations.isEmpty()) {
                    Toast.makeText(context, "검색 결과가 없습니다.", Toast.LENGTH_SHORT).show()
                }
            }
        }

        AlertDialog(
            onDismissRequest = { showDialog = false },
            containerColor = SurfaceDark,
            title = {
                Text("위치 검색", color = TextPrimary, fontWeight = FontWeight.Bold)
            },
            text = {
                Column(modifier = Modifier.fillMaxWidth()) {
                    OutlinedTextField(
                        value = query,
                        onValueChange = { query = it },
                        placeholder = { Text("동, 읍, 면 또는 도로명 입력") },
                        leadingIcon = {
                            Icon(Icons.Default.Search, contentDescription = null, tint = TextSecondary)
                        },
                        trailingIcon = {
                            if (isSearching) {
                                CircularProgressIndicator(
                                    modifier = Modifier.size(20.dp),
                                    color = CelestialBlue,
                                    strokeWidth = 2.dp
                                )
                            } else if (query.isNotBlank()) {
                                IconButton(onClick = { executeOnlineSearch() }) {
                                    Icon(Icons.Default.Search, contentDescription = "검색", tint = AccentGold)
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
                                text = "검색 버튼을 눌러 온라인으로 위치를 찾을 수 있습니다.",
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
                                        text = "위도 ${String.format("%.4f", loc.latitude)}°, 경도 ${String.format("%.4f", loc.longitude)}°",
                                        color = TextSecondary,
                                        fontSize = 12.sp
                                    )
                                }
                                HorizontalDivider(color = BorderColor.copy(alpha = 0.5f))
                            }
                        }
                    }
                }
            },
            confirmButton = {
                TextButton(onClick = { showDialog = false }) {
                    Text("닫기", color = AccentGold)
                }
            }
        )
    }
}

private fun fetchGpsLocation(
    context: Context,
    onLocationChanged: (Double, Double) -> Unit
) {
    val locationManager = context.getSystemService(Context.LOCATION_SERVICE) as? LocationManager
    if (locationManager == null) {
        Toast.makeText(context, "위치 서비스를 사용할 수 없습니다.", Toast.LENGTH_SHORT).show()
        return
    }

    try {
        val providers = listOf(LocationManager.GPS_PROVIDER, LocationManager.NETWORK_PROVIDER)
        for (provider in providers) {
            if (locationManager.isProviderEnabled(provider)) {
                val loc = locationManager.getLastKnownLocation(provider)
                if (loc != null) {
                    onLocationChanged(loc.latitude, loc.longitude)
                    Toast.makeText(
                        context,
                        "GPS 수신 성공: 위도 ${String.format("%.4f", loc.latitude)}°, 경도 ${String.format("%.4f", loc.longitude)}°",
                        Toast.LENGTH_SHORT
                    ).show()
                    return
                }
            }
        }
        Toast.makeText(context, "최근 GPS 위치를 조회할 수 없습니다. 잠시 후 다시 시도하십시오.", Toast.LENGTH_LONG).show()
    } catch (e: SecurityException) {
        Toast.makeText(context, "위치 권한 오류: ${e.localizedMessage}", Toast.LENGTH_SHORT).show()
    }
}
