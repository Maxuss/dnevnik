pub mod diary;
pub mod model;

#[cfg(test)]
mod tests {
    use std::env;
    use chrono::{Duration, Utc};
    use crate::diary::Diary;
    use crate::model::lessons::{LessonActivity, ScheduleActivity};

    #[tokio::test]
    async fn test_basic_auth() -> anyhow::Result<()> {
        let diary = Diary::new(env::var("AUTH_TOKEN")?).await?;
        println!("Student: {} {} {}", diary.session.first_name, diary.session.last_name, diary.session.middle_name);
        let profile = diary.get_profile().await?;
        println!("School: {}", profile.details().school.full_name);
        let academic_years = diary.get_academic_years().await?;
        let current_year = academic_years
            .iter()
            .find(|year| year.is_current)
            .ok_or(anyhow::Error::msg("Could not find current academic year!"))?;
        println!("Current Year: {}, Start/End ({}/{})", current_year.description, current_year.begin_date, current_year.end_date);
        Ok(())
    }

    #[tokio::test]
    async fn test_schedule() -> anyhow::Result<()> {
        let diary = Diary::new(env::var("AUTH_TOKEN")?).await?;
        let schedule = diary.get_schedule(Utc::now() - Duration::days(1)).await?;
        println!("Schedule summary: {}", schedule.summary);
        let with_marks = schedule.lessons.iter().filter_map(|lesson|
            if let ScheduleActivity::Lesson(lesson) = lesson {
                Some(lesson)
            } else {
                None
            }
        ).filter(|lesson| !lesson.subject.marks.is_empty()).collect::<Vec<&LessonActivity>>();
        for lesson in with_marks {
            let mark = lesson.subject.marks.first().unwrap();
            println!("Got mark with value `{}` and weight `{}` on {} for '{}'", mark.value, mark.weight, lesson.subject.subject_name, mark.cause)
        }
        Ok(())
    }
}
