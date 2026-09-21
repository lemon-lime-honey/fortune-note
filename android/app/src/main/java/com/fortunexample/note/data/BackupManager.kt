package com.fortunexample.note.data

import android.content.Context
import android.content.Intent
import android.net.Uri
import androidx.core.content.FileProvider
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.intOrNull
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive
import java.io.File
import java.text.SimpleDateFormat
import java.util.Date
import java.util.Locale

object BackupManager {

    private val json = Json {
        ignoreUnknownKeys = true
        isLenient = true
    }

    fun generateBackupFileName(): String {
        val timestamp = SimpleDateFormat("yyyyMMdd_HHmmss", Locale.getDefault()).format(Date())
        return "fortune_note_backup_$timestamp.json"
    }

    suspend fun createBackupFileForShare(context: Context, jsonContent: String): Uri = withContext(Dispatchers.IO) {
        val backupDir = File(context.cacheDir, "backups")
        if (!backupDir.exists()) {
            backupDir.mkdirs()
        }
        val file = File(backupDir, generateBackupFileName())
        file.writeText(jsonContent, Charsets.UTF_8)
        FileProvider.getUriForFile(context, "${context.packageName}.fileprovider", file)
    }

    fun launchShareSheet(context: Context, fileUri: Uri) {
        val intent = Intent(Intent.ACTION_SEND).apply {
            type = "application/json"
            putExtra(Intent.EXTRA_STREAM, fileUri)
            putExtra(Intent.EXTRA_SUBJECT, "Fortune Note Data Backup")
            addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION)
        }
        context.startActivity(Intent.createChooser(intent, "백업 파일 공유 및 저장"))
    }

    suspend fun saveBackupToUri(context: Context, destinationUri: Uri, jsonContent: String): Result<Unit> = withContext(Dispatchers.IO) {
        runCatching {
            context.contentResolver.openOutputStream(destinationUri)?.use { out ->
                out.write(jsonContent.toByteArray(Charsets.UTF_8))
                out.flush()
            } ?: throw IllegalStateException("출력 스트림을 생성할 수 없습니다")
        }
    }

    suspend fun readAndParseBackup(context: Context, sourceUri: Uri): Result<BackupPreview> = withContext(Dispatchers.IO) {
        runCatching {
            val content = context.contentResolver.openInputStream(sourceUri)?.bufferedReader()?.use {
                it.readText()
            } ?: throw IllegalStateException("백업 파일을 읽을 수 없습니다")

            val jsonElement = json.parseToJsonElement(content)
            val jsonObj = jsonElement.jsonObject

            val app = jsonObj["app"]?.jsonPrimitive?.content
                ?: throw IllegalArgumentException("유효하지 않은 백업 형식입니다 (app 필드 누락)")
            if (app != "fortune-note") {
                throw IllegalArgumentException("fortune-note 앱의 백업 파일이 아닙니다 (식별자: $app)")
            }

            val version = jsonObj["version"]?.jsonPrimitive?.intOrNull ?: 1
            val exportedAt = jsonObj["exported_at"]?.jsonPrimitive?.content ?: "알 수 없음"
            val profileCount = jsonObj["profiles"]?.jsonArray?.size ?: 0
            val horaryCount = jsonObj["horary_questions"]?.jsonArray?.size ?: 0

            BackupPreview(
                app = app,
                version = version,
                exportedAt = exportedAt,
                profileCount = profileCount,
                horaryCount = horaryCount,
                rawJson = content
            )
        }
    }
}
