mod generate;

use crate::common::types::get_crypto;
use crate::config::AppConfig;
use crate::fetch::RpcFetcher;
use crate::qrs::{metadata_files, spec_files};
use crate::source::{save_source_info, Source};
use crate::updater::generate::{generate_metadata_qr, generate_spec_qr};
use log::{error, info, warn};
use std::process::exit;

pub(crate) fn update_metadata(
    config: AppConfig,
    signing_key: String,
    fetcher: RpcFetcher,
) -> anyhow::Result<()> {
    let metadata_qrs = metadata_files(&config.qr_dir)?;
    let specs_qrs = spec_files(&config.qr_dir)?;
    let mut is_changed = false;
    let mut error_fetching_data = false;
    for chain in config.chains {
        let encryption = get_crypto(&chain);
        if !specs_qrs.contains_key(&chain.portal_id()) {
            let specs_res = match fetcher.fetch_specs(&chain) {
                Ok(specs_res) => specs_res,
                Err(err) => {
                    error!("Can't get specs for {}. Error is {}", chain.name, err);
                    continue;
                }
            };

            generate_spec_qr(
                &specs_res,
                &config.qr_dir,
                &chain.portal_id(),
                signing_key.to_owned(),
                &encryption,
            )?;
            is_changed = true;
        }

        let fetched_meta = match fetcher.fetch_metadata(&chain) {
            Ok(fetched_meta) => fetched_meta,
            Err(err) => {
                error_fetching_data = true;
                warn!("Can't get metadata for {}. Error is {}", chain.name, err);
                continue;
            }
        };
        let version = fetched_meta.meta_values.version;
        // Skip if already have QR for the same version
        if let Some(map) = metadata_qrs.get(&chain.portal_id()) {
            if map.contains_key(&version) {
                continue;
            }
        }

        let path = generate_metadata_qr(
            &fetched_meta.meta_values,
            &fetched_meta.genesis_hash,
            &config.qr_dir,
            signing_key.to_owned(),
            &encryption,
            &chain.portal_id(),
        )?;
        let source = Source::Rpc {
            block: fetched_meta.block_hash,
        };
        save_source_info(&path, &source)?;
        is_changed = true;
    }

    if error_fetching_data {
        warn!("⚠️ Some chain data wasn't read. Please check the log!");
        exit(12);
    }

    if !is_changed {
        info!("🎉 Everything is up to date!");
    }

    Ok(())
}
