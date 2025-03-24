use std::{thread, time};

use std::fs;
use std::path::Path;

use anyhow::{anyhow, bail, Result};
use definitions::network_specs::NetworkSpecs;
use generate_message::helpers::{meta_fetch, specs_agnostic, MetaFetched};
use generate_message::parser::Token;
use log::warn;
use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::common::types::get_crypto;
use crate::config::{AppConfig, Chain};
use crate::export::{ExportData, ReactAssetPath};

pub(crate) trait Fetcher {
    fn fetch_specs(&self, chain: &Chain) -> Result<NetworkSpecs>;
    fn fetch_metadata(&self, chain: &Chain) -> Result<MetaFetched>;
}

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

impl Fetcher for RpcFetcher {
    fn fetch_specs(&self, chain: &Chain) -> Result<NetworkSpecs> {
        let specs = call_urls(&chain.rpc_endpoints, |url| {
            let optional_token_override = chain.token_decimals.zip(chain.token_unit.as_ref()).map(
                |(token_decimals, token_unit)| Token {
                    decimals: token_decimals,
                    unit: token_unit.to_string(),
                },
            );

            specs_agnostic(url, get_crypto(chain), optional_token_override, None)
        })
        .map_err(|e| anyhow!("{:?}", e))?;
        Ok(specs)
    }

    fn fetch_metadata(&self, chain: &Chain) -> Result<MetaFetched> {
        let meta = call_urls(&chain.rpc_endpoints, meta_fetch).map_err(|e| anyhow!("{:?}", e))?;
        Ok(meta)
    }
}

pub(crate) struct ConfigRpcFetcher;

impl Fetcher for ConfigRpcFetcher {
    fn fetch_specs(&self, chain: &Chain) -> Result<NetworkSpecs> {
        let specs = call_urls(&chain.rpc_endpoints, |url| {
            let optional_token_override = chain.token_decimals.zip(chain.token_unit.as_ref()).map(
                |(token_decimals, token_unit)| Token {
                    decimals: token_decimals,
                    unit: token_unit.to_string(),
                },
            );

            specs_agnostic(url, get_crypto(chain), optional_token_override, None)
        })
        .map_err(|e| anyhow!("{:?}", e))?;
        Ok(specs)
    }

    fn fetch_metadata(&self, _chain: &Chain) -> Result<MetaFetched> {
        bail!("Not implemented!");
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
