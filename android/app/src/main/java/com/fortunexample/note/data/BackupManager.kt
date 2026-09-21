package com.fortunexample.note.data

import android.content.Context
import android.net.Uri
import com.fortunexample.note.FortuneApp
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import java.io.File
import java.io.FileInputStream
import java.io.FileOutputStream
import java.text.SimpleDateFormat
import java.util.Date
import java.util.Locale

object BackupManager {

    private val SQLITE_HEADER = "SQLite format 3\u0000".toByteArray(Charsets.US_ASCII)

    fun generateBackupFileName(): String {
        val timestamp = SimpleDateFormat("yyyyMMdd_HHmmss", Locale.getDefault()).format(Date())
        return "fortune_note_backup_$timestamp.db"
    }

    suspend fun exportDatabase(context: Context, destinationUri: Uri): Result<Unit> = withContext(Dispatchers.IO) {
        runCatching {
            val dbFile = FortuneApp.instance.getDbFile()
            if (!dbFile.exists()) {
                throw IllegalStateException("데이터베이스 파일이 존재하지 않습니다")
            }

            context.contentResolver.openOutputStream(destinationUri)?.use { outStream ->
                FileInputStream(dbFile).use { inStream ->
                    inStream.copyTo(outStream)
                }
                outStream.flush()
            } ?: throw IllegalStateException("출력 스트림을 생성할 수 없습니다")
        }
    }

    suspend fun importDatabase(context: Context, sourceUri: Uri): Result<Unit> = withContext(Dispatchers.IO) {
        runCatching {
            val dbFile = FortuneApp.instance.getDbFile()
            val tempFile = File(context.filesDir, "temp_import.db")
            val backupFile = File(context.filesDir, "fortune_note.db.bak")

            // 1. 임시 파일로 복사
            context.contentResolver.openInputStream(sourceUri)?.use { inStream ->
                FileOutputStream(tempFile).use { outStream ->
                    inStream.copyTo(outStream)
                }
            } ?: throw IllegalStateException("백업 파일을 읽을 수 없습니다")

            // 2. SQLite 매직 헤더 검증
            if (tempFile.length() < 100) {
                tempFile.delete()
                throw IllegalArgumentException("유효하지 않은 데이터베이스 파일 크기입니다")
            }

            val header = ByteArray(16)
            FileInputStream(tempFile).use { it.read(header) }
            if (!header.contentEquals(SQLITE_HEADER)) {
                tempFile.delete()
                throw IllegalArgumentException("유효한 SQLite 데이터베이스 파일이 아닙니다")
            }

            // 3. 기존 DB 안전 백업
            if (dbFile.exists()) {
                dbFile.copyTo(backupFile, overwrite = true)
            }

            try {
                // WAL 및 SHM 부속 파일 제거
                val walFile = File(dbFile.parentFile, "${dbFile.name}-wal")
                val shmFile = File(dbFile.parentFile, "${dbFile.name}-shm")
                if (walFile.exists()) walFile.delete()
                if (shmFile.exists()) shmFile.delete()

                // 임시 파일을 메인 DB 파일로 대체
                if (!tempFile.renameTo(dbFile)) {
                    tempFile.copyTo(dbFile, overwrite = true)
                    tempFile.delete()
                }

                // 리포지토리 재초기화
                FortuneApp.instance.reloadRepository()

                // 성공 시 백업 사본 제거
                if (backupFile.exists()) backupFile.delete()
            } catch (e: Exception) {
                // 실패 시 롤백 복구
                if (backupFile.exists()) {
                    backupFile.copyTo(dbFile, overwrite = true)
                    backupFile.delete()
                    FortuneApp.instance.reloadRepository()
                }
                throw e
            }
        }
    }
}
