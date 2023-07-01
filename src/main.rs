
use clap::Parser;
use colored::Colorize;
use hackpng::Result;
use hackpng::cli::Cli;

fn main() -> Result<()> {

    let cli = Cli::parse();

    cli.run().unwrap_or_else(
        |e| 
        eprintln!("{}", e.to_string().bright_red())
    );
    
    Ok(())
}

