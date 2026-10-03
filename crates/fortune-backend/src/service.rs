use crate::db::{init_db, DbPool};
use crate::markdown::{
    format_horary_western_markdown, format_prashna_vedic_markdown,
    format_profile_saju_markdown, format_profile_vedic_markdown, format_profile_western_markdown,
};
use crate::models::{
    BackupData, CreateHoraryParams, CreateProfileParams, HoraryQuestion, ImportSummary, Profile,
    UpdateHoraryParams, UpdateProfileParams,
};
use crate::repository::{HoraryRepository, ProfileRepository};
use chrono::Utc;
use fortune_core::{
    calculate_fortune, compute_ephemeris, lookup_timezone_id, resolve_local_to_utc,
    FortuneCalculationInput, FortuneCalculationOutput, Gender, WesternHouseSystem,
};
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum FortuneBackendError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Resource not found: {0}")]
    NotFoundError(String),
    #[error("Calculation error: {0}")]
    CalculationError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
}

pub struct FortuneBackendService {
    pool: DbPool,
}

impl FortuneBackendService {
    pub fn new(db_path: &str) -> Result<Self, FortuneBackendError> {
        let pool = init_db(db_path)
            .map_err(|e| FortuneBackendError::DatabaseError(format!("Failed to init DB: {}", e)))?;
        Ok(Self { pool })
    }

    pub fn create_profile(
        &self,
        params: CreateProfileParams,
    ) -> Result<Profile, FortuneBackendError> {
        let name = params.name.trim().to_string();
        if name.is_empty() {
            return Err(FortuneBackendError::ValidationError("Name cannot be empty".to_string()));
        }

        let gender = params.gender.to_lowercase();
        if gender != "male" && gender != "female" {
            return Err(FortuneBackendError::ValidationError(
                "Gender must be 'male' or 'female'".to_string(),
            ));
        }

        let timezone_id = params.timezone_id.unwrap_or_else(|| {
            lookup_timezone_id(params.latitude, params.longitude)
                .unwrap_or_else(|_| "UTC".to_string())
        });

        let now = Utc::now().to_rfc3339();
        let profile = Profile {
            id: Uuid::new_v4().to_string(),
            name,
            gender,
            birth_year: params.birth_year,
            birth_month: params.birth_month,
            birth_day: params.birth_day,
            birth_hour: params.birth_hour,
            birth_minute: params.birth_minute,
            latitude: params.latitude,
            longitude: params.longitude,
            timezone_id,
            notes: params.notes.unwrap_or_default(),
            created_at: now.clone(),
            updated_at: now,
        };

        let conn = self
            .pool
            .lock()
            .map_err(|e| FortuneBackendError::DatabaseError(e.to_string()))?;
        ProfileRepository::insert(&conn, &profile)
            .map_err(|e| FortuneBackendError::DatabaseError(e.to_string()))?;

        Ok(profile)
    }

    pub fn get_profile(&self, id: &str) -> Result<Option<Profile>, FortuneBackendError> {
        let conn = self
            .pool
            .lock()
            .map_err(|e| FortuneBackendError::DatabaseError(e.to_string()))?;
        ProfileRepository::find_by_id(&conn, id)
            .map_err(|e| FortuneBackendError::DatabaseError(e.to_string()))
    }

    pub fn list_profiles(
        &self,
        search: Option<&str>,
    ) -> Result<Vec<Profile>, FortuneBackendError> {
        let conn = self
            .pool
            .lock()
            .map_err(|e| FortuneBackendError::DatabaseError(e.to_string()))?;
        ProfileRepository::list_all(&conn, search)
            .map_err(|e| FortuneBackendError::DatabaseError(e.to_string()))
    }

    pub fn update_profile(
        &self,
        id: &str,
        params: UpdateProfileParams,
    ) -> Result<Profile, FortuneBackendError> {
        let conn = self
            .pool
            .lock()
            .map_err(|e| FortuneBackendError::DatabaseError(e.to_string()))?;

        let existing = ProfileRepository::find_by_id(&conn, id)
            .map_err(|e| FortuneBackendError::DatabaseError(e.to_string()))?
            .ok_or_else(|| FortuneBackendError::NotFoundError(format!("Profile {} not found", id)))?;

        let name = params.name.trim().to_string();
        if name.is_empty() {
            return Err(FortuneBackendError::ValidationError("Name cannot be empty".to_string()));
        }

        let gender = params.gender.to_lowercase();
        if gender != "male" && gender != "female" {
            return Err(FortuneBackendError::ValidationError(
                "Gender must be 'male' or 'female'".to_string(),
            ));
        }

        let timezone_id = params.timezone_id.unwrap_or_else(|| {
            lookup_timezone_id(params.latitude, params.longitude)
                .unwrap_or_else(|_| existing.timezone_id.clone())
        });

        let updated = Profile {
            id: id.to_string(),
            name,
            gender,
            birth_year: params.birth_year,
            birth_month: params.birth_month,
            birth_day: params.birth_day,
            birth_hour: params.birth_hour,
            birth_minute: params.birth_minute,
            latitude: params.latitude,
            longitude: params.longitude,
            timezone_id,
            notes: params.notes.unwrap_or(existing.notes),
            created_at: existing.created_at,
            updated_at: Utc::now().to_rfc3339(),
        };

        ProfileRepository::update(&conn, &updated)
            .map_err(|e| FortuneBackendError::DatabaseError(e.to_string()))?;

        Ok(updated)
    }

    pub fn delete_profile(&self, id: &str) -> Result<bool, FortuneBackendError> {
        let conn = self
            .pool
            .lock()
            .map_err(|e| FortuneBackendError::DatabaseError(e.to_string()))?;
        ProfileRepository::delete(&conn, id)
            .map_err(|e| FortuneBackendError::DatabaseError(e.to_string()))
    }

    pub fn calculate_profile_chart(
        &self,
        id: &str,
    ) -> Result<FortuneCalculationOutput, FortuneBackendError> {
        self.calculate_profile_chart_with_house(id, None)
    }

    pub fn calculate_profile_chart_with_house(
        &self,
        id: &str,
        house_system_name: Option<&str>,
    ) -> Result<FortuneCalculationOutput, FortuneBackendError> {
        let profile = self
            .get_profile(id)?
            .ok_or_else(|| FortuneBackendError::NotFoundError(format!("Profile {} not found", id)))?;

        let gender = if profile.gender == "male" {
            Gender::Male
        } else {
            Gender::Female
        };

        let western_house_system = house_system_name.and_then(WesternHouseSystem::from_str_name);

        let input = FortuneCalculationInput {
            year: profile.birth_year,
            month: profile.birth_month,
            day: profile.birth_day,
            hour: profile.birth_hour,
            minute: profile.birth_minute,
            second: 0.0,
            latitude: profile.latitude,
            longitude: profile.longitude,
            gender,
            western_house_system,
        };

        calculate_fortune(input).map_err(FortuneBackendError::CalculationError)
    }

    pub fn export_profile_saju_markdown(&self, id: &str) -> Result<String, FortuneBackendError> {
        let profile = self
            .get_profile(id)?
            .ok_or_else(|| FortuneBackendError::NotFoundError(format!("Profile {} not found", id)))?;
        let chart = self.calculate_profile_chart(id)?;
        Ok(format_profile_saju_markdown(&profile, &chart))
    }

    pub fn export_profile_western_markdown(&self, id: &str) -> Result<String, FortuneBackendError> {
        let profile = self
            .get_profile(id)?
            .ok_or_else(|| FortuneBackendError::NotFoundError(format!("Profile {} not found", id)))?;
        let chart = self.calculate_profile_chart(id)?;
        Ok(format_profile_western_markdown(&profile, &chart))
    }

    pub fn export_profile_vedic_markdown(&self, id: &str) -> Result<String, FortuneBackendError> {
        let profile = self
            .get_profile(id)?
            .ok_or_else(|| FortuneBackendError::NotFoundError(format!("Profile {} not found", id)))?;
        let chart = self.calculate_profile_chart(id)?;
        Ok(format_profile_vedic_markdown(&profile, &chart))
    }

    pub fn calculate_profile_predictive_suite(
        &self,
        id: &str,
        target_year: i32,
        target_month: u32,
        target_day: u32,
    ) -> Result<fortune_core::western::predictive::PredictiveSuite, FortuneBackendError> {
        let profile = self
            .get_profile(id)?
            .ok_or_else(|| FortuneBackendError::NotFoundError(format!("Profile {} not found", id)))?;

        let tz_res = resolve_local_to_utc(
            profile.birth_year,
            profile.birth_month,
            profile.birth_day,
            profile.birth_hour,
            profile.birth_minute,
            0.0,
            profile.latitude,
            profile.longitude,
        )
        .map_err(|e| FortuneBackendError::CalculationError(format!("Timezone resolution error: {}", e)))?;

        let ephem = compute_ephemeris(
            tz_res.utc_year,
            tz_res.utc_month,
            tz_res.utc_day,
            tz_res.utc_hour,
            tz_res.utc_minute,
            tz_res.utc_second,
            profile.latitude,
            profile.longitude,
        )
        .map_err(|e| FortuneBackendError::CalculationError(format!("Ephemeris error: {}", e)))?;

        let house_result = fortune_core::western::compute_houses(
            ephem.ramc,
            profile.latitude,
            ephem.true_obliquity_deg,
            fortune_core::western::HouseSystem::Placidus,
        );

        let mut natal_planets = Vec::new();
        for b in &ephem.bodies {
            if b.name != "Rahu" && b.name != "Ketu" {
                natal_planets.push((b.name.as_str(), b.tropical_longitude));
            }
        }

        let suite = fortune_core::western::predictive::calculate_predictive_suite(
            profile.birth_year,
            profile.birth_month,
            profile.birth_day,
            profile.birth_hour,
            profile.birth_minute,
            0.0,
            profile.latitude,
            profile.longitude,
            target_year,
            target_month,
            target_day,
            house_result.asc,
            house_result.mc,
            &natal_planets,
        );

        Ok(suite)
    }

    pub fn create_horary(
        &self,
        params: CreateHoraryParams,
    ) -> Result<HoraryQuestion, FortuneBackendError> {
        let title = params.title.trim().to_string();
        if title.is_empty() {
            return Err(FortuneBackendError::ValidationError("Title cannot be empty".to_string()));
        }

        let timezone_id = params.timezone_id.unwrap_or_else(|| {
            lookup_timezone_id(params.latitude, params.longitude)
                .unwrap_or_else(|_| "UTC".to_string())
        });

        let now = Utc::now().to_rfc3339();
        let item = HoraryQuestion {
            id: Uuid::new_v4().to_string(),
            title,
            category: params.category,
            question_text: params.question_text,
            query_year: params.query_year,
            query_month: params.query_month,
            query_day: params.query_day,
            query_hour: params.query_hour,
            query_minute: params.query_minute,
            query_second: params.query_second,
            latitude: params.latitude,
            longitude: params.longitude,
            timezone_id,
            querent_profile_id: params.querent_profile_id,
            result_notes: params.result_notes.unwrap_or_default(),
            created_at: now.clone(),
            updated_at: now,
        };

        let conn = self
            .pool
            .lock()
            .map_err(|e| FortuneBackendError::DatabaseError(e.to_string()))?;
        HoraryRepository::insert(&conn, &item)
            .map_err(|e| FortuneBackendError::DatabaseError(e.to_string()))?;

        Ok(item)
    }

    pub fn get_horary(&self, id: &str) -> Result<Option<HoraryQuestion>, FortuneBackendError> {
        let conn = self
            .pool
            .lock()
            .map_err(|e| FortuneBackendError::DatabaseError(e.to_string()))?;
        HoraryRepository::find_by_id(&conn, id)
            .map_err(|e| FortuneBackendError::DatabaseError(e.to_string()))
    }

    pub fn list_horary(
        &self,
        category: Option<&str>,
        querent_id: Option<&str>,
    ) -> Result<Vec<HoraryQuestion>, FortuneBackendError> {
        let conn = self
            .pool
            .lock()
            .map_err(|e| FortuneBackendError::DatabaseError(e.to_string()))?;
        HoraryRepository::list_all(&conn, category, querent_id)
            .map_err(|e| FortuneBackendError::DatabaseError(e.to_string()))
    }

    pub fn update_horary(
        &self,
        id: &str,
        params: UpdateHoraryParams,
    ) -> Result<HoraryQuestion, FortuneBackendError> {
        let conn = self
            .pool
            .lock()
            .map_err(|e| FortuneBackendError::DatabaseError(e.to_string()))?;

        let existing = HoraryRepository::find_by_id(&conn, id)
            .map_err(|e| FortuneBackendError::DatabaseError(e.to_string()))?
            .ok_or_else(|| FortuneBackendError::NotFoundError(format!("Horary {} not found", id)))?;

        let title = params.title.trim().to_string();
        if title.is_empty() {
            return Err(FortuneBackendError::ValidationError("Title cannot be empty".to_string()));
        }

        let timezone_id = params.timezone_id.unwrap_or_else(|| {
            lookup_timezone_id(params.latitude, params.longitude)
                .unwrap_or_else(|_| existing.timezone_id.clone())
        });

        let updated = HoraryQuestion {
            id: id.to_string(),
            title,
            category: params.category,
            question_text: params.question_text,
            query_year: params.query_year,
            query_month: params.query_month,
            query_day: params.query_day,
            query_hour: params.query_hour,
            query_minute: params.query_minute,
            query_second: params.query_second,
            latitude: params.latitude,
            longitude: params.longitude,
            timezone_id,
            querent_profile_id: params.querent_profile_id.or(existing.querent_profile_id),
            result_notes: params.result_notes.unwrap_or(existing.result_notes),
            created_at: existing.created_at,
            updated_at: Utc::now().to_rfc3339(),
        };

        HoraryRepository::update(&conn, &updated)
            .map_err(|e| FortuneBackendError::DatabaseError(e.to_string()))?;

        Ok(updated)
    }

    pub fn delete_horary(&self, id: &str) -> Result<bool, FortuneBackendError> {
        let conn = self
            .pool
            .lock()
            .map_err(|e| FortuneBackendError::DatabaseError(e.to_string()))?;
        HoraryRepository::delete(&conn, id)
            .map_err(|e| FortuneBackendError::DatabaseError(e.to_string()))
    }

    pub fn calculate_horary_chart(
        &self,
        id: &str,
    ) -> Result<FortuneCalculationOutput, FortuneBackendError> {
        self.calculate_horary_chart_with_house(id, Some("Regiomontanus"))
    }

    pub fn calculate_horary_chart_with_house(
        &self,
        id: &str,
        house_system_name: Option<&str>,
    ) -> Result<FortuneCalculationOutput, FortuneBackendError> {
        let item = self
            .get_horary(id)?
            .ok_or_else(|| FortuneBackendError::NotFoundError(format!("Horary {} not found", id)))?;

        let western_house_system = house_system_name
            .and_then(WesternHouseSystem::from_str_name)
            .or(Some(WesternHouseSystem::Regiomontanus));

        let input = FortuneCalculationInput {
            year: item.query_year,
            month: item.query_month,
            day: item.query_day,
            hour: item.query_hour,
            minute: item.query_minute,
            second: item.query_second,
            latitude: item.latitude,
            longitude: item.longitude,
            gender: Gender::Male, // 호라리는 차트 자체의 하우스와 행성 배치 분석
            western_house_system,
        };

        calculate_fortune(input).map_err(FortuneBackendError::CalculationError)
    }


    pub fn export_horary_western_markdown(&self, id: &str) -> Result<String, FortuneBackendError> {
        let item = self
            .get_horary(id)?
            .ok_or_else(|| FortuneBackendError::NotFoundError(format!("Horary {} not found", id)))?;
        let chart = self.calculate_horary_chart(id)?;
        Ok(format_horary_western_markdown(&item, &chart))
    }

    pub fn export_prashna_vedic_markdown(&self, id: &str) -> Result<String, FortuneBackendError> {
        let item = self
            .get_horary(id)?
            .ok_or_else(|| FortuneBackendError::NotFoundError(format!("Horary {} not found", id)))?;
        let chart = self.calculate_horary_chart(id)?;
        Ok(format_prashna_vedic_markdown(&item, &chart))
    }

    pub fn calculate_direct(
        &self,
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: f64,
        latitude: f64,
        longitude: f64,
        is_male: bool,
    ) -> Result<FortuneCalculationOutput, FortuneBackendError> {
        let gender = if is_male {
            Gender::Male
        } else {
            Gender::Female
        };
        let input = FortuneCalculationInput {
            year,
            month,
            day,
            hour,
            minute,
            second,
            latitude,
            longitude,
            gender,
            western_house_system: None,
        };
        calculate_fortune(input).map_err(FortuneBackendError::CalculationError)
    }

    pub fn export_backup_json(&self) -> Result<String, FortuneBackendError> {
        let profiles = self.list_profiles(None)?;
        let horary_questions = self.list_horary(None, None)?;
        let backup = BackupData {
            app: "fortune-note".to_string(),
            version: 1,
            exported_at: Utc::now().to_rfc3339(),
            profiles,
            horary_questions,
        };
        serde_json::to_string_pretty(&backup)
            .map_err(|e| FortuneBackendError::SerializationError(e.to_string()))
    }

    pub fn import_backup_json(
        &self,
        json_str: &str,
        replace_existing: bool,
    ) -> Result<ImportSummary, FortuneBackendError> {
        let backup: BackupData = serde_json::from_str(json_str).map_err(|e| {
            FortuneBackendError::ValidationError(format!("유효하지 않은 백업 JSON 형식입니다: {}", e))
        })?;

        if backup.app != "fortune-note" {
            return Err(FortuneBackendError::ValidationError(
                "fortune-note 앱의 백업 데이터가 아닙니다".to_string(),
            ));
        }

        let mut conn = self
            .pool
            .lock()
            .map_err(|e| FortuneBackendError::DatabaseError(e.to_string()))?;

        let tx = conn
            .transaction()
            .map_err(|e| FortuneBackendError::DatabaseError(e.to_string()))?;

        if replace_existing {
            tx.execute("DELETE FROM horary_questions", [])
                .map_err(|e| FortuneBackendError::DatabaseError(e.to_string()))?;
            tx.execute("DELETE FROM profiles", [])
                .map_err(|e| FortuneBackendError::DatabaseError(e.to_string()))?;
        }

        let mut profile_count = 0;
        for p in &backup.profiles {
            if replace_existing {
                ProfileRepository::insert(&tx, p)
                    .map_err(|e| FortuneBackendError::DatabaseError(e.to_string()))?;
                profile_count += 1;
            } else {
                let exists: bool = tx
                    .query_row("SELECT COUNT(1) FROM profiles WHERE id = ?1", [&p.id], |r| {
                        Ok(r.get::<_, i64>(0)? > 0)
                    })
                    .map_err(|e| FortuneBackendError::DatabaseError(e.to_string()))?;

                if exists {
                    ProfileRepository::update(&tx, p)
                        .map_err(|e| FortuneBackendError::DatabaseError(e.to_string()))?;
                } else {
                    ProfileRepository::insert(&tx, p)
                        .map_err(|e| FortuneBackendError::DatabaseError(e.to_string()))?;
                }
                profile_count += 1;
            }
        }

        let mut horary_count = 0;
        for h in &backup.horary_questions {
            if replace_existing {
                HoraryRepository::insert(&tx, h)
                    .map_err(|e| FortuneBackendError::DatabaseError(e.to_string()))?;
                horary_count += 1;
            } else {
                let exists: bool = tx
                    .query_row(
                        "SELECT COUNT(1) FROM horary_questions WHERE id = ?1",
                        [&h.id],
                        |r| Ok(r.get::<_, i64>(0)? > 0),
                    )
                    .map_err(|e| FortuneBackendError::DatabaseError(e.to_string()))?;

                if exists {
                    HoraryRepository::update(&tx, h)
                        .map_err(|e| FortuneBackendError::DatabaseError(e.to_string()))?;
                } else {
                    HoraryRepository::insert(&tx, h)
                        .map_err(|e| FortuneBackendError::DatabaseError(e.to_string()))?;
                }
                horary_count += 1;
            }
        }

        tx.commit()
            .map_err(|e| FortuneBackendError::DatabaseError(e.to_string()))?;

        Ok(ImportSummary {
            imported_profiles: profile_count,
            imported_horary: horary_count,
            message: format!(
                "프로필 {}건, 질문 {}건이 성공적으로 복원되었습니다",
                profile_count, horary_count
            ),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_profile_flow() {
        let service = FortuneBackendService::new(":memory:").expect("service init failed");

        let profile = service
            .create_profile(CreateProfileParams {
                name: "테스트 인물".to_string(),
                gender: "female".to_string(),
                birth_year: 1995,
                birth_month: 10,
                birth_day: 25,
                birth_hour: 14,
                birth_minute: 30,
                latitude: 37.5665,
                longitude: 126.9780,
                timezone_id: None,
                notes: Some("비고".to_string()),
            })
            .expect("profile creation failed");

        assert_eq!(profile.timezone_id, "Asia/Seoul");

        let chart = service
            .calculate_profile_chart(&profile.id)
            .expect("chart calculation failed");
        assert_eq!(chart.saju.daeun_periods.len(), 10);

        let saju_md = service
            .export_profile_saju_markdown(&profile.id)
            .expect("saju markdown export failed");
        assert!(saju_md.contains("테스트 인물"));
        assert!(saju_md.contains("사주팔자"));
        assert!(!saju_md.contains("Western Tropical"));
        assert!(!saju_md.contains("Vedic Sidereal"));

        let western_md = service
            .export_profile_western_markdown(&profile.id)
            .expect("western markdown export failed");
        assert!(western_md.contains("Western Tropical Natal Chart"));
        assert!(western_md.contains("Major Aspects"));
        assert!(!saju_md.contains("Western Tropical"));

        let vedic_md = service
            .export_profile_vedic_markdown(&profile.id)
            .expect("vedic markdown export failed");
        assert!(vedic_md.contains("Vedic Sidereal & Vargas"));
        assert!(vedic_md.contains("Vimshottari Dasha Sequence"));
        assert!(!vedic_md.contains("사주팔자"));
    }

    #[test]
    fn test_service_horary_flow() {
        let service = FortuneBackendService::new(":memory:").expect("service init failed");

        let horary = service
            .create_horary(CreateHoraryParams {
                title: "사업 확장 제안 수락 여부".to_string(),
                category: "business".to_string(),
                question_text: "신규 프로젝트 계약을 지금 체결하는 것이 유리한가?".to_string(),
                query_year: 2026,
                query_month: 9,
                query_day: 21,
                query_hour: 15,
                query_minute: 45,
                query_second: 30.0,
                latitude: 37.5665,
                longitude: 126.9780,
                timezone_id: None,
                querent_profile_id: None,
                result_notes: None,
            })
            .expect("horary creation failed");

        assert_eq!(horary.timezone_id, "Asia/Seoul");

        let chart = service
            .calculate_horary_chart(&horary.id)
            .expect("horary chart calculation failed");
        assert_eq!(chart.western.planets.len(), 7);

        let western_md = service
            .export_horary_western_markdown(&horary.id)
            .expect("western horary markdown failed");
        assert!(western_md.contains("Western Tropical"));
        assert!(western_md.contains("Major Aspects"));
        assert!(!western_md.contains("Prashna Lagna"));

        let prashna_md = service
            .export_prashna_vedic_markdown(&horary.id)
            .expect("vedic prashna markdown failed");
        assert!(prashna_md.contains("Vedic Sidereal"));
        assert!(prashna_md.contains("Prashna Lagna"));
        assert!(!prashna_md.contains("Major Aspects"));
    }

    #[test]
    fn test_backup_and_restore_json_flow() {
        let service1 = FortuneBackendService::new(":memory:").expect("service1 init failed");

        let p1 = service1
            .create_profile(CreateProfileParams {
                name: "사용자A".to_string(),
                gender: "male".to_string(),
                birth_year: 1990,
                birth_month: 1,
                birth_day: 15,
                birth_hour: 8,
                birth_minute: 20,
                latitude: 37.5665,
                longitude: 126.9780,
                timezone_id: None,
                notes: Some("첫번째 인물".to_string()),
            })
            .expect("profile1 failed");

        let _h1 = service1
            .create_horary(CreateHoraryParams {
                title: "이직 성공 여부".to_string(),
                category: "career".to_string(),
                question_text: "올해 안에 이직할 수 있을까?".to_string(),
                query_year: 2026,
                query_month: 9,
                query_day: 21,
                query_hour: 23,
                query_minute: 40,
                query_second: 0.0,
                latitude: 37.5665,
                longitude: 126.9780,
                timezone_id: None,
                querent_profile_id: Some(p1.id.clone()),
                result_notes: None,
            })
            .expect("horary1 failed");

        // 1. JSON 내보내기 검증
        let backup_json = service1.export_backup_json().expect("export failed");
        assert!(backup_json.contains("fortune-note"));
        assert!(backup_json.contains("사용자A"));
        assert!(backup_json.contains("이직 성공 여부"));

        // 2. 새로운 깨끗한 인스턴스로 복원 (재설치 시뮬레이션)
        let service2 = FortuneBackendService::new(":memory:").expect("service2 init failed");
        assert_eq!(service2.list_profiles(None).unwrap().len(), 0);
        assert_eq!(service2.list_horary(None, None).unwrap().len(), 0);

        let summary = service2
            .import_backup_json(&backup_json, true)
            .expect("import failed");
        assert_eq!(summary.imported_profiles, 1);
        assert_eq!(summary.imported_horary, 1);

        let restored_profiles = service2.list_profiles(None).unwrap();
        assert_eq!(restored_profiles.len(), 1);
        assert_eq!(restored_profiles[0].name, "사용자A");

        let restored_horaries = service2.list_horary(None, None).unwrap();
        assert_eq!(restored_horaries.len(), 1);
        assert_eq!(restored_horaries[0].title, "이직 성공 여부");
        assert_eq!(restored_horaries[0].querent_profile_id, Some(p1.id));
    }
}
