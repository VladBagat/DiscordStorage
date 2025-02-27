use std::ops::Deref;
use std::{env, path};
use std::path::PathBuf;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::error;
use serenity::prelude::*;
use serenity::builder::{CreateAttachment, CreateMessage};
use dotenvy::dotenv;
use reqwest::get;
mod algorithm;
use algorithm::{deconstruct, reconstruct};
use std::io;
use std::fs::File;
use serenity::futures::StreamExt;
use std::path::Path;
use std::fs::create_dir_all;
use serenity::all::Ready;

struct Handler {}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Bot connected as {}", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {

        let command;
        let target;

        if msg.author.bot{
            return;
        }
        match strip_args(&msg.content){
            Some((com, tar)) => {
                command = com.to_lowercase();
                target = tar.to_string();
            }
            None => {
                panic!("Invalid command");
                //TODO: Make an appropriate Discord handle
            }
        }

        if command == "!upload" {
            match msg.channel_id.say(&ctx.http, "Trying to call algorithm").await {
                Ok(_) => {
                    let file_paths: Vec<std::path::PathBuf> = deconstruct(&target).unwrap();
                    for path in file_paths {
                        send_file(&ctx, &msg, path).await;
                    }
                },
                Err(why) => println!("Error sending message: {why:?}"),
            }
        }
        else if command == "!download" {
            let mut messages = msg.channel_id.messages_iter(&ctx).boxed();
            while let Some(message_result) = messages.next().await {
                match message_result {
                    Ok(message) => {
                        if message.author.bot && message.attachments.len() > 0 {
                            let download_path = message.attachments[0].url.clone();
                            download_file(&download_path, &message.content).await;
                            }
                    },
                    Err(error) => eprintln!("Uh oh! Error: {}", error),
                }
            }
            match msg.channel_id.say(&ctx.http, "Trying to call algorithm").await {
                Ok(_) => {reconstruct().unwrap();},
                Err(why) => println!("Error sending message: {why:?}"),
            }
        }
    }
}


async fn send_file(ctx: &Context, msg: &Message, path: PathBuf) {
    let file= [CreateAttachment::path(&path).await.unwrap()];
    let builder = CreateMessage::new().content(path.to_str().unwrap());
    match msg.channel_id.send_files(&ctx.http, file, builder).await {
        Ok(_) => {},
        Err(why) => println!("Error sending file: {why:?}"),
    }
}

async fn download_file(download_path: &str, save_path: &str) {
    let resp: reqwest::Response = reqwest::get(download_path).await.expect(format!("Failed to download a file from url {}", download_path).as_str());
    if let Some(parent) = Path::new(save_path).parent() {
        create_dir_all(parent).expect("Failed to create directory");
    }
    let mut out: File = File::create(save_path).expect("failed to create file");
    io::copy(&mut resp.bytes().await.unwrap().as_ref(), &mut out).expect("failed to copy content");
}           

fn strip_args(user_message: &str) -> Option<(&str, &str)> {
    let args: Vec<&str> = user_message.split_whitespace().collect();
    if args.len() < 2 {
        println!("Not enough arguments");
        return None;
    }
    let command = args[0];
    let target = args[1];

    Some((command, target))
}

#[tokio::main]
pub async fn discord(token: &str) -> Result<(), serenity::Error> {

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client = Client::builder(token, intents)
    .event_handler(Handler{}).await
    .map_err(|why| {return why})?;

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        return Err(why);
    }

    Ok(())
}
