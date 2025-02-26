#![allow(dead_code)]
#![allow(unused_imports)]


mod discord;
use discord::discord as discord_main;
use text_io::read;


fn main() {
     
     let token = start();
     discord_main(&token);
     
    
}

fn start() -> String {
     println!("Welcome to the Discord Store. Complete the following steps to get started.");
     print!("1. Enter a valid Discord Token: ");
     let token: String = read!("{}\n");
     token
}