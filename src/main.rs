
use clap::Parser;
use colored::Colorize;
use hackpng::Result;
use hackpng::cli::Cli;


fn main() -> Result<()> {

    let cli = Cli::parse();

    match cli.command {
        Some(command) => {
            command.run().unwrap_or_else(
                |e| 
                eprintln!("{}", e.to_string().bright_red())
            );
        }
        None => {}
    }
    
    Ok(())
}

