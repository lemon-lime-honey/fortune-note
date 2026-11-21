use crate::models::{HoraryQuestion, Profile};
use rusqlite::{params, Connection, Result, Row, ToSql};

const PROFILE_COLUMNS: &str = "id, name, gender, birth_year, birth_month, birth_day, \
    birth_hour, birth_minute, birth_second, latitude, longitude, timezone_id, \
    notes, created_at, updated_at";

const HORARY_COLUMNS: &str = "id, title, category, question_text, query_year, query_month, \
    query_day, query_hour, query_minute, query_second, latitude, longitude, timezone_id, \
    querent_profile_id, result_notes, created_at, updated_at";

const INSERT_PROFILE_SQL: &str = "INSERT INTO profiles (\
    id, name, gender, birth_year, birth_month, birth_day, \
    birth_hour, birth_minute, birth_second, latitude, longitude, timezone_id, \
    notes, updated_at, created_at\
) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)";

const UPDATE_PROFILE_SQL: &str = "UPDATE profiles \
    SET name = ?2, gender = ?3, birth_year = ?4, birth_month = ?5, \
        birth_day = ?6, birth_hour = ?7, birth_minute = ?8, birth_second = ?9, \
        latitude = ?10, longitude = ?11, timezone_id = ?12, \
        notes = ?13, updated_at = ?14 \
    WHERE id = ?1";

const INSERT_HORARY_SQL: &str = "INSERT INTO horary_questions (\
    id, title, category, question_text, query_year, query_month, \
    query_day, query_hour, query_minute, query_second, latitude, \
    longitude, timezone_id, querent_profile_id, result_notes, updated_at, created_at\
) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)";

const UPDATE_HORARY_SQL: &str = "UPDATE horary_questions \
    SET title = ?2, category = ?3, question_text = ?4, \
        query_year = ?5, query_month = ?6, query_day = ?7, \
        query_hour = ?8, query_minute = ?9, query_second = ?10, \
        latitude = ?11, longitude = ?12, timezone_id = ?13, \
        querent_profile_id = ?14, result_notes = ?15, updated_at = ?16 \
    WHERE id = ?1";

struct EventGeoRow {
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: f64,
    latitude: f64,
    longitude: f64,
    timezone_id: String,
}

fn read_event_geo(row: &Row<'_>, offset: usize) -> Result<EventGeoRow> {
    Ok(EventGeoRow {
        year: row.get(offset)?,
        month: row.get(offset + 1)?,
        day: row.get(offset + 2)?,
        hour: row.get(offset + 3)?,
        minute: row.get(offset + 4)?,
        second: row.get(offset + 5)?,
        latitude: row.get(offset + 6)?,
        longitude: row.get(offset + 7)?,
        timezone_id: row.get(offset + 8)?,
    })
}

fn row_to_profile(row: &Row<'_>) -> Result<Profile> {
    let geo = read_event_geo(row, 3)?;
    Ok(Profile {
        id: row.get(0)?,
        name: row.get(1)?,
        gender: row.get(2)?,
        birth_year: geo.year,
        birth_month: geo.month,
        birth_day: geo.day,
        birth_hour: geo.hour,
        birth_minute: geo.minute,
        birth_second: geo.second,
        latitude: geo.latitude,
        longitude: geo.longitude,
        timezone_id: geo.timezone_id,
        notes: row.get(12)?,
        created_at: row.get(13)?,
        updated_at: row.get(14)?,
    })
}

fn row_to_horary(row: &Row<'_>) -> Result<HoraryQuestion> {
    let geo = read_event_geo(row, 4)?;
    Ok(HoraryQuestion {
        id: row.get(0)?,
        title: row.get(1)?,
        category: row.get(2)?,
        question_text: row.get(3)?,
        query_year: geo.year,
        query_month: geo.month,
        query_day: geo.day,
        query_hour: geo.hour,
        query_minute: geo.minute,
        query_second: geo.second,
        latitude: geo.latitude,
        longitude: geo.longitude,
        timezone_id: geo.timezone_id,
        querent_profile_id: row.get(13)?,
        result_notes: row.get(14)?,
        created_at: row.get(15)?,
        updated_at: row.get(16)?,
    })
}

fn profile_common_params(p: &Profile) -> [&dyn ToSql; 14] {
    [
        &p.id, &p.name, &p.gender, &p.birth_year, &p.birth_month, &p.birth_day,
        &p.birth_hour, &p.birth_minute, &p.birth_second, &p.latitude, &p.longitude,
        &p.timezone_id, &p.notes, &p.updated_at,
    ]
}

fn horary_common_params(h: &HoraryQuestion) -> [&dyn ToSql; 16] {
    [
        &h.id, &h.title, &h.category, &h.question_text, &h.query_year, &h.query_month,
        &h.query_day, &h.query_hour, &h.query_minute, &h.query_second, &h.latitude,
        &h.longitude, &h.timezone_id, &h.querent_profile_id, &h.result_notes, &h.updated_at,
    ]
}

fn find_one_by_id<T>(
    conn: &Connection,
    table: &str,
    columns: &str,
    id: &str,
    mapper: fn(&Row<'_>) -> Result<T>,
) -> Result<Option<T>> {
    let sql = format!("SELECT {columns} FROM {table} WHERE id = ?1");
    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query(params![id])?;
    rows.next()?.map(mapper).transpose()
}

fn delete_by_id(conn: &Connection, table: &str, id: &str) -> Result<bool> {
    let sql = format!("DELETE FROM {table} WHERE id = ?1");
    Ok(conn.execute(&sql, params![id])? > 0)
}

fn escape_like_pattern(raw: &str) -> String {
    let mut escaped = String::with_capacity(raw.len());
    for ch in raw.chars() {
        if matches!(ch, '\\' | '%' | '_') {
            escaped.push('\\');
        }
        escaped.push(ch);
    }
    escaped
}

/// Trait defining the contract for Profile data access operations.
pub trait ProfileRepositoryTrait {
    fn insert(&self, conn: &Connection, profile: &Profile) -> Result<()>;
    fn find_by_id(&self, conn: &Connection, id: &str) -> Result<Option<Profile>>;
    fn list_all(&self, conn: &Connection, search: Option<&str>) -> Result<Vec<Profile>>;
    fn update(&self, conn: &Connection, profile: &Profile) -> Result<bool>;
    fn delete(&self, conn: &Connection, id: &str) -> Result<bool>;
}

/// Trait defining the contract for Horary data access operations.
pub trait HoraryRepositoryTrait {
    fn insert(&self, conn: &Connection, item: &HoraryQuestion) -> Result<()>;
    fn find_by_id(&self, conn: &Connection, id: &str) -> Result<Option<HoraryQuestion>>;
    fn list_all(
        &self,
        conn: &Connection,
        category: Option<&str>,
        querent_id: Option<&str>,
    ) -> Result<Vec<HoraryQuestion>>;
    fn update(&self, conn: &Connection, item: &HoraryQuestion) -> Result<bool>;
    fn delete(&self, conn: &Connection, id: &str) -> Result<bool>;
}

pub struct ProfileRepository;

impl ProfileRepository {
    pub fn insert(conn: &Connection, profile: &Profile) -> Result<()> {
        let mut bound: Vec<&dyn ToSql> = profile_common_params(profile).to_vec();
        bound.push(&profile.created_at);
        conn.execute(INSERT_PROFILE_SQL, bound.as_slice())?;
        Ok(())
    }

    pub fn find_by_id(conn: &Connection, id: &str) -> Result<Option<Profile>> {
        find_one_by_id(conn, "profiles", PROFILE_COLUMNS, id, row_to_profile)
    }

    pub fn list_all(conn: &Connection, search: Option<&str>) -> Result<Vec<Profile>> {
        if let Some(query) = search.map(str::trim).filter(|q| !q.is_empty()) {
            let pattern = format!("%{}%", escape_like_pattern(query));
            let sql = format!(
                "SELECT {PROFILE_COLUMNS} FROM profiles \
                 WHERE name LIKE ?1 ESCAPE '\\' OR notes LIKE ?1 ESCAPE '\\' \
                 ORDER BY created_at DESC"
            );
            let mut stmt = conn.prepare(&sql)?;
            return stmt.query_map(params![pattern], row_to_profile)?.collect();
        }
        let sql = format!("SELECT {PROFILE_COLUMNS} FROM profiles ORDER BY created_at DESC");
        let mut stmt = conn.prepare(&sql)?;
        stmt.query_map([], row_to_profile)?.collect()
    }

    pub fn update(conn: &Connection, profile: &Profile) -> Result<bool> {
        let rows = conn.execute(UPDATE_PROFILE_SQL, profile_common_params(profile).as_slice())?;
        Ok(rows > 0)
    }

    pub fn delete(conn: &Connection, id: &str) -> Result<bool> {
        delete_by_id(conn, "profiles", id)
    }
}

pub struct HoraryRepository;

impl HoraryRepository {
    pub fn insert(conn: &Connection, item: &HoraryQuestion) -> Result<()> {
        let mut bound: Vec<&dyn ToSql> = horary_common_params(item).to_vec();
        bound.push(&item.created_at);
        conn.execute(INSERT_HORARY_SQL, bound.as_slice())?;
        Ok(())
    }

    pub fn find_by_id(conn: &Connection, id: &str) -> Result<Option<HoraryQuestion>> {
        find_one_by_id(conn, "horary_questions", HORARY_COLUMNS, id, row_to_horary)
    }

    pub fn list_all(
        conn: &Connection,
        category: Option<&str>,
        querent_id: Option<&str>,
    ) -> Result<Vec<HoraryQuestion>> {
        let mut conditions = Vec::new();
        let mut params_vec: Vec<Box<dyn ToSql>> = Vec::new();

        if let Some(cat) = category.map(str::trim).filter(|c| !c.is_empty()) {
            conditions.push("category = ?");
            params_vec.push(Box::new(cat.to_string()));
        }
        if let Some(qid) = querent_id.map(str::trim).filter(|q| !q.is_empty()) {
            conditions.push("querent_profile_id = ?");
            params_vec.push(Box::new(qid.to_string()));
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };
        let sql = format!(
            "SELECT {HORARY_COLUMNS} FROM horary_questions {where_clause} ORDER BY created_at DESC"
        );
        let mut stmt = conn.prepare(&sql)?;
        let param_refs: Vec<&dyn ToSql> = params_vec.iter().map(|b| b.as_ref()).collect();
        stmt.query_map(param_refs.as_slice(), row_to_horary)?.collect()
    }

    pub fn update(conn: &Connection, item: &HoraryQuestion) -> Result<bool> {
        let rows = conn.execute(UPDATE_HORARY_SQL, horary_common_params(item).as_slice())?;
        Ok(rows > 0)
    }

    pub fn delete(conn: &Connection, id: &str) -> Result<bool> {
        delete_by_id(conn, "horary_questions", id)
    }
}

impl ProfileRepositoryTrait for ProfileRepository {
    fn insert(&self, conn: &Connection, profile: &Profile) -> Result<()> {
        Self::insert(conn, profile)
    }

    fn find_by_id(&self, conn: &Connection, id: &str) -> Result<Option<Profile>> {
        Self::find_by_id(conn, id)
    }

    fn list_all(&self, conn: &Connection, search: Option<&str>) -> Result<Vec<Profile>> {
        Self::list_all(conn, search)
    }

    fn update(&self, conn: &Connection, profile: &Profile) -> Result<bool> {
        Self::update(conn, profile)
    }

    fn delete(&self, conn: &Connection, id: &str) -> Result<bool> {
        Self::delete(conn, id)
    }
}

impl HoraryRepositoryTrait for HoraryRepository {
    fn insert(&self, conn: &Connection, item: &HoraryQuestion) -> Result<()> {
        Self::insert(conn, item)
    }

    fn find_by_id(&self, conn: &Connection, id: &str) -> Result<Option<HoraryQuestion>> {
        Self::find_by_id(conn, id)
    }

    fn list_all(
        &self,
        conn: &Connection,
        category: Option<&str>,
        querent_id: Option<&str>,
    ) -> Result<Vec<HoraryQuestion>> {
        Self::list_all(conn, category, querent_id)
    }

    fn update(&self, conn: &Connection, item: &HoraryQuestion) -> Result<bool> {
        Self::update(conn, item)
    }

    fn delete(&self, conn: &Connection, id: &str) -> Result<bool> {
        Self::delete(conn, id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_db;

    fn sample_profile(id: &str, name: &str, notes: &str) -> Profile {
        Profile {
            id: id.to_string(),
            name: name.to_string(),
            gender: "male".to_string(),
            birth_year: 1990,
            birth_month: 5,
            birth_day: 15,
            birth_hour: 9,
            birth_minute: 30,
            birth_second: 15.0,
            latitude: 37.5665,
            longitude: 126.9780,
            timezone_id: "Asia/Seoul".to_string(),
            notes: notes.to_string(),
            created_at: "2026-09-21T00:00:00Z".to_string(),
            updated_at: "2026-09-21T00:00:00Z".to_string(),
        }
    }

    #[test]
    fn test_profile_crud_and_like_escape() {
        let pool = init_db(":memory:").expect("DB init failed");
        let conn = pool.lock().unwrap();

        let p1 = sample_profile("prof-1", "홍길동", "테스트 메모");
        let p2 = sample_profile("prof-2", "100%_특수", "언더바_테스트");
        ProfileRepository::insert(&conn, &p1).expect("insert failed");
        ProfileRepository::insert(&conn, &p2).expect("insert p2 failed");

        let found = ProfileRepository::find_by_id(&conn, "prof-1")
            .expect("query failed")
            .expect("should find profile");
        assert_eq!(found.name, "홍길동");
        assert!((found.birth_second - 15.0).abs() < 1e-6);

        assert_eq!(ProfileRepository::list_all(&conn, Some("길동")).unwrap().len(), 1);
        let pct_hits = ProfileRepository::list_all(&conn, Some("%")).unwrap();
        assert_eq!(pct_hits.len(), 1);
        assert_eq!(pct_hits[0].id, "prof-2");
        let underscore_hits = ProfileRepository::list_all(&conn, Some("_")).unwrap();
        assert_eq!(underscore_hits.len(), 1);
        assert_eq!(underscore_hits[0].id, "prof-2");

        let mut updated = found;
        updated.name = "이순신".to_string();
        updated.updated_at = "2026-09-21T01:00:00Z".to_string();
        assert!(ProfileRepository::update(&conn, &updated).expect("update failed"));
        assert_eq!(
            ProfileRepository::find_by_id(&conn, "prof-1").unwrap().unwrap().name,
            "이순신"
        );

        assert!(ProfileRepository::delete(&conn, "prof-1").expect("delete failed"));
        assert!(ProfileRepository::find_by_id(&conn, "prof-1").unwrap().is_none());
    }

    #[test]
    fn test_horary_crud() {
        let pool = init_db(":memory:").expect("DB init failed");
        let conn = pool.lock().unwrap();

        let horary = HoraryQuestion {
            id: "horary-1".to_string(),
            title: "취업 성공 여부".to_string(),
            category: "career".to_string(),
            question_text: "이번 면접에 합격할 수 있을까요?".to_string(),
            query_year: 2026,
            query_month: 9,
            query_day: 21,
            query_hour: 15,
            query_minute: 30,
            query_second: 45.0,
            latitude: 37.5665,
            longitude: 126.9780,
            timezone_id: "Asia/Seoul".to_string(),
            querent_profile_id: None,
            result_notes: String::new(),
            created_at: "2026-09-21T06:30:45Z".to_string(),
            updated_at: "2026-09-21T06:30:45Z".to_string(),
        };

        HoraryRepository::insert(&conn, &horary).expect("insert horary failed");
        let found = HoraryRepository::find_by_id(&conn, "horary-1").unwrap().unwrap();
        assert_eq!(found.title, "취업 성공 여부");
        assert_eq!(HoraryRepository::list_all(&conn, Some("career"), None).unwrap().len(), 1);

        let mut updated = found;
        updated.result_notes = "최종 합격 통보 받음".to_string();
        assert!(HoraryRepository::update(&conn, &updated).expect("update failed"));
        assert_eq!(
            HoraryRepository::find_by_id(&conn, "horary-1").unwrap().unwrap().result_notes,
            "최종 합격 통보 받음"
        );

        assert!(HoraryRepository::delete(&conn, "horary-1").expect("delete failed"));
        assert!(HoraryRepository::find_by_id(&conn, "horary-1").unwrap().is_none());
    }
}
