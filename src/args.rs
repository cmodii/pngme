use clap::{command, Arg, Command};

const PNG_PATH_DESC: &str = "A valid file path to a .png file";
const CHUNK_TYPE_DESC: &str = "A 4 letter string that determines the characteristics of the chunk";

pub fn init_cmd() -> Command {
    command!()
        .about("(en/de)code hidden messages inside PNG files")
        .subcommand(Command::new("encode")
            .about("Encode a new chunk container with a specified message")
            .arg(
                Arg::new("File path")
                    .aliases(["PNG path", "Image path"])
                    .help(String::from(PNG_PATH_DESC))
                    .required(true)
                )
            .arg(
                Arg::new("Chunk type")
                    .help(String::from(CHUNK_TYPE_DESC))
                    .required(true)
            )
            .arg(
                Arg::new("Message").required(true)
            )
            .arg(
                Arg::new("Output file")
                    .short('o')
                    .required(false)
            )
        )
        .subcommand(Command::new("decode")
            .about("Decode a hidden message inside an existing chunk")
            .arg(
                Arg::new("File path")
                    .aliases(["PNG path", "Image path"])
                    .help(String::from(PNG_PATH_DESC))
                    .required(true)
                )
            .arg(
                Arg::new("Chunk type")
                    .help(String::from(CHUNK_TYPE_DESC))
                    .required(true)
            )
        )
        .subcommand(Command::new("remove")
            .about("Remove a chunk (container) for a hidden message")
            .arg(
                Arg::new("File path")
                    .aliases(["PNG path", "Image path"])
                    .help(String::from(PNG_PATH_DESC))
                    .required(true)
                )
            .arg(
                Arg::new("Chunk type")
                    .help(String::from(CHUNK_TYPE_DESC))
                    .required(true)
            )
        )
        .subcommand(Command::new("print")
            .about("Print all messages contained in a PNG file")
            .arg(
                Arg::new("File path")
                    .aliases(["PNG path", "Image path"])
                    .help(String::from(PNG_PATH_DESC))
                    .required(true)
                )
    )
}