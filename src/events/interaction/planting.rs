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
    EmojiId,
    GenericChannelId,
    MessageFlags,
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
  image_url: &'static str,
  unix_ts:   i64
}

/// Structure for Equipment guide images
struct MediaData {
  title:   &'static str,
  media:   Vec<&'static str>,
  unix_ts: i64
}

pub async fn planting_info_message(
  ctx: &Context,
  channel_id: GenericChannelId
) {
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

  let cv2 = CreateComponent::Container(CreateContainer::new(vec![
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
          // CreateSelectMenuOption::new("Animals 25", "equipment-animals25").emoji(fs25),
        ]
        .into()
      }
    ))),
  ]));

  channel_id
    .send_message(
      &ctx.http,
      CreateMessage::default().components(vec![cv2]).flags(MessageFlags::IS_COMPONENTS_V2)
    )
    .await
    .unwrap();
}

pub async fn planting_guide(
  ctx: &Context,
  interaction: &ComponentInteraction,
  server: &str
) -> AsahiResult {
  let data = match server {
    "grain22" => EmbedData {
      title:     "Grain 22",
      image_url: "https://cdn.discordapp.com/attachments/1291076787559989288/1471844013148475435/Grain_22_Planting_Guide.png",
      unix_ts:   1772269700
    },
    "animals22" => EmbedData {
      title:     "Animals 22",
      image_url: "https://cdn.discordapp.com/attachments/1291076787559989288/1471844012125323458/Animals_22_Planting_Guide.png",
      unix_ts:   1772269700
    },
    "grain25" => EmbedData {
      title:     "Grain 25",
      image_url: "https://cdn.discordapp.com/attachments/1291076787559989288/1471844013836468254/Grain_25_Planting_Guide.png",
      unix_ts:   1772269700
    },
    "animals25" => EmbedData {
      title:     "Animals 25",
      image_url: "https://cdn.discordapp.com/attachments/1291076787559989288/1471844012641091614/Animals_25_Planting_Guide.png",
      unix_ts:   1772269700
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
            .image(data.image_url)
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
  let data = match server {
    "grain22" => MediaData {
      title:   "Grain 22",
      media:   vec![],
      unix_ts: 1772269700
    },
    "animals22" => MediaData {
      title:   "Animals 22",
      media:   vec![],
      unix_ts: 1772269700
    },
    "grain25" => MediaData {
      title:   "Grain 25",
      media:   vec![
        "https://cdn.discordapp.com/attachments/1311282815601741844/1477346896522510437/Grain_RiceCrops.png",
        "https://cdn.discordapp.com/attachments/1311282815601741844/1477346897097392283/Grain_RootCrops.png",
        "https://cdn.discordapp.com/attachments/1311282815601741844/1477346897596252404/Grain_RootCrops2.png",
        "https://cdn.discordapp.com/attachments/1311282815601741844/1477346898007556168/Grain_Sugar-1.png",
        "https://cdn.discordapp.com/attachments/1311282815601741844/1477346896069656597/Grain_Combines.png",
        "https://cdn.discordapp.com/attachments/1311282815601741844/1477346898670256371/Grain_Trucks.png",
      ],
      unix_ts: 1772269700
    },
    "animals25" => MediaData {
      title:   "Animals 25",
      media:   vec![],
      unix_ts: 1772269700
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
    .map(|i| CreateMediaGalleryItem::new(CreateUnfurledMediaItem::new(*i)))
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
