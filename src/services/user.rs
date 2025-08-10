use crate::utils::errors::FlowError;
use crate::utils::types::{UserProfile, Permission};
// use crate::services::workspace::(create_workspace, load_libp2p_keypair);
use crate::services::workspace::create_workspace;
// use crate::services::workspace::
use crate::workspace::load_libp2p_keypair;

// use libp2p::PeerId;
use libp2p::identity::{Keypair, PeerId};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;

use crate::services::ssid::generate_ed25519_and_save;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub profile: Option<UserProfile>,
    pub permissions: Vec<Permission>,
    pub workspace_path: PathBuf,
}

impl User {
    /// Create a new user and initialize their workspace
pub fn new(username: &str, base_dir: &Path) -> Result<Self, FlowError> {
        let workspace_path = create_workspace(base_dir, username)?;
        
        let _peer_id_string = generate_ed25519_and_save(&workspace_path.join("keypair"))?;

        Ok(Self {
            username: username.to_string(),
            profile: None,
            permissions: Vec::new(),
            workspace_path,
        })
    }

    pub fn get_peer_id(&self) -> Result<PeerId, FlowError> {
        let keypair_path = self.workspace_path.join("keypair");
        let keypair = load_libp2p_keypair(&keypair_path)?;
        Ok(PeerId::from(keypair.public()))
    }
}
