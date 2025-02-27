# DiscordStore

**DiscordStore** is a Rust application that demonstrates how to use Discord as an unlimited file storage. It takes an arbitrary directory, splits it into 8 MB chunks, and uploads those chunks to Discord.

Created with inspiration from this [YouTube video](https://www.youtube.com/watch?v=c_arQ-6ElYI).

---

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (including Cargo and rustc)

---

## Installation and Usage

Since binaries are not yet provided, you must compile the application yourself:

1. **Clone the repository:**
   ```sh
   git clone https://github.com/VladBagat/DiscordStorage.git
   cd DiscordStorage
   ```
  
2. **Compile and run the application:**
   ```sh
   cargo check
   cargo build --release 
   cargo run
   ```
---
1. You will need your own Discord token for app to work. For that you will need to create an app/bot. You can see the [official guide](https://discord.com/developers/docs/quick-start/getting-started).
2. You need a discord server with your bot there. Use this [link generator](https://discordapi.com/permissions.html). I suggest enabling `Administrator` permissions (8), but enabling everything related to reading/sending messages should be enough. 
3. Find the binary `discordstore.exe` in the directory `/target/release`.
4. Place binary in any folder you have read/write access to and run it. Follow the instructions until the bot will be set up.
5. In a channel send message `!upload` to upload selected directory and command `!download` to download uploaded content to your machine. Upload/download directories will be in the same place where your binary is. Mind that those directories don't delete, so sending a file requires equal amount of space to copy contents into upload folder. (So when sending a 2GB file, ensure additional 2GB are available on your disk).
6. Create new channel for new upload, as proram reads **all** messages with attachements in a channel.

## Warning 

This software may violate Discord's Terms of Service if used extensively. Its primary purpose is to demonstrate Discord's capabilities as a file storage medium, not make it a permanent solution for large files. Please use it responsibly.
