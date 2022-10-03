//! Main module of this crate, allowing access to the diary

use crate::model::attendance::{Payload, StudentAttendance};
use crate::model::hw::{HomeworkAttachment, StudentHomework};
use crate::model::lessons::{
    AcademicYear, FinalMark, LessonInstance, LessonPlan, LessonScheduleItem, Schedule,
};
use crate::model::marks::GlobalAverageGrade;
use crate::model::{StudentProfile, StudentSession};
use anyhow::bail;
use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, REFERER, USER_AGENT};
use reqwest::{Client, ClientBuilder, Url};
use std::io::Cursor;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

pub const GLOBAL_DMR_URL: &str = "https://dnevnik.mos.ru";
pub const CORE_API: &str = "/core/api";
pub const MOBILE_API: &str = "/mobile/api";
pub const LMS_API: &str = "/lms/api";
pub const JERSEY_API: &str = "/jersey/api";
pub const REPORTS_API: &str = "/jersey/api";

lazy_static! {
    pub static ref PROFILE_ENDPOINT: String = format!("{}{}/profile", GLOBAL_DMR_URL, MOBILE_API);
    pub static ref SESSIONS_ENDPOINT: String = format!("{}{}/sessions", GLOBAL_DMR_URL, LMS_API);
    pub static ref ACADEMIC_YEARS_ENDPOINT: String =
        format!("{}{}/academic_years", GLOBAL_DMR_URL, CORE_API);
    pub static ref SCHEDULE_ENDPOINT: String = format!("{}{}/schedule", GLOBAL_DMR_URL, MOBILE_API);
    pub static ref FINAL_MARKS_PREV_YEAR_ENDPOINT: String =
        format!("{}{}/final_marks_prev_year", GLOBAL_DMR_URL, CORE_API);
    pub static ref LESSON_PLANS_ENDPOINT: String =
        format!("{}{}/lesson_plans", GLOBAL_DMR_URL, JERSEY_API);
    pub static ref STUDENT_HOMEWORKS_ENDPOINT: String =
        format!("{}{}/student_homeworks", GLOBAL_DMR_URL, CORE_API);
    pub static ref PROGRESS_ENDPOINT: String =
        format!("{}{}/progress/json", GLOBAL_DMR_URL, REPORTS_API);
    pub static ref VISITS_ENDPOINT: String = format!("{}{}/visits", GLOBAL_DMR_URL, MOBILE_API);
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Diary {
    client: Client,
    auth_token: String,
    pub profile: StudentProfile,
    student_id: u64,
}

#[derive(serde::Serialize)]
struct StudentAuth {
    auth_token: String,
}

impl Diary {
    #[allow(clippy::option_env_unwrap)]
    pub async fn new<S: Into<String>>(token: S) -> anyhow::Result<Self> {
        let str_token = token.into();
        let mut default_headers = HeaderMap::new();
        default_headers.append(
            USER_AGENT,
            HeaderValue::from_str(&format!(
                "Dnevnik-Mos-Rust/{}",
                option_env!("CARGO_PKG_VERSION").unwrap()
            ))?,
        );
        default_headers.append(
            REFERER,
            HeaderValue::from_str("https://dnevnik.mos.ru/diary/")?,
        );
        default_headers.append("Auth-Token", HeaderValue::from_str(&str_token)?);
        default_headers.append(AUTHORIZATION, HeaderValue::from_str(&str_token)?);
        let client = ClientBuilder::new()
            .default_headers(default_headers)
            .timeout(Duration::from_secs(10))
            .build()?;
        let profile: StudentProfile = client
            .get(Url::from_str(&PROFILE_ENDPOINT)?)
            .send()
            .await?
            .json()
            .await?;
        Ok(Self {
            client,
            auth_token: str_token,
            student_id: profile.account.id,
            profile,
        })
    }

    pub async fn session(&self) -> anyhow::Result<StudentSession> {
        self.client
            .post(Url::from_str(&SESSIONS_ENDPOINT)?)
            .json(&StudentAuth {
                auth_token: self.auth_token.clone(),
            })
            .send()
            .await?
            .json()
            .await
            .map_err(anyhow::Error::from)
    }

    pub async fn academic_years(&self) -> anyhow::Result<Vec<AcademicYear>> {
        self.client
            .get(Url::from_str(&ACADEMIC_YEARS_ENDPOINT)?)
            .send()
            .await?
            .json()
            .await
            .map_err(anyhow::Error::from)
    }

    pub async fn schedule(&self, date: DateTime<Utc>) -> anyhow::Result<Schedule> {
        let date = date.naive_utc().date();
        self.client
            .get(Url::from_str(&SCHEDULE_ENDPOINT)?)
            .query(&[("student_id", self.student_id)])
            .query(&[("date", date.to_string())])
            .send()
            .await?
            .json()
            .await
            .map_err(anyhow::Error::from)
    }

    pub async fn final_marks(&self, year: &AcademicYear) -> anyhow::Result<Vec<FinalMark>> {
        self.final_marks_id(year.id).await
    }

    pub async fn final_marks_id(&self, year_id: u16) -> anyhow::Result<Vec<FinalMark>> {
        self.client
            .get(Url::from_str(&FINAL_MARKS_PREV_YEAR_ENDPOINT)?)
            .query(&[("student_profile_id", self.student_id)])
            .query(&[("academic_year_id", year_id)])
            .query(&[("is_year_mark", true)])
            .header("Profile-Type", "student")
            .send()
            .await?
            .json()
            .await
            .map_err(anyhow::Error::from)
    }

    async fn lesson_schedule_item(&self, lesson_id: u64) -> anyhow::Result<LessonScheduleItem> {
        self.client
            .get(Url::from_str(&format!(
                "https://dnevnik.mos.ru/mobile/api/lesson_schedule_items/{}",
                lesson_id
            ))?)
            .query(&[("student_id", self.student_id)])
            .query(&[("type", "OO")])
            .send()
            .await?
            .json()
            .await
            .map_err(anyhow::Error::from)
    }

    /// Gets module lesson plan for the provided lesson.
    /// Returns `Err` when the lesson lacks a scheduled plan (at least according to API)
    pub async fn lesson_plan(&self, lesson: &LessonInstance) -> anyhow::Result<LessonPlan> {
        let schedule_item = self.lesson_schedule_item(lesson.schedule_id).await?;
        if schedule_item.plan_id.is_none() {
            bail!(
                "Could not get plan ID for the lesson {}!",
                lesson.subject_name
            )
        }
        self.lesson_plan_wid(schedule_item.plan_id.unwrap()).await
    }

    /// Gets module lesson plan with the provided lesson plan ID
    pub async fn lesson_plan_wid(&self, plan_id: u64) -> anyhow::Result<LessonPlan> {
        let ele: Vec<LessonPlan> = self
            .client
            .get(Url::from_str(&LESSON_PLANS_ENDPOINT)?)
            .query(&[("plan_id", plan_id)])
            .query(&[("ignore_owner", true)])
            .query(&[("with_modules", true)])
            .query(&[("with_topics", true)])
            .query(&[("status", "for_calendar_plan")])
            .header(ACCEPT, "application/json")
            .send()
            .await?
            .json()
            .await?;
        Ok(ele[0].to_owned())
    }

    pub async fn homework(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> anyhow::Result<Vec<StudentHomework>> {
        self.client
            .get(Url::from_str(&STUDENT_HOMEWORKS_ENDPOINT)?)
            .query(&[("begin_prepared_date", from.format("%d.%m.%Y").to_string())])
            .query(&[("end_prepared_date", to.format("%d.%m.%Y").to_string())])
            .query(&[("student_profile_id", self.student_id)])
            .send()
            .await?
            .json()
            .await
            .map_err(anyhow::Error::from)
    }

    pub async fn download_attachment(
        &self,
        path: PathBuf,
        attachment: &HomeworkAttachment,
    ) -> anyhow::Result<()> {
        let bytes = self
            .client
            .get(Url::from_str(&format!(
                "{}{}",
                GLOBAL_DMR_URL, attachment.relative_path
            ))?)
            .timeout(Duration::from_secs(15))
            .send()
            .await?
            .bytes()
            .await?;
        let mut file = tokio::fs::File::create(path).await?;
        let mut content = Cursor::new(bytes);
        let size_copied = tokio::io::copy(&mut content, &mut file).await?;
        if size_copied < attachment.file_size {
            bail!("Could not download file, size of file downloaded is less than size of file provided by the attachments ({} < {})", size_copied, attachment.file_size)
        }

        Ok(())
    }

    /// Gets the progress report for the current student
    pub async fn progress(&self) -> anyhow::Result<Vec<GlobalAverageGrade>> {
        // get current year
        let year = self.academic_years().await?.last().unwrap().id;
        self.client
            .get(Url::from_str(&PROGRESS_ENDPOINT)?)
            .query(&[("academic_year_id", year)])
            .query(&[("student_profile_id", self.student_id)])
            .send()
            .await?
            .json()
            .await
            .map_err(anyhow::Error::from)
    }

    pub async fn visits(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> anyhow::Result<Vec<StudentAttendance>> {
        let data: Payload<Vec<StudentAttendance>> = self
            .client
            .get(Url::from_str(&VISITS_ENDPOINT)?)
            .query(&[("from", from.date().to_string())])
            .query(&[("to", to.date().to_string())])
            .query(&[(
                "contract_id",
                self.profile.details().contract_id.ok_or_else(|| {
                    anyhow::Error::msg("Provided student profile did not have `contract_id`!")
                })?,
            )])
            .send()
            .await?
            .json()
            .await?;
        Ok(data.payload)
    }
}
