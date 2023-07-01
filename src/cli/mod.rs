mod subcommands;
mod encode;
mod decode;

use crate::Result;
use encode::encode;
use decode::decode;
use subcommands::Command;

#[derive(clap::Parser)]
#[command(author, version, about)]
pub struct Cli {

    #[command(subcommand)]
    pub command: Option<Command>

}

impl Cli {
    pub fn run(self) -> Result<()> {
        if let Some(command) = self.command {
            match command {
                Command::Encode(args) => {
                    encode(args)
                },
                Command::Decode(args) => {
                    decode(args)
                }
            }
        } else {
            Ok(())
        }
    }
}