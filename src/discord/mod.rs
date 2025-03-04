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
use serenity::all::EditMessage;
use uuid::Uuid;

struct Handler {}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Bot connected as {}", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {

        if msg.author.bot {
            return;
        }

        let (command, target, name) = match strip_args(&msg.content) {
            Some((com, tar, tar_name)) => (
                com.to_lowercase(),
                tar.to_string(),
                tar_name.to_string(),
            ),
            None => {
                if let Err(e) = msg.channel_id.say(&ctx.http, "Invalid command. Use !info for more information").await {
                    eprintln!("Failed to send message: {:?}", e);
                }
                return;
            }
        };

        match command.as_str() {
            "!upload" => {
                let id: String = Uuid::new_v4().to_string();
                match msg.channel_id.say(&ctx.http, format!("Upload: {}. ID: {}", name, &id)).await {
                    Ok(_) => {
                        msg.channel_id.say(&ctx.http, "====START OF UPLOAD====")
                            .await.expect("Failed to send upload start status message");
                        let file_paths: Vec<std::path::PathBuf> = deconstruct(&target).unwrap();
                        for path in file_paths {
                            send_file(&ctx, &msg, path).await;
                        }
                        msg.channel_id.say(&ctx.http, "====END OF UPLOAD====")
                            .await.expect("Failed to send upload success status message");
                    },
                    Err(why) => println!("Error sending message: {why:?}"),
                }        
            },
            "!download" => {
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
                    Ok(_) => {
                        reconstruct().unwrap();
                        msg.channel_id.say(&ctx.http, "Installation finished successfully")
                            .await.expect("Failed to send upload success status message");
                    },
                    Err(why) => println!("Error sending message: {why:?}"),
                }
            },
            "!info" => {
                let message: &str = "Welcome to DiscordStore! You are currently running version 1.\n\
                     \n\
                     You have access to the following commands:\n\
                     \n\
                     - `!upload <path> <name>`\n\
                       This command will search the given path (preferably absolute) and upload files to the current channel.
                       You **must** upload no more than 1 project per channel.\n\
                     \n\
                     - `!download`\n\
                       This command will download and format uploaded files in the current channel.\n\
                       Find the downloaded project in the same directory as the `discorstore.exe` file.";

                msg.channel_id.say(&ctx.http, message)
                        .await.expect("Failed to send !info command message");
            },
            _ => {
                msg.channel_id.say(&ctx.http, "Unknown command. Use !info for help.")
                        .await.expect("Failed to send unknown command message");
            }
        }
        
        if command == "!upload" {
                                
        }
        else if command == "!download" {
            
        }
        else {
            
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

fn strip_args(user_message: &str) -> Option<(&str, &str, &str)> {
    let args: Vec<&str> = user_message.split_whitespace().collect();
    let command = args.get(0).unwrap_or(&"Empty");
    let target = args.get(1).unwrap_or(&"Empty");
    let name = args.get(2).unwrap_or(&"Empty");
    Some((command.to_owned(), target.to_owned(), name.to_owned()))
}

#[tokio::main]
pub async fn discord(token: &str) -> Result<(), serenity::Error> {

    let mut client = Client::builder(token,  GatewayIntents::all())
    .event_handler(Handler{}).await
    .map_err(|why| {return why})?;

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        return Err(why);
    }

    Ok(())
}
