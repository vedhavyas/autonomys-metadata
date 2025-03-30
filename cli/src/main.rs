extern crate core;

mod cleaner;
mod collector;
mod common;
mod config;
mod export;
mod fetch;
mod file;
mod opts;
mod qrs;
mod source;
mod updater;

use crate::cleaner::clean;
use crate::collector::collect;
use crate::config::AppConfig;
use crate::fetch::RpcFetcher;
use crate::opts::{Opts, SubCommand};
use crate::updater::update_metadata;
use clap::Parser;
use env_logger::Env;
use log::error;
use std::process::exit;

/// Main entry point of the `metadata-cli`
fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format_target(false)
        .init();

    let opts: Opts = Opts::parse();
    let config = match AppConfig::load(&opts.config) {
        Ok(config) => config,
        Err(err) => {
            error!("{}", err);
            exit(1);
        }
    };

    let result = match opts.subcmd {
        SubCommand::Clean => clean(config),
        SubCommand::Collect => collect(config),
        SubCommand::Update(update_opts) => {
            update_metadata(config, update_opts.signing_key, RpcFetcher)
        }
    };

    if let Err(err) = result {
        error!("{}", err);
        exit(1);
    }
}
