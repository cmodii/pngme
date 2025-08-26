#![allow(dead_code)]
use args::{Args, Commands};
use clap::Parser;

mod commands;
mod chunk;
mod chunk_type;
mod args;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = Args::parse();

    match args.cmd {
        Commands::Encode { 
            filepath, 
            chunk_code, 
            message,
        } => {
            match commands::encode(&filepath, &chunk_code, message) {
                Ok(_) => {
                    println!("Successfully encoded message with code {0} inside {1}", chunk_code, filepath.into_os_string().into_string().unwrap());
                    Ok(())
                },
                Err(err) => {
                    eprintln!("{}", err);
                    Err(err)
            }   }
        },
        Commands::Decode {filepath, chunk_code} => {
            match commands::decode(&filepath, &chunk_code) {
                Ok(message) => {
                    println!("Message hidden within chunk \"{0}\" -> {1}", chunk_code, message);
                    Ok(())
                },
                Err(err) => Err(err)
            }
        },
        Commands::Remove {filepath, chunk_code} => {
            match commands::remove(&filepath, &chunk_code) {
                Ok(chunk) => {
                    println!("Removed chunk container (code: {})", str::from_utf8(&chunk.chunk_type().bytes()).unwrap());
                    Ok(())
                }
                Err(err) => Err(err)
            }
        },
        Commands::Print { filepath } => {
            commands::print(&filepath)
        },
    }
}
