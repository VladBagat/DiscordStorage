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
   git clone <repository_url>
   cd <repository_directory>
   ```
  
   
2. **Compile and run the application:**
   ```sh
   cargo check
   cargo build
   cargo run
   ```

---

## Warning 

This software may violate Discord's Terms of Service if used extensively. Its primary purpose is to demonstrate Discord's capabilities as a file storage medium, not make it a permanent solution for large files. Please use it responsibly.
