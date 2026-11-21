use crate::models::{CreateHoraryParams, UpdateHoraryParams};
use super::{from_json, to_json, FortuneBackendFfiError, FortuneClient};

#[uniffi::export]
impl FortuneClient {
    pub fn create_horary(&self, params_json: String) -> Result<String, FortuneBackendFfiError> {
        let params: CreateHoraryParams = from_json(&params_json, "create horary params")?;
        to_json(&self.service.create_horary(params)?, "horary")
    }

    pub fn get_horary(&self, id: String) -> Result<Option<String>, FortuneBackendFfiError> {
        self.service.get_horary(&id)?.map(|h| to_json(&h, "horary")).transpose()
    }

    pub fn list_horary(
        &self,
        category: Option<String>,
        querent_id: Option<String>,
    ) -> Result<String, FortuneBackendFfiError> {
        to_json(&self.service.list_horary(category.as_deref(), querent_id.as_deref())?, "horary list")
    }

    pub fn update_horary(&self, id: String, params_json: String) -> Result<String, FortuneBackendFfiError> {
        let params: UpdateHoraryParams = from_json(&params_json, "update horary params")?;
        to_json(&self.service.update_horary(&id, params)?, "horary")
    }

    pub fn delete_horary(&self, id: String) -> Result<bool, FortuneBackendFfiError> {
        Ok(self.service.delete_horary(&id)?)
    }

    pub fn calculate_horary_chart(&self, id: String) -> Result<String, FortuneBackendFfiError> {
        to_json(&self.service.calculate_horary_chart(&id)?, "chart")
    }

    pub fn calculate_horary_chart_with_house(
        &self,
        id: String,
        house_system: Option<String>,
    ) -> Result<String, FortuneBackendFfiError> {
        to_json(&self.service.calculate_horary_chart_with_house(&id, house_system.as_deref())?, "chart")
    }

    pub fn export_horary_western_markdown(&self, id: String) -> Result<String, FortuneBackendFfiError> {
        Ok(self.service.export_horary_western_markdown(&id)?)
    }

    pub fn export_prashna_vedic_markdown(&self, id: String) -> Result<String, FortuneBackendFfiError> {
        Ok(self.service.export_prashna_vedic_markdown(&id)?)
    }
}
