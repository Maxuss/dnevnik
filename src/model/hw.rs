use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct StudentHomework {
    /// ID of this homework
    pub id: u64,
    /// ID of this homework's student
    pub student_id: u64,
    /// Whether this homework is ready
    pub is_ready: bool,
    /// Entry for this homework
    pub homework_entry: HomeworkEntry,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HomeworkEntry {
    /// ID of this homework entry
    pub id: u64,
    /// Date at which this homework entry was created
    #[serde(deserialize_with = "datetime_de::deserialize_datetime")]
    pub created_at: NaiveDateTime,
    /// Date at which this homework entry was last updated
    #[serde(deserialize_with = "datetime_de::deserialize_datetime")]
    pub updated_at: NaiveDateTime,
    /// Date at which this homework was deleted, `None` if it wasn't deleted
    #[serde(deserialize_with = "datetime_de::deserialize_opt_datetime")]
    pub deleted_at: Option<NaiveDateTime>,
    /// Text description for this homework entry
    pub description: String,
    /// Expected time for this entry to take, in minutes
    #[serde(rename = "duration")]
    pub expected_duration: u32,
    /// All attachments to this homework entry
    pub attachments: Vec<HomeworkAttachment>,
    homework: InternalHomeworkEntry,
}

impl HomeworkEntry {
    pub fn subject(&self) -> &HomeworkSubject {
        &self.homework.subject
    }
}

#[derive(Debug, Clone, Deserialize)]
struct InternalHomeworkEntry {
    subject: HomeworkSubject,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HomeworkSubject {
    /// ID of this subject
    pub id: u64,
    /// Name of this subject
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HomeworkAttachment {
    /// ID of this attachment
    pub id: u64,
    /// Date at which this attachment was added
    #[serde(deserialize_with = "datetime_de::deserialize_datetime")]
    pub created_at: NaiveDateTime,
    /// Name of the attached file
    #[serde(rename = "file_file_name")]
    pub file_name: String,
    /// Size of the attached file in bytes
    #[serde(rename = "file_file_size")]
    pub file_size: u64,
    /// Type of the file's contents
    #[serde(rename = "file_content_type")]
    pub content_type: String,
    /// Relative path to this file URL
    #[serde(rename = "path")]
    pub relative_path: String,
}

#[doc(hidden)]
mod datetime_de {
    #[doc(hidden)]
    pub fn deserialize_datetime<'de, D>(d: D) -> Result<NaiveDateTime, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_str(DateTimeFromCustomFormatVisitor)
    }

    #[doc(hidden)]
    pub fn deserialize_opt_datetime<'de, D>(d: D) -> Result<Option<NaiveDateTime>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_option(OptionalDateTimeFormatVisitor)
    }

    use chrono::NaiveDateTime;
    use serde::de;
    use std::fmt;

    struct OptionalDateTimeFormatVisitor;

    struct DateTimeFromCustomFormatVisitor;

    impl<'de> de::Visitor<'de> for OptionalDateTimeFormatVisitor {
        type Value = Option<NaiveDateTime>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "null or a datetime string")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_some<D>(self, d: D) -> Result<Option<NaiveDateTime>, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            Ok(Some(d.deserialize_str(DateTimeFromCustomFormatVisitor)?))
        }
    }

    impl<'de> de::Visitor<'de> for DateTimeFromCustomFormatVisitor {
        type Value = NaiveDateTime;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a datetime string")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match NaiveDateTime::parse_from_str(value, "%d.%m.%Y %H:%M") {
                Ok(ndt) => Ok(ndt),
                Err(e) => Err(E::custom(format!("Parse error {} for {}", e, value))),
            }
        }
    }
}
