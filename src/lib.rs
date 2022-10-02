pub mod diary;
pub mod model;

#[cfg(test)]
mod tests {
    use crate::diary::Diary;
    use crate::model::lessons::{LessonActivity, ScheduleActivity};
    use chrono::{Duration, Utc};
    use dotenv::dotenv;
    use std::env;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_basic_auth() -> anyhow::Result<()> {
        dotenv()?;
        let diary = Diary::new(env::var("AUTH_TOKEN")?).await?;
        println!(
            "Student: {} {} {}",
            diary.profile.account.first_name,
            diary.profile.account.last_name,
            diary.profile.account.middle_name
        );
        let profile = &diary.profile;
        println!("School: {}", profile.details().school.full_name);
        let academic_years = diary.academic_years().await?;
        let current_year = academic_years
            .iter()
            .find(|year| year.is_current)
            .ok_or(anyhow::Error::msg("Could not find current academic year!"))?;
        println!(
            "Current Year: {}, Start/End ({}/{})",
            current_year.description, current_year.begin_date, current_year.end_date
        );
        let finals = diary.final_marks(current_year).await?;
        println!("{:#?}", finals);
        Ok(())
    }

    #[tokio::test]
    async fn test_schedule() -> anyhow::Result<()> {
        dotenv()?;
        let diary = Diary::new(env::var("AUTH_TOKEN")?).await?;
        let schedule = diary.schedule(Utc::now() - Duration::days(1)).await?;
        println!("Schedule summary: {}", schedule.summary);
        let with_marks = schedule
            .lessons
            .iter()
            .filter_map(|lesson| {
                if let ScheduleActivity::Lesson(lesson) = lesson {
                    Some(lesson)
                } else {
                    None
                }
            })
            .filter_map(|lesson| {
                if !lesson.subject.marks.is_empty() {
                    Some(&**lesson)
                } else {
                    None
                }
            })
            .collect::<Vec<&LessonActivity>>();
        for lesson in with_marks {
            let mark = lesson.subject.marks.first().unwrap();
            println!(
                "Got mark with value `{}` and weight `{}` on {} for '{}'",
                mark.value, mark.weight, lesson.subject.subject_name, mark.cause
            )
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_final_marks() -> anyhow::Result<()> {
        dotenv()?;
        let diary = Diary::new(env::var("AUTH_TOKEN")?).await?;
        let marks = diary.final_marks_id(4).await?;
        for mark in marks {
            println!("{}: {}", mark.subject_name, mark.value)
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_lesson_plans() -> anyhow::Result<()> {
        dotenv()?;
        let diary = Diary::new(env::var("AUTH_TOKEN")?).await?;
        let schedule = diary.schedule(Utc::now() - Duration::days(1)).await?;
        let lessons = schedule
            .lessons
            .iter()
            .filter_map(|ele| {
                if let ScheduleActivity::Lesson(lesson) = ele {
                    Some(&**lesson)
                } else {
                    None
                }
            })
            .collect::<Vec<&LessonActivity>>();
        for lesson in lessons {
            if let Ok(plan) = diary.lesson_plan(&lesson.subject).await {
                println!("{:#?}", plan);
            }
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_sessions() -> anyhow::Result<()> {
        dotenv()?;
        let diary = Diary::new(env::var("AUTH_TOKEN")?).await?;
        let session = diary.session().await?;
        println!("{:#?}", session);
        Ok(())
    }

    #[tokio::test]
    async fn test_homework_downloader() -> anyhow::Result<()> {
        dotenv()?;
        let diary = Diary::new(env::var("AUTH_TOKEN")?).await?;
        let homework = diary
            .homework(
                Utc::now() - Duration::days(2),
                Utc::now() + Duration::weeks(2),
            )
            .await?;
        for hw in homework {
            if !hw.homework_entry.attachments.is_empty() {
                let attachment = &hw.homework_entry.attachments[0];
                println!(
                    "Downloading attachment for {} ({})",
                    hw.homework_entry.subject().name,
                    attachment.file_name
                );
                let path = PathBuf::from(&attachment.file_name);
                diary.download_attachment(path.clone(), attachment).await?;
                tokio::fs::remove_file(path).await?;
            }
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_progress() -> anyhow::Result<()> {
        dotenv()?;
        let diary = Diary::new(env::var("AUTH_TOKEN")?).await?;
        let progress = diary.progress().await?;
        for grade in progress {
            println!(
                "Current average grade for {}: {}",
                grade.subject_name, grade.five
            )
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_visits() -> anyhow::Result<()> {
        dotenv()?;
        let diary = Diary::new(env::var("AUTH_TOKEN")?).await?;
        let visits = diary
            .visits(Utc::now() - Duration::days(7), Utc::now())
            .await?;
        for visit in visits {
            println!("Visit: {:#?}", visit);
        }
        Ok(())
    }
}
