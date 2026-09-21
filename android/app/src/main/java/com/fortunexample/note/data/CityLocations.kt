package com.fortunexample.note.data

data class CityLocation(
    val name: String,
    val province: String,
    val latitude: Double,
    val longitude: Double
) {
    val displayName: String get() = if (province.isEmpty()) name else "$province $name"
}

object CityDatabase {
    val CITIES = listOf(
        // 서울특별시
        CityLocation("서울 (중구)", "서울특별시", 37.5665, 126.9780),
        CityLocation("강남구", "서울특별시", 37.5172, 127.0473),
        CityLocation("강동구", "서울특별시", 37.5301, 127.1238),
        CityLocation("강북구", "서울특별시", 37.6396, 127.0257),
        CityLocation("강서구", "서울특별시", 37.5509, 126.8495),
        CityLocation("관악구", "서울특별시", 37.4784, 126.9516),
        CityLocation("광진구", "서울특별시", 37.5385, 127.0824),
        CityLocation("구로구", "서울특별시", 37.4954, 126.8874),
        CityLocation("금천구", "서울특별시", 37.4569, 126.8954),
        CityLocation("노원구", "서울특별시", 37.6542, 127.0568),
        CityLocation("도봉구", "서울특별시", 37.6688, 127.0471),
        CityLocation("동대문구", "서울특별시", 37.5744, 127.0400),
        CityLocation("동작구", "서울특별시", 37.5124, 126.9393),
        CityLocation("마포구", "서울특별시", 37.5663, 126.9016),
        CityLocation("서대문구", "서울특별시", 37.5791, 126.9368),
        CityLocation("서초구", "서울특별시", 37.4837, 127.0324),
        CityLocation("성동구", "서울특별시", 37.5633, 127.0371),
        CityLocation("성북구", "서울특별시", 37.5891, 127.0182),
        CityLocation("송파구", "서울특별시", 37.5145, 127.1058),
        CityLocation("양천구", "서울특별시", 37.5169, 126.8665),
        CityLocation("영등포구", "서울특별시", 37.5264, 126.8962),
        CityLocation("용산구", "서울특별시", 37.5326, 126.9900),
        CityLocation("은평구", "서울특별시", 37.6027, 126.9291),
        CityLocation("종로구", "서울특별시", 37.5730, 126.9794),
        CityLocation("중랑구", "서울특별시", 37.6063, 127.0928),

        // 광역시 / 특별자치시
        CityLocation("부산", "부산광역시", 35.1796, 129.0756),
        CityLocation("해운대구", "부산광역시", 35.1631, 129.1636),
        CityLocation("대구", "대구광역시", 35.8714, 128.6014),
        CityLocation("수성구", "대구광역시", 35.8580, 128.6306),
        CityLocation("인천", "인천광역시", 37.4563, 126.7052),
        CityLocation("송도 (연수구)", "인천광역시", 37.3850, 126.6540),
        CityLocation("광주", "광주광역시", 35.1595, 126.8526),
        CityLocation("대전", "대전광역시", 36.3504, 127.3845),
        CityLocation("유성구", "대전광역시", 36.3622, 127.3563),
        CityLocation("울산", "울산광역시", 35.5384, 129.3114),
        CityLocation("세종", "세종특별자치시", 36.4800, 127.2890),

        // 경기도
        CityLocation("수원", "경기도", 37.2636, 127.0286),
        CityLocation("성남 (분당구)", "경기도", 37.3827, 127.1189),
        CityLocation("판교", "경기도", 37.4014, 127.1086),
        CityLocation("용인", "경기도", 37.2411, 127.1776),
        CityLocation("수지구", "경기도", 37.3220, 127.0975),
        CityLocation("고양 (일산)", "경기도", 37.6584, 126.8320),
        CityLocation("부천", "경기도", 37.5034, 126.7660),
        CityLocation("화성 (동탄)", "경기도", 37.1995, 127.0789),
        CityLocation("안산", "경기도", 37.3219, 126.8309),
        CityLocation("남양주", "경기도", 37.6360, 127.2165),
        CityLocation("안양", "경기도", 37.3943, 126.9568),
        CityLocation("평택", "경기도", 36.9921, 127.1129),
        CityLocation("시흥", "경기도", 37.3802, 126.8029),
        CityLocation("파주", "경기도", 37.7600, 126.7799),
        CityLocation("의정부", "경기도", 37.7381, 127.0337),
        CityLocation("김포", "경기도", 37.6152, 126.7157),
        CityLocation("광명", "경기도", 37.4786, 126.8647),
        CityLocation("하남", "경기도", 37.5393, 127.2148),
        CityLocation("군포", "경기도", 37.3614, 126.9352),
        CityLocation("오산", "경기도", 37.1498, 127.0772),
        CityLocation("이천", "경기도", 37.2723, 127.4350),
        CityLocation("구리", "경기도", 37.5943, 127.1295),
        CityLocation("안성", "경기도", 37.0080, 127.2797),
        CityLocation("포천", "경기도", 37.8949, 127.2003),
        CityLocation("양평", "경기도", 37.4918, 127.4876),

        // 강원도
        CityLocation("춘천", "강원도", 37.8813, 127.7298),
        CityLocation("원주", "강원도", 37.3422, 127.9202),
        CityLocation("강릉", "강원도", 37.7519, 128.8761),
        CityLocation("속초", "강원도", 38.2070, 128.5918),

        // 충청남북도
        CityLocation("청주", "충청북도", 36.6424, 127.4890),
        CityLocation("충주", "충청북도", 36.9910, 127.9260),
        CityLocation("천안", "충청남도", 36.8151, 127.1139),
        CityLocation("아산", "충청남도", 36.7898, 127.0019),
        CityLocation("서산", "충청남도", 36.7845, 126.4503),
        CityLocation("당진", "충청남도", 36.8929, 126.6283),

        // 전라남북도
        CityLocation("전주", "전라북도", 35.8242, 127.1480),
        CityLocation("익산", "전라북도", 35.9483, 126.9576),
        CityLocation("군산", "전라북도", 35.9677, 126.7366),
        CityLocation("목포", "전라남도", 34.8118, 126.3922),
        CityLocation("여수", "전라남도", 34.7604, 127.6622),
        CityLocation("순천", "전라남도", 34.9506, 127.4872),
        CityLocation("나주", "전라남도", 35.0161, 126.7108),

        // 경상남북도
        CityLocation("포항", "경상북도", 36.0190, 129.3435),
        CityLocation("구미", "경상북도", 36.1195, 128.3446),
        CityLocation("경주", "경상북도", 35.8562, 129.2247),
        CityLocation("안동", "경상북도", 36.5684, 128.7294),
        CityLocation("창원", "경상남도", 35.2280, 128.6811),
        CityLocation("김해", "경상남도", 35.2285, 128.8894),
        CityLocation("진주", "경상남도", 35.1802, 128.1076),
        CityLocation("양산", "경상남도", 35.3350, 129.0373),
        CityLocation("거제", "경상남도", 34.8806, 128.6211),

        // 제주특별자치도
        CityLocation("제주", "제주특별자치도", 33.4996, 126.5312),
        CityLocation("서귀포", "제주특별자치도", 33.2541, 126.5601),

        // 해외 주요 도시
        CityLocation("도쿄", "일본", 35.6762, 139.6503),
        CityLocation("오사카", "일본", 34.6937, 135.5023),
        CityLocation("베이징", "중국", 39.9042, 116.4074),
        CityLocation("상하이", "중국", 31.2304, 121.4737),
        CityLocation("홍콩", "중국", 22.3193, 114.1694),
        CityLocation("타이베이", "대만", 25.0330, 121.5654),
        CityLocation("싱가포르", "싱가포르", 1.3521, 103.8198),
        CityLocation("방콕", "태국", 13.7563, 100.5018),
        CityLocation("런던", "영국", 51.5074, -0.1278),
        CityLocation("파리", "프랑스", 48.8566, 2.3522),
        CityLocation("베를린", "독일", 52.5200, 13.4050),
        CityLocation("뉴욕", "미국", 40.7128, -74.0060),
        CityLocation("로스앤젤레스", "미국", 34.0522, -118.2437),
        CityLocation("샌프란시스코", "미국", 37.7749, -122.4194),
        CityLocation("시애틀", "미국", 47.6062, -122.3321),
        CityLocation("토론토", "캐나다", 43.6532, -79.3832),
        CityLocation("시드니", "호주", -33.8688, 151.2093),
        CityLocation("멜버른", "호주", -37.8136, 144.9631)
    )

    fun search(query: String): List<CityLocation> {
        val q = query.trim()
        if (q.isEmpty()) return CITIES.take(8)
        return CITIES.filter {
            it.name.contains(q, ignoreCase = true) || it.province.contains(q, ignoreCase = true)
        }
    }
}
