use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Encode a new chunk container with a specified message
    Encode {
        filepath: PathBuf,
        chunk_code: String,
        message: String,
    },

    /// Decode a hidden message inside an existing chunk
    Decode {
        filepath: PathBuf,
        chunk_code: String,
    },

    /// Remove a chunk (container) for a hidden message
    Remove {
        filepath: PathBuf,
        chunk_code: String,
    },

    /// Print all messages contained in a PNG file
    Print {filepath: PathBuf}
}