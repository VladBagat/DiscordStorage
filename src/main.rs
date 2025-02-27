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
     match discord_main(&token){
          Ok(_) => {},
          Err(e) => {
               println!("Error creating client: {:?}", e);
               println!("Make sure your token is valid");
               user_interaction();
          },
     }
}
