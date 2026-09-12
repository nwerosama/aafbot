use {
  asahi::AsahiResult,
  poise::{
    ChoiceParameter,
    CreateReply,
    serenity_prelude::{
      CreateComponent,
      CreateContainer,
      CreateContainerComponent,
      CreateSeparator,
      CreateTextDisplay,
      MessageFlags
    }
  }
};

#[repr(u8)]
#[derive(Clone, Copy, ChoiceParameter)]
enum BirthMonth {
  January = 1,
  February,
  March,
  April,
  May,
  June,
  July,
  August,
  September,
  October,
  November,
  December
}

impl std::fmt::Display for BirthMonth {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>
  ) -> std::fmt::Result {
    (*self as u8).fmt(f)
  }
}

/// Birthday commands
#[poise::command(slash_command, subcommands("register", "delete", "upcoming"))]
pub(in crate::commands) async fn birthday(_: super::PoiseAppCtx<'_>) -> AsahiResult { Ok(()) }

/// Register a birthday
#[poise::command(slash_command)]
async fn register(
  ctx: super::PoiseAppCtx<'_>,
  #[min = 1]
  #[max = 31]
  #[description = "Your day of birth"]
  day: u8,
  #[description = "Your month of birth"] month: BirthMonth,
  #[min = 1948]
  #[description = "Your birth year (if you want your age visible, else leave it empty)"]
  year: Option<u16>
) -> AsahiResult {
  let year = year.unwrap_or(1) as i32;

  match sqlx::query!(
    "INSERT INTO birthdays (user_id, birth_date) VALUES ($1, MAKE_DATE($2, $3, $4))
    ON CONFLICT (user_id) DO UPDATE SET birth_date = EXCLUDED.birth_date",
    ctx.author().id.get() as i64,
    year,
    month as i32,
    day as i32
  )
  .execute(&ctx.data().database)
  .await
  {
    Ok(_) => {
      ctx
        .send(CreateReply::default().content(format!(
          "Registered successfully! Your birthday is `{month}/{day}{}`",
          if year > 1 { format!("/{year}") } else { String::new() }
        )))
        .await
        .unwrap();
    },
    Err(sqlx::Error::Database(e)) if e.is_check_violation() || e.message().contains("out of range") => {
      ctx.reply(format!("You entered an invalid date!\n-# {e:?}")).await.unwrap();
    },
    Err(e) => return Err(e.into())
  }

  Ok(())
}

/// Delete your birthday
#[poise::command(slash_command)]
async fn delete(ctx: super::PoiseAppCtx<'_>) -> AsahiResult {
  let result = sqlx::query!("DELETE FROM birthdays WHERE user_id = $1", ctx.author().id.get() as i64)
    .execute(&ctx.data().database)
    .await?;

  let m = if result.rows_affected() > 0 {
    "Okay, your birthday has been removed, you won't be mentioned in next birthday announcement!"
  } else {
    "Why are you trying to remove your birthday when you never even registered at all?"
  };

  ctx.reply(m).await.unwrap();

  Ok(())
}

/// Check whose birthday is coming up next
#[poise::command(slash_command)]
async fn upcoming(ctx: super::PoiseAppCtx<'_>) -> AsahiResult {
  let mut rows = sqlx::query!(
    "SELECT user_id, birth_date FROM birthdays ORDER BY (
      (EXTRACT(MONTH FROM birth_date)::INT * 100 + EXTRACT(DAY FROM birth_date)::INT)
      - (EXTRACT(MONTH FROM CURRENT_DATE)::INT * 100 + EXTRACT(DAY FROM CURRENT_DATE)::INT)
      + 10_000
    ) % 10_000 LIMIT 25"
  )
  .fetch_all(&ctx.data().database)
  .await?;

  rows.sort_by_key(|r| r.birth_date.month() as i32 * 100 + r.birth_date.day() as i32);

  if rows.is_empty() {
    ctx.reply("No birthdays registered yet!").await.unwrap();
    return Ok(());
  }

  let (current_month, current_day): (i32, i32) =
    sqlx::query!("SELECT EXTRACT(MONTH FROM CURRENT_DATE)::INT AS m, EXTRACT(DAY FROM CURRENT_DATE)::INT AS d")
      .fetch_one(&ctx.data().database)
      .await
      .map(|r| (r.m.unwrap_or(1), r.d.unwrap_or(1)))?;
  let next_month = current_month % 12 + 1;

  let mut today = Vec::new();
  let mut later = Vec::new();
  let mut this_month = Vec::new();
  let mut next_month_list = Vec::new();

  for row in rows {
    let month = row.birth_date.month() as i32;
    let day = row.birth_date.day() as i32;
    let line = format!("<@{}> ∙ **{day} {}**", row.user_id, row.birth_date.month());

    if month == current_month && day == current_day {
      today.push(line);
    } else if month == current_month {
      this_month.push(line);
    } else if month == next_month {
      next_month_list.push(line);
    } else {
      later.push(line);
    }
  }

  let mut cmpts = vec![
    CreateContainerComponent::TextDisplay(CreateTextDisplay::new("# Upcoming birthdays!")),
    CreateContainerComponent::Separator(CreateSeparator::new().divider(true)),
  ];

  let mut first = true;
  for (t, l) in [
    ("Today", &today),
    ("This month", &this_month),
    ("Next month", &next_month_list),
    ("Later this year", &later)
  ] {
    if !l.is_empty() {
      if !first {
        cmpts.push(CreateContainerComponent::Separator(CreateSeparator::new().divider(true)));
      }
      first = false;
      cmpts.push(CreateContainerComponent::TextDisplay(CreateTextDisplay::new(format!(
        "**{t}!**\n{}",
        l.join("\n")
      ))));
    }
  }

  ctx
    .send(
      CreateReply::default()
        .components(vec![CreateComponent::Container(
          CreateContainer::new(cmpts).accent_colour(ctx.data().embed_color)
        )])
        .flags(MessageFlags::IS_COMPONENTS_V2)
    )
    .await
    .unwrap();

  Ok(())
}
