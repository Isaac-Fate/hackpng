use crate::Result;

use super::{
    encode::{EncodeArgs, encode},
    decode::{DecodeArgs, decode}
};

#[derive(clap::Subcommand)]
pub enum Command {

    /// Encodes a message into a PNG file
    Encode(EncodeArgs),

    /// Decodes the message from the PNG file
    Decode(DecodeArgs)

}

impl Command {
    
    pub fn run(self) -> Result<()> {
        
        match self {
            Self::Encode(args) => {
                encode(args)?;
            },
            Self::Decode(args) => {
                decode(args)?;
            }
        }

        Ok(())
    }

}
