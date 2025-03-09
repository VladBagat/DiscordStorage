#![allow(dead_code)]

mod discord;
use discord::utils::{read_config, Config};
use discord::discord as discord_main;
use discord::Data;
use regex::Regex;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use colored::*;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::RwLock;

fn main() {
     let _ = user_interaction();
}

fn user_interaction() -> Result<(), Box<dyn Error>> {
     println!("{}", "Welcome to the Discord Store. Complete the following steps to get started.".green().bold());
    
     let theme = ColorfulTheme::default();
        
     println!("{}", "Checking for existing configuration...".blue());
     let config_result = read_config();
    
     let (mut config, modify_config, auto, discord_token) = match config_result {
          Ok(conf) => {
               println!("{}", "✓ Existing configuration found!".green());
               
               let modify = Confirm::with_theme(&theme)
                    .with_prompt("Do you want to modify your config?")
                    .default(false)
                    .interact()?;
                    
               let auto_setup = if modify {
                    let options = vec!["Auto", "Manual"];
                    let selection = Select::with_theme(&theme)
                         .with_prompt("Choose setup mode")
                         .default(0)
                         .items(&options)
                         .interact()?;
                    selection == 0
               } else {
                    false
               };
               
               (Arc::new(RwLock::new(conf.clone())), modify, auto_setup, conf.token)
          },
          Err(_) => {
               println!("{}", "No existing configuration found. Creating new config...".yellow());

               let discord_token: String = Input::with_theme(&theme)
                    .with_prompt("Enter a valid Discord Token")
                    .validate_with(|input: &String| -> Result<(), &str> {
                         if verify_token(input) {
                              Ok(())
                         } else {
                              Err("Token is invalid")
                         }
                    })
                    .interact()?;
               
               let options = vec!["Auto", "Manual"];
               let selection = Select::with_theme(&theme)
                    .with_prompt("Choose setup mode")
                    .default(0)
                    .items(&options)
                    .interact()?;
                    
               (Arc::new(RwLock::new(Config::default())), true, selection == 0, discord_token)
          }
     };
    
     if !auto && modify_config {
          println!("{}", "\nChannel Configuration".cyan().bold());
          println!("{}", "Please provide the following Discord IDs:".cyan());
          
          let category: u64 = Input::with_theme(&theme)
               .with_prompt("(1/3) Select category for channels (category ID)")
               .validate_with(|input: &String| -> Result<(), &str> {
                    if !verify_channel_id(input) {
                         return Err("Invalid channel ID format");
                    }

                    match input.parse::<u64>() {
                         Ok(_) => Ok(()),
                         Err(_) => Err("Not a valid number")
                    }
               })
               .interact_text()?
               .parse()?;
            
          let cache_channel: u64 = Input::with_theme(&theme)
               .with_prompt("(2/3) Select channel for caching (channel ID)")
               .validate_with(|input: &String| -> Result<(), &str> {
                    if !verify_channel_id(input) {
                         return Err("Invalid channel ID format");
                    }
                    
                    match input.parse::<u64>() {
                         Ok(val) => {
                         if val == category {
                              return Err("Cannot use the same ID for multiple channels");
                         }
                         Ok(())
                         },
                         Err(_) => Err("Not a valid number")
                    }
               })
               .interact_text()?
               .parse()?;
            
          let storage_channel: u64 = Input::with_theme(&theme)
               .with_prompt("(3/3) Select channel for storage (channel ID)")
               .validate_with(|input: &String| -> Result<(), &str> {
                    if !verify_channel_id(input) {
                         return Err("Invalid channel ID format");
                    }
                    
                    match input.parse::<u64>() {
                         Ok(val) => {
                         if val == category || val == cache_channel {
                              return Err("Cannot use the same ID for multiple channels");
                         }
                         Ok(())
                         },
                         Err(_) => Err("Not a valid number")
                    }
               })
               .interact_text()?
               .parse()?;
            
          config = Arc::new(RwLock::new(
               Config { 
               token: discord_token.clone(), 
               category, 
               cache_channel, 
               storage_channel 
               }
          ));

          println!("{}", "\n✓ Configuration complete!".green().bold());
     } else if auto && modify_config {
          println!("{}", "Bot will auto-configure after successfull connection".blue());
     }
    
     let data = Data {
          modify_config, 
          auto, 
          config
     };
    
     println!("{}", "Connecting to Discord...".blue());
     discord_main(&discord_token, data);
    
     Ok(())
}

fn verify_token(token: &str) -> bool {
     let token_regex = Regex::new(r"[\w-]{26}\.[\w-]{6}\.[\w-]{38}|[\w-]{24}\.[\w-]{6}\.[\w-]{38}").unwrap();
     token_regex.is_match(token)
}

fn verify_channel_id(id: &str) -> bool{
     let id_regex: Regex = Regex::new(r"[\d]{19}").unwrap();
     id_regex.is_match(id)
}