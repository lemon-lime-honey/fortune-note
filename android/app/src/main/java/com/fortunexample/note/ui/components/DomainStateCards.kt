package com.fortunexample.note.ui.components

import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.fortunexample.note.ui.theme.AccentPrimary
import com.fortunexample.note.ui.theme.BackgroundWhite
import com.fortunexample.note.ui.theme.CrimsonAlert

@Composable
fun LoadingStateBox(modifier: Modifier = Modifier) {
    Box(
        modifier = modifier
            .fillMaxWidth()
            .height(240.dp),
        contentAlignment = Alignment.Center
    ) {
        CircularProgressIndicator(color = AccentPrimary, strokeWidth = 2.dp)
    }
}

@Composable
fun DomainErrorCard(
    message: String,
    onRetry: () -> Unit
) {
    Surface(
        modifier = Modifier
            .fillMaxWidth()
            .padding(vertical = 12.dp),
        color = CrimsonAlert.copy(alpha = 0.06f),
        border = BorderStroke(1.dp, CrimsonAlert.copy(alpha = 0.35f)),
        shape = RoundedCornerShape(8.dp)
    ) {
        Column(
            modifier = Modifier
                .fillMaxWidth()
                .padding(16.dp),
            horizontalAlignment = Alignment.CenterHorizontally
        ) {
            Text(
                text = message,
                color = CrimsonAlert,
                fontSize = 13.sp,
                textAlign = TextAlign.Center
            )
            Spacer(modifier = Modifier.height(10.dp))
            Button(
                onClick = onRetry,
                colors = ButtonDefaults.buttonColors(containerColor = AccentPrimary)
            ) {
                Text("RETRY", color = BackgroundWhite, fontSize = 11.sp, fontWeight = FontWeight.Bold, style = TabularMonoStyle)
            }
        }
    }
}
