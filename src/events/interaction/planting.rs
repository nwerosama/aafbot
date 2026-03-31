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

/// Fetches Unix epoch from `timestamp.txt` on asset server
async fn fetch_unix_epoch(
  url: &str,
  guide_kind: GuideKind,
  server: &str
) -> i64 {
  reqwest::get(format!("{url}/files/{guide_kind}/{server}/timestamp.txt"))
    .await
    .expect("error fetching timestamp value")
    .text()
    .await
    .unwrap()
    .trim()
    .parse::<i64>()
    .unwrap_or(0)
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
  let base_url = var("AAF_ASSETS").expect("No 'AAF_ASSETS' key found");
  let image_url = format!("{base_url}/files/planting/{server}/planting-guide.png");
  let unix_ts = fetch_unix_epoch(&base_url, GuideKind::Planting, server).await;

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
  let base_url = var("AAF_ASSETS").expect("No 'AAF_ASSETS' key found");
  let guide_url = format!("{base_url}/files/equipment/{server}");
  let unix_ts = fetch_unix_epoch(&base_url, GuideKind::Equipment, server).await;

  let data = match server {
    "grain22" => MediaData {
      title: "Grain 22",
      media: vec![],
      unix_ts
    },
    "animals22" => MediaData {
      title: "Animals 22",
      media: vec![],
      unix_ts
    },
    "grain25" => MediaData {
      title: "Grain 25",
      media: vec![
        format!("{guide_url}/Grain_RiceCrops.png"),
        format!("{guide_url}/Grain_RootCrops.png"),
        format!("{guide_url}/Grain_RootCrops2.png"),
        format!("{guide_url}/Grain_Sugar-1.png"),
        format!("{guide_url}/Grain_Combines.png"),
        format!("{guide_url}/Grain_Trucks.png"),
      ],
      unix_ts
    },
    "animals25" => MediaData {
      title: "Animals 25",
      media: vec![
        format!("{guide_url}/Animals_Bailing.png"),
        format!("{guide_url}/Animals_Cotton.png"),
        format!("{guide_url}/Animals_Harvest.png"),
        format!("{guide_url}/Animals_Silage1.png"),
        format!("{guide_url}/Animals_Silage2.png"),
      ],
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
