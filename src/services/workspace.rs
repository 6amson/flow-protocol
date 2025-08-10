use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use libp2p::{identity::Keypair};
use crate::utils::errors::FlowError;

/// Create a workspace directory if it doesn't exist
pub fn create_workspace(base_dir: &Path, username: &str) -> io::Result<PathBuf> {
    let path = base_dir.join(format!("flow_{}_workspace", username));
    fs::create_dir_all(&path)?;
    Ok(path)
}

/// Loads a signing key from a given file path
pub fn load_libp2p_keypair(path: &Path) -> Result<Keypair, FlowError> {
    let bytes = fs::read(path)?;
    Keypair::from_protobuf_encoding(&bytes)
        .map_err(|e| FlowError::Ssid(e.to_string()))
}

/// Signs a message without storing keys in the struct
fn sign_message(message: &[u8], keypair_path: &Path) -> Result<Vec<u8>, FlowError> {
    let keypair = load_libp2p_keypair(keypair_path)?;
    keypair.sign(message)
        .map_err(|e| FlowError::Ssid(e.to_string()))
}
