use fortune_core::celestial::{CalendarDateTime, GeoCoordinate};
use fortune_core::{
    calculate_fortune, FortuneCalculationInput, FortuneCalculationOutput, Gender, WesternHouseSystem,
};

use crate::markdown::{format_horary_western_markdown, format_prashna_vedic_markdown};
use crate::models::HoraryQuestion;
use crate::repository::{HoraryRepositoryTrait, ProfileRepositoryTrait};
use super::calculation::parse_gender;
use super::{FortuneBackendError, FortuneBackendService};

impl<P, H> FortuneBackendService<P, H>
where
    P: ProfileRepositoryTrait,
    H: HoraryRepositoryTrait,
{
    fn load_horary_and_chart(
        &self,
        id: &str,
        house_system_name: Option<&str>,
    ) -> Result<(HoraryQuestion, FortuneCalculationOutput), FortuneBackendError> {
        let item = self
            .get_horary(id)?
            .ok_or_else(|| FortuneBackendError::NotFoundError(format!("Horary {} not found", id)))?;
        let western_house_system = house_system_name
            .and_then(WesternHouseSystem::from_str_name)
            .or(Some(WesternHouseSystem::Regiomontanus));
        let gender = item
            .querent_profile_id
            .as_deref()
            .and_then(|qid| self.get_profile(qid).ok().flatten())
            .map_or(Gender::Male, |p| parse_gender(&p.gender));

        let input = FortuneCalculationInput {
            year: item.query_year,
            month: item.query_month,
            day: item.query_day,
            hour: item.query_hour,
            minute: item.query_minute,
            second: item.query_second,
            latitude: item.latitude,
            longitude: item.longitude,
            gender,
            western_house_system,
            timezone_id: Some(item.timezone_id.clone()),
            is_horary: true,
            saju_options: Default::default(),
        };
        let chart = calculate_fortune(input).map_err(FortuneBackendError::CalculationError)?;
        Ok((item, chart))
    }

    pub fn calculate_horary_chart(&self, id: &str) -> Result<FortuneCalculationOutput, FortuneBackendError> {
        self.calculate_horary_chart_with_house(id, Some("Regiomontanus"))
    }

    pub fn calculate_horary_chart_with_house(
        &self,
        id: &str,
        house_system_name: Option<&str>,
    ) -> Result<FortuneCalculationOutput, FortuneBackendError> {
        Ok(self.load_horary_and_chart(id, house_system_name)?.1)
    }

    pub fn export_horary_western_markdown(&self, id: &str) -> Result<String, FortuneBackendError> {
        let (item, chart) = self.load_horary_and_chart(id, Some("Regiomontanus"))?;
        Ok(format_horary_western_markdown(&item, &chart))
    }

    pub fn export_prashna_vedic_markdown(&self, id: &str) -> Result<String, FortuneBackendError> {
        let (item, chart) = self.load_horary_and_chart(id, Some("Regiomontanus"))?;
        Ok(format_prashna_vedic_markdown(&item, &chart))
    }

    pub fn calculate_direct(
        &self,
        dt: CalendarDateTime,
        loc: GeoCoordinate,
        is_male: bool,
    ) -> Result<FortuneCalculationOutput, FortuneBackendError> {
        let input = FortuneCalculationInput {
            year: dt.year,
            month: dt.month,
            day: dt.day,
            hour: dt.hour,
            minute: dt.minute,
            second: dt.second,
            latitude: loc.latitude,
            longitude: loc.longitude,
            gender: if is_male { Gender::Male } else { Gender::Female },
            western_house_system: None,
            timezone_id: None,
            is_horary: false,
            saju_options: Default::default(),
        };
        calculate_fortune(input).map_err(FortuneBackendError::CalculationError)
    }
}
