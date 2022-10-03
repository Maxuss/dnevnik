use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Payload<T> {
    pub payload: T,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StudentAttendance {
    /// Date at which this attendance happened
    pub date: NaiveDate,
    /// Visits for this attendance
    pub visits: Vec<StudentVisit>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StudentVisit {
    /// Time of entrance to the school
    #[serde(rename = "in")]
    pub entrance: String,
    /// Time of exit out of the school
    #[serde(rename = "out")]
    pub exit: String,
    /// Time spent inside school
    pub duration: String,
    /// Address of the building
    pub address: String,
    /// Unknown, all of the values I've witnessed were `COMMON`
    #[serde(rename = "type")]
    pub visit_type: String,
    /// If this visit has a warning, whatever it means
    pub is_warning: bool,
    /// Short name of the school building
    pub short_name: String,
}
