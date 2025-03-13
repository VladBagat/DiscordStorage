mod algorithm;
use crate::discord::commands::algorithm::{deconstruct, reconstruct};
use poise::CreateReply;
use poise::
    serenity_prelude::{all::{ChannelId, CreateMessage, Message}, CreateAttachment, GetMessages, MessageId};
use serenity::all::EditMessage;
use uuid::Uuid;
use crate::discord::{Context, Error};
use std::path::{PathBuf, Path};
use std::fs::{File, create_dir_all};
use std::io;
use crate::discord::utils::Config;
use regex::Regex;
use poise::futures_util::StreamExt;
mod cache;
use crate::discord::commands::cache::CacheData;

#[poise::command(slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "This is an example bot made to showcase features of my custom Discord bot framework",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn upload(
    ctx: Context<'_>,
    #[description = "Path to read from"] path: String,
    #[description = "Name of the upload"] name: String
) -> Result<(), Error> {
    let config: &Config  = &*ctx.data().config.read().await;
    let storage_channel: ChannelId = ChannelId::new(config.storage_channel);
    let cache_channel: ChannelId = ChannelId::new(config.cache_channel);

    let id: String = Uuid::new_v4().to_string();
    storage_channel.say(ctx, format!("Upload: {:?}. ID: {}", &name, &id)).await?;

    ctx.send(acknowledge()).await?;
    let status_message_builder = CreateMessage::new().content(format!("Starting upload of {}.", &name));
    let mut status_message = ctx.channel_id().send_message(ctx, status_message_builder).await?;

    let first_message: Message = storage_channel.say(ctx, "====START OF UPLOAD====").await?;

    let file_paths: Vec<std::path::PathBuf> = deconstruct(&path.trim_matches('"')).unwrap();
    let total_files = &file_paths.len();
    let mut counter = 0;

    for path in file_paths {
        send_file(&ctx, storage_channel.clone(), path).await?;
        counter += 1;
        status_message.edit(ctx, EditMessage::new().content(format!("{}/{} files uploaded.", &counter, total_files))).await?;
    }

    let last_message: Message = storage_channel.say(ctx, "====END OF UPLOAD====").await?;

    cache_channel.say(ctx, format!("Upload: {:?}. ID: {}.\nStart: {}. End: {}. Files: {}.", &name, &id, first_message.id, last_message.id, total_files+1)).await?;

    status_message.edit(ctx, EditMessage::new().content("Upload successful")).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn download(
    ctx: Context<'_>,
    #[description = "Name of the upload"] name: String,
    #[autocomplete = "poise::builtins::autocomplete_command"]
    #[description = "Unique id to locate upload"] id: Option<String>
) -> Result<(), Error> {

    let id: String = match id {
        Some(val) => val,
        None => String::new()
    };

    let config: &Config  = &*ctx.data().config.read().await;
    let storage_channel: ChannelId = ChannelId::new(config.storage_channel);
    let cache_channel: ChannelId = ChannelId::new(config.cache_channel);

    ctx.send(acknowledge()).await?;
    let status_message_builder = CreateMessage::default().content(format!("Searching location of {}.", &name));
    let mut status_message = ctx.channel_id().send_message(&ctx.http(), status_message_builder).await?;

    let mut matched_message: Vec<CacheData> = vec![];
    let mut messages = cache_channel.messages_iter(&ctx).boxed();
    while let Some(Ok(message)) = messages.next().await {

        if !message.content.starts_with("DATA!") {
            break;
        }

        let cached_data = match fetch_cache_data(&message.content).await{
            Ok(data) => data,
            Err(e) => return Err(e)
        };

        if !id.is_empty() && cached_data.id == id && cached_data.name == name {
            matched_message.push(cached_data);
            break;
        }
        else if id.is_empty() && cached_data.name == name {
            matched_message.push(cached_data);
            
        }
    }

    match matched_message.len() {
        0 => {
            println!("Nothing happened");
        },
        1 => {
            let mut counter: i32 = 0;

            let cache = &matched_message[0];
            
            let mut start_id = cache.start_message_id;

            'L: loop {
                let builder = GetMessages::new().after(MessageId::new(start_id));
                let mut messages = storage_channel.messages(&ctx.http(), builder).await?;
                messages.reverse();
                for message in messages.iter() {
                    if message.content == "====END OF UPLOAD====" {
                        break 'L;
                    }
                    else if message.author.bot && message.attachments.len() > 0 {
                        let download_path = &message.attachments[0].url;
                        download_file(&download_path, &message.content).await?;
                        status_message.edit(ctx, EditMessage::new().content(format!("{}/{} files downloaded.", &counter, cache.total_files))).await?;
                        counter += 1;
                    }
                }
                start_id = messages[messages.len()-1].id.get();
            }
            reconstruct()?;
            status_message.edit(ctx, EditMessage::new().content("Installation successful")).await?;

        },
        _ => {
            println!("Nothing happened, but a lot was found");
        }
    }

    Ok(())
}

async fn send_file(ctx: &Context<'_>, c_id: ChannelId, path: PathBuf) -> Result<(), Error> {
    let file= CreateAttachment::path(&path).await.unwrap();
    let m = CreateMessage::default();
    let m = CreateMessage::content(m,format!("{:?}", path.display())).add_file(file);
    c_id.send_message(ctx, m).await?;   
    Ok(())
}

async fn download_file(download_path: &str, save_path: &str) -> Result<(), Error> {
    let resp: reqwest::Response = reqwest::get(download_path).await.expect(format!("Failed to download a file from url {}", download_path).as_str());
    if let Some(parent) = Path::new(save_path.trim_matches('"')).parent() {
        create_dir_all(parent).expect("Failed to create directory");
    }
    let mut out: File = File::create(Path::new(save_path.trim_matches('"'))).expect("Failed to create file");
    io::copy(&mut resp.bytes().await?.as_ref(), &mut out).expect(&format!("Failed to copy content for file {}", save_path));
    Ok(())
}     

async fn fetch_cache_data(message: &str) -> Result<CacheData, Error> {
    let data_pattern: Regex = Regex::new(r#"Upload: "(.*?)"\. ID: (.*)\nStart: (.*)\. End: (.*)\. Files: (\d*)\."#).unwrap();
    if let Some(cached_data) = data_pattern.captures(message) {
        Ok(CacheData::new()
        .name(cached_data.get(1).unwrap().as_str().to_string())
        .id(cached_data.get(2).unwrap().as_str().to_string())
        .start_message_id(cached_data.get(3).unwrap().as_str().to_string().parse()?)
        .end_message_id(cached_data.get(4).unwrap().as_str().to_string().parse()?)
        .total_files(cached_data.get(5).unwrap().as_str().parse()?)
        )
    }
    else {
        Err(format!("No information retrieved from {message}").into())
    }
}
fn acknowledge() -> CreateReply {
    let msg = "Acknowledged! See next message for completion status";
    let status_message_builder = CreateReply::default().content(msg).ephemeral(true);
    status_message_builder
}