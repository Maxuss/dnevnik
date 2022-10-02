//! Module that exports most needed structures for this crate
pub use crate::diary::Diary;
pub use crate::model::attendance::{StudentAttendance, StudentVisit};
pub use crate::model::hw::{HomeworkAttachment, HomeworkEntry, HomeworkSubject, StudentHomework};
pub use crate::model::lessons::{
    AcademicYear, LessonActivity, LessonInstance, Schedule, ScheduleActivity,
};
pub use crate::model::marks::{GlobalAverageGrade, LocalGradeMark, LocalGradeMarkValue};
pub use crate::model::{Account, StudentDetails, StudentProfile};
