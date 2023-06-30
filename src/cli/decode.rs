use std::{
    io::{Read, Write},
    fs::File,
    path::PathBuf
};

use colored::Colorize;

use crate::{
    Result,
    png::Png
};

#[derive(Debug, clap::Args)]
pub struct DecodeArgs {

    /// PNG file containing the message
    #[arg(value_name = "PNG")]
    png_filepath: PathBuf,

    /// Chunk type corresponding to the messsage chunk
    chunk_type: String,

    /// If set, the decoded message will be written into this file
    #[arg(short = 'o', long = "out", value_name = "OUTPUT_FILE")]
    output_filepath: Option<PathBuf>

}

pub fn decode(args: DecodeArgs) -> Result<()> {

    // read the PNG file
    let mut f = File::open(args.png_filepath)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    // create a Png object
    let png = Png::try_from(buffer.as_slice())?;

    // find the chunk containing the message
    let chunk = png.chunk_by_type(&args.chunk_type);

    // extract the embedded message in the chunk
    if let Some(chunk) = chunk {

        if let Some(output_filepath) = args.output_filepath {

            File::create(output_filepath)?
                .write(chunk.data())?;

        } else {

            println!("{}", chunk.data_as_string()?);

        }
        
    } else {
        println!("{}", "No message is found".bright_yellow())
    }

    Ok(())
}
