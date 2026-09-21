package com.fortunexample.note.ui.theme

import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.darkColorScheme
import androidx.compose.runtime.Composable

private val DarkColorScheme = darkColorScheme(
    primary = AccentGold,
    secondary = CelestialBlue,
    tertiary = VedicGreen,
    background = DeepNavy,
    surface = SurfaceDark,
    onPrimary = DeepNavy,
    onSecondary = DeepNavy,
    onBackground = TextPrimary,
    onSurface = TextPrimary
)

@Composable
fun FortuneNoteTheme(
    content: @Composable () -> Unit
) {
    MaterialTheme(
        colorScheme = DarkColorScheme,
        content = content
    )
}
