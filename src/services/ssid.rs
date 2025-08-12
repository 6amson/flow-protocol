use crate::utils::errors::FlowError;
use libp2p::{PeerId, identity::Keypair};
use std::{
    fs::{self, File},
    io::Write,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

/// Generates a libp2p Ed25519 keypair, saves it to `path`,
/// and returns the PeerId as a string.
pub fn generate_ed25519_and_save(path: &PathBuf) -> Result<String, FlowError> {
    let kp = Keypair::generate_ed25519();
    let peer_id = PeerId::from(kp.public()).to_string();

    let priv_bytes = kp
        .to_protobuf_encoding()
        .map_err(|e| FlowError::EncryptionError(e.to_string()))?;

    fs::create_dir_all(path.parent().unwrap_or_else(|| Path::new(".")))
        .map_err(|e| FlowError::FileError(e.to_string()))?;

    let mut file = File::create(path).map_err(|e| FlowError::FileError(e.to_string()))?;
    file.write_all(&priv_bytes)
        .map_err(|e| FlowError::FileError(e.to_string()))?;
    file.flush()
        .map_err(|e| FlowError::FileError(e.to_string()))?;

    // secure file to: rw---------- for user only, read and write.
    let mut perms = file.metadata()?.permissions();
    perms.set_mode(0o600);
    fs::set_permissions(path, perms).map_err(|ee| FlowError::PermissionError(ee.to_string()))?;

    Ok(peer_id)
}

/// Loads a signing key from a given file path
fn load_libp2p_keypair(path: &Path) -> Result<Keypair, FlowError> {
    let bytes = fs::read(path).map_err(|e| FlowError::FileError(e.to_string()))?;
    let keypair = Keypair::from_protobuf_encoding(&bytes)
        .map_err(|e| FlowError::EncryptionError(e.to_string()))?;
    Ok(keypair)
}

/// Signs a message without storing keys in the struct
fn sign_message(message: &[u8], keypair_path: &Path) -> Result<Vec<u8>, FlowError> {
    let keypair = load_libp2p_keypair(keypair_path)?;
    let signature = keypair
        .sign(message)
        .map_err(|e| FlowError::EncryptionError(e.to_string()))?;

    // we can do lot of work from here, sync, notification, gossip layer can derive their events from here etc.
    Ok(signature)
}

pub fn get_peer_id(keypair_path: &Path) -> Result<PeerId, FlowError> {
    let keypair = load_libp2p_keypair(keypair_path)
        .map_err(|e| FlowError::EncryptionError((e.to_string())))?;
    Ok(PeerId::from(keypair.public()))
}
