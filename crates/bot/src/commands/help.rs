use {
  asahi::AsahiResult,
  poise::serenity_prelude::{
    ButtonStyle,
    CreateActionRow,
    CreateAllowedMentions,
    CreateButton,
    CreateComponent,
    CreateMessage
  }
};

/// Reminder message for needing admin assistance
#[poise::command(slash_command, rename = "help")]
pub async fn admin_help(ctx: super::PoiseAppCtx<'_>) -> AsahiResult {
  let admin_role = aaf_shared::load_env("AAF_ADMIN_ROLE");
  let (fs22, fs25) = crate::events::util::fs_emojis(ctx.serenity_context());

  let row = CreateActionRow::Buttons(Cow::Owned(vec![
    CreateButton::new("btn-role-fs22")
      .label("FS22 Role")
      .emoji(fs22)
      .style(ButtonStyle::Primary),
    CreateButton::new("btn-role-fs25")
      .label("FS25 Role")
      .emoji(fs25)
      .style(ButtonStyle::Success),
  ]));

  ctx
    .channel_id()
    .send_message(
      ctx.http(),
      CreateMessage::default()
        .allowed_mentions(CreateAllowedMentions::new().empty_roles())
        .content(
          [
            ":loudspeaker: Reminder for everyone :loudspeaker:",
            "When you need staff help, please make sure to:",
            &format!("1. Tag <@&{admin_role}> in your message"),
            "2. Include the game version (**22** or **25**)",
            "3. State the server you're on",
            "4. :no_entry_sign: Do not tag the staff directly!",
            "5. Use the appropriate game channels if possible. Access with buttons below!",
            "This makes it easier (and faster!) for staff to help you. Messages without this info may just be redirected to the rules.",
            ":ear_of_rice: Let's keep things growing smoothly!"
          ]
          .join("\n")
        )
        .components(vec![CreateComponent::ActionRow(row)])
    )
    .await
    .unwrap();

  ctx.send(poise::CreateReply::default().content("Sent!").ephemeral(true)).await.unwrap();

  Ok(())
}
