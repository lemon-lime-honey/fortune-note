//! Orchestration service layer for profiles, horary questions, chart calculations, and backups.

mod backup;
mod calculation;
mod horary_calc;

#[cfg(test)]
mod tests;

use std::sync::MutexGuard;
use chrono::{NaiveDate, Utc};
use fortune_core::{lookup_timezone_id, CalendarDateTime, GeoCoordinate};
use rusqlite::Connection;
use uuid::Uuid;

use crate::db::{init_db, DbPool};
use crate::models::{
    CreateHoraryParams, CreateProfileParams, HoraryQuestion, Profile, UpdateHoraryParams,
    UpdateProfileParams,
};
use crate::repository::{
    HoraryRepository, HoraryRepositoryTrait, ProfileRepository, ProfileRepositoryTrait,
};

pub use calculation::PredictiveSuiteRequest;

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

pub struct FortuneBackendService<P = ProfileRepository, H = HoraryRepository>
where
    P: ProfileRepositoryTrait,
    H: HoraryRepositoryTrait,
{
    pub(crate) pool: DbPool,
    pub(crate) profile_repo: P,
    pub(crate) horary_repo: H,
}

impl FortuneBackendService<ProfileRepository, HoraryRepository> {
    pub fn new(db_path: &str) -> Result<Self, FortuneBackendError> {
        let pool = init_db(db_path)
            .map_err(|e| FortuneBackendError::DatabaseError(format!("Failed to init DB: {}", e)))?;
        Ok(Self::with_repositories(pool, ProfileRepository, HoraryRepository))
    }
}

pub(crate) fn db_err(err: impl ToString) -> FortuneBackendError {
    FortuneBackendError::DatabaseError(err.to_string())
}

pub(crate) fn validate_datetime_and_coords(
    dt: CalendarDateTime,
    loc: GeoCoordinate,
) -> Result<(), FortuneBackendError> {
    if !(-4000..=3000).contains(&dt.year) {
        return Err(FortuneBackendError::ValidationError("Year must be between -4000 and 3000".into()));
    }
    if NaiveDate::from_ymd_opt(dt.year, dt.month, dt.day).is_none() {
        return Err(FortuneBackendError::ValidationError(format!(
            "Invalid calendar date: {:04}-{:02}-{:02}", dt.year, dt.month, dt.day
        )));
    }
    if dt.hour > 23 || dt.minute > 59 {
        return Err(FortuneBackendError::ValidationError("Hour must be 0..=23 and minute 0..=59".into()));
    }
    if !dt.second.is_finite() || !(0.0..60.0).contains(&dt.second) {
        return Err(FortuneBackendError::ValidationError("Second must be between 0.0 and 59.999".into()));
    }
    if !loc.latitude.is_finite() || !(-90.0..=90.0).contains(&loc.latitude) {
        return Err(FortuneBackendError::ValidationError("Latitude must be between -90 and 90".into()));
    }
    if !loc.longitude.is_finite() || !(-180.0..=180.0).contains(&loc.longitude) {
        return Err(FortuneBackendError::ValidationError("Longitude must be between -180 and 180".into()));
    }
    Ok(())
}

fn validate_profile_params(params: &CreateProfileParams) -> Result<(String, String), FortuneBackendError> {
    let name = params.name.trim().to_string();
    if name.is_empty() {
        return Err(FortuneBackendError::ValidationError("Name cannot be empty".to_string()));
    }
    let gender = params.gender.to_lowercase();
    if gender != "male" && gender != "female" {
        return Err(FortuneBackendError::ValidationError("Gender must be 'male' or 'female'".to_string()));
    }
    validate_datetime_and_coords(
        CalendarDateTime::new(
            params.birth_year, params.birth_month, params.birth_day,
            params.birth_hour, params.birth_minute, params.birth_second,
        ),
        GeoCoordinate::new(params.latitude, params.longitude),
    )?;
    Ok((name, gender))
}

fn resolve_timezone_or_fallback(
    explicit: Option<String>,
    latitude: f64,
    longitude: f64,
    fallback: &str,
) -> String {
    explicit.unwrap_or_else(|| {
        lookup_timezone_id(latitude, longitude).unwrap_or_else(|_| fallback.to_string())
    })
}

fn build_profile(
    id: String,
    params: CreateProfileParams,
    created_at: String,
    fallback_tz: &str,
    fallback_notes: String,
) -> Result<Profile, FortuneBackendError> {
    let (name, gender) = validate_profile_params(&params)?;
    let timezone_id = resolve_timezone_or_fallback(params.timezone_id, params.latitude, params.longitude, fallback_tz);
    Ok(Profile {
        id,
        name,
        gender,
        birth_year: params.birth_year,
        birth_month: params.birth_month,
        birth_day: params.birth_day,
        birth_hour: params.birth_hour,
        birth_minute: params.birth_minute,
        birth_second: params.birth_second,
        latitude: params.latitude,
        longitude: params.longitude,
        timezone_id,
        notes: params.notes.unwrap_or(fallback_notes),
        created_at,
        updated_at: Utc::now().to_rfc3339(),
    })
}

fn build_horary(
    id: String,
    params: CreateHoraryParams,
    created_at: String,
    fallback_tz: &str,
    fallback_notes: String,
) -> Result<HoraryQuestion, FortuneBackendError> {
    let title = params.title.trim().to_string();
    if title.is_empty() {
        return Err(FortuneBackendError::ValidationError("Title cannot be empty".to_string()));
    }
    validate_datetime_and_coords(
        CalendarDateTime::new(
            params.query_year, params.query_month, params.query_day,
            params.query_hour, params.query_minute, params.query_second,
        ),
        GeoCoordinate::new(params.latitude, params.longitude),
    )?;
    let timezone_id = resolve_timezone_or_fallback(params.timezone_id, params.latitude, params.longitude, fallback_tz);
    let querent_profile_id = params
        .querent_profile_id
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    Ok(HoraryQuestion {
        id,
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
        querent_profile_id,
        result_notes: params.result_notes.unwrap_or(fallback_notes),
        created_at,
        updated_at: Utc::now().to_rfc3339(),
    })
}

impl<P, H> FortuneBackendService<P, H>
where
    P: ProfileRepositoryTrait,
    H: HoraryRepositoryTrait,
{
    pub fn with_repositories(pool: DbPool, profile_repo: P, horary_repo: H) -> Self {
        Self { pool, profile_repo, horary_repo }
    }

    pub(crate) fn lock_conn(&self) -> Result<MutexGuard<'_, Connection>, FortuneBackendError> {
        self.pool.lock().map_err(db_err)
    }

    pub fn create_profile(&self, params: CreateProfileParams) -> Result<Profile, FortuneBackendError> {
        let profile = build_profile(Uuid::new_v4().to_string(), params, Utc::now().to_rfc3339(), "UTC", String::new())?;
        let conn = self.lock_conn()?;
        self.profile_repo.insert(&conn, &profile).map_err(db_err)?;
        Ok(profile)
    }

    pub fn get_profile(&self, id: &str) -> Result<Option<Profile>, FortuneBackendError> {
        let conn = self.lock_conn()?;
        self.profile_repo.find_by_id(&conn, id).map_err(db_err)
    }

    pub fn list_profiles(&self, search: Option<&str>) -> Result<Vec<Profile>, FortuneBackendError> {
        let conn = self.lock_conn()?;
        self.profile_repo.list_all(&conn, search).map_err(db_err)
    }

    pub fn update_profile(&self, id: &str, params: UpdateProfileParams) -> Result<Profile, FortuneBackendError> {
        let conn = self.lock_conn()?;
        let existing = self.profile_repo
            .find_by_id(&conn, id)
            .map_err(db_err)?
            .ok_or_else(|| FortuneBackendError::NotFoundError(format!("Profile {} not found", id)))?;
        let updated = build_profile(id.to_string(), params, existing.created_at, &existing.timezone_id, existing.notes)?;
        self.profile_repo.update(&conn, &updated).map_err(db_err)?;
        Ok(updated)
    }

    pub fn delete_profile(&self, id: &str) -> Result<bool, FortuneBackendError> {
        let conn = self.lock_conn()?;
        self.profile_repo.delete(&conn, id).map_err(db_err)
    }

    pub fn create_horary(&self, params: CreateHoraryParams) -> Result<HoraryQuestion, FortuneBackendError> {
        let item = build_horary(Uuid::new_v4().to_string(), params, Utc::now().to_rfc3339(), "UTC", String::new())?;
        let conn = self.lock_conn()?;
        self.horary_repo.insert(&conn, &item).map_err(db_err)?;
        Ok(item)
    }

    pub fn get_horary(&self, id: &str) -> Result<Option<HoraryQuestion>, FortuneBackendError> {
        let conn = self.lock_conn()?;
        self.horary_repo.find_by_id(&conn, id).map_err(db_err)
    }

    pub fn list_horary(
        &self,
        category: Option<&str>,
        querent_id: Option<&str>,
    ) -> Result<Vec<HoraryQuestion>, FortuneBackendError> {
        let conn = self.lock_conn()?;
        self.horary_repo.list_all(&conn, category, querent_id).map_err(db_err)
    }

    pub fn update_horary(&self, id: &str, params: UpdateHoraryParams) -> Result<HoraryQuestion, FortuneBackendError> {
        let conn = self.lock_conn()?;
        let existing = self.horary_repo
            .find_by_id(&conn, id)
            .map_err(db_err)?
            .ok_or_else(|| FortuneBackendError::NotFoundError(format!("Horary {} not found", id)))?;
        let updated = build_horary(id.to_string(), params, existing.created_at, &existing.timezone_id, existing.result_notes)?;
        self.horary_repo.update(&conn, &updated).map_err(db_err)?;
        Ok(updated)
    }

    pub fn delete_horary(&self, id: &str) -> Result<bool, FortuneBackendError> {
        let conn = self.lock_conn()?;
        self.horary_repo.delete(&conn, id).map_err(db_err)
    }
}
