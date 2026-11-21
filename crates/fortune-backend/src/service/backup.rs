use chrono::Utc;
use fortune_core::{CalendarDateTime, GeoCoordinate};
use rusqlite::{Result as SqlResult, Transaction};

use crate::models::{BackupData, HoraryQuestion, ImportSummary, Profile};
use crate::repository::{HoraryRepositoryTrait, ProfileRepositoryTrait};
use super::{db_err, validate_datetime_and_coords, FortuneBackendError, FortuneBackendService};

fn validate_backup_profile(p: &Profile) -> Result<(), FortuneBackendError> {
    if p.id.trim().is_empty() || p.name.trim().is_empty() {
        return Err(FortuneBackendError::ValidationError("Backup profile id/name cannot be empty".into()));
    }
    if !p.gender.eq_ignore_ascii_case("male") && !p.gender.eq_ignore_ascii_case("female") {
        return Err(FortuneBackendError::ValidationError(format!("Invalid backup gender: {}", p.gender)));
    }
    validate_datetime_and_coords(
        CalendarDateTime::new(p.birth_year, p.birth_month, p.birth_day, p.birth_hour, p.birth_minute, p.birth_second),
        GeoCoordinate::new(p.latitude, p.longitude),
    )
}

fn validate_backup_horary(h: &HoraryQuestion) -> Result<(), FortuneBackendError> {
    if h.id.trim().is_empty() || h.title.trim().is_empty() {
        return Err(FortuneBackendError::ValidationError("Backup horary id/title cannot be empty".into()));
    }
    validate_datetime_and_coords(
        CalendarDateTime::new(h.query_year, h.query_month, h.query_day, h.query_hour, h.query_minute, h.query_second),
        GeoCoordinate::new(h.latitude, h.longitude),
    )
}

fn upsert_row<FUpdate, FInsert>(
    tx: &Transaction<'_>,
    exists_sql: &str,
    id: &str,
    update_fn: FUpdate,
    insert_fn: FInsert,
) -> Result<(), FortuneBackendError>
where
    FUpdate: FnOnce() -> SqlResult<bool>,
    FInsert: FnOnce() -> SqlResult<()>,
{
    // Always verify existence within the active transaction so duplicate IDs in a single backup update safely.
    let exists = tx.query_row(exists_sql, [id], |r| Ok(r.get::<_, i64>(0)? > 0)).map_err(db_err)?;
    if exists {
        update_fn().map(|_| ()).map_err(db_err)
    } else {
        insert_fn().map_err(db_err)
    }
}

impl<P, H> FortuneBackendService<P, H>
where
    P: ProfileRepositoryTrait,
    H: HoraryRepositoryTrait,
{
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
            return Err(FortuneBackendError::ValidationError("fortune-note 앱의 백업 데이터가 아닙니다".to_string()));
        }

        for p in &backup.profiles {
            validate_backup_profile(p)?;
        }
        for h in &backup.horary_questions {
            validate_backup_horary(h)?;
        }

        let mut conn = self.lock_conn()?;
        let tx = conn.transaction().map_err(db_err)?;

        if replace_existing {
            tx.execute("DELETE FROM horary_questions", []).map_err(db_err)?;
            tx.execute("DELETE FROM profiles", []).map_err(db_err)?;
        }

        for p in &backup.profiles {
            upsert_row(
                &tx,
                "SELECT COUNT(1) FROM profiles WHERE id = ?1",
                &p.id,
                || self.profile_repo.update(&tx, p),
                || self.profile_repo.insert(&tx, p),
            )?;
        }
        for h in &backup.horary_questions {
            upsert_row(
                &tx,
                "SELECT COUNT(1) FROM horary_questions WHERE id = ?1",
                &h.id,
                || self.horary_repo.update(&tx, h),
                || self.horary_repo.insert(&tx, h),
            )?;
        }

        tx.commit().map_err(db_err)?;

        let imported_profiles = backup.profiles.len() as u32;
        let imported_horary = backup.horary_questions.len() as u32;
        Ok(ImportSummary {
            imported_profiles,
            imported_horary,
            message: format!("프로필 {}건, 질문 {}건이 성공적으로 복원되었습니다", imported_profiles, imported_horary),
        })
    }
}
