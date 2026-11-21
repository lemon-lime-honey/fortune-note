package com.fortunexample.note.data

import java.time.LocalDate

const val DEFAULT_LATITUDE = 37.5665
const val DEFAULT_LONGITUDE = 126.9780
const val DEFAULT_BIRTH_YEAR = "2000"
const val DEFAULT_BIRTH_MONTH = "1"
const val DEFAULT_BIRTH_DAY = "1"
const val DEFAULT_BIRTH_HOUR = "12"
const val DEFAULT_BIRTH_MINUTE = "0"
const val DEFAULT_BIRTH_SECOND = "0"
const val DEFAULT_HORARY_CATEGORY = "Career"
const val DEFAULT_HOUSE_SYSTEM = "Regiomontanus"

data class DateTimeFormFields(
    val yearStr: String,
    val monthStr: String,
    val dayStr: String,
    val hourStr: String,
    val minuteStr: String,
    val secondStr: String,
    val allowBlankSecond: Boolean = false
)

data class ValidatedDateTime(
    val year: Int,
    val month: Int,
    val day: Int,
    val hour: Int,
    val minute: Int,
    val second: Double
)

sealed interface FormValidationResult {
    data class Valid(val value: ValidatedDateTime) : FormValidationResult
    data class Invalid(val message: String) : FormValidationResult
}

fun validateDateTimeAndCoordinates(
    fields: DateTimeFormFields,
    latitude: Double,
    longitude: Double
): FormValidationResult {
    val y = fields.yearStr.toIntOrNull()
    if (y == null || y !in -4000..3000) {
        return FormValidationResult.Invalid("올바른 연도를 입력해 주세요 (-4000~3000).")
    }
    val m = fields.monthStr.toIntOrNull()
    if (m == null || m !in 1..12) {
        return FormValidationResult.Invalid("올바른 월을 입력해 주세요 (1~12).")
    }
    val d = fields.dayStr.toIntOrNull()
    if (d == null || d !in 1..31) {
        return FormValidationResult.Invalid("올바른 일을 입력해 주세요 (1~31).")
    }
    val h = fields.hourStr.toIntOrNull()
    if (h == null || h !in 0..23) {
        return FormValidationResult.Invalid("올바른 시를 입력해 주세요 (0~23).")
    }
    val min = fields.minuteStr.toIntOrNull()
    if (min == null || min !in 0..59) {
        return FormValidationResult.Invalid("올바른 분을 입력해 주세요 (0~59).")
    }
    val sec = if (fields.allowBlankSecond && fields.secondStr.isBlank()) {
        0.0
    } else {
        fields.secondStr.toDoubleOrNull()
    }
    if (sec == null || !sec.isFinite() || sec < 0.0 || sec >= 60.0) {
        return FormValidationResult.Invalid("올바른 초를 입력해 주세요 (0~59.99).")
    }
    val calendarValid = runCatching { LocalDate.of(y, m, d) }.isSuccess
    if (!calendarValid) {
        return FormValidationResult.Invalid("존재하지 않는 날짜입니다.")
    }
    if (!latitude.isFinite() || latitude !in -90.0..90.0) {
        return FormValidationResult.Invalid("올바른 위도를 입력해 주세요 (-90~90).")
    }
    if (!longitude.isFinite() || longitude !in -180.0..180.0) {
        return FormValidationResult.Invalid("올바른 경도를 입력해 주세요 (-180~180).")
    }
    return FormValidationResult.Valid(ValidatedDateTime(y, m, d, h, min, sec))
}

fun computeCompletedAge(
    birthYear: Int,
    birthMonth: Int,
    birthDay: Int,
    targetYear: Int,
    targetMonth: Int,
    targetDay: Int
): Int {
    if (targetYear < birthYear) return 0
    val rawYears = targetYear - birthYear
    val beforeBirthday = targetMonth < birthMonth ||
        (targetMonth == birthMonth && targetDay < birthDay)
    return if (beforeBirthday) (rawYears - 1).coerceAtLeast(0) else rawYears
}
