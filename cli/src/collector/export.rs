use crate::common::path::{ContentType, QrFileName, QrPath};
use crate::common::types::MetaVersion;
use crate::export::{ExportChainSpec, ExportData, MetadataQr, QrCode, ReactAssetPath};
use crate::fetch::{fetch_deployed_data, RpcFetcher};
use crate::qrs::{collect_metadata_qrs, metadata_files, spec_files};
use crate::AppConfig;
use anyhow::{Context, Result};
use indexmap::IndexMap;
use log::{info, warn};

pub(crate) fn export_specs(config: &AppConfig, fetcher: RpcFetcher) -> Result<ExportData> {
    let all_specs = spec_files(&config.qr_dir)?;
    let all_metadata = metadata_files(&config.qr_dir)?;
    let online = fetch_deployed_data(config).ok();

    let mut export_specs = IndexMap::new();
    for chain in &config.chains {
        info!("Collecting {} info...", chain.name);
        let specs = match fetcher.fetch_specs(chain) {
            Ok(specs) => specs,
            Err(e) => {
                if let Some(online_specs) = online.as_ref() {
                    if let Some(online_chain_specs) = online_specs.get(&chain.portal_id()) {
                        warn!(
                            "Unable to fetch specs for {}. Keep current online specs. Err: {}.",
                            chain.name, e
                        );
                        export_specs.insert(chain.portal_id(), online_chain_specs.clone());
                        continue;
                    }
                }
                return Err(e);
            }
        };
        let meta = fetcher.fetch_metadata(chain)?;
        let live_meta_version = meta.meta_values.version;

        let metadata_qrs =
            collect_metadata_qrs(&all_metadata, &chain.portal_id(), &live_meta_version)?;

        let specs_qr = all_specs
            .get(&chain.portal_id().replace(" ", "_"))
            .with_context(|| format!("No specs qr found for {}", chain.portal_id()))?
            .clone();

        let latest_metadata_path = QrPath {
            dir: config.qr_dir.clone(),
            file_name: QrFileName::new(
                &chain.portal_id().to_lowercase().replace(" ", "_"),
                ContentType::Metadata(meta.meta_values.version),
            ),
        };

        export_specs.insert(
            chain.portal_id(),
            ExportChainSpec {
                title: chain.name.clone(),
                color: chain.color.clone(),
                rpc_endpoint: chain.rpc_endpoints[0].clone(), // keep only the first one
                genesis_hash: format!("0x{}", hex::encode(specs.genesis_hash)),
                unit: specs.unit,
                icon: chain.icon.clone(),
                decimals: specs.decimals,
                base58prefix: specs.base58prefix,
                specs_qr: QrCode::from_qr_path(config, specs_qr, &chain.verifier)?,
                latest_metadata: ReactAssetPath::from_fs_path(
                    &latest_metadata_path.to_path_buf(),
                    &config.public_dir,
                )?,
                metadata_qr: export_live_metadata(
                    config,
                    metadata_qrs,
                    &live_meta_version,
                    &chain.verifier,
                ),
                live_meta_version,
                testnet: chain.testnet.unwrap_or(false),
            },
        );
    }
    Ok(export_specs)
}

fn export_live_metadata(
    config: &AppConfig,
    qrs: Vec<QrPath>,
    live_version: &MetaVersion,
    verifier_name: &String,
) -> Option<MetadataQr> {
    qrs.into_iter()
        .find(
            |qr| matches!(qr.file_name.content_type, ContentType::Metadata(v) if v==*live_version),
        )
        .map(|qr| MetadataQr {
            version: *live_version,
            file: QrCode::from_qr_path(config, qr, verifier_name).unwrap(),
        })
}
