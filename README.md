# Dnevnik-Mos-Rust

A rust API wrapper for accessing the https://dnevnik.mos.ru internal API

All API info was gathered by reverse engineering the network requests.

## Goals
- [X] Marks API (mostly)
- [X] Lessons API
- [X] Attendance/visits API
- [ ] Basic Authentication
- [ ] Teacher-Side API

## Usage

```rust
use dnevnik::prelude::*;

// assuming you are using tokio, anyhow and chrono
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let auth_token: String = "Your auth token";
    let diary: Diary = Diary::new(auth_token).await?;
    let student_profile: StudentProfile = diary.profile;
    // Gets homework for the last week
    let homework: Vec<StudentHomework> = diary.homework(
        Utc::now() - Duration::days(7),
        Utc::now()
    ).await?;
    // Downloads all attachment files from this homework
    homework.iter().for_each(|hw| {
        if !hw.homework_entry.attachments.is_empty() {
            let attachment: &HomeworkAttachment = &hw.homework_entry.attachments[0];
            println!(
                "Downloading attachment for {} (path: {})",
                hw.homework_entry.subject().name,
                attachment.file_name
            );
            let path: PathBuf = PathBuf::from(&attachment.file_name);
            diary.download_attachment(path.clone(), attachment).await?;
        }
    });
    Ok(())
}
```

More examples are TBD