use crate::models::{CreateHoraryParams, CreateProfileParams, UpdateHoraryParams, UpdateProfileParams};
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

#[derive(uniffi::Object)]
pub struct FortuneClient {
    service: FortuneBackendService,
}

#[uniffi::export]
impl FortuneClient {
    #[uniffi::constructor]
    pub fn new(db_path: String) -> Result<Arc<Self>, FortuneBackendFfiError> {
        let service = FortuneBackendService::new(&db_path)?;
        Ok(Arc::new(Self { service }))
    }

    pub fn create_profile(&self, params_json: String) -> Result<String, FortuneBackendFfiError> {
        let params: CreateProfileParams = serde_json::from_str(&params_json).map_err(|e| {
            FortuneBackendFfiError::SerializationError {
                detail: format!("Invalid JSON: {}", e),
            }
        })?;

        let profile = self.service.create_profile(params)?;
        serde_json::to_string(&profile).map_err(|e| {
            FortuneBackendFfiError::SerializationError {
                detail: format!("Failed to serialize profile: {}", e),
            }
        })
    }

    pub fn get_profile(&self, id: String) -> Result<Option<String>, FortuneBackendFfiError> {
        let profile = self.service.get_profile(&id)?;
        match profile {
            Some(p) => {
                let json = serde_json::to_string(&p).map_err(|e| {
                    FortuneBackendFfiError::SerializationError {
                        detail: format!("Failed to serialize profile: {}", e),
                    }
                })?;
                Ok(Some(json))
            }
            None => Ok(None),
        }
    }

    pub fn list_profiles(&self, search: Option<String>) -> Result<String, FortuneBackendFfiError> {
        let profiles = self.service.list_profiles(search.as_deref())?;
        serde_json::to_string(&profiles).map_err(|e| {
            FortuneBackendFfiError::SerializationError {
                detail: format!("Failed to serialize profiles: {}", e),
            }
        })
    }

    pub fn update_profile(
        &self,
        id: String,
        params_json: String,
    ) -> Result<String, FortuneBackendFfiError> {
        let params: UpdateProfileParams = serde_json::from_str(&params_json).map_err(|e| {
            FortuneBackendFfiError::SerializationError {
                detail: format!("Invalid JSON: {}", e),
            }
        })?;

        let profile = self.service.update_profile(&id, params)?;
        serde_json::to_string(&profile).map_err(|e| {
            FortuneBackendFfiError::SerializationError {
                detail: format!("Failed to serialize profile: {}", e),
            }
        })
    }

    pub fn delete_profile(&self, id: String) -> Result<bool, FortuneBackendFfiError> {
        Ok(self.service.delete_profile(&id)?)
    }

    pub fn calculate_profile_chart(&self, id: String) -> Result<String, FortuneBackendFfiError> {
        let chart = self.service.calculate_profile_chart(&id)?;
        serde_json::to_string(&chart).map_err(|e| {
            FortuneBackendFfiError::SerializationError {
                detail: format!("Failed to serialize chart: {}", e),
            }
        })
    }

    pub fn export_profile_markdown(&self, id: String) -> Result<String, FortuneBackendFfiError> {
        Ok(self.service.export_profile_markdown(&id)?)
    }

    pub fn create_horary(&self, params_json: String) -> Result<String, FortuneBackendFfiError> {
        let params: CreateHoraryParams = serde_json::from_str(&params_json).map_err(|e| {
            FortuneBackendFfiError::SerializationError {
                detail: format!("Invalid JSON: {}", e),
            }
        })?;

        let horary = self.service.create_horary(params)?;
        serde_json::to_string(&horary).map_err(|e| {
            FortuneBackendFfiError::SerializationError {
                detail: format!("Failed to serialize horary: {}", e),
            }
        })
    }

    pub fn get_horary(&self, id: String) -> Result<Option<String>, FortuneBackendFfiError> {
        let horary = self.service.get_horary(&id)?;
        match horary {
            Some(h) => {
                let json = serde_json::to_string(&h).map_err(|e| {
                    FortuneBackendFfiError::SerializationError {
                        detail: format!("Failed to serialize horary: {}", e),
                    }
                })?;
                Ok(Some(json))
            }
            None => Ok(None),
        }
    }

    pub fn list_horary(
        &self,
        category: Option<String>,
        querent_id: Option<String>,
    ) -> Result<String, FortuneBackendFfiError> {
        let items = self.service.list_horary(category.as_deref(), querent_id.as_deref())?;
        serde_json::to_string(&items).map_err(|e| {
            FortuneBackendFfiError::SerializationError {
                detail: format!("Failed to serialize horary list: {}", e),
            }
        })
    }

    pub fn update_horary(
        &self,
        id: String,
        params_json: String,
    ) -> Result<String, FortuneBackendFfiError> {
        let params: UpdateHoraryParams = serde_json::from_str(&params_json).map_err(|e| {
            FortuneBackendFfiError::SerializationError {
                detail: format!("Invalid JSON: {}", e),
            }
        })?;

        let horary = self.service.update_horary(&id, params)?;
        serde_json::to_string(&horary).map_err(|e| {
            FortuneBackendFfiError::SerializationError {
                detail: format!("Failed to serialize horary: {}", e),
            }
        })
    }

    pub fn delete_horary(&self, id: String) -> Result<bool, FortuneBackendFfiError> {
        Ok(self.service.delete_horary(&id)?)
    }

    pub fn calculate_horary_chart(&self, id: String) -> Result<String, FortuneBackendFfiError> {
        let chart = self.service.calculate_horary_chart(&id)?;
        serde_json::to_string(&chart).map_err(|e| {
            FortuneBackendFfiError::SerializationError {
                detail: format!("Failed to serialize chart: {}", e),
            }
        })
    }

    pub fn export_horary_markdown(&self, id: String) -> Result<String, FortuneBackendFfiError> {
        Ok(self.service.export_horary_markdown(&id)?)
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
    ) -> Result<String, FortuneBackendFfiError> {
        let chart = self.service.calculate_direct(
            year, month, day, hour, minute, second, latitude, longitude, is_male,
        )?;
        serde_json::to_string(&chart).map_err(|e| {
            FortuneBackendFfiError::SerializationError {
                detail: format!("Failed to serialize chart: {}", e),
            }
        })
    }
}

#[uniffi::export]
pub fn fortune_backend_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_ffi() {
        assert_eq!(fortune_backend_version(), "0.1.0");

        let client = FortuneClient::new(":memory:".to_string()).expect("client init failed");

        let json = client
            .calculate_direct(2024, 3, 20, 12, 0, 0.0, 37.5665, 126.9780, true)
            .expect("direct calculation failed");

        assert!(json.contains("\"timezone_id\""));
        assert!(json.contains("\"western\""));
        assert!(json.contains("\"vedic\""));
        assert!(json.contains("\"saju\""));
    }

    #[test]
    fn test_client_ffi_crud() {
        let client = FortuneClient::new(":memory:".to_string()).expect("client init failed");

        // Profile test
        let create_profile_json = r#"{
            "name": "홍길동",
            "gender": "male",
            "birth_year": 1990,
            "birth_month": 5,
            "birth_day": 15,
            "birth_hour": 9,
            "birth_minute": 30,
            "latitude": 37.5665,
            "longitude": 126.9780
        }"#;

        let profile_str = client
            .create_profile(create_profile_json.to_string())
            .expect("create profile failed");
        assert!(profile_str.contains("홍길동"));

        let profile: serde_json::Value = serde_json::from_str(&profile_str).unwrap();
        let profile_id = profile["id"].as_str().unwrap().to_string();

        let found = client
            .get_profile(profile_id.clone())
            .expect("get profile failed")
            .expect("should find profile");
        assert!(found.contains("홍길동"));

        let list = client
            .list_profiles(Some("길동".to_string()))
            .expect("list profiles failed");
        assert!(list.contains("홍길동"));

        let chart_json = client
            .calculate_profile_chart(profile_id.clone())
            .expect("calc profile chart failed");
        assert!(chart_json.contains("\"western\""));

        let md = client
            .export_profile_markdown(profile_id.clone())
            .expect("export profile md failed");
        assert!(md.contains("홍길동"));
        assert!(md.contains("사주팔자"));

        // Horary test
        let create_horary_json = format!(
            r#"{{
                "title": "이직 문의",
                "category": "career",
                "question_text": "올해 안에 이직이 가능할까요?",
                "query_year": 2026,
                "query_month": 9,
                "query_day": 21,
                "query_hour": 16,
                "query_minute": 0,
                "query_second": 0.0,
                "latitude": 37.5665,
                "longitude": 126.9780,
                "querent_profile_id": "{}"
            }}"#,
            profile_id
        );

        let horary_str = client
            .create_horary(create_horary_json)
            .expect("create horary failed");
        assert!(horary_str.contains("이직 문의"));

        let horary: serde_json::Value = serde_json::from_str(&horary_str).unwrap();
        let horary_id = horary["id"].as_str().unwrap().to_string();

        let horary_chart_json = client
            .calculate_horary_chart(horary_id.clone())
            .expect("calc horary chart failed");
        assert!(horary_chart_json.contains("\"ephemeris\""));

        let horary_md = client
            .export_horary_markdown(horary_id.clone())
            .expect("export horary md failed");
        assert!(horary_md.contains("이직 문의"));

        assert!(client.delete_horary(horary_id).expect("delete horary failed"));
        assert!(client.delete_profile(profile_id).expect("delete profile failed"));
    }
}
