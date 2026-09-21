use crate::db::{init_db, DbPool};
use crate::markdown::{format_horary_markdown, format_profile_markdown};
use crate::models::{
    CreateHoraryParams, CreateProfileParams, HoraryQuestion, Profile, UpdateHoraryParams,
    UpdateProfileParams,
};
use crate::repository::{HoraryRepository, ProfileRepository};
use chrono::Utc;
use fortune_core::{
    calculate_fortune, lookup_timezone_id, FortuneCalculationInput, FortuneCalculationOutput,
    Gender,
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
        let profile = self
            .get_profile(id)?
            .ok_or_else(|| FortuneBackendError::NotFoundError(format!("Profile {} not found", id)))?;

        let gender = if profile.gender == "male" {
            Gender::Male
        } else {
            Gender::Female
        };

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
        };

        calculate_fortune(input).map_err(FortuneBackendError::CalculationError)
    }

    pub fn export_profile_markdown(&self, id: &str) -> Result<String, FortuneBackendError> {
        let profile = self
            .get_profile(id)?
            .ok_or_else(|| FortuneBackendError::NotFoundError(format!("Profile {} not found", id)))?;
        let chart = self.calculate_profile_chart(id)?;
        Ok(format_profile_markdown(&profile, &chart))
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
        let item = self
            .get_horary(id)?
            .ok_or_else(|| FortuneBackendError::NotFoundError(format!("Horary {} not found", id)))?;

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
        };

        calculate_fortune(input).map_err(FortuneBackendError::CalculationError)
    }

    pub fn export_horary_markdown(&self, id: &str) -> Result<String, FortuneBackendError> {
        let item = self
            .get_horary(id)?
            .ok_or_else(|| FortuneBackendError::NotFoundError(format!("Horary {} not found", id)))?;
        let chart = self.calculate_horary_chart(id)?;
        Ok(format_horary_markdown(&item, &chart))
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
        };
        calculate_fortune(input).map_err(FortuneBackendError::CalculationError)
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

        let markdown = service
            .export_profile_markdown(&profile.id)
            .expect("markdown export failed");
        assert!(markdown.contains("테스트 인물"));
        assert!(markdown.contains("사주팔자"));
        assert!(markdown.contains("서양 점성술"));
        assert!(markdown.contains("베다 점성술"));
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

        let markdown = service
            .export_horary_markdown(&horary.id)
            .expect("horary markdown failed");
        assert!(markdown.contains("사업 확장 제안 수락 여부"));
        assert!(markdown.contains("호라리 차트 행성 및 하우스 상태"));
        assert!(markdown.contains("프라슈나 차트"));
    }
}
