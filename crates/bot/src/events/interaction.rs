mod button_role;
mod planting;

pub use planting::planting_info_message;

use {
  asahi::{
    AsahiResult,
    debug,
    warn
  },
  button_role::{
    button_role,
    env2id
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
      _ if id.starts_with("btn-") => {
        if let Some(role_id) = env2id(id) {
          button_role(ctx, cmpt, role_id).await?;
        } else {
          warn!("got unmapped button: {id}")
        }
      },
      _ => warn!("unimplemented route: {id}")
    }
  }

  Ok(())
}

pub mod util {
  use {
    crate::data::BotData,
    poise::serenity_prelude::{
      Context,
      EmojiId,
      ReactionType,
      small_fixed_array::FixedString
    }
  };

  pub fn reaction(
    id: u64,
    name: &str
  ) -> ReactionType {
    ReactionType::Custom {
      id:       EmojiId::new(id),
      animated: false,
      name:     Some(FixedString::from_str_trunc(name))
    }
  }

  pub fn fs_emojis(ctx: &Context) -> (ReactionType, ReactionType) {
    let fs22 = reaction(ctx.data::<BotData>().emojis.fs22, "fs22");
    let fs25 = reaction(ctx.data::<BotData>().emojis.fs25, "fs25");
    (fs22, fs25)
  }
}
