mod horary;
mod predictive;

#[cfg(test)]
mod tests;

use crate::models::{CreateProfileParams, UpdateProfileParams};
use crate::service::{FortuneBackendError, FortuneBackendService};
use std::sync::Arc;

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum FortuneBackendFfiError {
    #[error("Database error: {detail}")]
    DatabaseError { detail: String },
    #[error("Not found: {detail}")]
    NotFoundError { detail: String },
    #[error("Calculation error: {detail}")]
    CalculationError { detail: String },
    #[error("Serialization error: {detail}")]
    SerializationError { detail: String },
    #[error("Validation error: {detail}")]
    ValidationError { detail: String },
}

impl From<FortuneBackendError> for FortuneBackendFfiError {
    fn from(err: FortuneBackendError) -> Self {
        match err {
            FortuneBackendError::DatabaseError(m) => FortuneBackendFfiError::DatabaseError { detail: m },
            FortuneBackendError::NotFoundError(m) => FortuneBackendFfiError::NotFoundError { detail: m },
            FortuneBackendError::CalculationError(m) => FortuneBackendFfiError::CalculationError { detail: m },
            FortuneBackendError::SerializationError(m) => FortuneBackendFfiError::SerializationError { detail: m },
            FortuneBackendError::ValidationError(m) => FortuneBackendFfiError::ValidationError { detail: m },
        }
    }
}

#[inline]
pub(crate) fn from_json<T: serde::de::DeserializeOwned>(
    json_str: &str,
    entity: &str,
) -> Result<T, FortuneBackendFfiError> {
    serde_json::from_str(json_str).map_err(|e| FortuneBackendFfiError::SerializationError {
        detail: format!("Invalid {} JSON: {}", entity, e),
    })
}

#[inline]
pub(crate) fn to_json<T: serde::Serialize>(
    data: &T,
    entity: &str,
) -> Result<String, FortuneBackendFfiError> {
    serde_json::to_string(data).map_err(|e| FortuneBackendFfiError::SerializationError {
        detail: format!("Failed to serialize {}: {}", entity, e),
    })
}

#[derive(uniffi::Object)]
pub struct FortuneClient {
    pub(crate) service: FortuneBackendService,
}

#[uniffi::export]
impl FortuneClient {
    #[uniffi::constructor]
    pub fn new(db_path: String) -> Result<Arc<Self>, FortuneBackendFfiError> {
        let service = FortuneBackendService::new(&db_path)?;
        Ok(Arc::new(Self { service }))
    }

    pub fn create_profile(&self, params_json: String) -> Result<String, FortuneBackendFfiError> {
        let params: CreateProfileParams = from_json(&params_json, "create profile params")?;
        to_json(&self.service.create_profile(params)?, "profile")
    }

    pub fn get_profile(&self, id: String) -> Result<Option<String>, FortuneBackendFfiError> {
        self.service.get_profile(&id)?.map(|p| to_json(&p, "profile")).transpose()
    }

    pub fn list_profiles(&self, search: Option<String>) -> Result<String, FortuneBackendFfiError> {
        to_json(&self.service.list_profiles(search.as_deref())?, "profiles")
    }

    pub fn update_profile(&self, id: String, params_json: String) -> Result<String, FortuneBackendFfiError> {
        let params: UpdateProfileParams = from_json(&params_json, "update profile params")?;
        to_json(&self.service.update_profile(&id, params)?, "profile")
    }

    pub fn delete_profile(&self, id: String) -> Result<bool, FortuneBackendFfiError> {
        Ok(self.service.delete_profile(&id)?)
    }

    pub fn calculate_profile_chart(&self, id: String) -> Result<String, FortuneBackendFfiError> {
        to_json(&self.service.calculate_profile_chart(&id)?, "chart")
    }

    pub fn calculate_profile_chart_with_house(
        &self,
        id: String,
        house_system: Option<String>,
    ) -> Result<String, FortuneBackendFfiError> {
        to_json(&self.service.calculate_profile_chart_with_house(&id, house_system.as_deref())?, "chart")
    }

    pub fn calculate_profile_saju(&self, id: String) -> Result<String, FortuneBackendFfiError> {
        to_json(&self.service.calculate_profile_saju(&id)?, "saju output")
    }

    pub fn calculate_profile_western(
        &self,
        id: String,
        house_system: Option<String>,
    ) -> Result<String, FortuneBackendFfiError> {
        to_json(&self.service.calculate_profile_western(&id, house_system.as_deref())?, "western output")
    }

    pub fn calculate_profile_vedic(&self, id: String) -> Result<String, FortuneBackendFfiError> {
        to_json(&self.service.calculate_profile_vedic(&id)?, "vedic output")
    }
}

#[uniffi::export]
pub fn fortune_backend_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
