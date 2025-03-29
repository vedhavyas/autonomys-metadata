use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
pub(crate) struct Opts {
    /// Path to config file
    #[arg(short, long, default_value = "config.toml")]
    pub(crate) config: PathBuf,

    #[clap(subcommand)]
    pub(crate) subcmd: SubCommand,
}

/// You can find all available commands below.
#[derive(Subcommand)]
pub(crate) enum SubCommand {
    /// Remove unused QR codes
    Clean,

    /// Generate json data file for frontend
    Collect,

    /// Check updates
    Update(UpdateOpts),
}

#[derive(Parser)]
pub(crate) struct UpdateOpts {
    #[clap(long, default_value = "")]
    pub(crate) signing_key: String,
}

#[derive(Parser)]
pub(crate) struct ChainsOpts {
    #[clap(long, default_value = "prod")]
    pub(crate) env: String,

    #[clap(long, default_value = "v5")]
    pub(crate) version: String,
}
