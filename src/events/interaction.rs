mod planting;

pub use planting::planting_info_message;

use {
  asahi::{
    AsahiResult,
    debug,
    warn
  },
  planting::{
    equipment_guide,
    planting_guide
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

    debug!("received request for {id}");

    match id {
      "planting-main" => {
        if let ComponentInteractionDataKind::StringSelect { values } = &cmpt.data.kind
          && let Some(chosen) = values.first()
        {
          debug!("attempting to call planting_guide for {chosen}");
          let server = chosen.strip_prefix("planting-").unwrap_or_default();
          planting_guide(ctx, cmpt, server).await.unwrap()
        }
      },
      "equipment-main" => {
        if let ComponentInteractionDataKind::StringSelect { values } = &cmpt.data.kind
          && let Some(chosen) = values.first()
        {
          debug!("attempting to call equipment_guide for {chosen}");
          let server = chosen.strip_prefix("equipment-").unwrap_or_default();
          equipment_guide(ctx, cmpt, server).await.unwrap()
        }
      },
      _ => warn!("unimplemented route: {id}")
    }
  }

  Ok(())
}
