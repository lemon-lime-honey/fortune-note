use chrono::Utc;
use rusqlite::{Result as SqlResult, Transaction};

use crate::models::{BackupData, ImportSummary};
use crate::repository::{HoraryRepositoryTrait, ProfileRepositoryTrait};
use super::{db_err, FortuneBackendError, FortuneBackendService};

fn upsert_row<FUpdate, FInsert>(
    tx: &Transaction<'_>,
    exists_sql: &str,
    id: &str,
    replace_existing: bool,
    update_fn: FUpdate,
    insert_fn: FInsert,
) -> Result<(), FortuneBackendError>
where
    FUpdate: FnOnce() -> SqlResult<bool>,
    FInsert: FnOnce() -> SqlResult<()>,
{
    let exists = if replace_existing {
        false
    } else {
        tx.query_row(exists_sql, [id], |r| Ok(r.get::<_, i64>(0)? > 0)).map_err(db_err)?
    };
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
                replace_existing,
                || self.profile_repo.update(&tx, p),
                || self.profile_repo.insert(&tx, p),
            )?;
        }
        for h in &backup.horary_questions {
            upsert_row(
                &tx,
                "SELECT COUNT(1) FROM horary_questions WHERE id = ?1",
                &h.id,
                replace_existing,
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
