#![allow(dead_code)]
#![allow(unused_imports)]


mod discord;
use discord::discord as discord_main;
use serenity::all::token;
use text_io::read;
use std::path::PathBuf;

fn main() {
     user_interaction();
}

fn user_interaction() {
     println!("Welcome to the Discord Store. Complete the following steps to get started.");
     print!("1. Enter a valid Discord Token: ");
     let token: String = read!("{}\n");
     let token = token.trim().to_string();
     //It would be a good idea to validate the token here
     let directory;  
     loop {
          print!("2. Choose a directory to upload: ");
          let dir: String = read!("{}\n");
          let dir = dir.trim();
  
          if PathBuf::from(dir).is_dir() {
              directory = dir.to_string(); 
              break;
          } else {
              println!("Directory is invalid");
          }
     }

     match discord_main(&token, &directory){
          Ok(_) => {println!("Bot connected successfully");},
          Err(e) => {
               println!("Error creating client: {:?}", e);
               user_interaction();
          },
     }
}
