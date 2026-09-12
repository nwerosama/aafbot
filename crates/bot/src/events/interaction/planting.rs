use {
  aaf_shared::{
    assets::Manifest,
    load_env
  },
  asahi::{
    AsahiResult,
    warn
  },
  poise::serenity_prelude::{
    ComponentInteraction,
    Context,
    CreateActionRow,
    CreateButton,
    CreateComponent,
    CreateContainer,
    CreateContainerComponent,
    CreateInteractionResponse,
    CreateInteractionResponseMessage,
    CreateMediaGallery,
    CreateMediaGalleryItem,
    CreateSelectMenu,
    CreateSelectMenuKind,
    CreateSelectMenuOption,
    CreateSeparator,
    CreateTextDisplay,
    CreateUnfurledMediaItem,
    EditMessage,
    GenericChannelId,
    MessageFlags,
    MessageId,
    ReactionType,
    builder::CreateMessage,
    small_fixed_array::FixedString
  }
};

/// Structure for guide images
struct MediaData {
  title:   &'static str,
  media:   Vec<String>,
  unix_ts: i64
}

#[non_exhaustive]
pub enum GuideKind {
  Planting,
  Equipment
}

impl std::fmt::Display for GuideKind {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>
  ) -> std::fmt::Result {
    match self {
      Self::Planting => "planting",
      Self::Equipment => "equipment"
    }
    .fmt(f)
  }
}

/// Fetches the timestamp and URLs from a generated manifest
async fn get_manifest(
  guide_kind: GuideKind,
  server: &str
) -> Manifest {
  let base = load_env("AAF_ASSETS");
  reqwest::get(format!("{base}/files/manifest/{guide_kind}/{server}"))
    .await
    .expect("error fetching manifest")
    .json::<Manifest>()
    .await
    .expect("manifest not available")
}

fn planting_components(ctx: &'_ Context) -> CreateComponent<'_> {
  let (fs22, fs25) = super::util::fs_emojis(ctx);

  let handbook_url = load_env("AAF_PLANTING_HANDBOOK");

  CreateComponent::Container(CreateContainer::new(vec![
    CreateContainerComponent::ActionRow(CreateActionRow::Buttons(
      vec![
        CreateButton::new_link(handbook_url)
          .label("Handbook")
          .emoji(ReactionType::Unicode(FixedString::from_str_trunc("📗"))),
      ]
      .into()
    )),
    CreateContainerComponent::Separator(CreateSeparator::new()),
    CreateContainerComponent::TextDisplay(CreateTextDisplay::new("Planting guide")),
    CreateContainerComponent::ActionRow(CreateActionRow::SelectMenu(CreateSelectMenu::new(
      "planting-main",
      CreateSelectMenuKind::String {
        options: vec![
          CreateSelectMenuOption::new("Select this to view again", "planting-noop").default_selection(true),
          CreateSelectMenuOption::new("Grain 22", "planting-grain22").emoji(fs22.clone()),
          CreateSelectMenuOption::new("Animals 22", "planting-animals22").emoji(fs22.clone()),
          CreateSelectMenuOption::new("Silage 25", "planting-silage25").emoji(fs25.clone()),
          CreateSelectMenuOption::new("Grain 25", "planting-grain25").emoji(fs25.clone()),
          CreateSelectMenuOption::new("Animals 25", "planting-animals25").emoji(fs25.clone()),
        ]
        .into()
      }
    ))),
    CreateContainerComponent::TextDisplay(CreateTextDisplay::new("Equipment guide")),
    CreateContainerComponent::ActionRow(CreateActionRow::SelectMenu(CreateSelectMenu::new(
      "equipment-main",
      CreateSelectMenuKind::String {
        options: vec![
          CreateSelectMenuOption::new("Select this to view again", "equipment-noop").default_selection(true),
          CreateSelectMenuOption::new("Silage 22", "equipment-silage22").emoji(fs22.clone()),
          CreateSelectMenuOption::new("Grain 22", "equipment-grain22").emoji(fs22.clone()),
          CreateSelectMenuOption::new("Silage 25", "equipment-silage25").emoji(fs25.clone()),
          CreateSelectMenuOption::new("Grain 25", "equipment-grain25").emoji(fs25.clone()),
          CreateSelectMenuOption::new("Animals 25", "equipment-animals25").emoji(fs25),
        ]
        .into()
      }
    ))),
  ]))
}

pub async fn planting_info_message(
  ctx: &Context,
  channel_id: GenericChannelId,
  message_id: Option<MessageId>
) {
  if let Some(msg_id) = message_id {
    channel_id
      .edit_message(
        &ctx.http,
        msg_id,
        EditMessage::default()
          .components(vec![planting_components(ctx)])
          .flags(MessageFlags::IS_COMPONENTS_V2)
      )
      .await
      .expect("discord threw an error");
  } else {
    channel_id
      .send_message(
        &ctx.http,
        CreateMessage::default()
          .components(vec![planting_components(ctx)])
          .flags(MessageFlags::IS_COMPONENTS_V2)
      )
      .await
      .expect("discord threw an error");
  }
}

pub async fn unified_guide(
  ctx: &Context,
  interaction: &ComponentInteraction,
  guide_kind: GuideKind,
  server: &str
) -> AsahiResult {
  if server == "noop" {
    let _ = interaction.create_response(&ctx.http, CreateInteractionResponse::Acknowledge).await;
    return Ok(())
  }

  let manifest = get_manifest(guide_kind, server).await;
  let media = manifest.media;
  let unix_ts = manifest.timestamp;

  let data = match server {
    "silage22" => MediaData {
      title: "Silage 22",
      media,
      unix_ts
    },
    "silage25" => MediaData {
      title: "Silage 25",
      media,
      unix_ts
    },
    "grain22" => MediaData {
      title: "Grain 22",
      media,
      unix_ts
    },
    "grain25" => MediaData {
      title: "Grain 25",
      media,
      unix_ts
    },
    "animals22" => MediaData {
      title: "Animals 22",
      media,
      unix_ts
    },
    "animals25" => MediaData {
      title: "Animals 25",
      media,
      unix_ts
    },
    _ => return Ok(())
  };

  if data.media.is_empty() {
    warn!("Empty media for {server}, abort!");
    interaction
      .create_response(
        &ctx.http,
        CreateInteractionResponse::Message(
          CreateInteractionResponseMessage::default()
            .content("There's no media found for this server!")
            .flags(MessageFlags::EPHEMERAL)
        )
      )
      .await
      .unwrap();
  }

  let items: Vec<CreateMediaGalleryItem> = data
    .media
    .iter()
    .map(|i| CreateMediaGalleryItem::new(CreateUnfurledMediaItem::new(format!("{i}?v={}", data.unix_ts))))
    .collect();

  interaction
    .create_response(
      &ctx.http,
      CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::default()
          .components(vec![CreateComponent::Container(CreateContainer::new(vec![
            CreateContainerComponent::TextDisplay(CreateTextDisplay::new(format!("**{}** - (<t:{}:s>)", data.title, data.unix_ts))),
            CreateContainerComponent::MediaGallery(CreateMediaGallery::new(items)),
          ]))])
          .flags(MessageFlags::IS_COMPONENTS_V2 | MessageFlags::EPHEMERAL)
      )
    )
    .await
    .unwrap();

  Ok(())
}
