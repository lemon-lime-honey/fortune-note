package com.fortunexample.note.ui.theme

import androidx.compose.ui.graphics.Color

// Paper Light Theme Palette
val BackgroundWhite = Color(0xFFFFFFFF)
val SurfaceLight = Color(0xFFF8FAFC)
val CardLight = Color(0xFFFFFFFF)
val AccentPrimary = Color(0xFF0F172A)    // 딥 차콜/슬레이트
val TextPrimary = Color(0xFF0F172A)      // 선명한 흑색
val TextSecondary = Color(0xFF475569)    // 가독성 높은 미디엄 슬레이트
val BorderColor = Color(0xFFE2E8F0)      // 은은한 그레이 라인

// 기존 심볼 호환용 (다크 톤 제거, 라이트 톤 매핑)
val DeepNavy = BackgroundWhite
val SurfaceDark = SurfaceLight
val CardDark = CardLight
val AccentGold = AccentPrimary // 주황색 전면 퇴출 -> 딥 차콜

// 포인트 컬러 (주황색 완전 배제, 표준 대비 준수)
val CelestialBlue = Color(0xFF1D4ED8)    // 클래식 로열 블루
val AstrologicalRed = Color(0xFFB91C1C)  // 클래식 버건디 레드
val AstrologicalGreen = Color(0xFF15803D)// 클래식 포레스트 그린
val VedicGreen = AstrologicalGreen       // 클래식 포레스트 그린
val SajuSlate = Color(0xFF334155)        // 정갈한 다크 슬레이트
val SajuAmber = SajuSlate                // 주황색 제거
