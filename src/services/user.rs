use crate::utils::errors::FlowError;
use crate::utils::types::{Permission, UserProfile};
// use crate::services::workspace::(create_workspace, load_libp2p_keypair);
// use crate::services::{ssid::get_peer_id, workspace::create_workspace};// use libp2p::PeerId;
use libp2p::identity::PeerId;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::services::ssid::generate_ed25519_and_save;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub peer_id: String,
    pub profile: Option<UserProfile>,
    pub permissions: Vec<Permission>,
}

impl User {
    /// Create a new user and initialize their workspace
    pub fn new(username: &str, peer_id: &str) -> Self {
        Self {
            username: username.to_string(),
            peer_id: peer_id.to_string(),
            profile: None,
            permissions: Vec::new(),
        }
    }
}
