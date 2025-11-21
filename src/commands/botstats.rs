use {
  asahi::{
    AsahiResult,
    utils::{
      format_bytes,
      format_duration,
      os::{
        get_hostname,
        get_memory,
        get_os_info,
        get_uptime
      }
    }
  },
  poise::{
    CreateReply,
    serenity_prelude::CreateEmbed
  }
};

/// Fetches the bot statistics (nerd stats)
#[poise::command(slash_command, install_context = "Guild", interaction_context = "Guild")]
pub async fn botstats(ctx: super::PoiseContext<'_>) -> AsahiResult {
  let bot_name = ctx.cache().current_user().name.clone();

  let (sys_up, sys_mem, proc_up, proc_mem) = (get_uptime().system, get_memory().system, get_uptime().process, get_memory().process);

  let node_stats = [
    format!("**Hostname:** `{}`", get_hostname()),
    format!("**OS:** `{}`", get_os_info()),
    format!(
      "**Uptime:**\n- **Host:** `{}`\n- **Bot:** `{}`",
      format_duration(sys_up),
      format_duration(proc_up)
    ),
    format!(
      "**Memory:**\n- **Host:** `{}`/`{}`\n- **Bot:** `{}`",
      format_bytes(sys_mem.used),
      format_bytes(sys_mem.total),
      format_bytes(proc_mem)
    )
  ]
  .join("\n");

  let embed = CreateEmbed::new()
    .color(ctx.data().embed_color)
    .title(format!("{bot_name} - Statistics"))
    .fields([("Node stats", node_stats, true)]);

  ctx.send(CreateReply::default().embed(embed)).await.unwrap();

  Ok(())
}
