use std::io::Cursor;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;
use anyhow::bail;
use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use reqwest::{Client, ClientBuilder, Url};
use reqwest::header::{ACCEPT, AUTHORIZATION, COOKIE, HeaderMap, HeaderValue, REFERER, USER_AGENT};
use crate::model::{StudentProfile, StudentSession};
use crate::model::hw::{HomeworkAttachment, StudentHomework};
use crate::model::lessons::{AcademicYear, FinalMark, LessonInstance, LessonPlan, LessonScheduleItem, Schedule};

pub const GLOBAL_DMR_URL: &'static str = "https://dnevnik.mos.ru";
pub const CORE_API: &'static str = "/core/api";
pub const MOBILE_API: &'static str = "/mobile/api";
pub const LMS_API: &'static str = "/lms/api";
pub const JERSEY_API: &'static str = "/jersey/api";

lazy_static! {
    pub static ref PROFILE_ENDPOINT: String = format!("{}{}/profile", GLOBAL_DMR_URL, MOBILE_API);
    pub static ref SESSIONS_ENDPOINT: String = format!("{}{}/sessions", GLOBAL_DMR_URL, LMS_API);
    pub static ref ACADEMIC_YEARS_ENDPOINT: String = format!("{}{}/academic_years", GLOBAL_DMR_URL, CORE_API);
    pub static ref SCHEDULE_ENDPOINT: String = format!("{}{}/schedule", GLOBAL_DMR_URL, MOBILE_API);
    pub static ref FINAL_MARKS_PREV_YEAR_ENDPOINT: String = format!("{}{}/final_marks_prev_year", GLOBAL_DMR_URL, CORE_API);
    pub static ref LESSON_PLANS_ENDPOINT: String = format!("{}{}/lesson_plans", GLOBAL_DMR_URL, JERSEY_API);
    pub static ref STUDENT_HOMEWORKS_ENDPOINT: String = format!("{}{}/student_homeworks", GLOBAL_DMR_URL, CORE_API);
}

#[allow(unused)]
pub struct Diary {
    client: Client,
    auth_token: String,
    pub profile: StudentProfile,
    student_id: u64
}

#[derive(serde::Serialize)]
struct StudentAuth {
    auth_token: String
}

impl Diary {
    pub async fn new<S: Into<String>>(token: S) -> anyhow::Result<Self> {
        let str_token = token.into();
        let mut default_headers = HeaderMap::new();
        default_headers.append(USER_AGENT, HeaderValue::from_str(&format!("Dnevnik-Mos-Rust/{}", option_env!("CARGO_PKG_VERSION").unwrap()))?);
        default_headers.append(REFERER, HeaderValue::from_str("https://dnevnik.mos.ru/diary/")?);
        default_headers.append("Auth-Token", HeaderValue::from_str(&str_token)?);
        default_headers.append(AUTHORIZATION, HeaderValue::from_str(&str_token)?);
        default_headers.append(COOKIE, HeaderValue::from_str("spa_id=2d8921d4; aupd_current_role=2:1; cluster_id=1; subsystem_id=2; cluster=0; aupd_token=eyJhbGciOiJSUzI1NiJ9.eyJzdWIiOiIyMDM0MDM0IiwiYXVkIjoiMjoxIiwibmJmIjoxNjY0MDIzMTgwLCJtc2giOiJhZWNkYzkwMi04ZGNkLTRkMWEtOTZhNS02MmNmYThiZmRkNjgiLCJpc3MiOiJodHRwczpcL1wvc2Nob29sLm1vcy5ydSIsInJscyI6InsxOlsyMDoyOltdLDMwOjQ6W10sNDA6MTpbXSwxODM6MTY6W11dfSIsInptayI6ZmFsc2UsImV4cCI6MTY2NDg4NzE4MCwiaWF0IjoxNjY0MDIzMTgwLCJqdGkiOiIxZTVhY2VmNi04ZDJjLTRmYWEtOWFjZS1mNjhkMmNmZmFhY2EiLCJzc28iOiI1YjYzNDBkZC1kODJhLTQ0MGEtYjQxMC1hODQ0YzQ2MmFiMDIifQ.Kl02bUfguATLzWlxb4R6pxmyBMGDogAvAVvnFSx0dUwZlPclGh4j2K9EOGhM9oGaxR0PxO9eEzLm6hKtsKVuVW02DssT0BOMUfj4qj0jdHKB3TbtsT67EIL4eQxg5cZ4AmTzjgS35GuRSpYJMUON9Exlca2TEr3-0FII99Fxm494rIkuviZFC66HFhfW5rWFbH6puqQytM3OIPHQroF8rt3s6SjoyVANL02uQ8ijp_uo186KbAfy17LtFy-BHx7iM3UKmD6JnR6iaiCuaG8OqjIkKFtkNEXYoaHPVrt1su2c89ZGnubVEWFdGU1nrrTr3i5J3Smy_scpqD4J-XNsaQ; obr_id=2034034; mos_id=Cg8qAmMuBIGy4UTuHnWtAgA=; auth_flag=main; Ltpatoken2=Q6SsKZ3GOuUCUNEeR+ulYEGTNEB9REvCbYQqxOOBPlV1pU9/9BRyKnUx95FJwydQVlhthp5JZ3GYZO8J8y7XocOG3SaAe+VCo3Yiq/vSUzmI37T3rjE8UXS8SiXS5tBrpjCT2PrS/bmWFsgqZBzrU4jgpcN4SrVFNfTN1ER176EqPHLuGhCRl587wsvuodrP6NxSVvYKOPAf4NdM7kIrKZvJQbpCZF10Mol5vqMuFWwTG6PJBAThRvD31muCvq4e6xW6HPeAGPA1W3iXvq4/37MU5K/U0stU8FoRJydYo3IZHAvUno/UKTjPOBNl5sthhFKT1OZOP0vURDYnfXTc1Q==; sbp_sid=000000000000000000000000000000000000; disable_aupd_auth=; from_pgu=true; JSESSIONID=5B05C788EFF05152CF27C7D9ED5EF5EC; aid=10; auth_token=eyJhbGciOiJSUzI1NiJ9.eyJzdWIiOiIyMDM0MDM0IiwiYXVkIjoiMjoxIiwibmJmIjoxNjY0MDIzMTgwLCJtc2giOiJhZWNkYzkwMi04ZGNkLTRkMWEtOTZhNS02MmNmYThiZmRkNjgiLCJpc3MiOiJodHRwczpcL1wvc2Nob29sLm1vcy5ydSIsInJscyI6InsxOlsyMDoyOltdLDMwOjQ6W10sNDA6MTpbXSwxODM6MTY6W11dfSIsInptayI6ZmFsc2UsImV4cCI6MTY2NDg4NzE4MCwiaWF0IjoxNjY0MDIzMTgwLCJqdGkiOiIxZTVhY2VmNi04ZDJjLTRmYWEtOWFjZS1mNjhkMmNmZmFhY2EiLCJzc28iOiI1YjYzNDBkZC1kODJhLTQ0MGEtYjQxMC1hODQ0YzQ2MmFiMDIifQ.Kl02bUfguATLzWlxb4R6pxmyBMGDogAvAVvnFSx0dUwZlPclGh4j2K9EOGhM9oGaxR0PxO9eEzLm6hKtsKVuVW02DssT0BOMUfj4qj0jdHKB3TbtsT67EIL4eQxg5cZ4AmTzjgS35GuRSpYJMUON9Exlca2TEr3-0FII99Fxm494rIkuviZFC66HFhfW5rWFbH6puqQytM3OIPHQroF8rt3s6SjoyVANL02uQ8ijp_uo186KbAfy17LtFy-BHx7iM3UKmD6JnR6iaiCuaG8OqjIkKFtkNEXYoaHPVrt1su2c89ZGnubVEWFdGU1nrrTr3i5J3Smy_scpqD4J-XNsaQ; profile_id=342218; profile_roles=student; is_auth=true; user_id=323611; active_student=342218")?);
        let client = ClientBuilder::new().default_headers(default_headers).timeout(Duration::from_secs(10)).build()?;
        let profile: StudentProfile = client.get(Url::from_str(&PROFILE_ENDPOINT)?).send().await?.json().await?;
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
                auth_token: self.auth_token.clone()
            })
            .send().await?
            .json().await
            .map_err(anyhow::Error::from)
    }

    pub async fn academic_years(&self) -> anyhow::Result<Vec<AcademicYear>> {
        self.client.get(Url::from_str(&ACADEMIC_YEARS_ENDPOINT)?).send().await?.json().await.map_err(anyhow::Error::from)
    }

    pub async fn schedule(&self, date: DateTime<Utc>) -> anyhow::Result<Schedule> {
        let date = date.naive_utc().date();
        self.client
            .get(Url::from_str(&SCHEDULE_ENDPOINT)?)
            .query(&[("student_id", self.student_id)])
            .query(&[("date", date.to_string())])
            .send().await?
            .json().await
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
            .send().await?
            .json().await
            .map_err(anyhow::Error::from)
    }

    async fn lesson_schedule_item(&self, lesson_id: u64) -> anyhow::Result<LessonScheduleItem> {
        self.client
            .get(Url::from_str(&format!("https://dnevnik.mos.ru/mobile/api/lesson_schedule_items/{}", lesson_id))?)
            .query(&[("student_id", self.student_id)])
            .query(&[("type", "OO")])
            .send().await?
            .json().await
            .map_err(anyhow::Error::from)
    }

    /// Gets module lesson plan for the provided lesson.
    /// Returns `Err` when the lesson lacks a scheduled plan (at least according to API)
    pub async fn lesson_plan(&self, lesson: &LessonInstance) -> anyhow::Result<LessonPlan> {
        let schedule_item = self.lesson_schedule_item(lesson.schedule_id).await?;
        if let None = schedule_item.plan_id {
            bail!("Could not get plan ID for the lesson {}!", lesson.subject_name)
        }
        let ele: Vec<LessonPlan> = self.client
            .get(Url::from_str(&LESSON_PLANS_ENDPOINT)?)
            .query(&[("plan_id", schedule_item.plan_id)])
            .query(&[("ignore_owner", true)])
            .query(&[("with_modules", true)])
            .query(&[("with_topics", true)])
            .query(&[("status", "for_calendar_plan")])
            .header(ACCEPT, "application/json")
            .send().await?
            .json().await?;
        return Ok(ele[0].to_owned());
    }

    pub async fn homework(&self, from: DateTime<Utc>, to: DateTime<Utc>) -> anyhow::Result<Vec<StudentHomework>> {
        self.client
            .get(Url::from_str(&STUDENT_HOMEWORKS_ENDPOINT)?)
            .query(&[("begin_prepared_date", from.format("%d.%m.%Y").to_string())])
            .query(&[("end_prepared_date", to.format("%d.%m.%Y").to_string())])
            .query(&[("student_profile_id", self.student_id)])
            .send().await?
            .json().await
            .map_err(anyhow::Error::from)
    }

    pub async fn download_attachment(&self, path: PathBuf, attachment: &HomeworkAttachment) -> anyhow::Result<()> {
        let bytes = self.client
            .get(Url::from_str(&format!("{}{}", GLOBAL_DMR_URL, attachment.relative_path))?)
            .timeout(Duration::from_secs(15))
            .send().await?
            .bytes().await?;
        let mut file = tokio::fs::File::create(path).await?;
        let mut content = Cursor::new(bytes);
        let size_copied = tokio::io::copy(&mut content, &mut file).await?;
        if size_copied != attachment.file_size {
            bail!("Could not download file, size of file downloaded is less than size of file provided by the attachments ({} < {})", size_copied, attachment.file_size)
        }

        Ok(())
    }
}