use std::{path::Path, fs::File};

use clap::{Args, Parser, Subcommand};

use crate::png::chunk_type::ChunkType;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct StgArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Encode hidden message
    Encode(EncodeArgs),
    /// Decode hidden message
    Decode(DecodeArgs),
    /// Remove message
    Remode(RemoveArgs),
    /// Print file path
    Print(PrintArgs),
}

#[derive(Debug, Args)]
pub struct EncodeArgs {
    pub filepath: String,
    pub chunk_type: String,
    pub message: String,
    pub output_file: Option<String>,
}

#[derive(Debug, Args)]
pub struct DecodeArgs {
    pub filepath: String,
    pub chunk_type: String,
}

#[derive(Debug, Args)]
pub struct RemoveArgs {
    pub filepath: String,
    pub chunk_type: String,
}

#[derive(Debug, Args)]
pub struct PrintArgs {
    pub filepath: String,
}

