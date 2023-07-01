use super::{
    encode::EncodeArgs,
    decode::DecodeArgs
};

#[derive(clap::Subcommand)]
pub enum Command {

    /// Encodes a message into a PNG file
    Encode(EncodeArgs),

    /// Decodes the message from the PNG file
    Decode(DecodeArgs)

}

