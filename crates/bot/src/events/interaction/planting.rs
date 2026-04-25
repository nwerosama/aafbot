use {
  crate::data::BotData,
  asahi::AsahiResult,
  poise::serenity_prelude::{
    ComponentInteraction,
    Context,
    CreateActionRow,
    CreateButton,
    CreateComponent,
    CreateContainer,
    CreateContainerComponent,
    CreateEmbed,
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
    EmojiId,
    GenericChannelId,
    MessageFlags,
    MessageId,
    ReactionType,
    Timestamp,
    builder::CreateMessage,
    small_fixed_array::FixedString
  },
  std::env::var
};

/// Structure for Planting guide embeds
struct EmbedData {
  title:     &'static str,
  image_url: String,
  unix_ts:   i64
}

/// Structure for Equipment guide images
struct MediaData {
  title:   &'static str,
  media:   Vec<String>,
  unix_ts: i64
}

/// Assets server manifest
#[derive(serde::Deserialize)]
struct Manifest {
  /// Retrieved from `timestamp.txt`
  timestamp: i64,
  /// List of visible files in the directory
  media:     Vec<String>
}

#[non_exhaustive]
enum GuideKind {
  Planting,
  Equipment
}

impl std::fmt::Display for GuideKind {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>
  ) -> std::fmt::Result {
    let k = match self {
      Self::Planting => "planting",
      Self::Equipment => "equipment"
    };
    write!(f, "{k}")
  }
}

/// Fetches the timestamp and URLs from a generated manifest
async fn get_manifest(
  guide_kind: GuideKind,
  server: &str
) -> Manifest {
  let base = var("AAF_ASSETS").expect("No 'AAF_ASSETS' key found");
  reqwest::get(format!("{base}/files/manifest/{guide_kind}/{server}"))
    .await
    .expect("error fetching manifest")
    .json::<Manifest>()
    .await
    .expect("manifest not available")
}

fn planting_components(ctx: &'_ Context) -> CreateComponent<'_> {
  let fs22 = ReactionType::Custom {
    id:       EmojiId::new(ctx.data::<BotData>().emojis.fs22),
    animated: false,
    name:     Some(FixedString::from_str_trunc("fs22"))
  };
  let fs25 = ReactionType::Custom {
    id:       EmojiId::new(ctx.data::<BotData>().emojis.fs25),
    animated: false,
    name:     Some(FixedString::from_str_trunc("fs25"))
  };

  let handbook_url = var("AAF_PLANTING_HANDBOOK").expect("No 'AAF_PLANTING_HANDBOOK' key found");

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
          CreateSelectMenuOption::new("Grain 22", "planting-grain22").emoji(fs22.clone()),
          CreateSelectMenuOption::new("Animals 22", "planting-animals22").emoji(fs22.clone()),
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
          // comment out the ones that we do not have images for
          // CreateSelectMenuOption::new("Grain 22", "equipment-grain22").emoji(fs22.clone()),
          // CreateSelectMenuOption::new("Animals 22", "equipment-animals22").emoji(fs22),
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

pub async fn planting_guide(
  ctx: &Context,
  interaction: &ComponentInteraction,
  server: &str
) -> AsahiResult {
  let manifest = get_manifest(GuideKind::Planting, server).await;
  let image_url = manifest.media.first().expect("no media in manifest").clone().to_string();
  let unix_ts = manifest.timestamp;

  let data = match server {
    "grain22" => EmbedData {
      title: "Grain 22",
      image_url,
      unix_ts
    },
    "animals22" => EmbedData {
      title: "Animals 22",
      image_url,
      unix_ts
    },
    "grain25" => EmbedData {
      title: "Grain 25",
      image_url,
      unix_ts
    },
    "animals25" => EmbedData {
      title: "Animals 25",
      image_url,
      unix_ts
    },
    _ => return Ok(())
  };

  interaction
    .create_response(
      &ctx.http,
      CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::default().ephemeral(true).add_embed(
          CreateEmbed::default()
            .color(ctx.data::<BotData>().embed_color)
            .title(data.title)
            .image(format!("{}?v={}", data.image_url, data.unix_ts))
            .timestamp(Timestamp::from_unix_timestamp(data.unix_ts).expect("Time went on an adventure"))
        )
      )
    )
    .await
    .unwrap();

  Ok(())
}

pub async fn equipment_guide(
  ctx: &Context,
  interaction: &ComponentInteraction,
  server: &str
) -> AsahiResult {
  let manifest = get_manifest(GuideKind::Equipment, server).await;
  let media = manifest.media;
  let unix_ts = manifest.timestamp;

  let data = match server {
    "grain25" => MediaData {
      title: "Grain 25",
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
    asahi::warn!("Empty media for {server}, abort!");
    return Ok(());
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
