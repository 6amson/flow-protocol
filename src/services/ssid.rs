use libp2p::{identity::Keypair, PeerId};
use std::{fs, path::Path};
use crate::utils::errors::FlowError;

/// Generates a libp2p Ed25519 keypair, saves it to `path`,
/// and returns the PeerId as a string.
pub fn generate_ed25519_and_save(path: &Path) -> Result<String, FlowError> {
    let kp = Keypair::generate_ed25519();
    let peer_id = PeerId::from(kp.public()).to_string();

    let priv_bytes = kp
        .to_protobuf_encoding()
        .map_err(|e| FlowError::Ssid(e.to_string()))?;

    fs::create_dir_all(path.parent().unwrap_or_else(|| Path::new(".")))?;
    fs::write(path, priv_bytes)?;

    Ok(peer_id)
}
