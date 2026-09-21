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
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.GpsFixed
import androidx.compose.material.icons.filled.LocationCity
import androidx.compose.material.icons.filled.Search
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.core.content.ContextCompat
import com.fortunexample.note.data.CityDatabase
import com.fortunexample.note.data.CityLocation
import com.fortunexample.note.ui.theme.*

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun LocationPicker(
    latitude: Double,
    longitude: Double,
    onLocationChanged: (Double, Double) -> Unit,
    modifier: Modifier = Modifier
) {
    val context = LocalContext.current
    var showCityDialog by remember { mutableStateOf(false) }
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
                text = "관측 위치 (좌표)",
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

            // 도시 검색 및 GPS 캡처 액션 버튼
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.spacedBy(8.dp)
            ) {
                Button(
                    onClick = { showCityDialog = true },
                    modifier = Modifier.weight(1f),
                    colors = ButtonDefaults.buttonColors(containerColor = CardDark),
                    shape = RoundedCornerShape(8.dp)
                ) {
                    Icon(Icons.Default.LocationCity, contentDescription = null, tint = CelestialBlue, modifier = Modifier.size(18.dp))
                    Spacer(modifier = Modifier.width(6.dp))
                    Text("도시 검색", color = TextPrimary, fontSize = 13.sp)
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

            // 수동 미세 조정 접기/펼치기 버튼
            TextButton(
                onClick = { showManualEdit = !showManualEdit },
                contentPadding = PaddingValues(0.dp)
            ) {
                Text(
                    text = if (showManualEdit) "수동 미세 조정 닫기 ▲" else "수동으로 위경도 숫자 직접 수정 ▼",
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

    // 도시 검색 모달 다이얼로그
    if (showCityDialog) {
        var query by remember { mutableStateOf("") }
        val searchResults by remember(query) { derivedStateOf { CityDatabase.search(query) } }

        AlertDialog(
            onDismissRequest = { showCityDialog = false },
            containerColor = SurfaceDark,
            title = {
                Text("출생 / 관측 도시 선택", color = TextPrimary, fontWeight = FontWeight.Bold)
            },
            text = {
                Column(modifier = Modifier.fillMaxWidth()) {
                    OutlinedTextField(
                        value = query,
                        onValueChange = { query = it },
                        placeholder = { Text("도시 또는 구 이름 (예: 강남, 수원, 부산, 도쿄)") },
                        leadingIcon = { Icon(Icons.Default.Search, contentDescription = null, tint = TextSecondary) },
                        modifier = Modifier.fillMaxWidth(),
                        singleLine = true
                    )
                    Spacer(modifier = Modifier.height(12.dp))
                    LazyColumn(
                        modifier = Modifier
                            .fillMaxWidth()
                            .height(280.dp)
                    ) {
                        items(searchResults) { city ->
                            Column(
                                modifier = Modifier
                                    .fillMaxWidth()
                                    .clickable {
                                        onLocationChanged(city.latitude, city.longitude)
                                        showCityDialog = false
                                        Toast.makeText(context, "${city.displayName} 좌표가 적용되었습니다.", Toast.LENGTH_SHORT).show()
                                    }
                                    .padding(vertical = 10.dp, horizontal = 4.dp)
                            ) {
                                Text(text = city.displayName, color = TextPrimary, fontWeight = FontWeight.SemiBold, fontSize = 15.sp)
                                Text(
                                    text = "위도 ${String.format("%.4f", city.latitude)}°, 경도 ${String.format("%.4f", city.longitude)}°",
                                    color = TextSecondary,
                                    fontSize = 12.sp
                                )
                            }
                            HorizontalDivider(color = BorderColor.copy(alpha = 0.5f))
                        }
                    }
                }
            },
            confirmButton = {
                TextButton(onClick = { showCityDialog = false }) {
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
