package com.fortunexample.note.ui.components

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp

@Composable
fun DateInputRow(
    yearStr: String,
    monthStr: String,
    dayStr: String,
    onYearChange: (String) -> Unit,
    onMonthChange: (String) -> Unit,
    onDayChange: (String) -> Unit,
    yearLabel: String = "년도",
    spacing: Dp = 8.dp
) {
    Row(horizontalArrangement = Arrangement.spacedBy(spacing)) {
        OutlinedTextField(
            value = yearStr,
            onValueChange = onYearChange,
            label = { Text(yearLabel) },
            keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number, imeAction = ImeAction.Next),
            modifier = Modifier.weight(1.2f),
            singleLine = true
        )
        OutlinedTextField(
            value = monthStr,
            onValueChange = onMonthChange,
            label = { Text("월") },
            keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number, imeAction = ImeAction.Next),
            modifier = Modifier.weight(1f),
            singleLine = true
        )
        OutlinedTextField(
            value = dayStr,
            onValueChange = onDayChange,
            label = { Text("일") },
            keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number, imeAction = ImeAction.Next),
            modifier = Modifier.weight(1f),
            singleLine = true
        )
    }
}

@Composable
fun TimeInputRow(
    hourStr: String,
    minuteStr: String,
    secondStr: String,
    onHourChange: (String) -> Unit,
    onMinuteChange: (String) -> Unit,
    onSecondChange: (String) -> Unit,
    spacing: Dp = 8.dp
) {
    Row(horizontalArrangement = Arrangement.spacedBy(spacing)) {
        OutlinedTextField(
            value = hourStr,
            onValueChange = onHourChange,
            label = { Text("시") },
            keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number, imeAction = ImeAction.Next),
            modifier = Modifier.weight(1f),
            singleLine = true
        )
        OutlinedTextField(
            value = minuteStr,
            onValueChange = onMinuteChange,
            label = { Text("분") },
            keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number, imeAction = ImeAction.Next),
            modifier = Modifier.weight(1f),
            singleLine = true
        )
        OutlinedTextField(
            value = secondStr,
            onValueChange = onSecondChange,
            label = { Text("초") },
            keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number, imeAction = ImeAction.Done),
            modifier = Modifier.weight(1f),
            singleLine = true
        )
    }
}
