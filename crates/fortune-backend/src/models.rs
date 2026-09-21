use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub gender: String, // "male" or "female"
    pub birth_year: i32,
    pub birth_month: u32,
    pub birth_day: u32,
    pub birth_hour: u32,
    pub birth_minute: u32,
    pub latitude: f64,
    pub longitude: f64,
    pub timezone_id: String,
    pub notes: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProfileParams {
    pub name: String,
    pub gender: String,
    pub birth_year: i32,
    pub birth_month: u32,
    pub birth_day: u32,
    pub birth_hour: u32,
    pub birth_minute: u32,
    pub latitude: f64,
    pub longitude: f64,
    pub timezone_id: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProfileParams {
    pub name: String,
    pub gender: String,
    pub birth_year: i32,
    pub birth_month: u32,
    pub birth_day: u32,
    pub birth_hour: u32,
    pub birth_minute: u32,
    pub latitude: f64,
    pub longitude: f64,
    pub timezone_id: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HoraryQuestion {
    pub id: String,
    pub title: String,
    pub category: String,
    pub question_text: String,
    pub query_year: i32,
    pub query_month: u32,
    pub query_day: u32,
    pub query_hour: u32,
    pub query_minute: u32,
    pub query_second: f64,
    pub latitude: f64,
    pub longitude: f64,
    pub timezone_id: String,
    pub querent_profile_id: Option<String>,
    pub result_notes: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateHoraryParams {
    pub title: String,
    pub category: String,
    pub question_text: String,
    pub query_year: i32,
    pub query_month: u32,
    pub query_day: u32,
    pub query_hour: u32,
    pub query_minute: u32,
    pub query_second: f64,
    pub latitude: f64,
    pub longitude: f64,
    pub timezone_id: Option<String>,
    pub querent_profile_id: Option<String>,
    pub result_notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateHoraryParams {
    pub title: String,
    pub category: String,
    pub question_text: String,
    pub query_year: i32,
    pub query_month: u32,
    pub query_day: u32,
    pub query_hour: u32,
    pub query_minute: u32,
    pub query_second: f64,
    pub latitude: f64,
    pub longitude: f64,
    pub timezone_id: Option<String>,
    pub querent_profile_id: Option<String>,
    pub result_notes: Option<String>,
}
