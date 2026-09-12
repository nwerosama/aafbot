use {
  aaf_shared::load_env,
  asahi::{
    AsahiCoordinator,
    AsahiResult,
    error
  },
  poise::serenity_prelude::{
    CreateComponent,
    CreateContainer,
    CreateContainerComponent,
    CreateMessage,
    CreateSeparator,
    CreateTextDisplay,
    GenericChannelId,
    Http,
    MessageFlags
  },
  std::sync::Arc
};

pub struct Birthday {
  pub db:     sqlx::PgPool,
  pub http:   Arc<Http>,
  pub accent: u32
}

struct UserBirthday {
  user_id: i64,
  age:     Option<i16>
}

#[asahi::async_trait]
impl AsahiCoordinator for Birthday {
  fn name(&self) -> &'static str { "Birthday" }

  fn interval(&self) -> u64 { 3600 }

  async fn main_loop(&self) -> AsahiResult {
    if sqlx::query_scalar!("SELECT EXISTS(SELECT 1 FROM birthday_announcements WHERE announced_on = CURRENT_DATE)")
      .fetch_one(&self.db)
      .await?
      .unwrap_or(false)
    {
      return Ok(());
    }

    let users: Vec<UserBirthday> = sqlx::query_as!(
      UserBirthday,
      "SELECT user_id,
      CASE WHEN EXTRACT(YEAR FROM birth_date) > 1 THEN EXTRACT(YEAR FROM CURRENT_DATE)::SMALLINT - EXTRACT(YEAR FROM birth_date)::SMALLINT END AS age
      FROM birthdays WHERE EXTRACT(MONTH FROM birth_date) = EXTRACT(MONTH FROM CURRENT_DATE)
      AND (EXTRACT(DAY FROM birth_date) = EXTRACT(DAY FROM CURRENT_DATE) OR (
        EXTRACT(MONTH FROM birth_date) = 2 AND EXTRACT(DAY FROM birth_date) = 29 AND EXTRACT(MONTH FROM CURRENT_DATE) = 2 AND EXTRACT(DAY FROM \
       CURRENT_DATE) = 28
        AND NOT (EXTRACT(YEAR FROM CURRENT_DATE)::INT % 4 = 0 AND (EXTRACT(YEAR FROM CURRENT_DATE)::INT % 100 != 0 OR EXTRACT(YEAR FROM \
       CURRENT_DATE)::INT % 400 = 0))
      ))"
    )
    .fetch_all(&self.db)
    .await?;

    sqlx::query!("INSERT INTO birthday_announcements (announced_on) VALUES (CURRENT_DATE) ON CONFLICT DO NOTHING")
      .execute(&self.db)
      .await?;

    if users.is_empty() {
      return Ok(());
    }

    let channel_id: Option<i64> = sqlx::query_scalar!("SELECT channel_id FROM birthday_conf")
      .fetch_optional(&self.db)
      .await?;

    let Some(channel_id) = channel_id else {
      error!("{} birthday(s) but no announcement channel", users.len());
      return Ok(());
    };

    let bday_map = users
      .iter()
      .map(|u| match u.age {
        Some(age) => format!("<@{}> is turning **{age}** years old!", u.user_id),
        None => format!("<@{}>", u.user_id)
      })
      .collect::<Vec<_>>()
      .join("\n");

    if let Err(e) = GenericChannelId::new(channel_id as u64)
      .send_message(
        &self.http,
        CreateMessage::default()
          .components(vec![CreateComponent::Container(
            CreateContainer::new(vec![
              CreateContainerComponent::TextDisplay(CreateTextDisplay::new(format!(
                "# :birthday: Birthday announcement!\n-# <@&{}>",
                load_env("AAF_ADMIN_ROLE")
              ))),
              CreateContainerComponent::Separator(CreateSeparator::new().divider(true)),
              CreateContainerComponent::TextDisplay(CreateTextDisplay::new(bday_map)),
            ])
            .accent_colour(self.accent)
          )])
          .flags(MessageFlags::IS_COMPONENTS_V2)
      )
      .await
    {
      error!("Failed to announce birthdays due to an error: {e:?}")
    }

    Ok(())
  }
}
