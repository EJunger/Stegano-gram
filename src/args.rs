use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct StgArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType
}

#[derive(Debug, Subcommand)]
pub enum EnitiyType {
    /// Encode hidden message
    Encode(EncodeCommand),
    /// Decode hidden message
    Decode(DecodeCommand),
    /// Remove message
    Remode(RemoveCommand),
    /// Print file path
    Print(PrintCommand),
}


#[derive(Debug, Args)]
pub struct EncodeCommand {
    #[clap(subcommand)]
    pub cmd: EncodeSubcommand,
}

pub struct DecodeCommand {}

pub struct RemoveCommand {}

pub struct PrintCommand {}
