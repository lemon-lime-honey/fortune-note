package com.fortunexample.note.ui.screens

import android.widget.Toast
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.filled.FileDownload
import androidx.compose.material.icons.filled.FileUpload
import androidx.compose.material.icons.filled.Info
import androidx.compose.material.icons.filled.Share
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.fortunexample.note.FortuneApp
import com.fortunexample.note.data.BackupManager
import com.fortunexample.note.data.BackupPreview
import com.fortunexample.note.ui.theme.*
import kotlinx.coroutines.launch

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun DataManagementScreen(
    profileCount: Int,
    horaryCount: Int,
    onBack: () -> Unit,
    onDataRestored: () -> Unit
) {
    val context = LocalContext.current
    val scope = rememberCoroutineScope()

    var previewData by remember { mutableStateOf<BackupPreview?>(null) }
    var showPreviewDialog by remember { mutableStateOf(false) }
    var replaceExisting by remember { mutableStateOf(false) }
    var isProcessing by remember { mutableStateOf(false) }

    // 파일 저장 SAF 런처
    val exportFileLauncher = rememberLauncherForActivityResult(
        contract = ActivityResultContracts.CreateDocument("application/json")
    ) { uri ->
        if (uri != null) {
            scope.launch {
                isProcessing = true
                FortuneApp.repository.exportBackupJson()
                    .onSuccess { jsonStr ->
                        BackupManager.saveBackupToUri(context, uri, jsonStr)
                            .onSuccess {
                                Toast.makeText(context, "백업 파일이 성공적으로 저장되었습니다.", Toast.LENGTH_SHORT).show()
                            }
                            .onFailure {
                                Toast.makeText(context, "파일 저장 실패: ${it.localizedMessage}", Toast.LENGTH_LONG).show()
                            }
                    }
                    .onFailure {
                        Toast.makeText(context, "데이터 직렬화 실패: ${it.localizedMessage}", Toast.LENGTH_LONG).show()
                    }
                isProcessing = false
            }
        }
    }

    // 파일 선택 SAF 런처
    val importFileLauncher = rememberLauncherForActivityResult(
        contract = ActivityResultContracts.OpenDocument()
    ) { uri ->
        if (uri != null) {
            scope.launch {
                isProcessing = true
                BackupManager.readAndParseBackup(context, uri)
                    .onSuccess { preview ->
                        previewData = preview
                        showPreviewDialog = true
                    }
                    .onFailure {
                        Toast.makeText(context, "백업 파일 검증 실패: ${it.localizedMessage}", Toast.LENGTH_LONG).show()
                    }
                isProcessing = false
            }
        }
    }

    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text("데이터 관리 및 백업", fontWeight = FontWeight.Bold, color = TextPrimary) },
                navigationIcon = {
                    IconButton(onClick = onBack) {
                        Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = "뒤로", tint = TextPrimary)
                    }
                },
                colors = TopAppBarDefaults.topAppBarColors(
                    containerColor = BackgroundWhite,
                    titleContentColor = TextPrimary
                )
            )
        },
        containerColor = BackgroundWhite
    ) { padding ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(padding)
                .verticalScroll(rememberScrollState())
                .padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(16.dp)
        ) {
            // 1. 현재 보관 현황 카드
            Card(
                shape = RoundedCornerShape(12.dp),
                colors = CardDefaults.cardColors(containerColor = SurfaceLight),
                border = BorderStroke(1.dp, BorderColor),
                modifier = Modifier.fillMaxWidth()
            ) {
                Column(modifier = Modifier.padding(16.dp)) {
                    Text(
                        text = "현재 저장된 데이터",
                        fontWeight = FontWeight.Bold,
                        fontSize = 15.sp,
                        color = TextPrimary
                    )
                    Spacer(modifier = Modifier.height(12.dp))
                    Row(
                        modifier = Modifier.fillMaxWidth(),
                        horizontalArrangement = Arrangement.SpaceAround
                    ) {
                        Column(horizontalAlignment = Alignment.CenterHorizontally) {
                            Text(text = "인물 프로필", fontSize = 12.sp, color = TextSecondary)
                            Spacer(modifier = Modifier.height(4.dp))
                            Text(text = "$profileCount 건", fontSize = 20.sp, fontWeight = FontWeight.Bold, color = AccentPrimary)
                        }
                        Column(horizontalAlignment = Alignment.CenterHorizontally) {
                            Text(text = "호라리/프라슈나 질문", fontSize = 12.sp, color = TextSecondary)
                            Spacer(modifier = Modifier.height(4.dp))
                            Text(text = "$horaryCount 건", fontSize = 20.sp, fontWeight = FontWeight.Bold, color = AccentPrimary)
                        }
                    }
                }
            }

            // 2. 데이터 내보내기 (Export)
            Card(
                shape = RoundedCornerShape(12.dp),
                colors = CardDefaults.cardColors(containerColor = BackgroundWhite),
                border = BorderStroke(1.dp, BorderColor),
                modifier = Modifier.fillMaxWidth()
            ) {
                Column(modifier = Modifier.padding(16.dp)) {
                    Text(
                        text = "데이터 내보내기 (백업)",
                        fontWeight = FontWeight.Bold,
                        fontSize = 15.sp,
                        color = TextPrimary
                    )
                    Spacer(modifier = Modifier.height(6.dp))
                    Text(
                        text = "전체 데이터를 JSON 형식으로 백업합니다. 카카오톡, 구글 드라이브 등으로 바로 전송하거나 기기에 직접 저장할 수 있습니다.",
                        fontSize = 12.sp,
                        color = TextSecondary,
                        lineHeight = 18.sp
                    )
                    Spacer(modifier = Modifier.height(14.dp))

                    // 시스템 공유 시트 버튼
                    Button(
                        onClick = {
                            scope.launch {
                                isProcessing = true
                                FortuneApp.repository.exportBackupJson()
                                    .onSuccess { jsonStr ->
                                        val uri = BackupManager.createBackupFileForShare(context, jsonStr)
                                        BackupManager.launchShareSheet(context, uri)
                                    }
                                    .onFailure {
                                        Toast.makeText(context, "백업 생성 실패: ${it.localizedMessage}", Toast.LENGTH_LONG).show()
                                    }
                                isProcessing = false
                            }
                        },
                        colors = ButtonDefaults.buttonColors(containerColor = AccentPrimary),
                        shape = RoundedCornerShape(8.dp),
                        modifier = Modifier.fillMaxWidth()
                    ) {
                        Icon(Icons.Default.Share, contentDescription = null, modifier = Modifier.size(18.dp))
                        Spacer(modifier = Modifier.width(8.dp))
                        Text("공유 창으로 전송 (카카오톡, 드라이브 등)", fontWeight = FontWeight.SemiBold)
                    }

                    Spacer(modifier = Modifier.height(8.dp))

                    // 로컬 파일 저장 버튼
                    OutlinedButton(
                        onClick = {
                            exportFileLauncher.launch(BackupManager.generateBackupFileName())
                        },
                        shape = RoundedCornerShape(8.dp),
                        colors = ButtonDefaults.outlinedButtonColors(contentColor = TextPrimary),
                        border = BorderStroke(1.dp, TextSecondary.copy(alpha = 0.5f)),
                        modifier = Modifier.fillMaxWidth()
                    ) {
                        Icon(Icons.Default.FileDownload, contentDescription = null, modifier = Modifier.size(18.dp))
                        Spacer(modifier = Modifier.width(8.dp))
                        Text("기기 폴더에 직접 파일로 저장", fontWeight = FontWeight.Medium)
                    }
                }
            }

            // 3. 데이터 복원 (Import)
            Card(
                shape = RoundedCornerShape(12.dp),
                colors = CardDefaults.cardColors(containerColor = BackgroundWhite),
                border = BorderStroke(1.dp, BorderColor),
                modifier = Modifier.fillMaxWidth()
            ) {
                Column(modifier = Modifier.padding(16.dp)) {
                    Text(
                        text = "데이터 불러오기 (복원)",
                        fontWeight = FontWeight.Bold,
                        fontSize = 15.sp,
                        color = TextPrimary
                    )
                    Spacer(modifier = Modifier.height(6.dp))
                    Text(
                        text = "기존에 내보낸 백업 파일(.json)을 선택하여 데이터를 복원합니다. 복원 전에 파일 내용을 미리 검증합니다.",
                        fontSize = 12.sp,
                        color = TextSecondary,
                        lineHeight = 18.sp
                    )
                    Spacer(modifier = Modifier.height(14.dp))

                    Button(
                        onClick = {
                            importFileLauncher.launch(arrayOf("application/json", "*/*"))
                        },
                        colors = ButtonDefaults.buttonColors(containerColor = SajuSlate),
                        shape = RoundedCornerShape(8.dp),
                        modifier = Modifier.fillMaxWidth()
                    ) {
                        Icon(Icons.Default.FileUpload, contentDescription = null, modifier = Modifier.size(18.dp))
                        Spacer(modifier = Modifier.width(8.dp))
                        Text("백업 파일 선택 및 복원", fontWeight = FontWeight.SemiBold)
                    }
                }
            }

            // 4. 안전 가이드라인 안내
            Card(
                shape = RoundedCornerShape(12.dp),
                colors = CardDefaults.cardColors(containerColor = SurfaceLight),
                border = BorderStroke(1.dp, BorderColor),
                modifier = Modifier.fillMaxWidth()
            ) {
                Row(
                    modifier = Modifier.padding(14.dp),
                    verticalAlignment = Alignment.Top
                ) {
                    Icon(
                        Icons.Default.Info,
                        contentDescription = null,
                        tint = TextSecondary,
                        modifier = Modifier.size(20.dp).padding(top = 2.dp)
                    )
                    Spacer(modifier = Modifier.width(10.dp))
                    Column {
                        Text(
                            text = "데이터 유실 방지 안내",
                            fontWeight = FontWeight.Bold,
                            fontSize = 13.sp,
                            color = TextPrimary
                        )
                        Spacer(modifier = Modifier.height(4.dp))
                        Text(
                            text = "이 앱은 외부 서버가 없는 100% 오프라인 앱입니다. 안드로이드 보안 정책상 앱을 삭제하면 기기 내부 데이터가 모두 삭제되므로, 앱 삭제 전 반드시 '공유 창으로 전송'을 눌러 카카오톡 나에게 보내기나 클라우드에 백업 파일을 보관하십시오.",
                            fontSize = 11.sp,
                            color = TextSecondary,
                            lineHeight = 16.sp
                        )
                    }
                }
            }
        }
    }

    // 복원 사전 확인 다이얼로그 (상용 앱 표준 UX)
    if (showPreviewDialog && previewData != null) {
        val preview = previewData!!
        AlertDialog(
            onDismissRequest = { showPreviewDialog = false },
            title = { Text("백업 데이터 검증 완료", fontWeight = FontWeight.Bold, fontSize = 16.sp, color = TextPrimary) },
            text = {
                Column(verticalArrangement = Arrangement.spacedBy(8.dp)) {
                    Text(
                        text = "선택한 백업 파일에서 유효한 데이터를 확인했습니다.",
                        fontSize = 13.sp,
                        color = TextPrimary
                    )
                    HorizontalDivider(color = BorderColor)
                    Text(text = "• 백업 생성 시각: ${preview.exportedAt.take(19).replace('T', ' ')}", fontSize = 12.sp, color = TextSecondary)
                    Text(text = "• 인물 프로필: ${preview.profileCount} 건", fontSize = 13.sp, fontWeight = FontWeight.SemiBold, color = TextPrimary)
                    Text(text = "• 호라리/프라슈나 질문: ${preview.horaryCount} 건", fontSize = 13.sp, fontWeight = FontWeight.SemiBold, color = TextPrimary)
                    HorizontalDivider(color = BorderColor)
                    Text(text = "복원 방식을 선택하십시오:", fontSize = 13.sp, fontWeight = FontWeight.Bold, color = TextPrimary)

                    Row(verticalAlignment = Alignment.CenterVertically) {
                        RadioButton(
                            selected = !replaceExisting,
                            onClick = { replaceExisting = false }
                        )
                        Text(
                            text = "기존 데이터 유지하고 합치기 (권장)",
                            fontSize = 12.sp,
                            color = TextPrimary
                        )
                    }
                    Row(verticalAlignment = Alignment.CenterVertically) {
                        RadioButton(
                            selected = replaceExisting,
                            onClick = { replaceExisting = true }
                        )
                        Text(
                            text = "기존 데이터를 모두 지우고 대체",
                            fontSize = 12.sp,
                            color = AstrologicalRed
                        )
                    }
                }
            },
            confirmButton = {
                Button(
                    onClick = {
                        showPreviewDialog = false
                        scope.launch {
                            isProcessing = true
                            FortuneApp.repository.importBackupJson(preview.rawJson, replaceExisting)
                                .onSuccess { summary ->
                                    Toast.makeText(context, summary.message, Toast.LENGTH_LONG).show()
                                    onDataRestored()
                                }
                                .onFailure {
                                    Toast.makeText(context, "복원 실패: ${it.localizedMessage}", Toast.LENGTH_LONG).show()
                                }
                            isProcessing = false
                        }
                    },
                    colors = ButtonDefaults.buttonColors(containerColor = AccentPrimary)
                ) {
                    Text("복원 실행")
                }
            },
            dismissButton = {
                TextButton(onClick = { showPreviewDialog = false }) {
                    Text("취소", color = TextSecondary)
                }
            },
            containerColor = BackgroundWhite
        )
    }
}
