package com.fortunexample.note.ui.components

import android.Manifest
import android.content.pm.PackageManager
import android.widget.Toast
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.GpsFixed
import androidx.compose.material.icons.filled.Search
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.core.content.ContextCompat
import com.fortunexample.note.ui.theme.*
import java.util.Locale
import kotlin.math.abs

@Composable
fun LocationPicker(
    latitude: Double,
    longitude: Double,
    onLocationChanged: (Double, Double) -> Unit,
    modifier: Modifier = Modifier,
    onValidationChanged: (Boolean) -> Unit = {}
) {
    val context = LocalContext.current
    var showDialog by remember { mutableStateOf(false) }
    var showManualEdit by remember { mutableStateOf(false) }
    var isFetchingGps by remember { mutableStateOf(false) }
    var activeGpsHandle by remember { mutableStateOf<GpsRequestHandle?>(null) }

    var latStr by remember { mutableStateOf(String.format(Locale.US, "%.4f", latitude)) }
    var lonStr by remember { mutableStateOf(String.format(Locale.US, "%.4f", longitude)) }

    DisposableEffect(Unit) {
        onDispose { activeGpsHandle?.cancel() }
    }

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

    val isLatInvalid = latStr.trim().toDoubleOrNull()?.let { it !in -90.0..90.0 } ?: true
    val isLonInvalid = lonStr.trim().toDoubleOrNull()?.let { it !in -180.0..180.0 } ?: true
    val isManualInputValid = !showManualEdit || (!isLatInvalid && !isLonInvalid)
    LaunchedEffect(isManualInputValid) { onValidationChanged(isManualInputValid) }

    fun triggerGpsFetch() {
        activeGpsHandle?.cancel()
        isFetchingGps = true
        activeGpsHandle = fetchCurrentGpsLocation(
            context = context,
            onSuccess = { lat, lon ->
                activeGpsHandle = null
                isFetchingGps = false
                onLocationChanged(lat, lon)
                val msg = "GPS 수신 성공: 위도 ${String.format(Locale.US, "%.4f", lat)}°, 경도 ${String.format(Locale.US, "%.4f", lon)}°"
                Toast.makeText(context, msg, Toast.LENGTH_SHORT).show()
            },
            onError = { msg ->
                activeGpsHandle = null
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
        if (granted) triggerGpsFetch()
        else Toast.makeText(context, "위치 권한이 거부되었습니다.", Toast.LENGTH_SHORT).show()
    }

    Card(
        shape = RoundedCornerShape(12.dp),
        colors = CardDefaults.cardColors(containerColor = SurfaceLight),
        border = BorderStroke(1.dp, BorderColor),
        modifier = modifier.fillMaxWidth()
    ) {
        Column(modifier = Modifier.padding(16.dp)) {
            Text("관측 위치", color = TextPrimary, fontWeight = FontWeight.Bold, fontSize = 15.sp)
            Spacer(modifier = Modifier.height(4.dp))
            Text(
                text = "현재 지정: 위도 ${String.format(Locale.US, "%.4f°", latitude)}, 경도 ${String.format(Locale.US, "%.4f°", longitude)}",
                color = TextSecondary,
                fontSize = 13.sp
            )
            Spacer(modifier = Modifier.height(12.dp))

            LocationActionButtons(
                isFetchingGps = isFetchingGps,
                onSearchClick = { showDialog = true },
                onGpsClick = {
                    if (isFetchingGps) return@LocationActionButtons
                    val fine = ContextCompat.checkSelfPermission(context, Manifest.permission.ACCESS_FINE_LOCATION) == PackageManager.PERMISSION_GRANTED
                    val coarse = ContextCompat.checkSelfPermission(context, Manifest.permission.ACCESS_COARSE_LOCATION) == PackageManager.PERMISSION_GRANTED
                    if (fine || coarse) triggerGpsFetch()
                    else permissionLauncher.launch(arrayOf(Manifest.permission.ACCESS_FINE_LOCATION, Manifest.permission.ACCESS_COARSE_LOCATION))
                }
            )

            Spacer(modifier = Modifier.height(8.dp))
            TextButton(
                onClick = {
                    showManualEdit = !showManualEdit
                    if (!showManualEdit) {
                        latStr = String.format(Locale.US, "%.4f", latitude)
                        lonStr = String.format(Locale.US, "%.4f", longitude)
                    }
                },
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
                ManualCoordinateInputs(
                    latStr = latStr,
                    lonStr = lonStr,
                    isLatInvalid = isLatInvalid,
                    isLonInvalid = isLonInvalid,
                    onLatChange = { filtered ->
                        latStr = filtered
                        filtered.toDoubleOrNull()?.takeIf { it in -90.0..90.0 }?.let { onLocationChanged(it, longitude) }
                    },
                    onLonChange = { filtered ->
                        lonStr = filtered
                        filtered.toDoubleOrNull()?.takeIf { it in -180.0..180.0 }?.let { onLocationChanged(latitude, it) }
                    }
                )
            }
        }
    }

    if (showDialog) {
        LocationSearchDialog(
            onLocationSelected = { lat, lon ->
                onLocationChanged(lat, lon)
                showDialog = false
            },
            onDismiss = { showDialog = false }
        )
    }
}

@Composable
private fun LocationActionButtons(
    isFetchingGps: Boolean,
    onSearchClick: () -> Unit,
    onGpsClick: () -> Unit
) {
    Row(modifier = Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.spacedBy(8.dp)) {
        OutlinedButton(
            onClick = onSearchClick,
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
            onClick = onGpsClick,
            enabled = !isFetchingGps,
            modifier = Modifier.weight(1f),
            colors = ButtonDefaults.outlinedButtonColors(containerColor = CardLight),
            border = BorderStroke(1.dp, BorderColor),
            shape = RoundedCornerShape(8.dp)
        ) {
            if (isFetchingGps) {
                CircularProgressIndicator(modifier = Modifier.size(16.dp), color = VedicGreen, strokeWidth = 2.dp)
            } else {
                Icon(Icons.Default.GpsFixed, contentDescription = null, tint = VedicGreen, modifier = Modifier.size(18.dp))
            }
            Spacer(modifier = Modifier.width(6.dp))
            Text(if (isFetchingGps) "수신 중..." else "현재 GPS", color = TextPrimary, fontSize = 13.sp)
        }
    }
}

@Composable
private fun ManualCoordinateInputs(
    latStr: String,
    lonStr: String,
    isLatInvalid: Boolean,
    isLonInvalid: Boolean,
    onLatChange: (String) -> Unit,
    onLonChange: (String) -> Unit
) {
    Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
        CoordinateInputField(
            value = latStr,
            label = "위도 (-90~90)",
            errorHint = "-90 ~ 90 범위 입력",
            isError = isLatInvalid,
            imeAction = ImeAction.Next,
            onFilteredChange = onLatChange,
            modifier = Modifier.weight(1f)
        )
        CoordinateInputField(
            value = lonStr,
            label = "경도 (-180~180)",
            errorHint = "-180 ~ 180 범위 입력",
            isError = isLonInvalid,
            imeAction = ImeAction.Done,
            onFilteredChange = onLonChange,
            modifier = Modifier.weight(1f)
        )
    }
}

@Composable
private fun CoordinateInputField(
    value: String,
    label: String,
    errorHint: String,
    isError: Boolean,
    imeAction: ImeAction,
    onFilteredChange: (String) -> Unit,
    modifier: Modifier = Modifier
) {
    OutlinedTextField(
        value = value,
        onValueChange = { raw ->
            onFilteredChange(raw.filter { it.isDigit() || it == '.' || it == '-' || it == '+' })
        },
        label = { Text(label) },
        isError = isError,
        supportingText = if (isError) { { Text(errorHint, fontSize = 11.sp) } } else null,
        keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Ascii, imeAction = imeAction),
        modifier = modifier,
        singleLine = true
    )
}
