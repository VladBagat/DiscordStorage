mod commands;
pub mod utils;
use ::serenity::all::{ChannelId, CreateChannel};
use utils::Config;
use poise::serenity_prelude as serenity;
use std::{sync::Arc, time::Duration};

use colored::*;


// Types used by all command functions
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// Custom user data passed to all command functions
#[derive(Debug)]
pub struct Data {
    pub modify_config: bool,
    pub auto: bool,
    pub config: Option<Config>,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            modify_config: false,
            auto: false,
            config: None,            
        }
    }
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("{} {}!", "✓ Connected to Discord as".green().bold(), data_about_bot.user.name.green().bold());
            if data.auto {
                println!("{}", "Running auto configuration...".blue());
            }
            
            if data.modify_config{
                if data.auto {
                    let guild = ctx.cache.guilds()[0];
                    let category_id: ChannelId = guild.create_channel(
                        &ctx.http,
                        CreateChannel::new("DiscordStorage").kind(serenity::ChannelType::Category)
                        ).await?.id;

                    let cache_channel_id: ChannelId = guild.create_channel(
                        &ctx.http,
                        CreateChannel::new("Cache").kind(serenity::ChannelType::Text).category(category_id)
                        ).await?.id;

                    let storage_channel_id: ChannelId = guild.create_channel(
                        &ctx.http,
                        CreateChannel::new("Storage").kind(serenity::ChannelType::Text).category(category_id)
                        ).await?.id;

                    cache_channel_id.say(&ctx.http,
                        "This channel is used for internal operations. Please, do not alter data in this channel."
                    ).await?;
                    storage_channel_id.say(&ctx.http,
                        "This channel is used for internal operations. Please, do not alter data in this channel."
                    ).await?;
                    
                    let category: u64 = category_id.get();
                    let cache_channel: u64 = cache_channel_id.get();
                    let storage_channel: u64 = storage_channel_id.get();

                    let config = Config { token:ctx.http.token().to_owned(), category, cache_channel, storage_channel };
                    utils::write_config(&config)?;
                }
                else {
                    utils::write_config(&data.config.as_ref().unwrap())?;
                }
            }
            
            if data.auto{
                println!("{}", "\n✓ Configuration complete!".green().bold());
            }
            
        }
        _ => {}
    }
    Ok(())
}

#[tokio::main]
pub async fn discord(token: &str, data: Data) {
    tracing_subscriber::fmt::init();

    // FrameworkOptions contains all of poise's configuration option in one struct
    // Every option can be omitted to use its default value
    let options = poise::FrameworkOptions {
        event_handler: |ctx, event, framework, data| {
            Box::pin(event_handler(ctx, event, framework, data))
        },
        commands: vec![commands::help(), commands::upload()],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("~".into()),
            edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                Duration::from_secs(3600),
            ))),
            additional_prefixes: vec![
                poise::Prefix::Literal("hey bot,"),
                poise::Prefix::Literal("hey bot"),
            ],
            ..Default::default()
        },
        // The global error handler for all error cases that may occur
        on_error: |error| Box::pin(on_error(error)),
        // This code is run before every command
        pre_command: |ctx| {
            Box::pin(async move {
                println!("Executing command {}...", ctx.command().qualified_name);
            })
        },
        // This code is run after a command if it was successful (returned Ok)
        post_command: |ctx| {
            Box::pin(async move {
                println!("Executed command {}!", ctx.command().qualified_name);
            })
        },
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(data)
            })
        })
        .options(options)
        .build();

    let intents =
        serenity::GatewayIntents::all();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
      
    client.unwrap().start().await.unwrap()
}