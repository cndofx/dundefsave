use std::{
    io::{Read, Write},
    path::Path,
};

use clap::Parser;
use compress::zlib;
use dundefsave::parser::{decompressed::GameStorage, CompressedSave};

mod cli;

fn main() -> eyre::Result<()> {
    let args = cli::Cli::parse();

    match &args.command {
        cli::Commands::Compress(args) => compress(&args.input, &args.output)?,
        cli::Commands::Decompress(args) => decompress(&args.input, &args.output)?,
        cli::Commands::ParseCompressed(args) => parse_compressed(&args.input)?,
        cli::Commands::ParseDecompressed(args) => parse_decompressed(&args.input)?,
    }

    Ok(())
}

fn compress(input: &Path, output: &Path) -> eyre::Result<()> {
    eprintln!("error: compression is not yet implemented");
    Ok(())
}

fn decompress(input: &Path, output: &Path) -> eyre::Result<()> {
    let mut file = std::fs::File::open(input)?;
    let compressed_save = CompressedSave::read(&mut file)?;
    let mut decompressed = Vec::with_capacity(compressed_save.whole_decompressed_size as usize);
    for block in compressed_save.data.iter() {
        zlib::Decoder::new(&block[..]).read_to_end(&mut decompressed)?;
    }
    let mut file = std::fs::File::create(output)?;
    file.write_all(&decompressed)?;
    Ok(())
}

fn parse_compressed(input: &Path) -> eyre::Result<()> {
    let mut file = std::fs::File::open(input)?;
    let compressed_save = CompressedSave::read(&mut file)?;
    let mut decompressed = Vec::with_capacity(compressed_save.whole_decompressed_size as usize);
    for block in compressed_save.data.iter() {
        zlib::Decoder::new(&block[..]).read_to_end(&mut decompressed)?;
    }
    let parsed = GameStorage::read(&mut decompressed.as_slice())?;
    dbg!(parsed);
    Ok(())
}

fn parse_decompressed(input: &Path) -> eyre::Result<()> {
    let mut file = std::fs::File::open(input)?;
    let parsed = GameStorage::read(&mut file)?;
    dbg!(parsed);
    Ok(())
}
