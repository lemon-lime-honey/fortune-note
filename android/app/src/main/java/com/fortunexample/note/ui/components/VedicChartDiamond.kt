package com.fortunexample.note.ui.components

import androidx.compose.foundation.Canvas
import androidx.compose.foundation.layout.aspectRatio
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.unit.dp
import com.fortunexample.note.data.VedicChartData
import com.fortunexample.note.ui.theme.AccentGold
import com.fortunexample.note.ui.theme.BorderColor
import com.fortunexample.note.ui.theme.VedicGreen

@Composable
fun VedicChartDiamond(
    chart: VedicChartData,
    modifier: Modifier = Modifier
) {
    Canvas(
        modifier = modifier
            .fillMaxWidth()
            .aspectRatio(1f)
            .padding(16.dp)
    ) {
        val w = size.width
        val h = size.height

        // 1. 외곽 사각형
        drawRect(
            color = BorderColor,
            topLeft = Offset.Zero,
            size = size,
            style = Stroke(width = 2f)
        )

        // 2. 대각선 (X자)
        drawLine(color = BorderColor, start = Offset(0f, 0f), end = Offset(w, h), strokeWidth = 1.5f)
        drawLine(color = BorderColor, start = Offset(w, 0f), end = Offset(0f, h), strokeWidth = 1.5f)

        // 3. 내부 마름모 (다이아몬드)
        val topMid = Offset(w / 2f, 0f)
        val rightMid = Offset(w, h / 2f)
        val bottomMid = Offset(w / 2f, h)
        val leftMid = Offset(0f, h / 2f)

        drawLine(color = AccentGold, start = topMid, end = rightMid, strokeWidth = 2f)
        drawLine(color = AccentGold, start = rightMid, end = bottomMid, strokeWidth = 2f)
        drawLine(color = AccentGold, start = bottomMid, end = leftMid, strokeWidth = 2f)
        drawLine(color = AccentGold, start = leftMid, end = topMid, strokeWidth = 2f)

        // 4. 중심 1번 라그나 바바 중심점 강조
        drawCircle(color = VedicGreen, radius = 6f, center = Offset(w / 2f, h * 0.28f))
    }
}
