use {
  aaf_shared::load_env,
  asahi::AsahiResult,
  poise::serenity_prelude::{
    ChannelId,
    ChannelType,
    CreateChannel,
    EditRole,
    PermissionOverwrite,
    PermissionOverwriteType,
    Permissions,
    RoleId
  }
};

/// Mentor commands
#[poise::command(slash_command, subcommands("add", "delete"))]
pub(in crate::commands) async fn mentor(_: super::PoiseAppCtx<'_>) -> AsahiResult { Ok(()) }

/// Create a mentor chat + role
#[poise::command(slash_command)]
async fn add(
  ctx: super::PoiseAppCtx<'_>,
  #[description = "Mentor name"] name: String,
  #[description = "Mentor emoji"] emoji: String
) -> AsahiResult {
  let http = ctx.http();
  let guild_id = ctx.guild_id().expect("guild not present");
  let reason = format!("Mentor creation on behalf of {}", ctx.author_member().await.unwrap().display_name());

  let role = guild_id
    .create_role(
      http,
      EditRole::new()
        .audit_log_reason(reason.clone())
        .name(format!("Mentor {name}"))
        .permissions(Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES | Permissions::READ_MESSAGE_HISTORY | Permissions::ADD_REACTIONS)
    )
    .await
    .expect("unable to create role due to an error");

  // sleep to allow for later retrieval to avoid edge cases
  ctx.defer().await.unwrap();
  tokio::time::sleep(std::time::Duration::from_secs(2)).await;

  let roles = guild_id.roles(http).await.expect("unable to get list of roles");
  let mentor_staff = roles.iter().find(|r| r.name == "Staff Mentor").expect("unable to get role");
  let hr_role = roles.iter().find(|r| r.name.starts_with("HR")).expect("unable to get role");

  let channel = guild_id
    .create_channel(
      http,
      CreateChannel::new(format!("{emoji}┇{name}-mentor-chat"))
        .audit_log_reason(reason)
        .kind(ChannelType::Text)
        .category(ChannelId::new(load_env("AAF_MENTOR_CATEGORY").parse::<u64>().unwrap()))
        .permissions(vec![
          PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL,
            deny:  Permissions::empty(),
            kind:  PermissionOverwriteType::Role(hr_role.id)
          },
          PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL,
            deny:  Permissions::empty(),
            kind:  PermissionOverwriteType::Role(mentor_staff.id)
          },
          PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL,
            deny:  Permissions::empty(),
            kind:  PermissionOverwriteType::Role(role.id)
          },
          PermissionOverwrite {
            allow: Permissions::empty(),
            deny:  Permissions::VIEW_CHANNEL,
            kind:  PermissionOverwriteType::Role(RoleId::new(guild_id.get()))
          },
        ])
    )
    .await
    .expect("unable to create channel due to an error");

  ctx.reply(format!("Created <#{}> + <@&{}>!", channel.id, role.id)).await.unwrap();

  Ok(())
}

/// Delete a mentor chat + role
#[poise::command(slash_command)]
async fn delete(
  ctx: super::PoiseAppCtx<'_>,
  #[description = "Mentor name to delete"] name: String
) -> AsahiResult {
  let http = ctx.http();
  let guild_id = ctx.guild_id().expect("guild not present");
  let reason = format!("Mentor deletion on behalf of {}", ctx.author_member().await.unwrap().display_name());

  let roles = guild_id.roles(http).await.expect("unable to get list of roles");
  let role_deleted = match roles.iter().find(|r| r.name == format!("Mentor {name}")) {
    Some(r) => guild_id.delete_role(http, r.id, Some(&reason)).await.is_ok(),
    None => false
  };

  let channels = guild_id.channels(http).await.expect("unable to retrieve the list of channels");
  let chat_deleted = match channels.iter().find(|c| c.base.name.contains(&name.to_lowercase())) {
    Some(c) => c.delete(http, Some(&reason)).await.is_ok(),
    None => false
  };

  ctx
    .reply(match (role_deleted, chat_deleted) {
      (true, true) => format!("Deleted mentor {name} successfully!"),
      (true, false) => "Deleted the role but channel was already gone!".to_string(),
      (false, true) => "Deleted the channel but role was already gone!".to_string(),
      (false, false) => "Neither the role or channel matches by that name!".to_string()
    })
    .await
    .unwrap();

  Ok(())
}
