use crate::model::lessons::MarkInstance;
use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct GlobalAverageGrade {
    /// Name of the subject this grade belongs to
    pub subject_name: String,
    /// Five-based average value for this grade
    #[serde(rename = "avg_five")]
    pub five: String,
    /// Hundred-based average value for this grade
    #[serde(rename = "avg_hundred")]
    pub hundred: String,
    /// Average grades for the underlying periods
    pub periods: Vec<PeriodAverageGrade>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PeriodAverageGrade {
    /// Name of this period
    pub name: String,
    /// Date at which this period starts
    #[serde(rename = "start_iso")]
    pub start: NaiveDate,
    /// Date at which this period ends
    #[serde(rename = "end_iso")]
    pub end: NaiveDate,
    /// Five-based average value for this grade
    #[serde(rename = "avg_five")]
    pub five: String,
    /// Hundred-based average value for this grade
    #[serde(rename = "avg_hundred")]
    pub hundred: String,
    /// All marks for this period
    pub marks: Vec<MarkInstance>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LocalGradeMark {
    /// ID of this mark
    pub id: u64,
    /// Comment to this mark
    pub comment: String,
    /// Weight of this mark's index
    pub weight: u8,
    /// Whether this mark is an exam mark
    pub is_exam: bool,
    /// Date at which this mark was created
    pub date: NaiveDate,
    /// Whether this mark is a point mark
    pub is_point: bool,
    /// ID of this mark's control form
    pub control_form_id: u64,
    /// System of this grade
    #[serde(rename = "grade_system_type")]
    pub grade_system: String,
    /// Name of the topic this mark belongs to
    pub topic_name: String,
    /// Name of the control form that this mark belongs to
    pub control_form_name: String,
    /// Values for this mark
    pub values: Vec<LocalGradeMarkValue>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LocalGradeMarkValue {
    /// Five based value for this mark
    pub five: f32,
    /// Hundred based value for this mark
    pub hundred: f32,
    /// Original value of this mark
    pub original: String,
}
