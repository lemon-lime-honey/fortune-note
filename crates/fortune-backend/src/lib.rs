//! Backend service and persistence layer for fortune-note.
//!
//! Provides:
//! - Embedded SQLite database for profiles and horary questions (`db`, `repository`)
//! - Orchestration service integrating `fortune-core` (`service`)
//! - LLM markdown export formatting (`markdown`)
//! - UniFFI mobile bindings boundary (`ffi`)

pub mod db;
pub mod ffi;
pub mod markdown;
pub mod models;
pub mod repository;
pub mod service;

uniffi::setup_scaffolding!();

pub use db::{init_db, DbPool};
pub use ffi::{fortune_backend_version, FortuneBackendFfiError, FortuneClient};
pub use markdown::{
    format_horary_western_markdown, format_prashna_vedic_markdown,
    format_profile_saju_markdown, format_profile_vedic_markdown, format_profile_western_markdown,
};
pub use models::{
    BackupData, CreateHoraryParams, CreateProfileParams, HoraryQuestion, ImportSummary, Profile,
    UpdateHoraryParams, UpdateProfileParams,
};
pub use repository::{HoraryRepository, ProfileRepository};
pub use service::{FortuneBackendError, FortuneBackendService};
