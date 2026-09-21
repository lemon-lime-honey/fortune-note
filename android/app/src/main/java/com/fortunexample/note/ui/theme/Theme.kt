package com.fortunexample.note.ui.theme

import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.lightColorScheme
import androidx.compose.runtime.Composable

private val LightColorScheme = lightColorScheme(
    primary = AccentPrimary,
    secondary = CelestialBlue,
    tertiary = VedicGreen,
    background = BackgroundWhite,
    surface = SurfaceLight,
    onPrimary = BackgroundWhite,
    onSecondary = BackgroundWhite,
    onBackground = TextPrimary,
    onSurface = TextPrimary
)

@Composable
fun FortuneNoteTheme(
    content: @Composable () -> Unit
) {
    MaterialTheme(
        colorScheme = LightColorScheme,
        content = content
    )
}
