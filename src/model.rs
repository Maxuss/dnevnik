pub mod attendance;
pub mod hw;
pub mod lessons;
pub mod marks;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StudentSession {
    /// Unique ID of this student
    pub id: u64,
    /// UUID of this student's session
    pub person_id: Uuid,
    /// Last name or surname of the account's owner
    pub last_name: String,
    /// First name of the account's owner
    pub first_name: String,
    /// Middle name or the patronymic of the account's owner
    pub middle_name: String,
    /// Date of birth of the account's owner
    pub date_of_birth: Option<NaiveDate>,
    /// Gender of the account's owner
    #[serde(rename = "sex")]
    pub gender: Option<String>,
    /// Phone number of the account's owner, excluding the regional phone number
    pub phone_number: String,
    /// Email address of the account's owner
    pub email: String,
    /// Individual insurance account number (SNILS) of the profile owner
    pub snils: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StudentProfile {
    /// SHA256 hash of this student's profile
    pub hash: String,
    /// Account bound to this student
    #[serde(rename = "profile")]
    pub account: Account,
    /// A single element list containing details of this student
    #[serde(rename = "children")]
    details: Vec<StudentDetails>,
}

impl StudentProfile {
    pub fn details(&self) -> &StudentDetails {
        &self.details[0]
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Account {
    /// Last name or surname of the account's owner
    pub last_name: String,
    /// First name of the account's owner
    pub first_name: String,
    /// Middle name or the patronymic of the account's owner
    pub middle_name: String,
    /// Date of birth of the account's owner
    pub birth_date: Option<NaiveDate>,
    /// Gender of the account's owner
    #[serde(rename = "sex")]
    pub gender: Option<String>,
    /// This is just an internal ID for the profile, and is not used in any
    /// student related methods
    pub user_id: Option<u32>,
    /// This is the actual id of the account, as opposed to the [user_id] field
    pub id: u64,
    /// Another id of the account, because developers of school.mos.ru were on hard drugs when coding the API
    pub contract_id: Option<u32>,
    /// Phone number of the account's owner, excluding the regional phone number
    pub phone: String,
    /// Email address of the account's owner
    pub email: String,
    /// Individual insurance account number (SNILS) of the profile owner
    pub snils: String,
    /// Type of the account, `"student"` for students and `"teacher"` for teachers and `null` for external accounts (e.g. representatives)
    #[serde(rename = "type")]
    pub profile_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct School {
    /// Unique ID of the school
    pub id: u32,
    /// Full name of the school
    #[serde(rename = "name")]
    pub full_name: String,
    /// Short name of the school
    pub short_name: String,
    /// County where this school is located
    pub county: String,
    /// Full name of the principal of this school
    pub principal: String,
    /// Contact phone for this school, excluding the regional number code
    pub phone: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SubjectGroup {
    /// Unique ID for this subject group
    pub id: u64,
    /// Name of this subject group
    pub name: String,
    /// Unique ID for the subject this group belongs to. `null` for external section groups.
    pub subject_id: Option<u64>,
    /// Use of this subject group is currently unknown, and it seems to only be false
    pub is_fake: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StudentDetails {
    /// Parent account containing some of the information for this profile
    #[serde(flatten)]
    pub parent_account: Account,
    /// School to which this student belongs
    pub school: School,
    /// Name of the class this student belongs to
    pub class_name: String,
    /// Grade of this student
    #[serde(rename = "class_level_id")]
    pub grade: u8,
    /// Unique unit ID for the class this student belongs to
    #[serde(rename = "class_unit_id")]
    pub class_id: u64,
    /// All the actual subject groups this student belongs to. Does not include external groups and section groups.
    #[serde(rename = "groups")]
    pub subjects: Vec<SubjectGroup>,
    /// Representatives of this student
    pub representatives: Vec<Account>,
    /// All the section subject groups this student belongs to. These groups aren't actually bound to any subject and their subject IDs will be `None`.
    pub sections: Vec<SubjectGroup>,
    /// Whether this student may be a legal representative. *Usually* `false`.
    pub is_legal_representative: bool,
    /// UUID that is bound to this student
    #[serde(rename = "contingent_guid")]
    pub uuid: Uuid,
    /// Another Unique ID of this account
    pub contract_id: Option<u32>,
}
