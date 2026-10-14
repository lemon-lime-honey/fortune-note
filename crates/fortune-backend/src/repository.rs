use crate::models::{HoraryQuestion, Profile};
use rusqlite::{params, Connection, Result};

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
    fn list_all(&self, conn: &Connection, category: Option<&str>, querent_id: Option<&str>) -> Result<Vec<HoraryQuestion>>;
    fn update(&self, conn: &Connection, item: &HoraryQuestion) -> Result<bool>;
    fn delete(&self, conn: &Connection, id: &str) -> Result<bool>;
}

pub struct ProfileRepository;

impl ProfileRepository {
    pub fn insert(conn: &Connection, profile: &Profile) -> Result<()> {
        conn.execute(
            r#"
            INSERT INTO profiles (
                id, name, gender, birth_year, birth_month, birth_day,
                birth_hour, birth_minute, latitude, longitude, timezone_id,
                notes, created_at, updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)
            "#,
            params![
                profile.id,
                profile.name,
                profile.gender,
                profile.birth_year,
                profile.birth_month,
                profile.birth_day,
                profile.birth_hour,
                profile.birth_minute,
                profile.latitude,
                profile.longitude,
                profile.timezone_id,
                profile.notes,
                profile.created_at,
                profile.updated_at,
            ],
        )?;
        Ok(())
    }

    pub fn find_by_id(conn: &Connection, id: &str) -> Result<Option<Profile>> {
        let mut stmt = conn.prepare(
            r#"
            SELECT id, name, gender, birth_year, birth_month, birth_day,
                   birth_hour, birth_minute, latitude, longitude, timezone_id,
                   notes, created_at, updated_at
            FROM profiles
            WHERE id = ?1
            "#,
        )?;

        let mut rows = stmt.query(params![id])?;
        if let Some(row) = rows.next()? {
            Ok(Some(Profile {
                id: row.get(0)?,
                name: row.get(1)?,
                gender: row.get(2)?,
                birth_year: row.get(3)?,
                birth_month: row.get(4)?,
                birth_day: row.get(5)?,
                birth_hour: row.get(6)?,
                birth_minute: row.get(7)?,
                latitude: row.get(8)?,
                longitude: row.get(9)?,
                timezone_id: row.get(10)?,
                notes: row.get(11)?,
                created_at: row.get(12)?,
                updated_at: row.get(13)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn list_all(conn: &Connection, search: Option<&str>) -> Result<Vec<Profile>> {
        let sql = match search {
            Some(query) if !query.trim().is_empty() => {
                let pattern = format!("%{}%", query.trim());
                let mut stmt = conn.prepare(
                    r#"
                    SELECT id, name, gender, birth_year, birth_month, birth_day,
                           birth_hour, birth_minute, latitude, longitude, timezone_id,
                           notes, created_at, updated_at
                    FROM profiles
                    WHERE name LIKE ?1 OR notes LIKE ?1
                    ORDER BY created_at DESC
                    "#,
                )?;
                let profile_iter = stmt.query_map(params![pattern], |row| {
                    Ok(Profile {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        gender: row.get(2)?,
                        birth_year: row.get(3)?,
                        birth_month: row.get(4)?,
                        birth_day: row.get(5)?,
                        birth_hour: row.get(6)?,
                        birth_minute: row.get(7)?,
                        latitude: row.get(8)?,
                        longitude: row.get(9)?,
                        timezone_id: row.get(10)?,
                        notes: row.get(11)?,
                        created_at: row.get(12)?,
                        updated_at: row.get(13)?,
                    })
                })?;
                let mut list = Vec::new();
                for p in profile_iter {
                    list.push(p?);
                }
                return Ok(list);
            }
            _ => {
                r#"
                SELECT id, name, gender, birth_year, birth_month, birth_day,
                       birth_hour, birth_minute, latitude, longitude, timezone_id,
                       notes, created_at, updated_at
                FROM profiles
                ORDER BY created_at DESC
                "#
            }
        };

        let mut stmt = conn.prepare(sql)?;
        let profile_iter = stmt.query_map([], |row| {
            Ok(Profile {
                id: row.get(0)?,
                name: row.get(1)?,
                gender: row.get(2)?,
                birth_year: row.get(3)?,
                birth_month: row.get(4)?,
                birth_day: row.get(5)?,
                birth_hour: row.get(6)?,
                birth_minute: row.get(7)?,
                latitude: row.get(8)?,
                longitude: row.get(9)?,
                timezone_id: row.get(10)?,
                notes: row.get(11)?,
                created_at: row.get(12)?,
                updated_at: row.get(13)?,
            })
        })?;

        let mut list = Vec::new();
        for p in profile_iter {
            list.push(p?);
        }
        Ok(list)
    }

    pub fn update(conn: &Connection, profile: &Profile) -> Result<bool> {
        let rows = conn.execute(
            r#"
            UPDATE profiles
            SET name = ?1, gender = ?2, birth_year = ?3, birth_month = ?4,
                birth_day = ?5, birth_hour = ?6, birth_minute = ?7,
                latitude = ?8, longitude = ?9, timezone_id = ?10,
                notes = ?11, updated_at = ?12
            WHERE id = ?13
            "#,
            params![
                profile.name,
                profile.gender,
                profile.birth_year,
                profile.birth_month,
                profile.birth_day,
                profile.birth_hour,
                profile.birth_minute,
                profile.latitude,
                profile.longitude,
                profile.timezone_id,
                profile.notes,
                profile.updated_at,
                profile.id,
            ],
        )?;
        Ok(rows > 0)
    }

    pub fn delete(conn: &Connection, id: &str) -> Result<bool> {
        let rows = conn.execute("DELETE FROM profiles WHERE id = ?1", params![id])?;
        Ok(rows > 0)
    }
}

pub struct HoraryRepository;

impl HoraryRepository {
    pub fn insert(conn: &Connection, item: &HoraryQuestion) -> Result<()> {
        conn.execute(
            r#"
            INSERT INTO horary_questions (
                id, title, category, question_text, query_year, query_month,
                query_day, query_hour, query_minute, query_second, latitude,
                longitude, timezone_id, querent_profile_id, result_notes,
                created_at, updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)
            "#,
            params![
                item.id,
                item.title,
                item.category,
                item.question_text,
                item.query_year,
                item.query_month,
                item.query_day,
                item.query_hour,
                item.query_minute,
                item.query_second,
                item.latitude,
                item.longitude,
                item.timezone_id,
                item.querent_profile_id,
                item.result_notes,
                item.created_at,
                item.updated_at,
            ],
        )?;
        Ok(())
    }

    pub fn find_by_id(conn: &Connection, id: &str) -> Result<Option<HoraryQuestion>> {
        let mut stmt = conn.prepare(
            r#"
            SELECT id, title, category, question_text, query_year, query_month,
                   query_day, query_hour, query_minute, query_second, latitude,
                   longitude, timezone_id, querent_profile_id, result_notes,
                   created_at, updated_at
            FROM horary_questions
            WHERE id = ?1
            "#,
        )?;

        let mut rows = stmt.query(params![id])?;
        if let Some(row) = rows.next()? {
            Ok(Some(HoraryQuestion {
                id: row.get(0)?,
                title: row.get(1)?,
                category: row.get(2)?,
                question_text: row.get(3)?,
                query_year: row.get(4)?,
                query_month: row.get(5)?,
                query_day: row.get(6)?,
                query_hour: row.get(7)?,
                query_minute: row.get(8)?,
                query_second: row.get(9)?,
                latitude: row.get(10)?,
                longitude: row.get(11)?,
                timezone_id: row.get(12)?,
                querent_profile_id: row.get(13)?,
                result_notes: row.get(14)?,
                created_at: row.get(15)?,
                updated_at: row.get(16)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn list_all(
        conn: &Connection,
        category: Option<&str>,
        querent_id: Option<&str>,
    ) -> Result<Vec<HoraryQuestion>> {
        let mut conditions = Vec::new();
        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(cat) = category {
            if !cat.trim().is_empty() {
                conditions.push("category = ?");
                params_vec.push(Box::new(cat.to_string()));
            }
        }

        if let Some(qid) = querent_id {
            if !qid.trim().is_empty() {
                conditions.push("querent_profile_id = ?");
                params_vec.push(Box::new(qid.to_string()));
            }
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let sql = format!(
            r#"
            SELECT id, title, category, question_text, query_year, query_month,
                   query_day, query_hour, query_minute, query_second, latitude,
                   longitude, timezone_id, querent_profile_id, result_notes,
                   created_at, updated_at
            FROM horary_questions
            {}
            ORDER BY created_at DESC
            "#,
            where_clause
        );

        let mut stmt = conn.prepare(&sql)?;
        let param_refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|b| b.as_ref()).collect();
        let iter = stmt.query_map(param_refs.as_slice(), |row| {
            Ok(HoraryQuestion {
                id: row.get(0)?,
                title: row.get(1)?,
                category: row.get(2)?,
                question_text: row.get(3)?,
                query_year: row.get(4)?,
                query_month: row.get(5)?,
                query_day: row.get(6)?,
                query_hour: row.get(7)?,
                query_minute: row.get(8)?,
                query_second: row.get(9)?,
                latitude: row.get(10)?,
                longitude: row.get(11)?,
                timezone_id: row.get(12)?,
                querent_profile_id: row.get(13)?,
                result_notes: row.get(14)?,
                created_at: row.get(15)?,
                updated_at: row.get(16)?,
            })
        })?;

        let mut list = Vec::new();
        for item in iter {
            list.push(item?);
        }
        Ok(list)
    }

    pub fn update(conn: &Connection, item: &HoraryQuestion) -> Result<bool> {
        let rows = conn.execute(
            r#"
            UPDATE horary_questions
            SET title = ?1, category = ?2, question_text = ?3,
                query_year = ?4, query_month = ?5, query_day = ?6,
                query_hour = ?7, query_minute = ?8, query_second = ?9,
                latitude = ?10, longitude = ?11, timezone_id = ?12,
                querent_profile_id = ?13, result_notes = ?14, updated_at = ?15
            WHERE id = ?16
            "#,
            params![
                item.title,
                item.category,
                item.question_text,
                item.query_year,
                item.query_month,
                item.query_day,
                item.query_hour,
                item.query_minute,
                item.query_second,
                item.latitude,
                item.longitude,
                item.timezone_id,
                item.querent_profile_id,
                item.result_notes,
                item.updated_at,
                item.id,
            ],
        )?;
        Ok(rows > 0)
    }

    pub fn delete(conn: &Connection, id: &str) -> Result<bool> {
        let rows = conn.execute("DELETE FROM horary_questions WHERE id = ?1", params![id])?;
        Ok(rows > 0)
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

    #[test]
    fn test_profile_crud() {
        let pool = init_db(":memory:").expect("DB init failed");
        let conn = pool.lock().unwrap();

        let profile = Profile {
            id: "prof-1".to_string(),
            name: "홍길동".to_string(),
            gender: "male".to_string(),
            birth_year: 1990,
            birth_month: 5,
            birth_day: 15,
            birth_hour: 9,
            birth_minute: 30,
            latitude: 37.5665,
            longitude: 126.9780,
            timezone_id: "Asia/Seoul".to_string(),
            notes: "테스트 메모".to_string(),
            created_at: "2026-09-21T00:00:00Z".to_string(),
            updated_at: "2026-09-21T00:00:00Z".to_string(),
        };

        ProfileRepository::insert(&conn, &profile).expect("insert failed");

        let found = ProfileRepository::find_by_id(&conn, "prof-1")
            .expect("query failed")
            .expect("should find profile");
        assert_eq!(found.name, "홍길동");

        let list = ProfileRepository::list_all(&conn, Some("길동")).expect("search failed");
        assert_eq!(list.len(), 1);

        let mut updated = found;
        updated.name = "이순신".to_string();
        updated.updated_at = "2026-09-21T01:00:00Z".to_string();
        assert!(ProfileRepository::update(&conn, &updated).expect("update failed"));

        let updated_found = ProfileRepository::find_by_id(&conn, "prof-1")
            .expect("query failed")
            .expect("should find profile");
        assert_eq!(updated_found.name, "이순신");

        assert!(ProfileRepository::delete(&conn, "prof-1").expect("delete failed"));
        let after_delete = ProfileRepository::find_by_id(&conn, "prof-1").expect("query failed");
        assert!(after_delete.is_none());
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

        let found = HoraryRepository::find_by_id(&conn, "horary-1")
            .expect("query failed")
            .expect("should find horary");
        assert_eq!(found.title, "취업 성공 여부");

        let list = HoraryRepository::list_all(&conn, Some("career"), None).expect("list failed");
        assert_eq!(list.len(), 1);

        let mut updated = found;
        updated.result_notes = "최종 합격 통보 받음".to_string();
        assert!(HoraryRepository::update(&conn, &updated).expect("update failed"));

        let updated_found = HoraryRepository::find_by_id(&conn, "horary-1")
            .expect("query failed")
            .expect("should find");
        assert_eq!(updated_found.result_notes, "최종 합격 통보 받음");

        assert!(HoraryRepository::delete(&conn, "horary-1").expect("delete failed"));
        assert!(HoraryRepository::find_by_id(&conn, "horary-1").expect("query failed").is_none());
    }
}
