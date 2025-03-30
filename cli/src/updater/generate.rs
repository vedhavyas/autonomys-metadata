use anyhow::{anyhow, bail};
use definitions::crypto::Encryption;
use definitions::metadata::MetaValues;
use definitions::network_specs::NetworkSpecs;
use definitions::qr_transfers::{ContentAddSpecs, ContentLoadMeta};
use generate_message::full_run;
use generate_message::parser::{
    Command as SignerCommand, Goal, Make, Msg, Signature, Sufficient, Verifier,
};
use log::info;
use sp_core::{ecdsa, sr25519, Pair, H256};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::common::path::{ContentType, QrFileName};

pub(crate) fn generate_metadata_qr(
    meta_values: &MetaValues,
    genesis_hash: &H256,
    target_dir: &Path,
    signing_key: String,
    encryption: &Encryption,
    portal_id: &str,
) -> anyhow::Result<PathBuf> {
    let content = ContentLoadMeta::generate(&meta_values.meta, genesis_hash);

    let file_name = QrFileName::new(
        &portal_id.to_lowercase(),
        ContentType::Metadata(meta_values.version),
    )
    .to_string();
    let path = target_dir.join(&file_name);
    info!("⚙️  Generating {}...", file_name);
    generate_qr(
        content.to_sign().as_slice(),
        &path,
        Msg::LoadMetadata,
        signing_key,
        encryption,
    )?;
    Ok(path)
}

pub(crate) fn generate_spec_qr(
    specs: &NetworkSpecs,
    target_dir: &Path,
    portal_id: &str,
    signing_key: String,
    encryption: &Encryption,
) -> anyhow::Result<PathBuf> {
    let file_name = QrFileName::new(&portal_id.to_lowercase(), ContentType::Specs).to_string();
    let path = target_dir.join(&file_name);
    let content = ContentAddSpecs::generate(specs);

    info!("⚙️  Generating {}...", file_name);
    generate_qr(
        content.to_sign().as_slice(),
        &path,
        Msg::AddSpecs,
        signing_key,
        encryption,
    )?;
    Ok(path)
}

fn generate_qr<P>(
    content: &[u8],
    target_path: P,
    msg_type: Msg,
    signing_key: String,
    encryption: &Encryption,
) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    let tmp_dir = tempfile::tempdir()?;
    let tmp_f_path = tmp_dir.path().join("content");
    let mut content_file = File::create(&tmp_f_path)?;
    content_file.write_all(content)?;

    let files_dir = target_path.as_ref().parent().unwrap().to_path_buf();

    let signature_params = match encryption {
        Encryption::Sr25519 => SR25519Sign {
            private_key: signing_key,
        }
        .sign(content)?,
        Encryption::Ethereum | Encryption::Ecdsa => EthereumSign {
            private_key: signing_key,
        }
        .sign(content)?,
        _ => bail!("Unsupported signature. Only SR25519, Ethereum, ECDSA are supported"),
    };
    let make = Make {
        goal: Goal::Qr,
        verifier: Verifier {
            verifier_alice: None,
            verifier_hex: Some(signature_params.0),
            verifier_file: None,
        },
        signature: Signature {
            signature_hex: Some(signature_params.1),
            signature_file: None,
        },
        sufficient: Sufficient {
            sufficient_hex: None,
            sufficient_file: None,
        },
        msg: msg_type,
        name: Some(target_path.as_ref().to_owned()),
        files_dir: files_dir.clone(),
        payload: tmp_f_path,
        export_dir: files_dir,
        crypto: Some(signature_params.2),
    };
    full_run(SignerCommand::Make(make)).map_err(|e| anyhow!("{:?}", e))
}

trait Signer {
    /// Return the information needed for the signing
    /// Should return 1. public key, 2. signature, 3. encryption type
    fn sign(&self, content: &[u8]) -> anyhow::Result<(String, String, Encryption)>;
}

struct EthereumSign {
    private_key: String,
}

struct SR25519Sign {
    private_key: String,
}

impl Signer for EthereumSign {
    fn sign(&self, content: &[u8]) -> anyhow::Result<(String, String, Encryption)> {
        let key_pair = match ecdsa::Pair::from_string(self.private_key.as_str(), None) {
            Ok(x) => x,
            Err(_e) => {
                bail!("❌ Key error. Generate metadata with `make updater` and sign manually");
            }
        };
        let signature = key_pair.sign(content);
        Ok((
            hex::encode(key_pair.public().0),
            hex::encode(signature.0),
            Encryption::Ethereum,
        ))
    }
}

impl Signer for SR25519Sign {
    fn sign(&self, content: &[u8]) -> anyhow::Result<(String, String, Encryption)> {
        let key_pair = match sr25519::Pair::from_string(self.private_key.as_str(), None) {
            Ok(x) => x,
            Err(_e) => {
                bail!("❌ Key error. Generate metadata with `make updater` and sign manually")
            }
        };
        let signature = key_pair.sign(content);
        Ok((
            hex::encode(key_pair.public().0),
            hex::encode(signature.0),
            Encryption::Sr25519,
        ))
    }
}
