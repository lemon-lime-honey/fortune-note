package com.fortunexample.note.ui.components

import android.content.Context
import android.location.Location
import android.location.LocationManager
import android.os.Build
import android.os.CancellationSignal
import android.os.Handler
import android.os.Looper
import androidx.core.content.ContextCompat
import androidx.core.location.LocationManagerCompat
import java.util.concurrent.atomic.AtomicBoolean

private const val LAST_KNOWN_MAX_AGE_MS = 5 * 60 * 1000L
private const val GPS_TIMEOUT_MS = 15_000L

class GpsRequestHandle(
    private val signal: CancellationSignal,
    private val cleanup: () -> Unit
) {
    fun cancel() {
        cleanup()
        signal.cancel()
    }
}

fun fetchCurrentGpsLocation(
    context: Context,
    onSuccess: (Double, Double) -> Unit,
    onError: (String) -> Unit
): GpsRequestHandle? {
    val locationManager = context.getSystemService(Context.LOCATION_SERVICE) as? LocationManager
    if (locationManager == null) {
        onError("위치 서비스를 사용할 수 없습니다.")
        return null
    }
    return try {
        val providers = candidateLocationProviders()
        val bestCached = findBestCachedLocation(locationManager, providers)
        if (bestCached != null && (System.currentTimeMillis() - bestCached.time) <= LAST_KNOWN_MAX_AGE_MS) {
            onSuccess(bestCached.latitude, bestCached.longitude)
            return null
        }
        val activeProvider = resolveActiveProvider(locationManager, providers)
        if (activeProvider == null) {
            if (bestCached != null) onSuccess(bestCached.latitude, bestCached.longitude)
            else onError("위치 서비스(GPS/네트워크)가 비활성화되어 있습니다.")
            return null
        }
        requestLiveLocationWithTimeout(context, locationManager, activeProvider, bestCached, onSuccess, onError)
    } catch (e: SecurityException) {
        onError("위치 권한 오류: ${e.localizedMessage}")
        null
    } catch (e: Exception) {
        onError("위치 조회 오류: ${e.localizedMessage ?: "알 수 없는 오류"}")
        null
    }
}

private fun candidateLocationProviders(): List<String> = buildList {
    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
        add(LocationManager.FUSED_PROVIDER)
    }
    add(LocationManager.GPS_PROVIDER)
    add(LocationManager.NETWORK_PROVIDER)
}

private fun findBestCachedLocation(locationManager: LocationManager, providers: List<String>): Location? {
    var bestCached: Location? = null
    for (provider in providers + LocationManager.PASSIVE_PROVIDER) {
        val loc = try {
            locationManager.getLastKnownLocation(provider)
        } catch (_: Exception) {
            null
        }
        if (loc != null && (bestCached == null || loc.time > bestCached.time)) {
            bestCached = loc
        }
    }
    return bestCached
}

private fun resolveActiveProvider(locationManager: LocationManager, providers: List<String>): String? =
    providers.firstOrNull { provider ->
        try {
            locationManager.isProviderEnabled(provider)
        } catch (_: Exception) {
            false
        }
    }

private fun requestLiveLocationWithTimeout(
    context: Context,
    locationManager: LocationManager,
    provider: String,
    bestCached: Location?,
    onSuccess: (Double, Double) -> Unit,
    onError: (String) -> Unit
): GpsRequestHandle {
    val settled = AtomicBoolean(false)
    val signal = CancellationSignal()
    val mainHandler = Handler(Looper.getMainLooper())
    val timeoutTask = Runnable {
        if (signal.isCanceled || !settled.compareAndSet(false, true)) return@Runnable
        signal.cancel()
        if (bestCached != null) onSuccess(bestCached.latitude, bestCached.longitude)
        else onError("GPS 응답 시간이 초과되었습니다. 실외에서 다시 시도하거나 위치 검색을 이용해 주세요.")
    }
    mainHandler.postDelayed(timeoutTask, GPS_TIMEOUT_MS)

    LocationManagerCompat.getCurrentLocation(
        locationManager,
        provider,
        signal,
        ContextCompat.getMainExecutor(context)
    ) { location ->
        if (signal.isCanceled || !settled.compareAndSet(false, true)) return@getCurrentLocation
        mainHandler.removeCallbacks(timeoutTask)
        when {
            location != null -> onSuccess(location.latitude, location.longitude)
            bestCached != null -> onSuccess(bestCached.latitude, bestCached.longitude)
            else -> onError("현재 GPS 위치를 조회할 수 없습니다. 잠시 후 다시 시도하십시오.")
        }
    }

    return GpsRequestHandle(signal) {
        settled.set(true)
        mainHandler.removeCallbacks(timeoutTask)
    }
}
