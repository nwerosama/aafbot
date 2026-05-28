mod planting;

pub use planting::planting_info_message;

use {
  asahi::{
    AsahiResult,
    debug,
    warn
  },
  planting::{
    GuideKind,
    unified_guide
  },
  poise::serenity_prelude::{
    ComponentInteractionDataKind,
    Context,
    Interaction
  }
};

pub async fn on_interaction(
  ctx: &Context,
  interaction: &Interaction
) -> AsahiResult {
  if let Interaction::Component(cmpt) = interaction {
    let id = cmpt.data.custom_id.as_str();

    match id {
      "planting-main" | "equipment-main" => {
        if let ComponentInteractionDataKind::StringSelect { values } = &cmpt.data.kind
          && let Some(chosen) = values.first()
        {
          let (kind, prefix) = if id == "planting-main" {
            (GuideKind::Planting, "planting-")
          } else {
            (GuideKind::Equipment, "equipment-")
          };

          debug!("attempting to call unified_guide for {chosen}");
          let server = chosen.strip_prefix(prefix).unwrap_or_default();
          unified_guide(ctx, cmpt, kind, server).await.unwrap()
        }
      },
      _ => warn!("unimplemented route: {id}")
    }
  }

  Ok(())
}
