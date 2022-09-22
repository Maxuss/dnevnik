use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AcademicYear {
    /// ID of this year
    pub id: u16,
    /// Description of this academic year
    #[serde(rename = "name")]
    pub description: String,
    /// When this year starts
    pub begin_date: NaiveDate,
    /// When this year starts
    pub end_date: NaiveDate,
    /// Whether this year is the current academic year
    #[serde(rename = "current_year")]
    pub is_current: bool
}