use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, time::SystemTime};

use crate::services::workspace::ContentType;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub resource_id: String,
    pub allowed_actions: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ContentItem {
    pub id: String,
    pub file_path: PathBuf,
    pub content_type: ContentType,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    pub owner: PeerId,
    pub size: u64,
}

#[derive(Debug, Clone)]
pub struct SignatureResult {
    pub signature: Vec<u8>,
    pub signature_path: PathBuf,
}
