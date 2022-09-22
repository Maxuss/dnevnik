pub mod diary;
pub mod model;

#[cfg(test)]
mod tests {
    use std::env;
    use crate::diary::Diary;

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
}
