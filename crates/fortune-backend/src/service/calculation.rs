use chrono::{Datelike, Utc};
use fortune_core::celestial::{normalize_western_body_name, CalendarDate};
use fortune_core::western::predictive::{
    calculate_predictive_suite, NatalPredictiveContext, PredictiveSuite, PredictiveTarget,
};
use fortune_core::{
    calculate_fortune, calculate_fortune_saju, calculate_fortune_vedic, calculate_fortune_western,
    resolve_fortune_context, FortuneCalculationInput, FortuneCalculationOutput, Gender,
    SajuCalculationOutput, VedicCalculationOutput, WesternCalculationOutput, WesternHouseSystem,
};

use crate::markdown::{
    format_profile_saju_markdown, format_profile_vedic_markdown,
    format_profile_western_markdown_with_predictive,
};
use crate::models::Profile;
use crate::repository::{HoraryRepositoryTrait, ProfileRepositoryTrait};
use super::{FortuneBackendError, FortuneBackendService};

/// Request payload for computing a profile's unified predictive suite.
#[derive(Debug, Clone, Copy)]
pub struct PredictiveSuiteRequest<'a> {
    pub profile_id: &'a str,
    pub target_year: i32,
    pub target_month: u32,
    pub target_day: u32,
    pub target_hour: Option<u32>,
    pub target_minute: Option<u32>,
    pub house_system_name: Option<&'a str>,
}

pub(crate) fn parse_gender(gender: &str) -> Gender {
    if gender.eq_ignore_ascii_case("female") { Gender::Female } else { Gender::Male }
}

fn profile_to_input(profile: &Profile, house_system_name: Option<&str>) -> FortuneCalculationInput {
    FortuneCalculationInput {
        year: profile.birth_year,
        month: profile.birth_month,
        day: profile.birth_day,
        hour: profile.birth_hour,
        minute: profile.birth_minute,
        second: profile.birth_second,
        latitude: profile.latitude,
        longitude: profile.longitude,
        gender: parse_gender(&profile.gender),
        western_house_system: house_system_name.and_then(WesternHouseSystem::from_str_name),
        timezone_id: Some(profile.timezone_id.clone()),
        is_horary: false,
        saju_options: Default::default(),
    }
}

impl<P, H> FortuneBackendService<P, H>
where
    P: ProfileRepositoryTrait,
    H: HoraryRepositoryTrait,
{
    fn require_profile(&self, id: &str) -> Result<Profile, FortuneBackendError> {
        self.get_profile(id)?
            .ok_or_else(|| FortuneBackendError::NotFoundError(format!("Profile {} not found", id)))
    }

    fn load_profile_and_chart(
        &self,
        id: &str,
        house_system_name: Option<&str>,
    ) -> Result<(Profile, FortuneCalculationOutput), FortuneBackendError> {
        let profile = self.require_profile(id)?;
        let chart = calculate_fortune(profile_to_input(&profile, house_system_name))
            .map_err(FortuneBackendError::CalculationError)?;
        Ok((profile, chart))
    }

    pub fn calculate_profile_chart(&self, id: &str) -> Result<FortuneCalculationOutput, FortuneBackendError> {
        self.calculate_profile_chart_with_house(id, None)
    }

    pub fn calculate_profile_chart_with_house(
        &self,
        id: &str,
        house_system_name: Option<&str>,
    ) -> Result<FortuneCalculationOutput, FortuneBackendError> {
        Ok(self.load_profile_and_chart(id, house_system_name)?.1)
    }

    pub fn calculate_profile_saju(&self, id: &str) -> Result<SajuCalculationOutput, FortuneBackendError> {
        let profile = self.require_profile(id)?;
        calculate_fortune_saju(profile_to_input(&profile, None))
            .map_err(FortuneBackendError::CalculationError)
    }

    pub fn calculate_profile_western(
        &self,
        id: &str,
        house_system_name: Option<&str>,
    ) -> Result<WesternCalculationOutput, FortuneBackendError> {
        let profile = self.require_profile(id)?;
        calculate_fortune_western(profile_to_input(&profile, house_system_name))
            .map_err(FortuneBackendError::CalculationError)
    }

    pub fn calculate_profile_vedic(&self, id: &str) -> Result<VedicCalculationOutput, FortuneBackendError> {
        let profile = self.require_profile(id)?;
        calculate_fortune_vedic(profile_to_input(&profile, None))
            .map_err(FortuneBackendError::CalculationError)
    }

    pub fn export_profile_saju_markdown(&self, id: &str) -> Result<String, FortuneBackendError> {
        let (profile, chart) = self.load_profile_and_chart(id, None)?;
        Ok(format_profile_saju_markdown(&profile, &chart))
    }

    pub fn export_profile_western_markdown(
        &self,
        id: &str,
        house_system_name: Option<&str>,
    ) -> Result<String, FortuneBackendError> {
        let (profile, chart) = self.load_profile_and_chart(id, house_system_name)?;
        let now = Utc::now();
        let suite = self
            .calculate_profile_predictive_suite(&PredictiveSuiteRequest {
                profile_id: id,
                target_year: now.year(),
                target_month: now.month(),
                target_day: now.day(),
                target_hour: None,
                target_minute: None,
                house_system_name,
            })
            .ok();
        Ok(format_profile_western_markdown_with_predictive(&profile, &chart, suite.as_ref()))
    }

    pub fn export_profile_vedic_markdown(&self, id: &str) -> Result<String, FortuneBackendError> {
        let (profile, chart) = self.load_profile_and_chart(id, None)?;
        Ok(format_profile_vedic_markdown(&profile, &chart))
    }

    pub fn calculate_profile_predictive_suite(
        &self,
        req: &PredictiveSuiteRequest<'_>,
    ) -> Result<PredictiveSuite, FortuneBackendError> {
        let profile = self.require_profile(req.profile_id)?;
        let input = profile_to_input(&profile, req.house_system_name);
        let (_, ephem) = resolve_fortune_context(&input).map_err(FortuneBackendError::CalculationError)?;

        let house_sys = input.western_house_system.unwrap_or_default().to_vedaksha_system();
        let house_result = fortune_core::western::compute_houses(
            ephem.ramc,
            profile.latitude,
            ephem.true_obliquity_deg,
            house_sys,
        );

        let normalized: Vec<(String, f64)> = ephem
            .bodies
            .iter()
            .map(|b| (normalize_western_body_name(&b.name), b.tropical_longitude))
            .collect();
        let natal_planets: Vec<(&str, f64)> = normalized.iter().map(|(n, l)| (n.as_str(), *l)).collect();

        let natal = NatalPredictiveContext {
            birth_dt: input.local_datetime(),
            location: input.location(),
            timezone_id: Some(&profile.timezone_id),
            ascendant: house_result.asc,
            midheaven: house_result.mc,
            planets: &natal_planets,
            true_obliquity_deg: Some(ephem.true_obliquity_deg),
        };
        let target = PredictiveTarget {
            date: CalendarDate::new(req.target_year, req.target_month, req.target_day),
            hour: req.target_hour,
            minute: req.target_minute,
            house_system: Some(house_sys),
        };

        Ok(calculate_predictive_suite(&natal, target))
    }
}
