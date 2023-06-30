use std::{
    io::{Read, Write},
    fs::File, 
    path::PathBuf,
    str::FromStr, 
    fmt::Display
};

use crate::{
    Result,
    png::Png,
    chunk::Chunk,
    chunk_type::ChunkType
};

#[derive(Debug, clap::Args)]
pub struct EncodeArgs {

    /// PNG file where the message is to encode
    #[arg(value_name = "PNG")]
    input_png_filepath: PathBuf,

    /// Chunk type corresponding to the messsage chunk
    chunk_type: String,

    /// Message to encode
    #[arg(short, long = "msg")]
    message: Option<String>,

    /// If set, the message will be read from the file,
    /// and the --message option will be ignored
    #[arg(short = 'f', long = "msg-file", value_name = "MESSAGE_FILE")]
    message_filepath: Option<PathBuf>,

    /// Index of the message chunk to insert.
    #[arg(short = 'i', long = "index")]
    chunk_index: Option<usize>,

    /// If set, the PNG with encoded message will be saved in this file path
    #[arg(short, long = "out", value_name = "OUTPUT_FILE")]
    output_png_filepath: Option<PathBuf>

}

pub fn encode(args: EncodeArgs) -> Result<()> {

    // read the PNG file
    let mut f = File::open(&args.input_png_filepath)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    // create a Png object
    let mut png = Png::try_from(buffer.as_slice())?;

    // get message bytes
    let message_bytes: Vec<u8> = if let Some(message) = args.message {

        // the message is simply a string
        message.as_bytes().try_into()?

    } else if let Some(message_filepath) = args.message_filepath {

        // read message from file
        let mut bytes: Vec<u8> = vec![];
        File::open(message_filepath)?.read_to_end(&mut bytes)?;
        bytes

    } else {

        // no message is given
        return Err(Box::new(EncodeError::MissingMessage));

    };

    // create the chunk from the given chunk type and message
    let chunk = Chunk::new(
        ChunkType::from_str(&args.chunk_type)?,
        message_bytes
    );

    // encode the message into PNG
    match args.chunk_index {
        Some(index) => {
            png.insert_chunk(index, chunk)
        },
        None => {
            png.append_chunk(chunk);
        }
    }

    // decide the output file path
    let output_png_filepath = if let Some(output_png_filepath) = args.output_png_filepath {
        output_png_filepath
    } else {
        args.input_png_filepath
    };

    // wirte file
    let mut f = File::create(output_png_filepath)?;
    f.write(png.as_bytes().as_slice())?;

    Ok(())
}

#[derive(Debug)]
pub enum EncodeError {
    MissingMessage
}

impl std::error::Error for EncodeError {}

impl Display for EncodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingMessage => {
                write!(f, "{}", "Missing Message Error: one of --msg and --msg-file must be set")
            }
        }
    }
}