use {
  aaf_shared::load_env,
  asahi::AsahiResult,
  poise::serenity_prelude::{
    ComponentInteraction,
    Context,
    CreateAllowedMentions,
    CreateInteractionResponse,
    CreateInteractionResponseMessage,
    MessageFlags,
    RoleId
  }
};

pub async fn button_role(
  ctx: &Context,
  cmpt: &ComponentInteraction,
  role_id: RoleId
) -> AsahiResult {
  let member = cmpt.member.as_ref().expect("member not present");
  let reason = "Button pressed";

  let ctnt = if member.roles.contains(&role_id) {
    member.remove_role(&ctx.http, role_id, Some(reason)).await.unwrap();
    format!("Removed <@&{role_id}> from you!")
  } else {
    member.add_role(&ctx.http, role_id, Some(reason)).await.unwrap();
    format!("Gave you <@&{role_id}>!")
  };

  cmpt
    .create_response(
      &ctx.http,
      CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::default()
          .content(ctnt)
          .allowed_mentions(CreateAllowedMentions::new().empty_roles())
          .flags(MessageFlags::EPHEMERAL)
      )
    )
    .await
    .unwrap();

  Ok(())
}

/// Resolve envvar into RoleId
pub(in crate::events::interaction) fn env2id(id: &str) -> Option<RoleId> {
  let env = match id {
    "btn-role-fs22" => "AAF_FS22_ROLE",
    "btn-role-fs25" => "AAF_FS25_ROLE",
    _ => return None
  };

  load_env(env).parse::<u64>().ok().map(RoleId::new)
}
