use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Teacher {
    /// Last name or surname of the account's owner
    pub last_name: String,
    /// First name of the account's owner
    pub first_name: String,
    /// Middle name or the patronymic of the account's owner
    pub middle_name: String,
    /// Date of birth of this teacher
    pub birth_date: Option<NaiveDate>,
    /// Sex of this teacher
    pub sex: Option<String>,
    /// Unique ID of this teacher
    pub user_id: Option<u64>,
}

impl Teacher {
    pub fn name(&self) -> String {
        let mut name = self.last_name.clone();
        name.push(' ');
        name.push_str(&self.first_name);
        name.push(' ');
        name.push_str(&self.middle_name);
        name
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Grade {
    /// Five-base value of this grade
    #[serde(rename = "five")]
    pub five_based: f32,
    /// Hundred-base value of this grade
    #[serde(rename = "hundred")]
    pub hundred_based: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SystemBasedMarkValue {
    /// Name of this system
    pub name: String,
    /// Unknown, possibly the maximum mark in this system
    pub nmax: f32,
    /// Internal Unique ID for this grade system
    #[serde(rename = "grade_system_id")]
    pub internal_grade_system_id: u64,
    /// Internal name for this grade system
    #[serde(rename = "grade_system_type")]
    pub internal_grade_system_type: Option<String>,
    /// Graded value of this mark, contains the actual integer value in different systems
    pub grade: Grade,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarkInstance {
    /// Unique ID of this mark
    pub id: u64,
    /// String value representation of this mark
    pub value: String,
    /// Different grading system based values for this mark
    #[serde(rename = "values")]
    pub system_values: Vec<SystemBasedMarkValue>,
    /// Extra comment for this mark
    pub comment: Option<String>,
    /// Weight index for this mark
    pub weight: f32,
    /// Unknown, possibly the time when this mark was changed from `point` to an actual mark
    pub point_date: Option<NaiveDateTime>,
    /// Name of the control form that belongs to this mark
    #[serde(rename = "control_form_name")]
    pub cause: String,
    /// Time at which this mark was created
    pub created_at: NaiveDateTime,
    /// Time at which this mark was updated. Equal to `created_at` if the mark was not updated.
    pub updated_at: NaiveDateTime,
    /// Whether this mark is a control examination mark.
    pub is_exam: bool,
    /// Whether this mark is a point, that should be corrected
    pub is_point: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LessonInstance {
    /// A unique ID for this scheduled item
    #[serde(rename = "schedule_item_id")]
    pub schedule_id: u64,
    /// A unique ID for this lesson's subject
    pub subject_id: u64,
    /// Name of this lesson's subject
    pub subject_name: String,
    /// Teacher for this lesson
    pub teacher: Teacher,
    /// All the marks bound to this lesson instance
    pub marks: Vec<MarkInstance>,
    /// Homework for this lesson
    pub homework: String,
    /// Whether this lesson was cancelled
    pub is_cancelled: bool,
    /// Whether you have missed this lesson
    pub is_missed_lesson: bool,
    /// Whether this is a virtual lesson
    pub is_virtual: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Schedule {
    /// Summary for this day's schedule
    pub summary: String,
    /// Date of this schedule
    pub date: NaiveDate,
    /// All lessons and breaks in this schedule
    #[serde(rename = "activities")]
    pub lessons: Vec<ScheduleActivity>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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
    pub is_current: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LessonActivity {
    /// A string containing info for this lesson
    pub info: Option<String>,
    /// UTC time when this lesson begins
    #[serde(rename = "begin_utc")]
    #[serde(with = "chrono::serde::ts_seconds")]
    pub begin: DateTime<Utc>,
    /// UTC time when this lesson ends
    #[serde(rename = "end_utc")]
    #[serde(with = "chrono::serde::ts_seconds")]
    pub end: DateTime<Utc>,
    /// String representation of time when this lesson begins
    #[serde(rename = "begin_time")]
    pub begin_str: String,
    /// String representation of time when this lesson ends
    #[serde(rename = "end_time")]
    pub end_str: String,
    /// Room number where this lesson takes place
    #[serde(rename = "room_number")]
    pub room: String,
    /// Name of the room where this lesson takes place
    pub room_name: String,
    /// Building in which this lesson takes place
    #[serde(rename = "building_name")]
    pub building: String,
    /// Lesson instance for this activity
    #[serde(rename = "lesson")]
    pub subject: LessonInstance,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BreakActivity {
    /// A string containing info about this break. `"Перемена"` usually
    pub info: String,
    /// UTC time when this lesson begins
    #[serde(rename = "begin_utc")]
    #[serde(with = "chrono::serde::ts_seconds")]
    pub begin: DateTime<Utc>,
    /// UTC time when this lesson ends
    #[serde(rename = "end_utc")]
    #[serde(with = "chrono::serde::ts_seconds")]
    pub end: DateTime<Utc>,
    /// Time in seconds that this break takes
    pub duration: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "type")]
pub enum ScheduleActivity {
    /// Means that this activity is a lesson
    Lesson(Box<LessonActivity>),
    /// Means that this activity
    Break(BreakActivity),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FinalMark {
    /// Value of this mark
    pub value: f32,
    /// Type of the grading system
    #[serde(rename = "grade_system_type")]
    pub grade_system: String,
    /// Whether the student was attested this year
    pub attested: bool,
    /// Whether the student had academical debt this year
    #[serde(rename = "academic_debt")]
    pub has_debt: bool,
    /// ID of the subject this mark belongs to
    pub subject_id: u64,
    /// Name of the subject this mark belongs to
    pub subject_name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LessonScheduleItem {
    /// ID of this schedule item
    pub id: u64,
    /// Plan ID of this lesson
    pub plan_id: Option<u64>,
    /// ID of this lesson's subject
    pub subject_id: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModuleTopic {
    /// ID of this topic
    pub id: u64,
    /// Name of this topic's theme
    pub name: String,
    /// Whether this topic is repeated later
    pub repeatable: bool,
    /// ID for this topic's theme frame
    pub theme_frame_id: Option<u64>,
    /// A \# prefixed string containing accent hex color for this topic
    pub color: Option<String>,
    /// Date at which this topic was created
    pub created_at: NaiveDateTime,
    /// Date at which this topic was updated
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlanModule {
    /// ID of this module
    pub id: u64,
    /// Name of this module
    pub name: String,
    /// Internal one-based ordinal ID of this module
    pub ordinal: Option<u64>,
    /// Date at which this module was created
    pub created_at: NaiveDateTime,
    /// Date at which this module was updated
    pub updated_at: NaiveDateTime,
    /// List of all topics for this module
    pub topics: Vec<ModuleTopic>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LessonPlan {
    /// ID of this lesson's plan
    pub id: u64,
    /// Name of this plan
    pub name: String,
    /// ID of this plan's template
    pub template_id: u64,
    /// ID of this plan's subject
    pub subject_id: u64,
    /// ID of this plan's teacher
    pub teacher_id: u64,
    /// Count of lessons in this plan
    pub lesson_count: u64,
    /// Count of modules in this plan
    pub module_count: u64,
    /// Date at which this plan was created
    pub created_at: NaiveDateTime,
    /// Date at which this plan was updated
    pub updated_at: NaiveDateTime,
    /// List, containing all of the modules for this plan
    pub modules: Vec<PlanModule>,
}
