mod algorithm;
use crate::discord::commands::algorithm::deconstruct;
use serenity::all::{ChannelId, CreateMessage};
use uuid::Uuid;
use crate::discord::{Context, Error};
use poise::serenity_prelude::CreateAttachment;
use std::path::{PathBuf, Path};
use std::fs::{File, create_dir_all};
use std::io;

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
    #[description = "`<path>` to read from"] path: String,
    #[description = "`<name>` to save file under"] name: String
) -> Result<(), Error> {
    let c_id: ChannelId = ctx.channel_id();
    let id: String = Uuid::new_v4().to_string();
    c_id.say(ctx, format!("Upload: {:?}. ID: {}", &name, &id)).await?;
        
    c_id.say(ctx, "====START OF UPLOAD====").await?;
    let file_paths: Vec<std::path::PathBuf> = deconstruct(&path).unwrap();
    for path in file_paths {
        send_file(&ctx, c_id.clone(), path).await?;
    }
    c_id.say(ctx, "====END OF UPLOAD====").await?;
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