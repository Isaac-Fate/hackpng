mod subcommands;
mod encode;
mod decode;

pub use subcommands::Command;

#[derive(clap::Parser)]
#[command(author, version, about)]
pub struct Cli {

    #[command(subcommand)]
    pub command: Option<Command>

}