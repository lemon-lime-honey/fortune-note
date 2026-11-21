use fortune_core::{CalendarDate, CalendarDateTime, GeoCoordinate};
use serde::Deserialize;

use crate::service::PredictiveSuiteRequest;
use super::{from_json, to_json, FortuneBackendFfiError, FortuneClient};

#[derive(Deserialize)]
struct PredictiveSuiteJsonRequest {
    target_year: i32,
    target_month: u32,
    target_day: u32,
    #[serde(default)]
    target_hour: Option<u32>,
    #[serde(default)]
    target_minute: Option<u32>,
    #[serde(default)]
    house_system: Option<String>,
}

#[derive(Deserialize)]
struct DirectCalcRequest {
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    #[serde(default)]
    second: f64,
    latitude: f64,
    longitude: f64,
    is_male: bool,
}

#[derive(Deserialize)]
struct ProfectionRequest {
    birth_year: i32,
    birth_month: u32,
    birth_day: u32,
    target_year: i32,
    target_month: u32,
    target_day: u32,
    natal_ascendant: f64,
    natal_planets: Vec<(String, f64)>,
}

#[derive(Deserialize)]
struct SolarArcRequest {
    birth_year: i32,
    birth_month: u32,
    birth_day: u32,
    target_year: i32,
    target_month: u32,
    target_day: u32,
    natal_planets: Vec<(String, f64)>,
}

#[uniffi::export]
impl FortuneClient {
    pub fn export_profile_saju_markdown(&self, id: String) -> Result<String, FortuneBackendFfiError> {
        Ok(self.service.export_profile_saju_markdown(&id)?)
    }

    pub fn export_profile_western_markdown(
        &self,
        id: String,
        house_system: Option<String>,
    ) -> Result<String, FortuneBackendFfiError> {
        Ok(self.service.export_profile_western_markdown(&id, house_system.as_deref())?)
    }

    pub fn export_profile_vedic_markdown(&self, id: String) -> Result<String, FortuneBackendFfiError> {
        Ok(self.service.export_profile_vedic_markdown(&id)?)
    }

    pub fn calculate_profile_predictive_suite(
        &self,
        id: String,
        request_json: String,
    ) -> Result<String, FortuneBackendFfiError> {
        let req: PredictiveSuiteJsonRequest = from_json(&request_json, "predictive suite request")?;
        let suite = self.service.calculate_profile_predictive_suite(&PredictiveSuiteRequest {
            profile_id: &id,
            target_year: req.target_year,
            target_month: req.target_month,
            target_day: req.target_day,
            target_hour: req.target_hour,
            target_minute: req.target_minute,
            house_system_name: req.house_system.as_deref(),
        })?;
        to_json(&suite, "predictive suite")
    }

    pub fn calculate_annual_profection(&self, request_json: String) -> Result<String, FortuneBackendFfiError> {
        let req: ProfectionRequest = from_json(&request_json, "annual profection request")?;
        let tuples: Vec<(&str, f64)> = req.natal_planets.iter().map(|(n, l)| (n.as_str(), *l)).collect();
        let res = fortune_core::western::predictive::calculate_annual_profection(
            CalendarDate::new(req.birth_year, req.birth_month, req.birth_day),
            CalendarDate::new(req.target_year, req.target_month, req.target_day),
            req.natal_ascendant,
            &tuples,
        );
        to_json(&res, "annual profection")
    }

    pub fn calculate_solar_arc(&self, request_json: String) -> Result<String, FortuneBackendFfiError> {
        let req: SolarArcRequest = from_json(&request_json, "solar arc request")?;
        let tuples: Vec<(&str, f64)> = req.natal_planets.iter().map(|(n, l)| (n.as_str(), *l)).collect();
        let (arc, directed) = fortune_core::western::predictive::calculate_solar_arc(
            req.birth_year, req.birth_month, req.birth_day,
            req.target_year, req.target_month, req.target_day, &tuples,
        );
        to_json(&serde_json::json!({ "solar_arc_degrees": arc, "directed_planets": directed }), "solar arc")
    }

    pub fn calculate_direct(&self, request_json: String) -> Result<String, FortuneBackendFfiError> {
        let req: DirectCalcRequest = from_json(&request_json, "direct calculation request")?;
        let chart = self.service.calculate_direct(
            CalendarDateTime::new(req.year, req.month, req.day, req.hour, req.minute, req.second),
            GeoCoordinate::new(req.latitude, req.longitude),
            req.is_male,
        )?;
        to_json(&chart, "chart")
    }

    pub fn export_backup_json(&self) -> Result<String, FortuneBackendFfiError> {
        Ok(self.service.export_backup_json()?)
    }

    pub fn import_backup_json(
        &self,
        json_str: String,
        replace_existing: bool,
    ) -> Result<String, FortuneBackendFfiError> {
        to_json(&self.service.import_backup_json(&json_str, replace_existing)?, "import summary")
    }
}
