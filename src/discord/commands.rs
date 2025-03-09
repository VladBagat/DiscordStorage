mod algorithm;
use crate::discord::commands::algorithm::deconstruct;
use poise::{serenity_prelude::{all::{ChannelId, CreateMessage, Message}, CreateAttachment}, CreateReply};
use uuid::Uuid;
use crate::discord::{Context, Error};
use std::path::{PathBuf, Path};
use std::fs::{File, create_dir_all};
use std::io;
use crate::discord::utils::Config;

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
    let status_message_builder = CreateReply::default().content(format!("Starting upload of {}.", &name)).ephemeral(true);
    let status_message = ctx.send(status_message_builder).await?;
    let first_message: Message = storage_channel.say(ctx, "====START OF UPLOAD====").await?;
    let file_paths: Vec<std::path::PathBuf> = deconstruct(&path).unwrap();
    let total_files = &file_paths.len();
    let mut counter = 0;
    for path in file_paths {
        send_file(&ctx, storage_channel.clone(), path).await?;
        counter += 1;
        status_message.edit(ctx, CreateReply::default().content(format!("{}/{} files uploaded.", &counter, total_files)).ephemeral(true)).await?;
    }
    let last_message: Message = storage_channel.say(ctx, "====END OF UPLOAD====").await?;

    cache_channel.say(ctx, format!("Upload: {:?}. ID: {}\nStart: {}. End: {}", &name, &id, first_message.id, last_message.id)).await?;

    status_message.edit(ctx, CreateReply::default().content("Upload successful").ephemeral(true)).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn downloadload(
    ctx: Context<'_>,
    #[description = "Name of the upload"] name: String,
    #[autocomplete = "poise::builtins::autocomplete_command"]
    #[description = "Unique id to locate upload"] id: String
) -> Result<(), Error> {
    Ok(())
}
async fn send_file(ctx: &Context<'_>, c_id: ChannelId, path: PathBuf) -> Result<(), Error> {
    let file= CreateAttachment::path(&path).await.unwrap();
    let m = CreateMessage::default();
    let m = CreateMessage::content(m,format!("{:?}", path.display())).add_file(file);
    c_id.send_message(ctx, m).await?;   
    Ok(())
}

async fn download_file(download_path: &str, save_path: &str) {
    let resp: reqwest::Response = reqwest::get(download_path).await.expect(format!("Failed to download a file from url {}", download_path).as_str());
    if let Some(parent) = Path::new(save_path).parent() {
        create_dir_all(parent).expect("Failed to create directory");
    }
    let mut out: File = File::create(save_path).expect("Failed to create file");
    io::copy(&mut resp.bytes().await.unwrap().as_ref(), &mut out).expect("Failed to copy content");
}     