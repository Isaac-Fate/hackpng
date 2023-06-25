use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::str::FromStr;
use clap::Parser;
use hackpng::Result;
use hackpng::cli::{Cli, Commands};
use hackpng::png::Png;
use hackpng::chunk::Chunk;
use hackpng::chunk_type::ChunkType;

fn main() -> Result<()> {

    let cli = Cli::parse();

    match &cli.command {
        Some(command) => match command {
            Commands::Encode { 
                    input_png_filepath,
                    chunk_type, 
                    message,
                    output_png_filepath
                } => {
                encode(input_png_filepath, chunk_type, message, output_png_filepath)?;
            },

            Commands::Decode { png_filepath, chunk_type } => {

                // get the decoded message
                let message = decode(png_filepath, chunk_type)?;

                // print the message
                if let Some(message) = message {
                    println!("The secret message is: {:?}", message);
                }

            }
        },
        None => {}
    }
    
    Ok(())
}

fn encode(
        input_png_filepath: &PathBuf, 
        chunk_type: &str, 
        message: &str,
        output_png_filepath: &Option<PathBuf>
    ) -> Result<()> {

    // read the PNG file
    let mut f = File::open(input_png_filepath)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    // create a Png object
    let mut png = Png::try_from(buffer.as_slice())?;

    // create the chunk from the given chunk type and message
    let chunk = Chunk::new(
        ChunkType::from_str(chunk_type)?,
        message.as_bytes().try_into()?
    );

    // encode the message into PNG
    png.append_chunk(chunk);

    // decide the output file path
    let output_png_filepath = if let Some(output_png_filepath) = output_png_filepath {
        output_png_filepath
    } else {
        input_png_filepath
    };

    // wirte file
    let mut f = File::create(output_png_filepath)?;
    f.write(png.as_bytes().as_slice())?;

    Ok(())
}

fn decode(
        png_filepath: &PathBuf, 
        chunk_type: &str
    ) -> Result<Option<String>> {

    // read the PNG file
    let mut f = File::open(png_filepath)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    // create a Png object
    let png = Png::try_from(buffer.as_slice())?;

    // find the chunk containing the message
    let chunk = png.chunk_by_type(chunk_type);
    
    match chunk {
        Some(chunk) => Ok(Some(chunk.data_as_string()?)),
        None => Ok(None)
    }
}
