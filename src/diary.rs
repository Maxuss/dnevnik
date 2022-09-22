use std::str::FromStr;
use std::time::Duration;
use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use reqwest::{Client, ClientBuilder, Url};
use reqwest::header::{HeaderMap, HeaderValue, REFERER, USER_AGENT};
use crate::model::{StudentProfile, StudentSession};
use crate::model::lessons::{AcademicYear, Schedule};

pub const GLOBAL_DMR_URL: &'static str = "https://dnevnik.mos.ru";
pub const CORE_API: &'static str = "/core/api";
pub const MOBILE_API: &'static str = "/mobile/api";
pub const LMS_API: &'static str = "/lms/api";

lazy_static! {
    pub static ref PROFILE_ENDPOINT: String = format!("{}{}/profile", GLOBAL_DMR_URL, MOBILE_API);
    pub static ref SESSIONS_ENDPOINT: String = format!("{}{}/sessions", GLOBAL_DMR_URL, LMS_API);
    pub static ref ACADEMIC_YEARS_ENDPOINT: String = format!("{}{}/academic_years", GLOBAL_DMR_URL, CORE_API);
    pub static ref SCHEDULE_ENDPOINT: String = format!("{}{}/schedule", GLOBAL_DMR_URL, MOBILE_API);
}

#[allow(unused)]
pub struct Diary {
    client: Client,
    auth_token: String,
    pub session: StudentSession,
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
        let client = ClientBuilder::new().default_headers(default_headers).timeout(Duration::from_secs(10)).build()?;
        let session: StudentSession = client.post(Url::from_str(&SESSIONS_ENDPOINT)?).json(&StudentAuth {
            auth_token: str_token.clone()
        }).send().await?.json().await?;
        let profile: StudentProfile = client.get(Url::from_str(&PROFILE_ENDPOINT)?).send().await?.json().await?;
        Ok(Self {
            client,
            auth_token: str_token,
            student_id: profile.account.id,
            session,
        })
    }

    pub async fn get_profile(&self) -> anyhow::Result<StudentProfile> {
        self.client.get(Url::from_str(&PROFILE_ENDPOINT)?).send().await?.json().await.map_err(anyhow::Error::from)
    }

    pub async fn get_academic_years(&self) -> anyhow::Result<Vec<AcademicYear>> {
        self.client.get(Url::from_str(&ACADEMIC_YEARS_ENDPOINT)?).send().await?.json().await.map_err(anyhow::Error::from)
    }

    pub async fn get_schedule(&self, date: DateTime<Utc>) -> anyhow::Result<Schedule> {
        let date = date.naive_utc().date();
        self.client
            .get(Url::from_str(&SCHEDULE_ENDPOINT)?)
            .query(&[("student_id", self.student_id)])
            .query(&[("date", date.to_string())])
            .send().await?
            .json().await
            .map_err(anyhow::Error::from)
    }
}