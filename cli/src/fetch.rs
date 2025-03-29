use crate::common::types::get_crypto;
use crate::config::{AppConfig, Chain};
use crate::export::{ExportData, ReactAssetPath};
use anyhow::{anyhow, bail, Result};
use definitions::metadata::runtime_metadata_from_slice;
use definitions::network_specs::NetworkSpecs;
use frame_metadata::v14::META_RESERVED;
use frame_metadata::{RuntimeMetadata, RuntimeMetadataPrefixed};
use generate_message::helpers::{meta_fetch, specs_agnostic, MetaFetched};
use generate_message::parser::Token;
use log::{info, warn};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use sp_core::{Decode, Encode};
use sp_runtime::RuntimeString;
use sp_version::RuntimeVersion;
use std::fs;
use std::path::Path;
use std::{thread, time};

// try to call all urls unless successful
fn call_urls<F, T>(urls: &[String], f: F) -> Result<T>
where
    F: Fn(&str) -> Result<T, generate_message::Error>,
{
    for url in urls.iter() {
        for i in 1..3 {
            match f(url) {
                Ok(res) => return Ok(res),
                Err(e) => warn!("Failed to fetch {}: {:?}", url, e),
            }
            let interval_seconds = time::Duration::from_secs(i);
            thread::sleep(interval_seconds);
        }
    }
    bail!("Error calling chain node");
}

pub(crate) struct RpcFetcher;

impl RpcFetcher {
    pub fn fetch_specs(&self, chain: &Chain) -> Result<NetworkSpecs> {
        let specs = call_urls(&chain.rpc_endpoints, |url| {
            let optional_token_override = chain.token_decimals.zip(chain.token_unit.as_ref()).map(
                |(token_decimals, token_unit)| Token {
                    decimals: token_decimals,
                    unit: token_unit.to_string(),
                },
            );

            let mut specs = specs_agnostic(
                url,
                get_crypto(chain),
                optional_token_override,
                Some(chain.name.clone()),
            )?;

            if chain.empty_path {
                specs.path_id = "".into();
            }

            if chain.icon != specs.logo {
                specs.logo = chain.icon.clone();
            }

            if specs.name != chain.name.clone() {
                specs.name = chain.name.clone();
                specs.title = to_title_case(chain.name.clone());
            }

            specs.color = chain.color.clone();
            Ok(specs)
        })
        .map_err(|e| anyhow!("{:?}", e))?;
        Ok(specs)
    }

    pub fn fetch_metadata(&self, chain: &Chain) -> Result<MetaFetched> {
        let mut meta =
            call_urls(&chain.rpc_endpoints, meta_fetch).map_err(|e| anyhow!("{:?}", e))?;
        if meta.meta_values.name != chain.name {
            meta.meta_values.name = chain.name.clone();
            let updated_meta =
                override_spec_name(meta.meta_values.meta.as_slice(), chain.name.clone());
            meta.meta_values.meta = updated_meta;
            Ok(meta)
        } else {
            Ok(meta)
        }
    }
}

#[derive(Serialize, Deserialize)]
struct PkgJson {
    homepage: String,
}
pub(crate) fn fetch_deployed_data(config: &AppConfig) -> Result<ExportData> {
    let pkg_json = fs::read_to_string(Path::new("package.json"))?;
    let pkg_json: PkgJson = serde_json::from_str(&pkg_json)?;

    let data_file = ReactAssetPath::from_fs_path(&config.data_file, &config.public_dir)?;
    let url = Url::parse(&pkg_json.homepage)?;
    let url = url.join(&data_file.to_string())?;

    Ok(reqwest::blocking::get(url)?.json::<ExportData>()?)
}

fn override_spec_name(meta: &[u8], spec_name: String) -> Vec<u8> {
    let runtime_metadata = runtime_metadata_from_slice(meta).unwrap();
    let runtime_metadata = match runtime_metadata {
        RuntimeMetadata::V14(mut metadata_v14) => {
            for x in metadata_v14.pallets.iter_mut() {
                if x.name == "System" {
                    for y in x.constants.iter_mut() {
                        if y.name == "Version" {
                            let mut runtime_version =
                                RuntimeVersion::decode(&mut y.value.as_slice()).unwrap();
                            if runtime_version.spec_name != RuntimeString::Owned(spec_name.clone())
                            {
                                info!(
                                    "⚙️  Overriding spec name from {} to {}",
                                    runtime_version.spec_name, spec_name
                                );
                                runtime_version.spec_name = RuntimeString::Owned(spec_name.clone());
                                runtime_version.impl_name = RuntimeString::Owned(spec_name.clone());
                            }
                            y.value = runtime_version.encode();
                            break;
                        }
                    }
                }
            }

            RuntimeMetadata::V14(metadata_v14)
        }
        _ => runtime_metadata,
    };

    RuntimeMetadataPrefixed(META_RESERVED, runtime_metadata).encode()
}

fn to_title_case(s: String) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
