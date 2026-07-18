mod commands;
mod data;
mod errors;
mod events;
mod tasuku;

use {
  asahi::{
    error,
    info,
    utils::database::prepare_tables
  },
  data::{
    BotData,
    Emojis
  },
  events::DiscordEvents,
  poise::{
    Framework,
    FrameworkOptions,
    PrefixFrameworkOptions,
    serenity_prelude::{
      ActivityData,
      ClientBuilder,
      CreateAllowedMentions,
      GatewayIntents,
      GuildId,
      OnlineStatus,
      Token,
      UserId
    }
  },
  std::{
    borrow::Cow,
    env::var,
    sync::Arc
  }
};

#[tokio::main]
async fn main() {
  asahi::utils::install_crypto_provider();
  asahi::log_init();

  let health_probe = Arc::new(asahi::Probe::new());
  health_probe.spawn_server(9100);

  let bot_data = Arc::new(BotData {
    embed_color: if cfg!(feature = "production") { 0x0FD4F2 } else { 0xD1A0F5 },
    main_guild:  if cfg!(feature = "production") {
      GuildId::new(1392400099258404976)
    } else {
      GuildId::new(1160707095323222068)
    },
    notify_dev:  UserId::new(190407856527376384),
    database:    aaf_shared::database::init("AAFBot").await,
    site_url:    var("AAF_SITE").expect("No 'AAF_SITE' key found!"),
    emojis:      Emojis {
      fs22: 1483331116227104851,
      fs25: 1483331150104756266
    }
  });

  if let Err(e) = prepare_tables(&bot_data.database, "schemas").await {
    error!("Failed to prepare the database: {e}");
    std::process::exit(1)
  }

  tasuku::init(bot_data.database.clone());

  let framework = Framework::builder()
    .options(FrameworkOptions {
      commands: commands::collect(),
      pre_command: |ctx| {
        Box::pin(async move {
          let guild_name: Cow<'_, str> = match ctx.guild() {
            Some(g) => Cow::Owned(g.name.clone().into()),
            None => Cow::Borrowed("Unknown Guild")
          };
          let guild_channel_name = match ctx.channel().await {
            Some(c) => format!("in #{}", c.guild().unwrap_or_default().base.name),
            None => String::from("")
          };
          let prefix = match ctx.command().prefix_action {
            Some(_) => ctx.framework().options.prefix_options.prefix.as_ref().map_or("@me ", |v| v),
            None => "/"
          };

          info!(
            "Discord({guild_name}): {} executed {prefix}{} {guild_channel_name}",
            ctx.author().name,
            ctx.command().qualified_name
          );
        })
      },
      prefix_options: PrefixFrameworkOptions {
        mention_as_prefix: true,
        case_insensitive_commands: true,
        execute_self_messages: false,
        execute_untracked_edits: false,
        ignore_bots: true,
        ..Default::default()
      },
      on_error: |error| Box::pin(async move { errors::fw_errors(error).await }),
      allowed_mentions: Some(CreateAllowedMentions::default().empty_roles().empty_users()),
      skip_checks_for_owners: true,
      initialize_owners: true,
      ..Default::default()
    })
    .build();

  let mut client = ClientBuilder::new(
    Token::from_env("DISCORD_TOKEN").expect("No 'DISCORD_TOKEN' found!"),
    GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT
  )
  .event_handler(Arc::new(DiscordEvents {
    probe: Arc::clone(&health_probe)
  }))
  .framework(Box::new(framework))
  .data(bot_data)
  .status(OnlineStatus::Online)
  .activity(ActivityData::custom("I love Taisho Sanke koi fish!"))
  .await
  .expect("Error creating Serenity client");

  let exit_signal = tokio::spawn(async move { shutdown::gracefully_shutdown().await });

  tokio::select! {
    client_result = client.start() => {
      if let Err(e) = client_result {
        error!("Serenity client failed: {e:#?}");
      }
    },
    shutdown = exit_signal => {
      if shutdown.unwrap() {
        std::process::exit(0);
      }
    }
  }
}

mod shutdown {
  use tokio::signal::unix::{
    SignalKind,
    signal
  };

  pub async fn gracefully_shutdown() -> bool {
    let [mut s1, mut s2, mut s3] = [
      signal(SignalKind::hangup()).unwrap(),
      signal(SignalKind::interrupt()).unwrap(),
      signal(SignalKind::terminate()).unwrap()
    ];

    tokio::select!(
      v = s1.recv() => v.unwrap(),
      v = s2.recv() => v.unwrap(),
      v = s3.recv() => v.unwrap(),
    );

    asahi::info!("Goodbye! 👋");
    true
  }
}
