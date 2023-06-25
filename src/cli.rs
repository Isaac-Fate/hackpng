use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {

    #[command(subcommand)]
    pub command: Option<Commands>
}

#[derive(Subcommand)]
pub enum Commands {

    /// Encodes a message into a PNG file
    Encode {

        /// The PNG file where the message will be encoded
        #[arg(value_name = "PNG")]
        input_png_filepath: PathBuf,

        /// Chunk type
        chunk_type: String,

        /// The message to encode
        message: String,

        /// If set, the PNG with encoded message will be saved to this file 
        /// and the orinial PNG file will not be changed
        #[arg(short, long = "out", value_name = "OUTPUT_PNG")]
        output_png_filepath: Option<PathBuf>,

    },

    /// Decodes the message from the PNG file
    Decode {

        /// The PNG file where the message is encoded
        #[arg(value_name = "PNG")]
        png_filepath: PathBuf,

        /// Chunk type
        chunk_type: String,

    }
}