use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Compress a save file")]
    Compress(CompressArgs),
    #[command(about = "Decompress a save file")]
    Decompress(CompressArgs),
}

#[derive(Args)]
pub struct CompressArgs {
    #[arg(help = "Input filename")]
    pub input: PathBuf,
    #[arg(help = "Output filename")]
    pub output: PathBuf,
}
