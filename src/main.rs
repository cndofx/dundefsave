use std::{
    io::{Read, Write},
    path::Path,
};

use clap::Parser;
use deku::DekuContainerRead;
use dundefsave::{crc, parser::CompressedSave};

mod cli;

fn main() -> eyre::Result<()> {
    let args = cli::Cli::parse();

    match &args.command {
        cli::Commands::Compress(args) => compress(&args.input, &args.output)?,
        cli::Commands::Decompress(args) => decompress(&args.input, &args.output)?,
    }

    Ok(())
}

fn compress(input: &Path, output: &Path) -> eyre::Result<()> {
    todo!()
}

fn decompress(input: &Path, output: &Path) -> eyre::Result<()> {
    let data = std::fs::read(input)?;
    let (_, compressed_save) = CompressedSave::from_bytes((&data, 0))?;
    let mut decompressed = Vec::with_capacity(compressed_save.decompressed_size as usize);
    compress::zlib::Decoder::new(&*compressed_save.data).read_to_end(&mut decompressed)?;
    let mut file = std::fs::File::create(output)?;
    file.write_all(&decompressed)?;
    Ok(())
}
